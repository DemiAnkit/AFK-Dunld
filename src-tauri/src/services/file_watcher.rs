// src-tauri/src/services/file_watcher.rs

use std::time::Duration;
use tracing::{debug, error, info};
use crate::state::app_state::AppState;
use crate::core::download_task::DownloadStatus;
use tauri::Emitter;

/// File watcher service that monitors downloaded files and syncs with database
pub struct FileWatcher;

impl FileWatcher {
    /// Start the file watcher service
    pub fn start(app_handle: tauri::AppHandle, state: AppState) {
        tauri::async_runtime::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(10)); // Check every 10 seconds
            
            loop {
                interval.tick().await;
                
                if let Err(e) = Self::sync_files(&app_handle, &state).await {
                    error!("File watcher sync error: {}", e);
                }
            }
        });
        
        info!("File watcher service started");
    }
    
    /// Sync files between disk and database
    async fn sync_files(
        app_handle: &tauri::AppHandle,
        state: &AppState,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Get all completed downloads from database
        let downloads = state.db.get_all_downloads().await?;
        
        let mut deleted_count = 0;
        
        for download in downloads {
            // Only check completed downloads
            if download.status != DownloadStatus::Completed {
                continue;
            }
            
            // Check if file exists on disk
            let file_exists = tokio::fs::metadata(&download.save_path).await.is_ok();
            
            if !file_exists {
                // File was deleted from disk - mark as missing in database
                debug!(
                    "File not found on disk, marking as missing: {} (ID: {})",
                    download.file_name,
                    download.id
                );
                
                // Update status to Failed with appropriate message
                state.db.update_status(
                    download.id,
                    DownloadStatus::Failed,
                ).await?;
                
                // Emit event to notify frontend
                let _ = app_handle.emit("file-deleted", serde_json::json!({
                    "id": download.id.to_string(),
                    "file_name": download.file_name,
                    "message": "File was deleted from download folder"
                }));
                
                deleted_count += 1;
            }
        }
        
        if deleted_count > 0 {
            info!("File watcher: Marked {} downloads as missing", deleted_count);
        }
        
        Ok(())
    }
    
    /// Manually trigger a sync (useful for user-initiated checks)
    pub async fn manual_sync(
        app_handle: &tauri::AppHandle,
        state: &AppState,
    ) -> Result<usize, String> {
        Self::sync_files(app_handle, state)
            .await
            .map_err(|e| e.to_string())?;
        
        // Count missing files
        let downloads = state.db.get_all_downloads()
            .await
            .map_err(|e| e.to_string())?;
        
        let missing_count = downloads
            .iter()
            .filter(|d| d.status == DownloadStatus::Failed)
            .count();
        
        Ok(missing_count)
    }
}
