use tauri::State;
use crate::state::app_state::AppState;
use crate::network::torrent_client_librqbit::{TorrentStats, TorrentState};
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
