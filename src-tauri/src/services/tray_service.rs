// src-tauri/src/services/tray_service.rs

use crate::utils::error::DownloadError;
use tauri::{App, AppHandle, Manager, Emitter};
use tauri::menu::{MenuBuilder, MenuItemBuilder};
use tauri::tray::{TrayIconBuilder, MouseButton, MouseButtonState};
use tauri::image::Image;

/// Setup system tray icon and menu
pub fn setup_tray(app: &mut App) -> Result<(), DownloadError> {
    tracing::info!("Setting up system tray...");
    
    // Load tray icon - try multiple paths for dev and production
    let icon_path = if let Ok(exe_path) = std::env::current_exe() {
        let exe_dir = exe_path.parent().unwrap();
        
        // Try production path (next to exe)
        let prod_path = exe_dir.join("icons").join("32x32.png");
        if prod_path.exists() {
            tracing::info!("Using production icon path: {:?}", prod_path);
            prod_path
        } else {
            // Try dev path - go up from target/debug to workspace root
            let workspace_root = exe_dir
                .parent() // target
                .and_then(|p| p.parent()) // workspace root
                .unwrap_or(exe_dir);
            
            // First try: workspace_root/src-tauri/icons/32x32.png
            let dev_path = workspace_root.join("src-tauri").join("icons").join("32x32.png");
            if dev_path.exists() {
                tracing::info!("Using dev icon path: {:?}", dev_path);
                dev_path
            } else {
                // Second try: if we're already in src-tauri, just use icons/32x32.png
                let local_path = workspace_root.join("icons").join("32x32.png");
                if local_path.exists() {
                    tracing::info!("Using local icon path: {:?}", local_path);
                    local_path
                } else {
                    // Last resort: embedded icon bytes
                    tracing::warn!("Icon file not found, trying fallback paths");
                    tracing::warn!("Tried prod_path: {:?}", prod_path);
                    tracing::warn!("Tried dev_path: {:?}", dev_path);
                    tracing::warn!("Tried local_path: {:?}", local_path);
                    dev_path // Will fail with proper error message
                }
            }
        }
    } else {
        std::path::PathBuf::from("src-tauri/icons/32x32.png")
    };
    
    tracing::info!("Loading tray icon from: {:?}", icon_path);
    
    let icon_bytes = std::fs::read(&icon_path)
        .map_err(|e| DownloadError::Unknown(format!("Failed to read tray icon from {:?}: {}", icon_path, e)))?;
    
    let icon_image = image::load_from_memory(&icon_bytes)
        .map_err(|e| DownloadError::Unknown(format!("Failed to decode tray icon: {}", e)))?;
    
    let rgba_data = icon_image.to_rgba8();
    let (width, height) = rgba_data.dimensions();
    let icon = Image::new_owned(rgba_data.into_raw(), width, height);

    // Build tray menu
    let menu = build_tray_menu(app)?;

    // Create tray icon
    let _tray = TrayIconBuilder::with_id("main")
        .icon(icon)
        .menu(&menu)
        .tooltip("AFK-Dunld - Download Manager")
        .on_menu_event(|app: &AppHandle, event: tauri::menu::MenuEvent| {
            let menu_id = event.id().as_ref();
            tracing::debug!("Tray menu event: {}", menu_id);
            
            match menu_id {
                "show_hide" => {
                    if let Some(window) = app.get_webview_window("main") {
                        if window.is_visible().unwrap_or(false) {
                            let _ = window.hide();
                        } else {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                }
                "pause_all" => {
                    let _ = app.emit("tray-pause-all", ());
                }
                "resume_all" => {
                    let _ = app.emit("tray-resume-all", ());
                }
                "cancel_all" => {
                    let _ = app.emit("tray-cancel-all", ());
                }
                "settings" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                    let _ = app.emit("tray-show-settings", ());
                }
                "quit" => {
                    app.exit(0);
                }
                _ => {}
            }
        })
        .on_tray_icon_event(|tray: &tauri::tray::TrayIcon, event: tauri::tray::TrayIconEvent| {
            // Handle tray icon click to show/hide window
            if let tauri::tray::TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    if window.is_visible().unwrap_or(false) {
                        let _ = window.hide();
                    } else {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
            }
        })
        .build(app)
        .map_err(|e| DownloadError::Unknown(format!("Failed to build tray: {}", e)))?;

    tracing::info!("System tray initialized successfully");
    Ok(())
}

/// Build the tray menu
fn build_tray_menu(app: &mut App) -> Result<tauri::menu::Menu<tauri::Wry>, DownloadError> {
    let show_hide = MenuItemBuilder::with_id("show_hide", "Show/Hide Window")
        .build(app)
        .map_err(|e| DownloadError::Unknown(format!("Failed to create menu item: {}", e)))?;

    let separator1 = tauri::menu::PredefinedMenuItem::separator(app)
        .map_err(|e| DownloadError::Unknown(format!("Failed to create separator: {}", e)))?;

    let pause_all = MenuItemBuilder::with_id("pause_all", "Pause All Downloads")
        .build(app)
        .map_err(|e| DownloadError::Unknown(format!("Failed to create menu item: {}", e)))?;

    let resume_all = MenuItemBuilder::with_id("resume_all", "Resume All Downloads")
        .build(app)
        .map_err(|e| DownloadError::Unknown(format!("Failed to create menu item: {}", e)))?;

    let cancel_all = MenuItemBuilder::with_id("cancel_all", "Cancel All Downloads")
        .build(app)
        .map_err(|e| DownloadError::Unknown(format!("Failed to create menu item: {}", e)))?;

    let separator2 = tauri::menu::PredefinedMenuItem::separator(app)
        .map_err(|e| DownloadError::Unknown(format!("Failed to create separator: {}", e)))?;

    let settings = MenuItemBuilder::with_id("settings", "Settings")
        .build(app)
        .map_err(|e| DownloadError::Unknown(format!("Failed to create menu item: {}", e)))?;

    let separator3 = tauri::menu::PredefinedMenuItem::separator(app)
        .map_err(|e| DownloadError::Unknown(format!("Failed to create separator: {}", e)))?;

    let quit = MenuItemBuilder::with_id("quit", "Quit")
        .build(app)
        .map_err(|e| DownloadError::Unknown(format!("Failed to create menu item: {}", e)))?;

    let menu = MenuBuilder::new(app)
        .item(&show_hide)
        .item(&separator1)
        .item(&pause_all)
        .item(&resume_all)
        .item(&cancel_all)
        .item(&separator2)
        .item(&settings)
        .item(&separator3)
        .item(&quit)
        .build()
        .map_err(|e| DownloadError::Unknown(format!("Failed to build menu: {}", e)))?;

    Ok(menu)
}

/// Update tray menu with download stats
pub async fn update_tray_stats(app: &AppHandle, active: usize, completed: usize) -> Result<(), String> {
    tracing::debug!("Updating tray stats: {} active, {} completed", active, completed);
    
    // Update tray tooltip with current stats
    if let Some(tray) = app.tray_by_id("main") {
        let tooltip = if active > 0 {
            format!("AFK-Dunld - {} active download(s)", active)
        } else {
            "AFK-Dunld - No active downloads".to_string()
        };
        
        let _ = tray.set_tooltip(Some(&tooltip));
    }
    
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
