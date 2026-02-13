// src-tauri/src/services/tray_service.rs

use crate::utils::error::DownloadError;
use tauri::{App, AppHandle, Manager, Emitter};

/// Setup system tray icon and menu
/// Note: Full tray implementation requires platform-specific setup
/// For now, this is a placeholder that can be extended
pub fn setup_tray(_app: &mut App) -> Result<(), DownloadError> {
    tracing::info!("System tray setup (placeholder - to be implemented with platform-specific code)");
    
    // TODO: Implement system tray with tauri's tray APIs
    // This requires:
    // 1. Creating tray icon
    // 2. Building context menu
    // 3. Handling tray events
    // 4. Window show/hide on click
    
    Ok(())
}

/// Update tray menu with download stats
pub async fn update_tray_stats(_app: &AppHandle, active: usize, completed: usize) -> Result<(), String> {
    tracing::debug!("Tray stats: {} active, {} completed", active, completed);
    // TODO: Update tray tooltip when tray is implemented
    Ok(())
}

/// Handle tray menu item clicks
#[tauri::command]
pub async fn handle_tray_menu_click(
    app_handle: AppHandle,
    menu_id: String,
) -> Result<(), String> {
    tracing::info!("Tray menu clicked: {}", menu_id);

    match menu_id.as_str() {
        "show_hide" => {
            if let Some(window) = app_handle.get_webview_window("main") {
                if window.is_visible().unwrap_or(false) {
                    window.hide().map_err(|e| e.to_string())?;
                } else {
                    window.show().map_err(|e| e.to_string())?;
                    window.set_focus().map_err(|e| e.to_string())?;
                }
            }
        }
        "pause_all" => {
            // Call pause_all command
            app_handle.emit("tray-pause-all", ()).map_err(|e| e.to_string())?;
        }
        "resume_all" => {
            // Call resume_all command
            app_handle.emit("tray-resume-all", ()).map_err(|e| e.to_string())?;
        }
        "cancel_all" => {
            // Call cancel_all command
            app_handle.emit("tray-cancel-all", ()).map_err(|e| e.to_string())?;
        }
        "settings" => {
            // Show settings page
            app_handle.emit("tray-show-settings", ()).map_err(|e| e.to_string())?;
        }
        _ => {}
    }

    Ok(())
}
