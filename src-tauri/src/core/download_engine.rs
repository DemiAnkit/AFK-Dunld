use futures_util::StreamExt;
use std::path::PathBuf;
use std::time::Instant;
use tokio::io::AsyncWriteExt;
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, warn};

use crate::core::checksum::{ChecksumVerifier, ChecksumAlgorithm};
use crate::core::chunk_manager::{Chunk, ChunkManager};
use crate::core::download_task::*;
use crate::core::resume_manager::{ResumeManager, ResumeData};
use crate::core::retry::{RetryHandler, RetryConfig};
use crate::core::segment_downloader::SegmentDownloader;
use crate::core::speed_limiter::SpeedLimiter;
use crate::network::http_client::HttpClient;
use crate::network::url_parser::UrlParser;
use crate::utils::constants::*;
use crate::utils::error::DownloadError;

/// Main download engine - orchestrates all download operations
pub struct DownloadEngine {
    /// HTTP client for making requests
    http_client: HttpClient,

    /// Global speed limiter
    speed_limiter: SpeedLimiter,

    /// Default download directory
    default_download_dir: PathBuf,
}

impl DownloadEngine {
    /// Create a new download engine
    pub fn new(
        proxy: Option<&crate::network::proxy_manager::ProxyConfig>,
        speed_limit: Option<u64>,
        download_dir: Option<PathBuf>,
    ) -> Result<Self, DownloadError> {
        let http_client = HttpClient::new(proxy)?;

        let speed_limiter = SpeedLimiter::new(speed_limit);

        let default_download_dir = download_dir.unwrap_or_else(
            || {
                dirs::download_dir().unwrap_or_else(|| {
                    dirs::home_dir()
                        .unwrap_or_else(|| PathBuf::from("."))
                        .join("Downloads")
                })
            },
        );

        // Ensure download directory exists
        std::fs::create_dir_all(&default_download_dir)
            .map_err(|e| {
                DownloadError::FileError(format!(
                    "Cannot create download dir: {}",
                    e
                ))
            })?;

        info!(
            "Download engine initialized. Default dir: {:?}",
            default_download_dir
        );

        Ok(Self {
            http_client,
            speed_limiter,
            default_download_dir,
        })
    }

    /// Get default download directory
    pub fn default_download_dir(&self) -> &PathBuf {
        &self.default_download_dir
    }

    /// Update global speed limit
    pub async fn set_speed_limit(&self, limit: Option<u64>) {
        self.speed_limiter.set_limit(limit).await;
        info!("Speed limit updated: {:?}", limit);
    }

    /// Fetch file information from URL
    pub async fn get_file_info(
        &self,
        url: &str,
    ) -> Result<FileInfo, DownloadError> {
        // Parse and validate URL
        let _ = UrlParser::parse(url)?;
        
        // Get file info from HTTP client
        let info = self.http_client.get_file_info(url).await?;
        
        Ok(FileInfo {
            file_name: info.file_name,
            total_size: info.total_size,
            content_type: info.content_type,
            supports_range: info.supports_range,
        })
    }

    /// Create a new download task from a request
    pub async fn create_task(
        &self,
        request: &AddDownloadRequest,
    ) -> Result<DownloadTask, DownloadError> {
        // Parse URL
        let parsed = UrlParser::parse(&request.url)?;

        // Fetch file info from server
        let file_info =
            self.http_client.get_file_info(&request.url).await?;

        // Determine save path
        let save_dir = request
            .save_path
            .as_ref()
            .map(PathBuf::from)
            .unwrap_or_else(|| {
                self.default_download_dir.clone()
            });

        // Determine file name: explicit file_name override, otherwise parsed filename
        let file_name = request
            .file_name
            .clone()
            .unwrap_or(parsed.filename);

        // Generate unique filename if needed
        let unique_name = self.unique_filename(&save_dir, &file_name);
        let save_path = save_dir.join(&unique_name);

        // Determine number of segments
        let segments = request
            .segments
            .unwrap_or(DEFAULT_SEGMENTS)
            .min(MAX_SEGMENTS);

        let mut task =
            DownloadTask::new(request.url.clone(), unique_name, save_path, segments);

        task.total_size = file_info.total_size;
        task.supports_range = file_info.supports_range;
        task.content_type = file_info.content_type;
        task.etag = file_info.etag;
        task.retry_count = request
            .max_retries
            .unwrap_or(DEFAULT_MAX_RETRIES) as u32;
        task.expected_checksum = request.expected_checksum.clone();
        task.checksum_algorithm = request
            .checksum_type
            .as_ref()
            .and_then(|s| ChecksumAlgorithm::from_str(s));

        info!(
            "Created download task: {} -> {:?} ({} segments, size: {})",
            task.url,
            task.save_path,
            task.segments,
            task.total_size
                .map(|s| format_bytes(s))
                .unwrap_or_else(|| "unknown".to_string())
        );

        Ok(task)
    }

