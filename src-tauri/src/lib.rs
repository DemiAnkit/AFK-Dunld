// src-tauri/src/lib.rs

pub mod core;
pub mod network;
pub mod database;
pub mod commands;
pub mod services;
pub mod state;
pub mod events;
pub mod utils;

use tauri::Manager;
use state::app_state::AppState;

pub fn run() {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "super_downloader=debug,info".into()),
        )
        .with_target(true)
        .with_thread_ids(true)
        .init();

    tracing::info!("Starting Super Downloader...");

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
        .plugin(tauri_plugin_single_instance::init(|_app, _args, _cwd| {
            tracing::info!("Another instance tried to start");
        }))
        .setup(|app| {
            tracing::info!("Setting up application...");

            // Get app data directory
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data directory");

            // Initialize app state
            let app_state = tauri::async_runtime::block_on(async {
                AppState::new(app_data_dir)
                    .await
                    .expect("Failed to initialize app state")
            });

            app.manage(app_state.clone());

            // Setup system tray
            services::tray_service::setup_tray(app)?;

            // Start clipboard monitor
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                services::clipboard_service::start_monitoring(handle).await;
            });

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
                            // TODO: Implement actual download restart logic
                            // This would typically involve:
                            // 1. Loading the download from database
                            // 2. Calling add_download or resume_download
                            tracing::info!("Starting scheduled download: {}", task.download_id);
                        });
                    }
                }
            });

            tracing::info!("Application setup complete!");
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
            commands::download_commands::get_download_progress,
            commands::download_commands::get_file_info,
            commands::download_commands::add_batch_downloads,
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
            // Settings commands
            commands::settings_commands::get_settings,
            commands::settings_commands::get_setting,
            commands::settings_commands::update_settings,
            commands::settings_commands::reset_settings,
            // System commands
            commands::system_commands::get_system_info,
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
            commands::logging_commands::get_download_history,
            commands::logging_commands::get_performance_metrics,
            commands::logging_commands::clear_logs,
            commands::logging_commands::clear_download_history,
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
            // Service commands
            services::clipboard_service::set_clipboard_monitoring,
            services::notification_service::set_notifications_enabled,
            services::notification_service::test_notification,
            services::tray_service::handle_tray_menu_click,
        ])
        .run(tauri::generate_context!())
        .expect("Error while running Super Downloader");
}