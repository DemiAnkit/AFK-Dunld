// src-tauri/src/services/clipboard_service.rs

use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_clipboard_manager::ClipboardExt;
use regex::Regex;
use std::sync::Arc;
use tokio::sync::RwLock;

/// URL patterns to detect download links
const URL_PATTERNS: &[&str] = &[
    r"https?://[^\s<>]+\.(zip|rar|7z|tar|gz|exe|msi|dmg|deb|rpm|apk)",
    r"https?://[^\s<>]+\.(mp4|mkv|avi|mov|webm|mp3|flac|wav|aac)",
    r"https?://[^\s<>]+\.(pdf|doc|docx|xls|xlsx|ppt|pptx)",
    r"https?://[^\s<>]+\.(jpg|jpeg|png|gif|webp|svg|bmp)",
    r"https?://[^\s<>]+\.(iso|img|bin)",
    r"https?://(?:www\.)?(?:youtube\.com|youtu\.be)/[^\s<>]+",
    r"https?://[^\s<>]+/download[^\s<>]*",
    r"https?://[^\s<>]+\?.*download.*",
];

/// Clipboard monitoring service
pub struct ClipboardMonitor {
    last_content: Arc<RwLock<String>>,
    url_regex: Regex,
    enabled: Arc<RwLock<bool>>,
}

impl ClipboardMonitor {
    pub fn new() -> Self {
        // Combine all URL patterns
        let combined_pattern = format!("({})", URL_PATTERNS.join("|"));
        let url_regex = Regex::new(&combined_pattern).unwrap();

        Self {
            last_content: Arc::new(RwLock::new(String::new())),
            url_regex,
            enabled: Arc::new(RwLock::new(true)),
        }
    }

    /// Check if monitoring is enabled
    pub async fn is_enabled(&self) -> bool {
        *self.enabled.read().await
    }

    /// Enable or disable monitoring
    pub async fn set_enabled(&self, enabled: bool) {
        *self.enabled.write().await = enabled;
        tracing::info!("Clipboard monitoring {}", if enabled { "enabled" } else { "disabled" });
    }

    /// Check clipboard for download URLs
    pub async fn check_clipboard(&self, app: &AppHandle) -> Result<(), String> {
        if !self.is_enabled().await {
            return Ok(());
        }

        // Read clipboard text
        let clipboard_text = match app.clipboard().read_text() {
            Ok(text) => text,
            Err(e) => {
                tracing::debug!("Failed to read clipboard: {}", e);
                return Ok(()); // Don't error out, just skip this check
            }
        };

        // Check if content changed
        let mut last_content = self.last_content.write().await;
        if clipboard_text == *last_content {
            return Ok(()); // Same content, ignore
        }

        // Update last content
        *last_content = clipboard_text.clone();
        drop(last_content);

        // Check for URLs
        if let Some(url) = self.extract_url(&clipboard_text) {
            tracing::info!("Detected download URL in clipboard: {}", url);
            
            // Emit event to frontend
            if let Err(e) = app.emit("clipboard-url-detected", url) {
                tracing::error!("Failed to emit clipboard event: {}", e);
            }
        }

        Ok(())
    }

    /// Extract download URL from text
    fn extract_url(&self, text: &str) -> Option<String> {
        // First try to match against our patterns
        if let Some(captures) = self.url_regex.captures(text) {
            if let Some(matched) = captures.get(0) {
                return Some(matched.as_str().to_string());
            }
        }

        // Fallback: check if the entire text looks like a URL
        if text.starts_with("http://") || text.starts_with("https://") {
            // Simple URL validation
            if text.len() < 2048 && !text.contains(' ') && !text.contains('\n') {
                return Some(text.trim().to_string());
            }
        }

        None
    }
}

/// Start clipboard monitoring service
pub async fn start_monitoring(app_handle: AppHandle) {
    tracing::info!("Starting clipboard monitoring service...");

    let monitor = Arc::new(ClipboardMonitor::new());
    
    // Check if monitoring should be enabled from settings
    if let Some(state) = app_handle.try_state::<crate::state::app_state::AppState>() {
        match state.db.get_setting("monitor_clipboard").await {
            Ok(Some(value)) => {
                let enabled = value.parse::<bool>().unwrap_or(true);
                monitor.set_enabled(enabled).await;
            }
            _ => {
                // Default to enabled
                monitor.set_enabled(true).await;
            }
        }
    }

    // Monitor loop - check every 2 seconds
    loop {
        if let Err(e) = monitor.check_clipboard(&app_handle).await {
            tracing::error!("Clipboard check error: {}", e);
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    }
}

/// Command to enable/disable clipboard monitoring
#[tauri::command]
pub async fn set_clipboard_monitoring(
    app_handle: AppHandle,
    enabled: bool,
) -> Result<(), String> {
    // Save to settings
    if let Some(state) = app_handle.try_state::<crate::state::app_state::AppState>() {
        state.db.set_setting("monitor_clipboard", &enabled.to_string())
            .await
            .map_err(|e| e.to_string())?;
    }

    tracing::info!("Clipboard monitoring set to: {}", enabled);
    Ok(())
}