    // ==========================================================
    //  MAIN DOWNLOAD ENTRY POINT
    // ==========================================================

    /// Start or resume a download
    ///
    /// This is the main entry point for downloading a file.
    /// It automatically chooses between single-segment and
    /// multi-segment download based on server capabilities
    /// and file size.
    pub async fn start_download(
        &self,
        task: &mut DownloadTask,
        cancel_token: CancellationToken,
        progress_tx: flume::Sender<DownloadProgress>,
    ) -> Result<(), DownloadError> {
        task.status = DownloadStatus::Connecting;
        Self::emit_progress(task, &progress_tx);

        // Check for existing resume state
        let temp_dir = self.get_temp_dir(task);
        let resume_data =
            ResumeManager::load(&temp_dir).await?;

        // Refresh file info (check if file changed on server)
        let file_info = self
            .http_client
            .get_file_info(&task.url)
            .await?;

        task.total_size = file_info.total_size;
        task.supports_range = file_info.supports_range;
        task.etag = file_info.etag.clone();

        // Determine download strategy
        let use_multi_segment = self.should_use_multi_segment(task);

        info!(
            "Download strategy for '{}': {}",
            task.file_name,
            if use_multi_segment {
                format!("multi-segment ({} segments)", task.segments)
            } else {
                "single-segment".to_string()
            }
        );

        task.status = DownloadStatus::Downloading;

        let result = if use_multi_segment {
            self.multi_segment_download(
                task,
                resume_data,
                cancel_token.clone(),
                progress_tx.clone(),
            )
            .await
        } else {
            self.single_segment_download(
                task,
                resume_data,
                cancel_token.clone(),
                progress_tx.clone(),
            )
            .await
        };

        match &result {
            Ok(()) => {
                // Verify checksum if provided
                if let (Some(expected), Some(algorithm)) = (
                    &task.expected_checksum,
                    &task.checksum_algorithm,
                ) {
                    task.status = DownloadStatus::Verifying;
                    Self::emit_progress(task, &progress_tx);

                    info!(
                        "Verifying checksum for '{}'...",
                        task.file_name
                    );

                    match ChecksumVerifier::verify(
                        &task.save_path,
                        expected,
                        algorithm,
                    )
                    .await
                    {
                        Ok(true) => {
                            info!(
                                "Checksum verified for '{}'",
                                task.file_name
                            );
                        }
                        Ok(false) | Err(_) => {
                            task.status = DownloadStatus::Failed;
                            task.error_message = Some(
                                "Checksum verification failed"
                                    .to_string(),
                            );
                            Self::emit_progress(
                                task,
                                &progress_tx,
                            );
                            return Err(
                                DownloadError::ChecksumMismatch {
                                    expected: expected.clone(),
                                    actual: "mismatch".to_string(),
                                },
                            );
                        }
                    }
                }

                // Clean up resume state
                let _ = ResumeManager::delete(&temp_dir).await;

                task.status = DownloadStatus::Completed;
                task.completed_at =
                    Some(chrono::Local::now().naive_local());
                task.speed = 0.0;
                Self::emit_progress(task, &progress_tx);

                info!(
                    "✅ Download completed: '{}' ({})",
                    task.file_name,
                    task.total_size
                        .map(|s| format_bytes(s))
                        .unwrap_or_default()
                );
            }
            Err(DownloadError::Cancelled) => {
                task.status = DownloadStatus::Cancelled;
                task.speed = 0.0;
                Self::emit_progress(task, &progress_tx);
                info!(
                    "Download cancelled: '{}'",
                    task.file_name
                );
            }
            Err(DownloadError::Paused) => {
                task.status = DownloadStatus::Paused;
                task.speed = 0.0;
                Self::emit_progress(task, &progress_tx);
                info!("Download paused: '{}'", task.file_name);
            }
            Err(e) => {
                task.status = DownloadStatus::Failed;
                task.error_message = Some(e.to_string());
                task.speed = 0.0;
                Self::emit_progress(task, &progress_tx);
                error!(
                    "❌ Download failed: '{}': {}",
                    task.file_name, e
                );
            }
        }

        result
    }

