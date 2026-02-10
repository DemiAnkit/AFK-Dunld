// src-tauri/src/core/segment_downloader.rs

use std::path::PathBuf;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio_util::sync::CancellationToken;
use futures_util::StreamExt;

use crate::core::chunk_manager::Chunk;
use crate::core::speed_limiter::SpeedLimiter;
use crate::core::retry::{RetryHandler, RetryConfig};
use crate::network::http_client::HttpClient;
use crate::utils::error::DownloadError;
use crate::utils::constants::*;

/// Result of a segment download
#[derive(Debug)]
pub struct SegmentResult {
    pub segment_id: u32,
    pub bytes_downloaded: u64,
    pub temp_path: PathBuf,
}

/// Downloads a single segment of a file
pub struct SegmentDownloader {
    http_client: HttpClient,
    speed_limiter: SpeedLimiter,
    retry_config: RetryConfig,
}

impl SegmentDownloader {
    pub fn new(
        http_client: HttpClient,
        speed_limiter: SpeedLimiter,
        retry_config: RetryConfig,
    ) -> Self {
        Self {
            http_client,
            speed_limiter,
            retry_config,
        }
    }

    /// Download a segment with retry support
    pub async fn download_segment(
        &self,
        url: &str,
        chunk: &Chunk,
        temp_path: &PathBuf,
        cancel_token: CancellationToken,
    ) -> Result<(), DownloadError> {
        let retry_handler = RetryHandler::new(self.retry_config.clone());
        let url = url.to_string();
        let chunk = chunk.clone();
        let temp_path = temp_path.clone();
        let client = self.http_client.clone();
        let limiter = self.speed_limiter.clone();
        let cancel = cancel_token.clone();

        retry_handler
            .execute(
                &format!("segment_{}", chunk.id),
                || {
                    let url = url.clone();
                    let chunk = chunk.clone();
                    let temp_path = temp_path.clone();
                    let client = client.clone();
                    let limiter = limiter.clone();
                    let cancel = cancel.clone();

                    async move {
                        Self::download_segment_inner(
                            &client,
                            &url,
                            &chunk,
                            &temp_path,
                            &limiter,
                            cancel,
                        )
                        .await
                    }
                },
            )
            .await
    }

    /// Inner download logic for a single segment
    async fn download_segment_inner(
        client: &HttpClient,
        url: &str,
        chunk: &Chunk,
        temp_path: &PathBuf,
        speed_limiter: &SpeedLimiter,
        cancel_token: CancellationToken,
    ) -> Result<(), DownloadError> {
        // Check for existing partial download (resume)
        let existing_bytes = if temp_path.exists() {
            tokio::fs::metadata(temp_path)
                .await
                .map(|m| m.len())
                .unwrap_or(0)
        } else {
            0
        };

        let actual_start = chunk.start + existing_bytes;

        // Segment already complete
        if actual_start > chunk.end {
            tracing::info!(
                "Segment {} already complete ({} bytes)",
                chunk.id,
                existing_bytes
            );
            return Ok(());
        }

        tracing::info!(
            "Downloading segment {}: bytes {}-{} (resume from {})",
            chunk.id,
            actual_start,
            chunk.end,
            existing_bytes
        );

        // Request the range
        let response = client
            .get_range(url, actual_start, chunk.end)
            .await?;

        // Open file for appending
        let mut file = tokio::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(temp_path)
            .await
            .map_err(|e| DownloadError::FileError(
                format!("Cannot open segment file: {}", e)
            ))?;

        let mut stream = response.bytes_stream();
        let mut total_written = existing_bytes;

        loop {
            tokio::select! {
                // Check for cancellation
                _ = cancel_token.cancelled() => {
                    file.flush().await
                        .map_err(|e| DownloadError::FileError(e.to_string()))?;
                    tracing::info!(
                        "Segment {} cancelled at {} bytes",
                        chunk.id,
                        total_written
                    );
                    return Err(DownloadError::Cancelled);
                }

                // Read next chunk from stream
                maybe_chunk = stream.next() => {
                    match maybe_chunk {
                        Some(Ok(data)) => {
                            // Apply speed limiting
                            speed_limiter.throttle(data.len()).await;

                            // Write to file
                            file.write_all(&data)
                                .await
                                .map_err(|e| DownloadError::FileError(
                                    format!("Write error: {}", e)
                                ))?;

                            total_written += data.len() as u64;
                        }
                        Some(Err(e)) => {
                            // Flush what we have so far (for resume)
                            let _ = file.flush().await;
                            return Err(DownloadError::NetworkError(
                                format!("Stream error on segment {}: {}", chunk.id, e)
                            ));
                        }
                        None => {
                            // Stream ended
                            break;
                        }
                    }
                }
            }
        }

        // Flush and sync
        file.flush()
            .await
            .map_err(|e| DownloadError::FileError(e.to_string()))?;
        file.sync_all()
            .await
            .map_err(|e| DownloadError::FileError(e.to_string()))?;

        tracing::info!(
            "Segment {} complete: {} bytes written",
            chunk.id,
            total_written
        );

        Ok(())
    }
}
