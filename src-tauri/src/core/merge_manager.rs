// src-tauri/src/core/merge_manager.rs

use std::path::{Path, PathBuf};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::utils::error::DownloadError;
use crate::utils::constants::BUFFER_SIZE;

pub struct MergeManager;

impl MergeManager {
    /// Merge downloaded segment files into the final output file
    ///
    /// # Arguments
    /// * `temp_dir` - Directory containing segment files (part_0, part_1, ...)
    /// * `output_path` - Final output file path
    /// * `num_segments` - Number of segments to merge
    /// * `expected_size` - Expected total file size (optional, for verification)
    pub async fn merge(
        temp_dir: &Path,
        output_path: &Path,
        num_segments: u32,
        expected_size: Option<u64>,
    ) -> Result<u64, DownloadError> {
        tracing::info!(
            "Merging {} segments into: {}",
            num_segments,
            output_path.display()
        );

        // Create output file
        let mut output = tokio::fs::File::create(output_path)
            .await
            .map_err(|e| DownloadError::MergeFailed(
                format!("Cannot create output file: {}", e)
            ))?;

        let mut total_bytes: u64 = 0;
        let mut buffer = vec![0u8; BUFFER_SIZE];

        for i in 0..num_segments {
            let part_path = temp_dir.join(format!("part_{}", i));

            if !part_path.exists() {
                return Err(DownloadError::MergeFailed(
                    format!("Segment file missing: {}", part_path.display())
                ));
            }

            let mut part_file = tokio::fs::File::open(&part_path)
                .await
                .map_err(|e| DownloadError::MergeFailed(
                    format!("Cannot open segment {}: {}", i, e)
                ))?;

            let part_size = part_file.metadata()
                .await
                .map(|m| m.len())
                .unwrap_or(0);

            tracing::debug!(
                "Merging segment {}: {} bytes",
                i,
                part_size
            );

            // Copy segment data to output
            loop {
                let bytes_read = part_file.read(&mut buffer)
                    .await
                    .map_err(|e| DownloadError::MergeFailed(
                        format!("Read error on segment {}: {}", i, e)
                    ))?;

                if bytes_read == 0 {
                    break;
                }

                output.write_all(&buffer[..bytes_read])
                    .await
                    .map_err(|e| DownloadError::MergeFailed(
                        format!("Write error during merge: {}", e)
                    ))?;

                total_bytes += bytes_read as u64;
            }
        }

        // Flush and sync
        output.flush()
            .await
            .map_err(|e| DownloadError::MergeFailed(
                format!("Flush error: {}", e)
            ))?;
        output.sync_all()
            .await
            .map_err(|e| DownloadError::MergeFailed(
                format!("Sync error: {}", e)
            ))?;

        // Verify size if expected
        if let Some(expected) = expected_size {
            if total_bytes != expected {
                tracing::error!(
                    "Merge size mismatch: expected {} bytes, got {} bytes",
                    expected,
                    total_bytes
                );
                // Clean up the bad file
                let _ = tokio::fs::remove_file(output_path).await;
                return Err(DownloadError::MergeFailed(
                    format!(
                        "Size mismatch: expected {} bytes, got {} bytes",
                        expected, total_bytes
                    )
                ));
            }
        }

        tracing::info!(
            "Merge complete: {} bytes written to {}",
            total_bytes,
            output_path.display()
        );

        Ok(total_bytes)
    }

    /// Clean up temporary segment files
    pub async fn cleanup(temp_dir: &Path) -> Result<(), DownloadError> {
        if temp_dir.exists() {
            tracing::debug!("Cleaning up temp dir: {}", temp_dir.display());
            tokio::fs::remove_dir_all(temp_dir)
                .await
                .map_err(|e| DownloadError::FileError(
                    format!("Failed to cleanup temp dir: {}", e)
                ))?;
        }
        Ok(())
    }
}