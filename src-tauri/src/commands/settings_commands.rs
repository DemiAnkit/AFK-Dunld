// src-tauri/src/commands/settings_commands.rs

use tauri::State;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::state::app_state::AppState;

/// Application settings structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub download_path: String,
    pub max_concurrent_downloads: u32,
    pub default_segments: u8,
    pub speed_limit: u64, // 0 = unlimited
    pub theme: String, // "light", "dark", "system"
    pub start_with_system: bool,
    pub show_notifications: bool,
    pub monitor_clipboard: bool,
    pub auto_start_downloads: bool,
    pub default_category: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            download_path: String::new(),
            max_concurrent_downloads: 3,
            default_segments: 4,
            speed_limit: 0,
            theme: "system".to_string(),
            start_with_system: false,
            show_notifications: true,
            monitor_clipboard: true,
            auto_start_downloads: false,
            default_category: "general".to_string(),
        }
    }
}

/// Get all settings
#[tauri::command]
pub async fn get_settings(state: State<'_, AppState>) -> Result<AppSettings, String> {
    let settings_map = state.db.get_all_settings()
        .await
        .map_err(|e| e.to_string())?;

    Ok(map_to_settings(&settings_map))
}

/// Get a single setting value
#[tauri::command]
pub async fn get_setting(
    state: State<'_, AppState>,
    key: String,
) -> Result<Option<String>, String> {
    state.db.get_setting(&key)
        .await
        .map_err(|e| e.to_string())
}

/// Update settings
#[tauri::command]
pub async fn update_settings(
    state: State<'_, AppState>,
    settings: AppSettings,
) -> Result<(), String> {
    // Convert settings to key-value pairs and save to database
    state.db.set_setting("download_path", &settings.download_path).await.map_err(|e| e.to_string())?;
    state.db.set_setting("max_concurrent_downloads", &settings.max_concurrent_downloads.to_string()).await.map_err(|e| e.to_string())?;
    state.db.set_setting("default_segments", &settings.default_segments.to_string()).await.map_err(|e| e.to_string())?;
    state.db.set_setting("speed_limit", &settings.speed_limit.to_string()).await.map_err(|e| e.to_string())?;
    state.db.set_setting("theme", &settings.theme).await.map_err(|e| e.to_string())?;
    state.db.set_setting("start_with_system", &settings.start_with_system.to_string()).await.map_err(|e| e.to_string())?;
    state.db.set_setting("show_notifications", &settings.show_notifications.to_string()).await.map_err(|e| e.to_string())?;
    state.db.set_setting("monitor_clipboard", &settings.monitor_clipboard.to_string()).await.map_err(|e| e.to_string())?;
    state.db.set_setting("auto_start_downloads", &settings.auto_start_downloads.to_string()).await.map_err(|e| e.to_string())?;
    state.db.set_setting("default_category", &settings.default_category).await.map_err(|e| e.to_string())?;

    tracing::info!("Settings updated successfully");
    Ok(())
}

/// Reset settings to defaults
#[tauri::command]
pub async fn reset_settings(state: State<'_, AppState>) -> Result<(), String> {
    let defaults = AppSettings::default();
    update_settings(state, defaults).await
}

/// Helper function to convert database map to AppSettings
fn map_to_settings(map: &HashMap<String, String>) -> AppSettings {
    AppSettings {
        download_path: map.get("download_path").cloned().unwrap_or_default(),
        max_concurrent_downloads: map.get("max_concurrent_downloads")
            .and_then(|s| s.parse().ok())
            .unwrap_or(3),
        default_segments: map.get("default_segments")
            .and_then(|s| s.parse().ok())
            .unwrap_or(4),
        speed_limit: map.get("speed_limit")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0),
        theme: map.get("theme").cloned().unwrap_or_else(|| "system".to_string()),
        start_with_system: map.get("start_with_system")
            .and_then(|s| s.parse().ok())
            .unwrap_or(false),
        show_notifications: map.get("show_notifications")
            .and_then(|s| s.parse().ok())
            .unwrap_or(true),
        monitor_clipboard: map.get("monitor_clipboard")
            .and_then(|s| s.parse().ok())
            .unwrap_or(true),
        auto_start_downloads: map.get("auto_start_downloads")
            .and_then(|s| s.parse().ok())
            .unwrap_or(false),
        default_category: map.get("default_category")
            .cloned()
            .unwrap_or_else(|| "general".to_string()),
    }
}