    /// Start a fresh download (no resume)
    async fn start_fresh_download(
        &self,
        task: &mut DownloadTask,
        cancel_token: CancellationToken,
        progress_tx: flume::Sender<DownloadProgress>,
    ) -> Result<(), DownloadError> {
        task.downloaded_size = 0;
        task.status = DownloadStatus::Downloading;

        let use_multi_segment = self.should_use_multi_segment(task);

        if use_multi_segment {
            self.multi_segment_download(
                task,
                None,
                cancel_token,
                progress_tx,
            )
            .await
        } else {
            self.single_segment_download(
                task,
                None,
                cancel_token,
                progress_tx,
            )
            .await
        }
    }

    // ==========================================================
    //  SINGLE SEGMENT DOWNLOAD
    // ==========================================================

    async fn single_segment_download(
        &self,
        task: &mut DownloadTask,
        _resume_data: Option<ResumeData>,
        cancel_token: CancellationToken,
        progress_tx: flume::Sender<DownloadProgress>,
    ) -> Result<(), DownloadError> {
        let client = self.http_client.clone();
        let retry_handler = RetryHandler::new(RetryConfig::default());

        let url = task.url.clone();
        let save_path = task.save_path.clone();

        // Use retry handler for the actual download
        let result = retry_handler.execute(
            &format!("single-segment download '{}'", task.file_name),
            || {
                let client = client.clone();
                let url = url.clone();
                let save_path = save_path.clone();
                let cancel = cancel_token.clone();

                async move {
                    Self::do_single_download(
                        client,
                        &url,
                        &save_path,
                        cancel,
                    )
                    .await
                }
            },
        )
        .await;

        match result {
            Ok(total_bytes) => {
                task.downloaded_size = total_bytes;
                Ok(())
            }
            Err(e) => {
                Err(e)
            }
        }
    }

    /// Perform the actual single-segment HTTP download
    async fn do_single_download(
        client: HttpClient,
        url: &str,
        save_path: &PathBuf,
        cancel_token: CancellationToken,
    ) -> Result<u64, DownloadError> {
        let response = client.get(url).await
            .map_err(|e| DownloadError::NetworkError(e.to_string()))?;

        let status = response.status();
        if !status.is_success() {
            return Err(DownloadError::ServerError { 
                status: status.as_u16(), 
                message: format!("Server returned {}", status)
            });
        }

        // Open file for writing
        let mut file = tokio::fs::File::create(save_path)
            .await
            .map_err(|e| {
                DownloadError::FileError(format!(
                    "Failed to create file: {}",
                    e
                ))
            })?;

        let mut stream = response.bytes_stream();
        let mut total_bytes: u64 = 0;

        loop {
            tokio::select! {
                _ = cancel_token.cancelled() => {
                    let _ = file.flush().await;
                    return Err(DownloadError::Cancelled);
                }

                chunk = stream.next() => {
                    match chunk {
                        Some(Ok(data)) => {
                            file.write_all(&data)
                                .await
                                .map_err(|e| {
                                    DownloadError::FileError(
                                        format!("Write error: {}", e)
                                    )
                                })?;

                            total_bytes += data.len() as u64;
                        }

                        Some(Err(e)) => {
                            let _ = file.flush().await;
                            return Err(
                                DownloadError::NetworkError(
                                    e.to_string()
                                )
                            );
                        }

                        None => break,
                    }
                }
            }
        }

        file.flush().await.map_err(|e| {
            DownloadError::FileError(format!(
                "Flush error: {}",
                e
            ))
        })?;

        Ok(total_bytes)
    }

