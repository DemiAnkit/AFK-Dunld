// src-tauri/src/services/tray_service.rs

use crate::utils::error::DownloadError;
use tauri::App;

pub fn setup_tray(_app: &mut App) -> Result<(), DownloadError> {
    // TODO: Implement system tray setup
    // For now, just return Ok to allow the app to start
    Ok(())
}
