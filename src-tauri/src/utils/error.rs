// src-tauri/src/utils/error.rs

use thiserror::Error;
use serde::Serialize;

/// Main application error type
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Download error: {0}")]
    Download(#[from] DownloadError),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("URL parse error: {0}")]
    UrlParse(#[from] url::ParseError),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("{0}")]
    Other(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Not implemented: {0}")]
    NotImplemented(String),

    #[error("Torrent error: {0}")]
    TorrentError(String),
}

/// Download-specific error type
#[derive(Error, Debug, Clone, Serialize)]
pub enum DownloadError {
    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("File system error: {0}")]
    FileError(String),

    #[error("URL parse error: {0}")]
    UrlError(String),

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("Server returned error: {status} - {message}")]
    ServerError { status: u16, message: String },

    #[error("Download cancelled")]
    Cancelled,

    #[error("Download paused")]
    Paused,

    #[error("Checksum mismatch: expected {expected}, got {actual}")]
    ChecksumMismatch { expected: String, actual: String },

    #[error("Max retries exceeded ({retries} attempts)")]
    MaxRetriesExceeded { retries: u32 },

    #[error("Server does not support range requests")]
    RangeNotSupported,

    #[error("File already exists: {0}")]
    FileExists(String),

    #[error("Insufficient disk space")]
    InsufficientDiskSpace,

    #[error("Segment download failed: segment {segment_id} - {message}")]
    SegmentFailed { segment_id: u32, message: String },

    #[error("Merge failed: {0}")]
    MergeFailed(String),

    #[error("Timeout after {seconds}s")]
    Timeout { seconds: u64 },

    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),

    #[error("Unknown error: {0}")]
    Unknown(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Permission denied")]
    PermissionDenied,

    #[error("Disk full")]
    DiskFull,

    #[error("Torrent error: {0}")]
    TorrentError(String),
}

// Allow DownloadError to be returned from Tauri commands
impl From<DownloadError> for String {
    fn from(err: DownloadError) -> Self {
        err.to_string()
    }
}

impl From<AppError> for String {
    fn from(err: AppError) -> Self {
        err.to_string()
    }
}

impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}