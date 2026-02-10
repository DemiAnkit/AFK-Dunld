use futures_util::StreamExt;
use reqwest::Client;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::io::AsyncWriteExt;
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

use crate::core::checksum::ChecksumVerifier;
use crate::core::chunk_manager::{Chunk, ChunkManager};
use crate::core::download_task::*;
use crate::core::resume_manager::{ResumeManager, ResumeState};
use crate::core::retry::{with_retry, RetryConfig};
use crate::core::segment_downloader::SegmentDownloader;
use crate::core::speed_limiter::SpeedLimiter;
use crate::network::http_client::HttpClient;
use crate::network::url_parser::UrlParser;
use crate::utils::constants::*;
use crate::utils::error::DownloadError;
use crate::utils::format;

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
        proxy: Option<&crate::network::http_client::ProxyConfig>,
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
                DownloadError::FileSystem(format!(
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
        UrlParser::validate(url)?;
        self.http_client.get_file_info(url).await
    }

    /// Create a new download task from a request
    pub async fn create_task(
        &self,
        request: &AddDownloadRequest,
    ) -> Result<DownloadTask, DownloadError> {
        // Validate URL
        UrlParser::validate(&request.url)?;

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

        // Determine file name
        let file_name = request
            .file_name
            .clone()
            .unwrap_or(file_info.file_name.clone());

        // Generate unique filename if needed
        let unique_name =
            UrlParser::unique_filename(&save_dir, &file_name);
        let save_path = save_dir.join(&unique_name);

        // Determine number of segments
        let segments = request
            .segments
            .unwrap_or(DEFAULT_SEGMENTS)
            .min(MAX_SEGMENTS);

        let mut task =
            DownloadTask::new(request.url.clone(), save_path, unique_name);

        task.total_size = file_info.total_size;
        task.supports_range = file_info.supports_range;
        task.content_type = file_info.content_type;
        task.etag = file_info.etag;
        task.segments = segments;
        task.max_retries =
            request.max_retries.unwrap_or(DEFAULT_MAX_RETRIES);
        task.expected_checksum =
            request.expected_checksum.clone();
        task.checksum_type = request.checksum_type.clone();
        task.speed_limit = request.speed_limit;
        task.category = request.category.clone();
        task.priority = request.priority.unwrap_or(0);

        info!(
            "Created download task: {} -> {:?} ({} segments, size: {})",
            task.url,
            task.save_path,
            task.segments,
            task.total_size
                .map(|s| format::format_bytes(s))
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
        Self::emit_progress(task, &[], &progress_tx);

        // Check for existing resume state
        let resume_state =
            ResumeManager::load_state(&task.save_path).await?;

        // Refresh file info (check if file changed on server)
        let file_info = self
            .http_client
            .get_file_info(&task.url)
            .await?;

        task.total_size = file_info.total_size;
        task.supports_range = file_info.supports_range;
        task.etag = file_info.etag.clone();

        // Validate resume state if exists
        if let Some(ref state) = resume_state {
            if !ResumeManager::validate_etag(
                &state.etag,
                &file_info.etag,
            ) {
                warn!(
                    "ETag changed, cannot resume. Starting fresh."
                );
                ResumeManager::delete_state(&task.save_path)
                    .await?;
                return self
                    .start_fresh_download(
                        task,
                        cancel_token,
                        progress_tx,
                    )
                    .await;
            }
        }

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
                resume_state,
                cancel_token.clone(),
                progress_tx.clone(),
            )
            .await
        } else {
            self.single_segment_download(
                task,
                resume_state,
                cancel_token.clone(),
                progress_tx.clone(),
            )
            .await
        };

        match &result {
            Ok(()) => {
                // Verify checksum if provided
                if let (Some(expected), Some(checksum_type)) = (
                    &task.expected_checksum,
                    &task.checksum_type,
                ) {
                    task.status = DownloadStatus::Verifying;
                    Self::emit_progress(task, &[], &progress_tx);

                    info!(
                        "Verifying checksum for '{}'...",
                        task.file_name
                    );

                    match ChecksumVerifier::verify(
                        &task.save_path,
                        expected,
                        checksum_type,
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
                                &[],
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
                ResumeManager::delete_state(&task.save_path)
                    .await?;

                task.status = DownloadStatus::Completed;
                task.completed_at =
                    Some(chrono::Local::now().naive_local());
                task.speed = 0.0;
                Self::emit_progress(task, &[], &progress_tx);

                info!(
                    "✅ Download completed: '{}' ({})",
                    task.file_name,
                    task.total_size
                        .map(|s| format::format_bytes(s))
                        .unwrap_or_default()
                );
            }
            Err(DownloadError::Cancelled) => {
                task.status = DownloadStatus::Cancelled;
                task.speed = 0.0;
                Self::emit_progress(task, &[], &progress_tx);
                info!(
                    "Download cancelled: '{}'",
                    task.file_name
                );
            }
            Err(DownloadError::Paused) => {
                task.status = DownloadStatus::Paused;
                task.speed = 0.0;
                Self::emit_progress(task, &[], &progress_tx);
                info!("Download paused: '{}'", task.file_name);
            }
            Err(e) => {
                task.status = DownloadStatus::Failed;
                task.error_message = Some(e.to_string());
                task.speed = 0.0;
                Self::emit_progress(task, &[], &progress_tx);
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
        resume_state: Option<ResumeState>,
        cancel_token: CancellationToken,
        progress_tx: flume::Sender<DownloadProgress>,
    ) -> Result<(), DownloadError> {
        let client = self.http_client.client_clone();
        let retry_config =
            RetryConfig::new(task.max_retries);

        // Determine starting position for resume
        let start_pos = resume_state
            .as_ref()
            .map(|s| s.total_downloaded)
            .unwrap_or(0);

        task.downloaded_size = start_pos;

        let url = task.url.clone();
        let save_path = task.save_path.clone();
        let task_speed_limit = task.speed_limit;
        let speed_limiter = if task_speed_limit.is_some() {
            SpeedLimiter::new(task_speed_limit)
        } else {
            self.speed_limiter.clone()
        };

        // Use retry wrapper for the actual download
        let download_result = with_retry(
            &retry_config,
            &format!("single-segment download '{}'", task.file_name),
            || {
                let client = client.clone();
                let url = url.clone();
                let save_path = save_path.clone();
                let cancel = cancel_token.clone();
                let limiter = speed_limiter.clone();
                let start = start_pos;

                async move {
                    Self::do_single_download(
                        client,
                        &url,
                        &save_path,
                        start,
                        cancel,
                        limiter,
                    )
                    .await
                }
            },
        )
        .await;

        match download_result {
            Ok((total_bytes, _)) => {
                task.downloaded_size = total_bytes;
                Ok(())
            }
            Err(e) => {
                // Save resume state on failure
                if task.supports_range
                    && task.downloaded_size > 0
                {
                    let state = ResumeState {
                        task_id: task.id,
                        url: task.url.clone(),
                        total_size: task.total_size,
                        etag: task.etag.clone(),
                        last_modified: None,
                        chunks: vec![Chunk {
                            id: 0,
                            start: 0,
                            end: task
                                .total_size
                                .unwrap_or(0)
                                .saturating_sub(1),
                            downloaded: task.downloaded_size,
                        }],
                        total_downloaded: task.downloaded_size,
                        saved_at: chrono::Local::now()
                            .naive_local(),
                    };
                    let _ = ResumeManager::save_state(
                        &task.save_path,
                        &state,
                    )
                    .await;
                }
                Err(e)
            }
        }
    }

    /// Perform the actual single-segment HTTP download
    async fn do_single_download(
        client: Client,
        url: &str,
        save_path: &PathBuf,
        start_pos: u64,
        cancel_token: CancellationToken,
        speed_limiter: SpeedLimiter,
    ) -> Result<(u64, f64), DownloadError> {
        let mut request = client.get(url);

        // Add Range header for resume
        if start_pos > 0 {
            request = request.header(
                reqwest::header::RANGE,
                format!("bytes={}-", start_pos),
            );
            info!("Resuming from byte {}", start_pos);
        }

        let response = request
            .send()
            .await
            .map_err(|e| DownloadError::Network(e.to_string()))?;

        let status = response.status();
        if !status.is_success()
            && status.as_u16() != 206
        {
            return Err(DownloadError::HttpStatus {
                status: status.as_u16(),
                message: format!(
                    "Server returned {}",
                    status
                ),
            });
        }

        // Open file for writing
        let mut file = if start_pos > 0 {
            tokio::fs::OpenOptions::new()
                .append(true)
                .open(save_path)
                .await
                .map_err(|e| {
                    DownloadError::FileSystem(format!(
                        "Failed to open file for resume: {}",
                        e
                    ))
                })?
        } else {
            tokio::fs::File::create(save_path)
                .await
                .map_err(|e| {
                    DownloadError::FileSystem(format!(
                        "Failed to create file: {}",
                        e
                    ))
                })?
        };

        let mut stream = response.bytes_stream();
        let mut total_bytes = start_pos;
        let mut speed_bytes: u64 = 0;
        let mut speed_timer = Instant::now();
        let mut current_speed: f64 = 0.0;

        loop {
            tokio::select! {
                _ = cancel_token.cancelled() => {
                    let _ = file.flush().await;
                    return Err(DownloadError::Cancelled);
                }

                chunk = stream.next() => {
                    match chunk {
                        Some(Ok(data)) => {
                            speed_limiter
                                .throttle(data.len())
                                .await;

                            file.write_all(&data)
                                .await
                                .map_err(|e| {
                                    DownloadError::FileSystem(
                                        format!("Write error: {}", e)
                                    )
                                })?;

                            total_bytes += data.len() as u64;
                            speed_bytes += data.len() as u64;

                            // Calculate speed
                            let elapsed = speed_timer
                                .elapsed()
                                .as_secs_f64();
                            if elapsed >= 0.5 {
                                current_speed = speed_bytes
                                    as f64
                                    / elapsed;
                                speed_bytes = 0;
                                speed_timer = Instant::now();
                            }
                        }

                        Some(Err(e)) => {
                            let _ = file.flush().await;
                            return Err(
                                DownloadError::Network(
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
            DownloadError::FileSystem(format!(
                "Flush error: {}",
                e
            ))
        })?;

        Ok((total_bytes, current_speed))
    }

    // ==========================================================
    //  MULTI-SEGMENT DOWNLOAD
    // ==========================================================

    async fn multi_segment_download(
        &self,
        task: &mut DownloadTask,
        resume_state: Option<ResumeState>,
        cancel_token: CancellationToken,
        progress_tx: flume::Sender<DownloadProgress>,
    ) -> Result<(), DownloadError> {
        let total_size = task.total_size.ok_or(
            DownloadError::Unknown(
                "Cannot use multi-segment without known file size"
                    .into(),
            ),
        )?;

        // Create or restore chunks
        let chunks = if let Some(ref state) = resume_state {
            info!(
                "Resuming multi-segment download with {} chunks",
                state.chunks.len()
            );
            state.chunks.clone()
        } else {
            ChunkManager::split(total_size, task.segments)
        };

        let num_segments = chunks.len();
        info!(
            "Multi-segment download: {} segments for {} file",
            num_segments,
            format::format_bytes(total_size)
        );

        // Create temp directory for segments
        let temp_dir = task
            .save_path
            .parent()
            .unwrap_or(&self.default_download_dir)
            .join(format!(".sd_{}", task.id));

        tokio::fs::create_dir_all(&temp_dir)
            .await
            .map_err(|e| {
                DownloadError::FileSystem(format!(
                    "Failed to create temp dir: {}",
                    e
                ))
            })?;

        // Shared progress tracking
        let segment_progress = Arc::new(RwLock::new(
            chunks.iter().map(|c| c.downloaded).collect::<Vec<u64>>(),
        ));

        // Speed limiter for this download
        let speed_limiter = if task.speed_limit.is_some() {
            SpeedLimiter::new(task.speed_limit)
        } else {
            self.speed_limiter.clone()
        };

        let client = self.http_client.client_clone();

        // Spawn download tasks for each segment
        let mut handles = Vec::with_capacity(num_segments);

        for chunk in &chunks {
            if chunk.is_complete() {
                info!(
                    "Segment {} already complete, skipping",
                    chunk.id
                );
                continue;
            }

            let segment_dl = SegmentDownloader::new(
                client.clone(),
                speed_limiter.clone(),
            );

            let url = task.url.clone();
            let chunk_clone = chunk.clone();
            let temp_path =
                temp_dir.join(format!("segment_{}", chunk.id));
            let cancel = cancel_token.clone();
            let progress = segment_progress.clone();
            let retry_config =
                RetryConfig::new(task.max_retries);
            let segment_id = chunk.id;

            let handle = tokio::spawn(async move {
                with_retry(
                    &retry_config,
                    &format!("segment {}", segment_id),
                    || {
                        let dl = SegmentDownloader::new(
                            client.clone(),
                            speed_limiter.clone(),
                        );
                        let url = url.clone();
                        let chunk = chunk_clone.clone();
                        let path = temp_path.clone();
                        let cancel = cancel.clone();
                        let progress = progress.clone();

                        async move {
                            dl.download_segment(
                                &url,
                                &chunk,
                                &path,
                                cancel,
                                progress,
                            )
                            .await
                        }
                    },
                )
                .await
            });

            handles.push((chunk.id, handle));
        }

        // Progress reporter task
        let progress_data = segment_progress.clone();
        let progress_tx_clone = progress_tx.clone();
        let task_id = task.id;
        let task_file_name = task.file_name.clone();
        let chunks_for_progress = chunks.clone();
        let cancel_for_progress = cancel_token.clone();

        let progress_handle = tokio::spawn(async move {
            let mut last_total: u64 = 0;
            let mut speed_timer = Instant::now();

            loop {
                tokio::select! {
                    _ = tokio::time::sleep(Duration::from_millis(
                        PROGRESS_UPDATE_INTERVAL_MS
                    )) => {}
                    _ = cancel_for_progress.cancelled() => break,
                }

                let segment_data =
                    progress_data.read().await.clone();

                let total_downloaded: u64 =
                    segment_data.iter().sum();

                // Calculate speed
                let elapsed =
                    speed_timer.elapsed().as_secs_f64();
                let speed = if elapsed > 0.0 {
                    (total_downloaded - last_total) as f64
                        / elapsed
                } else {
                    0.0
                };

                // Calculate ETA
                let remaining =
                    total_size.saturating_sub(total_downloaded);
                let eta = if speed > 0.0 {
                    Some((remaining as f64 / speed) as u64)
                } else {
                    None
                };

                let percent = (total_downloaded as f64
                    / total_size as f64)
                    * 100.0;

                // Build segment progress
                let seg_progress: Vec<SegmentProgress> =
                    chunks_for_progress
                        .iter()
                        .enumerate()
                        .map(|(i, chunk)| SegmentProgress {
                            segment_id: chunk.id,
                            start: chunk.start,
                            end: chunk.end,
                            downloaded: segment_data
                                .get(i)
                                .copied()
                                .unwrap_or(0),
                            total: chunk.size(),
                            speed: 0.0,
                            status: if segment_data
                                .get(i)
                                .copied()
                                .unwrap_or(0)
                                >= chunk.size()
                            {
                                SegmentStatus::Completed
                            } else if segment_data
                                .get(i)
                                .copied()
                                .unwrap_or(0)
                                > 0
                            {
                                SegmentStatus::Downloading
                            } else {
                                SegmentStatus::Pending
                            },
                        })
                        .collect();

                let _ =
                    progress_tx_clone.send(DownloadProgress {
                        id: task_id,
                        downloaded_size: total_downloaded,
                        total_size: Some(total_size),
                        speed,
                        eta,
                        status: DownloadStatus::Downloading,
                        percent,
                        segment_progress: seg_progress,
                    });

                last_total = total_downloaded;
                speed_timer = Instant::now();
            }
        });

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

        // Stop progress reporter
        cancel_token.cancel();
        let _ = progress_handle.await;

        // Check for errors
        if !segment_errors.is_empty() {
            // Save resume state for partial downloads
            let current_progress =
                segment_progress.read().await.clone();

            let resume_chunks: Vec<Chunk> = chunks
                .iter()
                .enumerate()
                .map(|(i, c)| {
                    let mut chunk = c.clone();
                    chunk.downloaded =
                        current_progress
                            .get(i)
                            .copied()
                            .unwrap_or(0);
                    chunk
                })
                .collect();

            let total_downloaded: u64 =
                current_progress.iter().sum();

            let state = ResumeState {
                task_id: task.id,
                url: task.url.clone(),
                total_size: task.total_size,
                etag: task.etag.clone(),
                last_modified: None,
                chunks: resume_chunks,
                total_downloaded,
                saved_at: chrono::Local::now().naive_local(),
            };

            let _ = ResumeManager::save_state(
                &task.save_path,
                &state,
            )
            .await;

            task.downloaded_size = total_downloaded;

            // Return first error (check for cancellation first)
            for (_, error) in &segment_errors {
                if matches!(error, DownloadError::Cancelled) {
                    return Err(DownloadError::Cancelled);
                }
            }

            let (seg_id, error) =
                segment_errors.into_iter().next().unwrap();
            return Err(DownloadError::SegmentFailed {
                segment_id: seg_id,
                reason: error.to_string(),
            });
        }

        // All segments complete - merge files
        info!("All segments complete. Merging...");
        task.status = DownloadStatus::Merging;

        let merge_progress = DownloadProgress {
            id: task.id,
            downloaded_size: total_size,
            total_size: Some(total_size),
            speed: 0.0,
            eta: None,
            status: DownloadStatus::Merging,
            percent: 100.0,
            segment_progress: vec![],
        };
        let _ = progress_tx.send(merge_progress);

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

    /// Emit a progress update
    fn emit_progress(
        task: &DownloadTask,
        segment_progress: &[SegmentProgress],
        tx: &flume::Sender<DownloadProgress>,
    ) {
        let _ = tx.send(DownloadProgress {
            id: task.id,
            downloaded_size: task.downloaded_size,
            total_size: task.total_size,
            speed: task.speed,
            eta: task.eta,
            status: task.status.clone(),
            percent: task.progress_percent(),
            segment_progress: segment_progress.to_vec(),
        });
    }
}