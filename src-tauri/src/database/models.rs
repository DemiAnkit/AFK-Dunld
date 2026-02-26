use serde::{Deserialize, Serialize};

/// Database row for a download
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadRow {
    pub id: String,
    pub url: String,
    pub final_url: Option<String>,
    pub file_name: String,
    pub save_path: String,
    pub total_size: Option<i64>,
    pub downloaded_size: i64,
    pub status: String,
    pub segments: i32,
    pub supports_range: bool,
    pub content_type: Option<String>,
    pub etag: Option<String>,
    pub expected_checksum: Option<String>,
    pub actual_checksum: Option<String>,
    pub checksum_algorithm: Option<String>,
    pub retry_count: i32,
    pub error_message: Option<String>,
    pub created_at: String,
    pub completed_at: Option<String>,
    pub priority: i32,
    pub category: Option<String>,
    pub segment_progress: Option<String>,
}

/// Database row for a torrent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TorrentRow {
    pub info_hash: String,
    pub name: String,
    pub total_size: i64,
    pub piece_length: i64,
    pub num_pieces: i64,
    pub save_path: String,
    pub priority: i32,
    pub category: Option<String>,
    pub added_time: String,
    pub completed_time: Option<String>,
    pub state: String,
    pub downloaded_size: i64,
    pub uploaded_size: i64,
    pub download_rate: i64,
    pub upload_rate: i64,
    pub peers: i32,
    pub seeders: i32,
    pub progress: f64,
    pub eta: Option<i64>,
}

/// Database row for a torrent file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TorrentFileRow {
    pub id: Option<i64>,
    pub info_hash: String,
    pub path: String,
    pub size: i64,
}

/// Database row for torrent bandwidth limits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TorrentBandwidthRow {
    pub info_hash: String,
    pub download_limit: Option<i64>,
    pub upload_limit: Option<i64>,
    pub enabled: bool,
}

/// Database row for torrent schedules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TorrentScheduleRow {
    pub info_hash: String,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub days_of_week: Option<String>, // JSON array of day numbers
    pub enabled: bool,
}
