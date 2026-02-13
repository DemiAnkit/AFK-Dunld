// src-tauri/src/core/chunk_manager.rs

use crate::utils::constants::*;

/// Represents a byte range for a download segment
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Chunk {
    /// Segment ID (0-indexed)
    pub id: u32,
    /// Start byte (inclusive)
    pub start: u64,
    /// End byte (inclusive)
    pub end: u64,
}

impl Chunk {
    /// Size of this chunk in bytes
    pub fn size(&self) -> u64 {
        self.end - self.start + 1
    }
}

pub struct ChunkManager;

impl ChunkManager {
    /// Split a file into chunks for multi-segment download
    ///
    /// # Arguments
    /// * `total_size` - Total file size in bytes
    /// * `requested_segments` - Desired number of segments
    ///
    /// # Returns
    /// Vector of chunks. May return fewer segments than requested
    /// if the file is too small.
    pub fn split(total_size: u64, requested_segments: u8) -> Vec<Chunk> {
        // Don't split if file is too small
        if total_size < MIN_SIZE_FOR_SEGMENTS {
            tracing::debug!(
                "File too small for segmentation ({} bytes), using 1 segment",
                total_size
            );
            return vec![Chunk {
                id: 0,
                start: 0,
                end: total_size - 1,
            }];
        }

        // Calculate actual number of segments
        // Ensure each segment is at least MIN_SEGMENT_SIZE
        let max_possible = (total_size / MIN_SEGMENT_SIZE).max(1) as u8;
        let num_segments = requested_segments
            .min(max_possible)
            .min(MAX_SEGMENTS)
            .max(1);

        let segment_size = total_size / num_segments as u64;
        let mut chunks = Vec::with_capacity(num_segments as usize);

        for i in 0..num_segments {
            let start = i as u64 * segment_size;
            let end = if i == num_segments - 1 {
                // Last segment gets all remaining bytes
                total_size - 1
            } else {
                (i as u64 + 1) * segment_size - 1
            };

            chunks.push(Chunk {
                id: i as u32,
                start,
                end,
            });
        }

        tracing::info!(
            "Split {} bytes into {} segments (avg {} bytes each)",
            total_size,
            chunks.len(),
            segment_size,
        );

        // Log each chunk for debugging
        for chunk in &chunks {
            tracing::debug!(
                "  Segment {}: bytes {}-{} ({} bytes)",
                chunk.id, chunk.start, chunk.end, chunk.size()
            );
        }

        chunks
    }

    /// Re-split chunks for resume, accounting for already downloaded bytes
    #[allow(dead_code)]
    pub fn split_for_resume(
        total_size: u64,
        segments: u8,
        downloaded_ranges: &[(u64, u64)], // (start, bytes_downloaded)
    ) -> Vec<Chunk> {
        let mut chunks = Self::split(total_size, segments);

        // Adjust chunks based on already downloaded data
        for chunk in &mut chunks {
            for &(range_start, bytes_downloaded) in downloaded_ranges {
                if range_start == chunk.start {
                    chunk.start += bytes_downloaded;
                    break;
                }
            }
        }

        // Remove completed chunks
        chunks.retain(|c| c.start <= c.end);

        chunks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_basic() {
        let chunks = ChunkManager::split(10_000_000, 4);
        assert_eq!(chunks.len(), 4);

        // Verify no gaps or overlaps
        for i in 1..chunks.len() {
            assert_eq!(chunks[i].start, chunks[i - 1].end + 1);
        }

        // Verify covers entire file
        assert_eq!(chunks[0].start, 0);
        assert_eq!(chunks.last().unwrap().end, 9_999_999);

        // Verify total size
        let total: u64 = chunks.iter().map(|c| c.size()).sum();
        assert_eq!(total, 10_000_000);
    }

    #[test]
    fn test_split_small_file() {
        // File smaller than MIN_SIZE_FOR_SEGMENTS
        let chunks = ChunkManager::split(500_000, 8);
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].start, 0);
        assert_eq!(chunks[0].end, 499_999);
    }

    #[test]
    fn test_split_odd_size() {
        let chunks = ChunkManager::split(10_000_001, 3);
        assert_eq!(chunks.len(), 3);

        let total: u64 = chunks.iter().map(|c| c.size()).sum();
        assert_eq!(total, 10_000_001);
    }

    #[test]
    fn test_segment_sizes_balanced() {
        let chunks = ChunkManager::split(100_000_000, 8);
        assert_eq!(chunks.len(), 8);

        // All segments except last should be equal
        let first_size = chunks[0].size();
        for chunk in &chunks[..chunks.len() - 1] {
            assert_eq!(chunk.size(), first_size);
        }
    }
}