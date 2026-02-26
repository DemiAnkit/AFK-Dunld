use tauri::State;
use crate::state::app_state::AppState;
use crate::network::torrent_client_librqbit::{TorrentStats, TorrentState, TorrentInfo};
use crate::network::torrent_helpers::{TorrentPriority, BandwidthLimit, TorrentSchedule, TorrentMetadata};
use crate::network::torrent_advanced::{
    WebSeed, WebSeedType, EncryptionConfig, EncryptionMode, IpFilter, 
    AdvancedTorrentOptions, TorrentAdvancedConfig
};
use std::path::PathBuf;

#[tauri::command]
pub async fn add_torrent_file(
    state: State<'_, AppState>,
    file_path: String,
) -> Result<String, String> {
    let path = PathBuf::from(file_path);
    
    state
        .torrent_client
        .add_torrent_file(&path)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn add_magnet_link(
    state: State<'_, AppState>,
    magnet_link: String,
) -> Result<String, String> {
    state
        .torrent_client
        .add_magnet(&magnet_link)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_torrent_stats(
    state: State<'_, AppState>,
    info_hash: String,
) -> Result<TorrentStats, String> {
    state
        .torrent_client
        .get_stats(&info_hash)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_torrent_state(
    state: State<'_, AppState>,
    info_hash: String,
) -> Result<TorrentState, String> {
    // Stub implementation - torrent client disabled
    state
        .torrent_client
        .get_stats(&info_hash)
        .await
        .map(|_| TorrentState::Paused)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn pause_torrent(
    state: State<'_, AppState>,
    info_hash: String,
) -> Result<(), String> {
    state
        .torrent_client
        .pause(&info_hash)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn resume_torrent(
    state: State<'_, AppState>,
    info_hash: String,
) -> Result<(), String> {
    state
        .torrent_client
        .resume(&info_hash)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn remove_torrent(
    state: State<'_, AppState>,
    info_hash: String,
    delete_files: Option<bool>,
) -> Result<(), String> {
    state
        .torrent_client
        .remove(&info_hash, delete_files.unwrap_or(false))
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_torrents(
    state: State<'_, AppState>,
) -> Result<Vec<TorrentInfo>, String> {
    state
        .torrent_client
        .list_torrents()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_torrent_info(
    state: State<'_, AppState>,
    info_hash: String,
) -> Result<TorrentInfo, String> {
    state
        .torrent_client
        .get_torrent_info(&info_hash)
        .await
        .map_err(|e| e.to_string())
}

// Priority management commands

#[tauri::command]
pub async fn set_torrent_priority(
    state: State<'_, AppState>,
    info_hash: String,
    priority: i32,
) -> Result<(), String> {
    let priority = TorrentPriority::from_i32(priority);
    state
        .torrent_client
        .set_priority(&info_hash, priority)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_torrent_priority(
    state: State<'_, AppState>,
    info_hash: String,
) -> Result<i32, String> {
    state
        .torrent_client
        .get_priority(&info_hash)
        .await
        .map(|p| p.to_i32())
        .map_err(|e| e.to_string())
}

// Bandwidth limit commands

#[tauri::command]
pub async fn set_torrent_bandwidth_limit(
    state: State<'_, AppState>,
    info_hash: String,
    download_limit: Option<u64>,
    upload_limit: Option<u64>,
) -> Result<(), String> {
    let limit = BandwidthLimit::new(download_limit, upload_limit);
    state
        .torrent_client
        .set_bandwidth_limit(&info_hash, limit)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_torrent_bandwidth_limit(
    state: State<'_, AppState>,
    info_hash: String,
) -> Result<BandwidthLimit, String> {
    state
        .torrent_client
        .get_bandwidth_limit(&info_hash)
        .await
        .map_err(|e| e.to_string())
}

// Schedule commands

#[tauri::command]
pub async fn set_torrent_schedule(
    state: State<'_, AppState>,
    info_hash: String,
    start_time: Option<String>,
    end_time: Option<String>,
    days_of_week: Vec<u8>,
    enabled: bool,
) -> Result<(), String> {
    let mut schedule = TorrentSchedule::default();
    schedule.start_time = start_time;
    schedule.end_time = end_time;
    schedule.days_of_week = days_of_week;
    schedule.enabled = enabled;
    
    state
        .torrent_client
        .set_schedule(&info_hash, schedule)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_torrent_schedule(
    state: State<'_, AppState>,
    info_hash: String,
) -> Result<TorrentSchedule, String> {
    state
        .torrent_client
        .get_schedule(&info_hash)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn is_torrent_scheduled_active(
    state: State<'_, AppState>,
    info_hash: String,
) -> Result<bool, String> {
    state
        .torrent_client
        .is_scheduled_active(&info_hash)
        .await
        .map_err(|e| e.to_string())
}

// Tag and category commands

#[tauri::command]
pub async fn add_torrent_tag(
    state: State<'_, AppState>,
    info_hash: String,
    tag: String,
) -> Result<(), String> {
    state
        .torrent_client
        .add_tag(&info_hash, tag)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn remove_torrent_tag(
    state: State<'_, AppState>,
    info_hash: String,
    tag: String,
) -> Result<(), String> {
    state
        .torrent_client
        .remove_tag(&info_hash, &tag)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_torrent_category(
    state: State<'_, AppState>,
    info_hash: String,
    category: Option<String>,
) -> Result<(), String> {
    state
        .torrent_client
        .set_category(&info_hash, category)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_torrent_metadata(
    state: State<'_, AppState>,
    info_hash: String,
) -> Result<TorrentMetadata, String> {
    state
        .torrent_client
        .get_metadata(&info_hash)
        .await
        .map_err(|e| e.to_string())
}

// Advanced features commands

#[tauri::command]
pub async fn add_web_seed(
    state: State<'_, AppState>,
    info_hash: String,
    url: String,
    seed_type: String,
) -> Result<(), String> {
    let web_seed_type = match seed_type.as_str() {
        "GetRight" => WebSeedType::GetRight,
        "WebSeed" => WebSeedType::WebSeed,
        _ => return Err("Invalid web seed type".to_string()),
    };
    
    let web_seed = WebSeed::new(url, web_seed_type);
    state
        .torrent_client
        .add_web_seed(&info_hash, web_seed)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn remove_web_seed(
    state: State<'_, AppState>,
    info_hash: String,
    url: String,
) -> Result<(), String> {
    state
        .torrent_client
        .remove_web_seed(&info_hash, &url)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_web_seeds(
    state: State<'_, AppState>,
    info_hash: String,
) -> Result<Vec<WebSeed>, String> {
    state
        .torrent_client
        .get_web_seeds(&info_hash)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_encryption_config(
    state: State<'_, AppState>,
    info_hash: String,
    enabled: bool,
    mode: String,
    prefer_encrypted: bool,
) -> Result<(), String> {
    let encryption_mode = match mode.as_str() {
        "Disabled" => EncryptionMode::Disabled,
        "Enabled" => EncryptionMode::Enabled,
        "Required" => EncryptionMode::Required,
        _ => return Err("Invalid encryption mode".to_string()),
    };
    
    let encryption = EncryptionConfig::new(enabled, encryption_mode, prefer_encrypted);
    state
        .torrent_client
        .set_encryption(&info_hash, encryption)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_encryption_config(
    state: State<'_, AppState>,
    info_hash: String,
) -> Result<EncryptionConfig, String> {
    state
        .torrent_client
        .get_encryption(&info_hash)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn add_blocked_ip(
    state: State<'_, AppState>,
    info_hash: String,
    ip: String,
) -> Result<(), String> {
    state
        .torrent_client
        .add_blocked_ip(&info_hash, ip)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn remove_blocked_ip(
    state: State<'_, AppState>,
    info_hash: String,
    ip: String,
) -> Result<(), String> {
    state
        .torrent_client
        .remove_blocked_ip(&info_hash, &ip)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_ip_filter(
    state: State<'_, AppState>,
    info_hash: String,
) -> Result<IpFilter, String> {
    state
        .torrent_client
        .get_ip_filter(&info_hash)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_ip_filter(
    state: State<'_, AppState>,
    info_hash: String,
    ip_filter: IpFilter,
) -> Result<(), String> {
    state
        .torrent_client
        .set_ip_filter(&info_hash, ip_filter)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_advanced_config(
    state: State<'_, AppState>,
    info_hash: String,
) -> Result<TorrentAdvancedConfig, String> {
    state
        .torrent_client
        .get_advanced_config(&info_hash)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_advanced_config(
    state: State<'_, AppState>,
    info_hash: String,
    config: TorrentAdvancedConfig,
) -> Result<(), String> {
    state
        .torrent_client
        .set_advanced_config(&info_hash, config)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_seed_ratio_limit(
    state: State<'_, AppState>,
    info_hash: String,
    ratio: Option<f64>,
) -> Result<(), String> {
    state
        .torrent_client
        .set_seed_ratio_limit(&info_hash, ratio)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_max_connections(
    state: State<'_, AppState>,
    info_hash: String,
    max_connections: Option<usize>,
) -> Result<(), String> {
    state
        .torrent_client
        .set_max_connections(&info_hash, max_connections)
        .await
        .map_err(|e| e.to_string())
}
