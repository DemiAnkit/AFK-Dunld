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
