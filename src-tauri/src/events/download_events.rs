use tauri::{AppHandle, Emitter};
use tracing::error;

use crate::core::download_task::{
    DownloadProgress, DownloadTask,
};

/// Emit download progress to the frontend
pub fn emit_progress(
    app_handle: &AppHandle,
    progress: &DownloadProgress,
) {
    if let Err(e) =
        app_handle.emit("download-progress", progress)
    {
        error!("Failed to emit progress: {}", e);
    }
}

/// Emit download completed event
pub fn emit_completed(
    app_handle: &AppHandle,
    task: &DownloadTask,
) {
    if let Err(e) = app_handle.emit("download-complete", task)
    {
        error!("Failed to emit complete: {}", e);
    }
}

/// Emit download failed event
pub fn emit_failed(
    app_handle: &AppHandle,
    task: &DownloadTask,
) {
    if let Err(e) = app_handle.emit("download-failed", task) {
        error!("Failed to emit failed: {}", e);
    }
}

/// Emit download paused event
pub fn emit_paused(
    app_handle: &AppHandle,
    task: &DownloadTask,
) {
    if let Err(e) = app_handle.emit("download-paused", task) {
        error!("Failed to emit paused: {}", e);
    }
}

/// Emit clipboard URL detected event
pub fn emit_clipboard_url(
    app_handle: &AppHandle,
    url: &str,
) {
    if let Err(e) =
        app_handle.emit("clipboard-url-detected", url)
    {
        error!("Failed to emit clipboard URL: {}", e);
    }
}

/// Emit global speed update
pub fn emit_global_speed(
    app_handle: &AppHandle,
    speed: f64,
) {
    if let Err(e) =
        app_handle.emit("global-speed-update", speed)
    {
        error!("Failed to emit global speed: {}", e);
    }
}

/// Emit download status change event for tray updates
pub fn emit_status_change(
    app_handle: &AppHandle,
    active_count: usize,
    completed_count: usize,
) {
    // Update tray tooltip with stats
    let tray_handle = app_handle.clone();
    tauri::async_runtime::spawn(async move {
        if let Err(e) = crate::services::tray_service::update_tray_stats(
            &tray_handle,
            active_count,
            completed_count,
        )
        .await
        {
            error!("Failed to update tray stats: {}", e);
        }
    });
}