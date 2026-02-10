// src-tauri/src/utils/constants.rs

/// Default number of download segments
pub const DEFAULT_SEGMENTS: u8 = 8;

/// Maximum number of segments per download
pub const MAX_SEGMENTS: u8 = 32;

/// Minimum file size for multi-segment download (1 MB)
pub const MIN_SIZE_FOR_SEGMENTS: u64 = 1_048_576;

/// Minimum segment size (256 KB)
pub const MIN_SEGMENT_SIZE: u64 = 262_144;

/// Default max concurrent downloads
pub const DEFAULT_MAX_CONCURRENT: u32 = 5;

/// Default connection timeout in seconds
pub const DEFAULT_CONNECT_TIMEOUT: u64 = 30;

/// Default read timeout in seconds
pub const DEFAULT_READ_TIMEOUT: u64 = 60;

/// Default max retries
pub const DEFAULT_MAX_RETRIES: u32 = 5;

/// Default retry delay in milliseconds
pub const DEFAULT_RETRY_DELAY_MS: u64 = 1000;

/// Max retry delay in milliseconds (30 seconds)
pub const MAX_RETRY_DELAY_MS: u64 = 30_000;

/// Progress update interval in milliseconds
pub const PROGRESS_UPDATE_INTERVAL_MS: u64 = 200;

/// Buffer size for file I/O (64 KB)
pub const BUFFER_SIZE: usize = 65_536;

/// Speed calculation window in seconds
pub const SPEED_WINDOW_SECONDS: f64 = 3.0;

/// Clipboard check interval in milliseconds
pub const CLIPBOARD_CHECK_INTERVAL_MS: u64 = 1000;

/// User agent string
pub const USER_AGENT: &str = "SuperDownloader/1.0 (Rust/Tauri)";

/// App name
pub const APP_NAME: &str = "SuperDownloader";

/// Database file name
pub const DB_FILE_NAME: &str = "downloads.db";

/// Temp directory prefix
pub const TEMP_DIR_PREFIX: &str = ".sdl_parts_";