    // ==========================================================
    //  MULTI-SEGMENT DOWNLOAD
    // ==========================================================

    async fn multi_segment_download(
        &self,
        task: &mut DownloadTask,
        _resume_data: Option<ResumeData>,
        cancel_token: CancellationToken,
        progress_tx: flume::Sender<DownloadProgress>,
    ) -> Result<(), DownloadError> {
        let total_size = task.total_size.ok_or(
            DownloadError::Unknown(
                "Cannot use multi-segment without known file size"
                    .into(),
            ),
        )?;

        // Create chunks
        let chunks = ChunkManager::split(total_size, task.segments);

        let num_segments = chunks.len();
        info!(
            "Multi-segment download: {} segments for {} file",
            num_segments,
            format_bytes(total_size)
        );

        // Create temp directory for segments
        let temp_dir = self.get_temp_dir(task);

        tokio::fs::create_dir_all(&temp_dir)
            .await
            .map_err(|e| {
                DownloadError::FileError(format!(
                    "Failed to create temp dir: {}",
                    e
                ))
            })?;

        // Spawn download tasks for each segment
        let mut handles = Vec::with_capacity(num_segments);

        for chunk in &chunks {
            let segment_dl = SegmentDownloader::new(
                self.http_client.clone(),
                self.speed_limiter.clone(),
                RetryConfig::default(),
            );

            let url = task.url.clone();
            let chunk_clone = chunk.clone();
            let temp_path =
                temp_dir.join(format!("segment_{}", chunk.id));
            let cancel = cancel_token.clone();

            let handle = tokio::spawn(async move {
                segment_dl.download_segment(
                    &url,
                    &chunk_clone,
                    &temp_path,
                    cancel,
                )
                .await
            });

            handles.push((chunk.id, handle));
        }

        // Wait for all segments to complete
        let mut segment_errors: Vec<(u32, DownloadError)> =
            Vec::new();

        for (segment_id, handle) in handles {
            match handle.await {
                Ok(Ok(())) => {
                    debug!("Segment {} task completed", segment_id);
                }
                Ok(Err(e)) => {
                    error!(
                        "Segment {} failed: {}",
                        segment_id, e
                    );
                    segment_errors.push((segment_id, e));
                }
                Err(e) => {
                    error!(
                        "Segment {} panicked: {}",
                        segment_id, e
                    );
                    segment_errors.push((
                        segment_id,
                        DownloadError::Unknown(format!(
                            "Task panicked: {}",
                            e
                        )),
                    ));
                }
            }
        }

        // Check for errors
        if !segment_errors.is_empty() {
            // Return first error
            for (_, error) in &segment_errors {
                if matches!(error, DownloadError::Cancelled) {
                    return Err(DownloadError::Cancelled);
                }
            }

            let (seg_id, error) =
                segment_errors.into_iter().next().unwrap();
            return Err(DownloadError::SegmentFailed { 
                segment_id: seg_id, 
                message: error.to_string() 
            });
        }

        // All segments complete - merge files
        info!("All segments complete. Merging...");
        task.status = DownloadStatus::Merging;

        self.merge_segments(
            &temp_dir,
            &task.save_path,
            &chunks,
        )
        .await?;

        // Clean up temp directory
        if let Err(e) =
            tokio::fs::remove_dir_all(&temp_dir).await
        {
            warn!("Failed to clean up temp dir: {}", e);
        }

        task.downloaded_size = total_size;

        Ok(())
    }

    // ==========================================================
    //  MERGE SEGMENTS
    // ==========================================================

