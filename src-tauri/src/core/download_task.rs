// src-tauri/src/core/download_task.rs

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

/// Status of a download
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DownloadStatus {
    Queued,
    Connecting,
    Downloading,
    Paused,
    Completed,
    Failed,
    Cancelled,
    Merging,
    Verifying,
}

impl DownloadStatus {
    pub fn is_active(&self) -> bool {
        matches!(
            self,
            DownloadStatus::Connecting
                | DownloadStatus::Downloading
                | DownloadStatus::Merging
                | DownloadStatus::Verifying
        )
    }

    pub fn is_resumable(&self) -> bool {
        matches!(self, DownloadStatus::Paused | DownloadStatus::Failed)
    }

    pub fn is_terminal(&self) -> bool {
        matches!(self, DownloadStatus::Completed | DownloadStatus::Cancelled)
    }

    pub fn as_str(&self) -> &str {
        match self {
            DownloadStatus::Queued => "Queued",
            DownloadStatus::Connecting => "Connecting",
            DownloadStatus::Downloading => "Downloading",
            DownloadStatus::Paused => "Paused",
            DownloadStatus::Completed => "Completed",
            DownloadStatus::Failed => "Failed",
            DownloadStatus::Cancelled => "Cancelled",
            DownloadStatus::Merging => "Merging",
            DownloadStatus::Verifying => "Verifying",
        }
    }
}

impl std::fmt::Display for DownloadStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// A single download task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadTask {
    /// Unique identifier
    pub id: Uuid,

    /// Original URL
    pub url: String,

    /// Final URL after redirects
    pub final_url: Option<String>,

    /// File name
    pub file_name: String,

    /// Full save path
    pub save_path: PathBuf,

    /// Total file size in bytes (None if unknown)
    pub total_size: Option<u64>,

    /// Total bytes downloaded so far
    pub downloaded_size: u64,

    /// Current status
    pub status: DownloadStatus,

    /// Current download speed in bytes/sec
    pub speed: f64,

    /// Estimated time remaining in seconds
    pub eta: Option<u64>,

    /// Number of download segments
    pub segments: u8,

    /// Whether the server supports range requests
    pub supports_range: bool,

    /// Content type from server
    pub content_type: Option<String>,

    /// ETag from server (for resume verification)
    pub etag: Option<String>,

    /// Expected checksum (user provided)
    pub expected_checksum: Option<String>,

    /// Actual checksum (computed after download)
    pub actual_checksum: Option<String>,

    /// Checksum algorithm (md5, sha256, etc.)
    pub checksum_algorithm: Option<crate::core::checksum::ChecksumAlgorithm>,

    /// Number of retry attempts
    pub retry_count: u32,

    /// Error message if failed
    pub error_message: Option<String>,

    /// When the download was created
    pub created_at: NaiveDateTime,

    /// When the download completed
    pub completed_at: Option<NaiveDateTime>,

    /// Priority (lower = higher priority)
    pub priority: u32,

    /// Category/group
    pub category: Option<String>,

    /// Segment progress details
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub segment_progress: Vec<SegmentProgress>,
}

/// Progress of a single segment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentProgress {
    pub segment_id: u32,
    pub start_byte: u64,
    pub end_byte: u64,
    pub downloaded: u64,
    pub status: SegmentStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SegmentStatus {
    Pending,
    Downloading,
    Completed,
    Failed,
    Paused,
}

/// Progress event sent to the frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressEvent {
    pub id: Uuid,
    pub downloaded_size: u64,
    pub total_size: Option<u64>,
    pub speed: f64,
    pub eta: Option<u64>,
    pub status: DownloadStatus,
    pub segments: Vec<SegmentProgress>,
    pub percent: f64,
}

/// Download progress information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadProgress {
    pub id: Uuid,
    pub downloaded_size: u64,
    pub total_size: Option<u64>,
    pub speed: f64,
    pub eta: Option<u64>,
    pub status: DownloadStatus,
    pub percent: f64,
    pub error_message: Option<String>,
}

/// File information from URL
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub file_name: String,
    pub total_size: Option<u64>,
    pub content_type: Option<String>,
    pub supports_range: bool,
}

/// Checksum type (alias for ChecksumAlgorithm)
pub type ChecksumType = crate::core::checksum::ChecksumAlgorithm;

impl DownloadTask {
    /// Create a new download task
    pub fn new(url: String, file_name: String, save_path: PathBuf, segments: u8) -> Self {
        Self {
            id: Uuid::new_v4(),
            url,
            final_url: None,
            file_name,
            save_path,
            total_size: None,
            downloaded_size: 0,
            status: DownloadStatus::Queued,
            speed: 0.0,
            eta: None,
            segments,
            supports_range: false,
            content_type: None,
            etag: None,
            expected_checksum: None,
            actual_checksum: None,
            checksum_algorithm: None,
            retry_count: 0,
            error_message: None,
            created_at: chrono::Local::now().naive_local(),
            completed_at: None,
            priority: 100,
            category: None,
            segment_progress: Vec::new(),
        }
    }

    /// Calculate download percentage
    pub fn percent(&self) -> f64 {
        match self.total_size {
            Some(total) if total > 0 => (self.downloaded_size as f64 / total as f64) * 100.0,
            _ => 0.0,
        }
    }

    /// Convert to progress event
    pub fn to_progress_event(&self) -> ProgressEvent {
        ProgressEvent {
            id: self.id,
            downloaded_size: self.downloaded_size,
            total_size: self.total_size,
            speed: self.speed,
            eta: self.eta,
            status: self.status,
            segments: self.segment_progress.clone(),
            percent: self.percent(),
        }
    }
}
