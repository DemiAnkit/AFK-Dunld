// src-tauri/src/services/notification_service.rs

use tauri::{AppHandle, Manager};
use tauri_plugin_notification::NotificationExt;

/// Notification types
#[derive(Debug, Clone)]
pub enum NotificationType {
    DownloadComplete { file_name: String, size: u64 },
    DownloadFailed { file_name: String, error: String },
    AllDownloadsComplete { count: usize },
    LowDiskSpace { available: u64 },
    ClipboardUrlDetected { url: String },
}

/// Notification service
pub struct NotificationService;

impl NotificationService {
    /// Send a notification
    pub async fn send(app: &AppHandle, notification_type: NotificationType) -> Result<(), String> {
        // Check if notifications are enabled
        if !Self::is_enabled(app).await {
            return Ok(());
        }

        match notification_type {
            NotificationType::DownloadComplete { file_name, size } => {
                Self::send_download_complete(app, &file_name, size).await
            }
            NotificationType::DownloadFailed { file_name, error } => {
                Self::send_download_failed(app, &file_name, &error).await
            }
            NotificationType::AllDownloadsComplete { count } => {
                Self::send_all_complete(app, count).await
            }
            NotificationType::LowDiskSpace { available } => {
                Self::send_low_disk_space(app, available).await
            }
            NotificationType::ClipboardUrlDetected { url } => {
                Self::send_clipboard_url(app, &url).await
            }
        }
    }

    /// Check if notifications are enabled
    async fn is_enabled(app: &AppHandle) -> bool {
        if let Some(state) = app.try_state::<crate::state::app_state::AppState>() {
            match state.db.get_setting("show_notifications").await {
                Ok(Some(value)) => value.parse::<bool>().unwrap_or(true),
                _ => true, // Default to enabled
            }
        } else {
            true
        }
    }

    /// Send download complete notification
    async fn send_download_complete(app: &AppHandle, file_name: &str, size: u64) -> Result<(), String> {
        let size_str = format_bytes(size);
        
        app.notification()
            .builder()
            .title("Download Complete")
            .body(format!("{} ({})", file_name, size_str))
            .icon("download")
            .show()
            .map_err(|e| e.to_string())?;

        tracing::info!("Notification sent: Download complete - {}", file_name);
        Ok(())
    }

    /// Send download failed notification
    async fn send_download_failed(app: &AppHandle, file_name: &str, error: &str) -> Result<(), String> {
        app.notification()
            .builder()
            .title("Download Failed")
            .body(format!("{}\nError: {}", file_name, error))
            .icon("error")
            .show()
            .map_err(|e| e.to_string())?;

        tracing::info!("Notification sent: Download failed - {}", file_name);
        Ok(())
    }

    /// Send all downloads complete notification
    async fn send_all_complete(app: &AppHandle, count: usize) -> Result<(), String> {
        app.notification()
            .builder()
            .title("All Downloads Complete")
            .body(format!("{} download(s) finished successfully!", count))
            .icon("success")
            .show()
            .map_err(|e| e.to_string())?;

        tracing::info!("Notification sent: All downloads complete");
        Ok(())
    }

    /// Send low disk space notification
    async fn send_low_disk_space(app: &AppHandle, available: u64) -> Result<(), String> {
        app.notification()
            .builder()
            .title("Low Disk Space")
            .body(format!("Only {} available. Downloads may fail.", format_bytes(available)))
            .icon("warning")
            .show()
            .map_err(|e| e.to_string())?;

        tracing::warn!("Notification sent: Low disk space");
        Ok(())
    }

    /// Send clipboard URL detected notification
    async fn send_clipboard_url(app: &AppHandle, url: &str) -> Result<(), String> {
        let display_url = if url.len() > 50 {
            format!("{}...", &url[..47])
        } else {
            url.to_string()
        };

        app.notification()
            .builder()
            .title("Download Link Detected")
            .body(format!("Found in clipboard: {}", display_url))
            .icon("info")
            .show()
            .map_err(|e| e.to_string())?;

        tracing::info!("Notification sent: Clipboard URL detected");
        Ok(())
    }
}

/// Format bytes to human readable string
fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    if bytes == 0 {
        return "0 B".to_string();
    }
    let exp = (bytes as f64).log(1024.0).min(4.0) as usize;
    let value = bytes as f64 / 1024f64.powi(exp as i32);
    format!("{:.2} {}", value, UNITS[exp])
}

/// Command to enable/disable notifications
#[tauri::command]
pub async fn set_notifications_enabled(
    app_handle: AppHandle,
    enabled: bool,
) -> Result<(), String> {
    // Save to settings
    if let Some(state) = app_handle.try_state::<crate::state::app_state::AppState>() {
        state.db.set_setting("show_notifications", &enabled.to_string())
            .await
            .map_err(|e| e.to_string())?;
    }

    tracing::info!("Notifications set to: {}", enabled);
    Ok(())
}

/// Command to test notification
#[tauri::command]
pub async fn test_notification(app_handle: AppHandle) -> Result<(), String> {
    NotificationService::send(
        &app_handle,
        NotificationType::DownloadComplete {
            file_name: "test-file.zip".to_string(),
            size: 1024 * 1024 * 50, // 50 MB
        },
    )
    .await
}