    /// Merge downloaded segments into the final file
    async fn merge_segments(
        &self,
        temp_dir: &PathBuf,
        output_path: &PathBuf,
        chunks: &[Chunk],
    ) -> Result<(), DownloadError> {
        let mut output =
            tokio::fs::File::create(output_path)
                .await
                .map_err(|e| {
                    DownloadError::MergeFailed(format!(
                        "Cannot create output file: {}",
                        e
                    ))
                })?;

        for chunk in chunks {
            let segment_path =
                temp_dir.join(format!("segment_{}", chunk.id));

            if !segment_path.exists() {
                return Err(DownloadError::MergeFailed(
                    format!(
                        "Segment {} file not found",
                        chunk.id
                    ),
                ));
            }

            // Read and write in chunks to avoid loading
            // entire segment into memory
            let mut segment_file =
                tokio::fs::File::open(&segment_path)
                    .await
                    .map_err(|e| {
                        DownloadError::MergeFailed(format!(
                            "Cannot open segment {}: {}",
                            chunk.id, e
                        ))
                    })?;

            let bytes_copied =
                tokio::io::copy(&mut segment_file, &mut output)
                    .await
                    .map_err(|e| {
                        DownloadError::MergeFailed(format!(
                            "Copy error for segment {}: {}",
                            chunk.id, e
                        ))
                    })?;

            debug!(
                "Merged segment {}: {} bytes",
                chunk.id, bytes_copied
            );
        }

        output.flush().await.map_err(|e| {
            DownloadError::MergeFailed(format!(
                "Flush error: {}",
                e
            ))
        })?;

        info!(
            "Successfully merged {} segments into {:?}",
            chunks.len(),
            output_path
        );

        Ok(())
    }

    // ==========================================================
    //  HELPERS
    // ==========================================================

    fn get_temp_dir(&self, task: &DownloadTask) -> PathBuf {
        task.save_path
            .parent()
            .unwrap_or(&self.default_download_dir)
            .join(format!(".sd_{}", task.id))
    }

    /// Decide whether to use multi-segment download
    fn should_use_multi_segment(
        &self,
        task: &DownloadTask,
    ) -> bool {
        // Must support range requests
        if !task.supports_range {
            return false;
        }

        // Must know the file size
        let total_size = match task.total_size {
            Some(size) => size,
            None => return false,
        };

        // Must be larger than minimum size
        if total_size < MIN_SIZE_FOR_SEGMENTS {
            return false;
        }

        // Must have more than 1 segment configured
        if task.segments <= 1 {
            return false;
        }

        true
    }

    /// Generate a unique filename if file already exists
    fn unique_filename(&self, dir: &PathBuf, filename: &str) -> String {
        let path = dir.join(filename);
        if !path.exists() {
            return filename.to_string();
        }

        let path_buf = PathBuf::from(filename);
        let stem = path_buf
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("download")
            .to_string();
        let ext = path_buf
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_string();

        for i in 1..1000 {
            let new_name = if ext.is_empty() {
                format!("{} ({})", stem, i)
            } else {
                format!("{} ({}).{}", stem, i, ext)
            };
            if !dir.join(&new_name).exists() {
                return new_name;
            }
        }

        filename.to_string()
    }

    /// Emit a progress update
    fn emit_progress(
        task: &DownloadTask,
        tx: &flume::Sender<DownloadProgress>,
    ) {
        let _ = tx.send(DownloadProgress {
            id: task.id,
            downloaded_size: task.downloaded_size,
            total_size: task.total_size,
            speed: task.speed,
            eta: task.eta,
            status: task.status.clone(),
            percent: task.percent(),
            error_message: task.error_message.clone(),
        });
    }
}

/// Format bytes to human readable string
fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    if bytes == 0 {
        return "0 B".to_string();
    }
    let exp = (bytes as f64).log(1024.0).min(4.0) as usize;
    let value = bytes as f64 / 1024f64.powi(exp as i32);
    format!("{:.2} {}", value, UNITS[exp])
}

/// Request to add a new download.
///
/// NOTE: This type is shared with the Tauri commands layer
/// (`src-tauri/src/commands/download_commands.rs`). Keep the
/// fields in sync with that struct.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct AddDownloadRequest {
    pub url: String,
    pub save_path: Option<String>,
    pub segments: Option<u8>,
    pub max_retries: Option<u32>,
    pub expected_checksum: Option<String>,
    pub checksum_type: Option<String>,
    pub file_name: Option<String>,
    pub category: Option<String>,
    pub priority: Option<u32>,

    // YouTube-specific fields
    pub youtube_format: Option<String>,        // "video" or "audio"
    pub youtube_quality: Option<String>,       // "2160p", "1080p", etc.
    pub youtube_video_format: Option<String>,  // "mp4", "mkv", "webm"
    pub youtube_audio_format: Option<String>,  // "mp3", "aac", "flac"
}

