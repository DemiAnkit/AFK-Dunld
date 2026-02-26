// src-tauri/src/main.rs
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod core;
mod network;
mod database;
mod commands;
mod services;
mod state;
mod utils;
mod events;

use tauri::{Manager, Emitter};
use state::app_state::AppState;
use url::Url;

// Handle deep link protocol from browser extensions
async fn handle_deep_link(
    url: String,
    app_handle: tauri::AppHandle,
    state: AppState,
) -> Result<(), String> {
    tracing::info!("Received deep link: {}", url);
    
    // Parse the URL
    let parsed_url = Url::parse(&url).map_err(|e| format!("Invalid URL: {}", e))?;
    
    // Handle different paths
    match parsed_url.path() {
        "/download" | "download" => {
            // Extract query parameters
            let query_pairs: std::collections::HashMap<String, String> = parsed_url
                .query_pairs()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect();
            
            let download_url = query_pairs.get("url")
                .ok_or("Missing URL parameter")?;
            
            let referrer = query_pairs.get("referrer").cloned();
            let filename = query_pairs.get("filename").cloned();
            
            // Add download using internal helper
            match commands::download_commands::add_download_internal(
                download_url.clone(),
                None,
                filename,
                referrer,
                state.clone(),
            ).await {
                Ok(download_id) => {
                    tracing::info!("Download added from deep link: {}", download_id);
                    
                    // Show window and bring to front
                    if let Some(window) = app_handle.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                    
                    // Send notification
                    let _ = app_handle.emit("download-added", &download_id);
                    
                    Ok(())
                }
                Err(e) => Err(format!("Failed to add download: {}", e))
            }
        }
        "/open" | "open" => {
            // Just bring app to front
            if let Some(window) = app_handle.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
            Ok(())
        }
        _ => {
            Err(format!("Unknown deep link path: {}", parsed_url.path()))
        }
    }
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("super_downloader=debug")
        .init();

    // Check if running in native messaging mode
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "--native-messaging" {
        // Run as native messaging host for browser extension
        tracing::info!("Starting in native messaging mode");
        
        // We need to run the native messaging host synchronously
        // For this, we'll use tokio runtime directly
        let runtime = tokio::runtime::Runtime::new().expect("Failed to create runtime");
        runtime.block_on(async {
            // Create a minimal app handle for native messaging
            // This is a simplified version that doesn't need the full Tauri app
            use std::io::{self, Read, Write};
            use serde_json::json;
            
            let mut stdin = io::stdin();
            let mut stdout = io::stdout();
            
            loop {
                let mut length_bytes = [0u8; 4];
                if let Err(e) = stdin.read_exact(&mut length_bytes) {
                    if e.kind() == io::ErrorKind::UnexpectedEof {
                        break;
                    }
                    tracing::error!("Failed to read message length: {}", e);
                    break;
                }
                
                let length = u32::from_le_bytes(length_bytes) as usize;
                if length == 0 || length > 1024 * 1024 {
                    break;
                }
                
                let mut buffer = vec![0u8; length];
                if let Err(e) = stdin.read_exact(&mut buffer) {
                    tracing::error!("Failed to read message: {}", e);
                    break;
                }
                
                // Parse the message
                if let Ok(msg) = serde_json::from_slice::<serde_json::Value>(&buffer) {
                    tracing::debug!("Received message: {:?}", msg);
                    
                    // Simple response for now - in production, this would integrate with the app
                    let response = match msg.get("type").and_then(|t| t.as_str()) {
                        Some("ping") => json!({
                            "type": "pong",
                            "version": env!("CARGO_PKG_VERSION"),
                            "app_name": "AFK-Dunld"
                        }),
                        Some("add_download") => {
                            // TODO: Queue the download to be added when app starts
                            json!({
                                "type": "download_added",
                                "success": true,
                                "message": "Download queued"
                            })
                        },
                        _ => json!({
                            "type": "error",
                            "message": "Unknown message type"
                        })
                    };
                    
                    // Send response
                    if let Ok(response_str) = serde_json::to_string(&response) {
                        let response_len = (response_str.len() as u32).to_le_bytes();
                        let _ = stdout.write_all(&response_len);
                        let _ = stdout.write_all(response_str.as_bytes());
                        let _ = stdout.flush();
                    }
                } else {
                    tracing::error!("Failed to parse message");
                    break;
                }
            }
        });
        
        return;
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_single_instance::init(|_app, _args, _cwd| {}))
        .plugin(tauri_plugin_deep_link::init())
        .setup(|app| {
            // Get app data directory
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data directory");

            let app_handle = app.handle().clone();
            let app_state = tauri::async_runtime::block_on(async {
                AppState::new(app_data_dir, &app_handle).await.expect("Failed to initialize app state")
            });

            app.manage(app_state.clone());

            // Setup system tray
            services::tray_service::setup_tray(app)?;

            // Setup deep link handler for browser extension protocol (Tauri v2)
            let app_handle = app.handle().clone();
            let state_for_deeplink = app_state.clone();
            
            #[cfg(any(target_os = "macos", target_os = "linux", target_os = "windows"))]
            {
                use tauri_plugin_deep_link::DeepLinkExt;
                
                app.handle().plugin(
                    tauri_plugin_deep_link::Builder::new()
                        .with_handler(move |url| {
                            tracing::info!("Deep link received: {}", url);
                            
                            let handle = app_handle.clone();
                            let state = state_for_deeplink.clone();
                            
                            tauri::async_runtime::spawn(async move {
                                if let Err(e) = handle_deep_link(url, handle, state).await {
                                    tracing::error!("Deep link handling failed: {}", e);
                                }
                            });
                        })
                        .build(),
                )?;
            }

            // Start clipboard monitor
            let handle = app.handle().clone();
            let _state_clone = app_state.clone();
            tauri::async_runtime::spawn(async move {
                services::clipboard_service::start_monitoring(handle).await;
            });

            // Start file watcher service
            let handle = app.handle().clone();
            let state_for_watcher = app_state.clone();
            services::file_watcher::FileWatcher::start(handle, state_for_watcher);

            // Start scheduler and listen for scheduled tasks
            let state_for_scheduler = app_state.clone();
            tauri::async_runtime::spawn(async move {
                // Start the scheduler
                if let Err(e) = state_for_scheduler.scheduler.start().await {
                    tracing::error!("Failed to start scheduler: {}", e);
                    return;
                }

                // Get the receiver
                let mut receiver_opt = state_for_scheduler.scheduled_task_receiver.write().await;
                if let Some(mut receiver) = receiver_opt.take() {
                    drop(receiver_opt);
                    
                    // Listen for scheduled tasks
                    while let Some(task) = receiver.recv().await {
                        tracing::info!("Scheduled task triggered: {} for download {}", task.id, task.download_id);
                        
                        // Get the download from database and start it
                        let state_clone = state_for_scheduler.clone();
                        tokio::spawn(async move {
                            // Load the download from database
                            match state_clone.db.get_download(task.download_id).await {
                                Ok(Some(download_task)) => {
                                    tracing::info!("Loaded scheduled download: {}", download_task.id);
                                    
                                    // Check if download is already active
                                    let active_downloads = state_clone.active_downloads.read().await;
                                    if active_downloads.contains_key(&download_task.id) {
                                        tracing::warn!("Download {} is already active, skipping", download_task.id);
                                        return;
                                    }
                                    drop(active_downloads);
                                    
                                    // Resume or restart the download based on status
                                    match download_task.status {
                                        core::download_task::DownloadStatus::Paused => {
                                            // Resume paused download
                                            if let Err(e) = commands::download_commands::resume_download_internal(
                                                download_task.id,
                                                state_clone.clone()
                                            ).await {
                                                tracing::error!("Failed to resume scheduled download {}: {}", download_task.id, e);
                                            } else {
                                                tracing::info!("Successfully resumed scheduled download: {}", download_task.id);
                                            }
                                        },
                                        core::download_task::DownloadStatus::Failed | 
                                        core::download_task::DownloadStatus::Cancelled => {
                                            // Retry failed/cancelled downloads
                                            if let Err(e) = commands::download_commands::retry_download_internal(
                                                download_task.id,
                                                state_clone.clone()
                                            ).await {
                                                tracing::error!("Failed to retry scheduled download {}: {}", download_task.id, e);
                                            } else {
                                                tracing::info!("Successfully retried scheduled download: {}", download_task.id);
                                            }
                                        },
                                        core::download_task::DownloadStatus::Queued => {
                                            // Start queued download
                                            if let Err(e) = commands::download_commands::add_download_internal(
                                                download_task.url.clone(),
                                                Some(download_task.save_path.to_string_lossy().to_string()),
                                                Some(download_task.file_name.clone()),
                                                None,
                                                state_clone.clone()
                                            ).await {
                                                tracing::error!("Failed to start scheduled download {}: {}", download_task.id, e);
                                            } else {
                                                tracing::info!("Successfully started scheduled download: {}", download_task.id);
                                            }
                                        },
                                        _ => {
                                            tracing::info!("Download {} is in state {:?}, no action needed", 
                                                download_task.id, download_task.status);
                                        }
                                    }
                                },
                                Ok(None) => {
                                    tracing::warn!("Scheduled download {} not found in database", task.download_id);
                                },
                                Err(e) => {
                                    tracing::error!("Failed to load scheduled download {}: {}", task.download_id, e);
                                }
                            }
                        });
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Download commands
            commands::download_commands::add_download,
            commands::download_commands::pause_download,
            commands::download_commands::resume_download,
            commands::download_commands::cancel_download,
            commands::download_commands::remove_download,
            commands::download_commands::retry_download,
            commands::download_commands::get_all_downloads,
            commands::download_commands::get_file_info,
            commands::download_commands::add_batch_downloads,
            commands::download_commands::get_download_progress,
            commands::download_commands::pause_all,
            commands::download_commands::resume_all,
            commands::download_commands::cancel_all,
            commands::download_commands::open_file,
            commands::download_commands::open_file_location,
            commands::download_commands::get_global_stats,
            commands::download_commands::set_speed_limit,
            commands::download_commands::get_queue_info,
            commands::download_commands::set_max_concurrent,
            commands::download_commands::check_file_exists,
            commands::download_commands::get_file_size,
            // YouTube/video download commands
            commands::download_commands::check_ytdlp_installed,
            commands::download_commands::get_video_info,
            commands::download_commands::get_video_qualities,
            commands::download_commands::check_is_playlist,
            // yt-dlp management commands
            commands::ytdlp_commands::update_ytdlp,
            commands::ytdlp_commands::get_ytdlp_version,
            commands::ytdlp_commands::get_bundled_ytdlp_version,
            // History commands
            commands::history_commands::get_download_history,
            commands::history_commands::get_history_stats,
            commands::history_commands::clear_download_history,
            commands::history_commands::delete_download_from_history,
            commands::history_commands::delete_downloads_bulk,
            commands::history_commands::clear_old_history,
            commands::history_commands::export_history,
            // Settings commands
            commands::settings_commands::get_settings,
            commands::settings_commands::get_setting,
            commands::settings_commands::update_settings,
            commands::settings_commands::reset_settings,
            // System commands
            commands::system_commands::get_system_info,
            commands::system_commands::open_download_folder,
            commands::system_commands::check_disk_space,
            // Scheduler commands
            commands::scheduler_commands::schedule_download,
            commands::scheduler_commands::cancel_scheduled_download,
            commands::scheduler_commands::update_scheduled_download,
            commands::scheduler_commands::get_scheduled_downloads,
            commands::scheduler_commands::start_scheduler,
            commands::scheduler_commands::stop_scheduler,
            commands::scheduler_commands::is_scheduler_running,
            // FTP commands
            commands::ftp_commands::ftp_connect,
            commands::ftp_commands::ftp_disconnect,
            commands::ftp_commands::ftp_list_files,
            commands::ftp_commands::ftp_download_file,
            commands::ftp_commands::ftp_get_file_size,
            commands::ftp_commands::ftp_upload_file,
            // SFTP commands
            commands::sftp_commands::sftp_connect,
            commands::sftp_commands::sftp_disconnect,
            commands::sftp_commands::sftp_list_files,
            commands::sftp_commands::sftp_download_file,
            commands::sftp_commands::sftp_get_file_size,
            commands::sftp_commands::sftp_upload_file,
            commands::sftp_commands::sftp_get_file_info,
            // Category commands
            commands::category_commands::get_categories,
            commands::category_commands::get_category,
            commands::category_commands::create_category,
            commands::category_commands::update_category,
            commands::category_commands::delete_category,
            commands::category_commands::get_category_stats,
            commands::category_commands::assign_download_category,
            commands::category_commands::auto_categorize_download,
            // Logging commands
            commands::logging_commands::get_logs,
            commands::logging_commands::get_logs_by_level,
            commands::logging_commands::get_logs_by_category,
            commands::logging_commands::get_logger_download_history,
            commands::logging_commands::get_performance_metrics,
            commands::logging_commands::clear_logs,
            commands::logging_commands::clear_logger_download_history,
            // Security commands
            commands::security_commands::encrypt_credential,
            commands::security_commands::decrypt_credential,
            commands::security_commands::validate_url,
            commands::security_commands::validate_file_path,
            commands::security_commands::validate_category_name,
            commands::security_commands::validate_color,
            commands::security_commands::sanitize_input,
            commands::security_commands::check_rate_limit,
            // Torrent commands
            commands::torrent_commands::add_torrent_file,
            commands::torrent_commands::add_magnet_link,
            commands::torrent_commands::get_torrent_stats,
            commands::torrent_commands::get_torrent_state,
            commands::torrent_commands::pause_torrent,
            commands::torrent_commands::resume_torrent,
            commands::torrent_commands::remove_torrent,
            commands::torrent_commands::list_torrents,
            commands::torrent_commands::get_torrent_info,
            commands::torrent_commands::set_torrent_priority,
            commands::torrent_commands::get_torrent_priority,
            commands::torrent_commands::set_torrent_bandwidth_limit,
            commands::torrent_commands::get_torrent_bandwidth_limit,
            commands::torrent_commands::set_torrent_schedule,
            commands::torrent_commands::get_torrent_schedule,
            commands::torrent_commands::is_torrent_scheduled_active,
            commands::torrent_commands::add_torrent_tag,
            commands::torrent_commands::remove_torrent_tag,
            commands::torrent_commands::set_torrent_category,
            commands::torrent_commands::get_torrent_metadata,
            commands::torrent_commands::add_web_seed,
            commands::torrent_commands::remove_web_seed,
            commands::torrent_commands::get_web_seeds,
            commands::torrent_commands::set_encryption_config,
            commands::torrent_commands::get_encryption_config,
            commands::torrent_commands::add_blocked_ip,
            commands::torrent_commands::remove_blocked_ip,
            commands::torrent_commands::get_ip_filter,
            commands::torrent_commands::set_ip_filter,
            commands::torrent_commands::get_advanced_config,
            commands::torrent_commands::set_advanced_config,
            commands::torrent_commands::set_seed_ratio_limit,
            commands::torrent_commands::set_max_connections,
            // Service commands
            services::clipboard_service::set_clipboard_monitoring,
            services::notification_service::set_notifications_enabled,
            services::notification_service::test_notification,
            services::tray_service::handle_tray_menu_click,
            // Browser extension commands
            commands::browser_commands::add_download_from_browser,
            commands::browser_commands::is_browser_extension_available,
            commands::browser_commands::install_browser_extension_support,
            commands::browser_commands::uninstall_browser_extension_support,
        ])
        .run(tauri::generate_context!())
        .expect("error while running application");
}
