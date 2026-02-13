// src-tauri/src/commands/download_commands.rs
use tauri::State;
use tauri::Emitter;
use uuid::Uuid;
use std::path::PathBuf;

use crate::state::app_state::{AppState, ActiveDownload};
use crate::core::download_engine::AddDownloadRequest;
use crate::network::youtube_downloader::{YouTubeDownloader, YouTubeDownloadOptions, VideoInfo, QualityOption};
use crate::core::download_task::{
    DownloadTask, DownloadStatus, DownloadProgress, FileInfo
};

/// Sanitize filename by removing or replacing invalid characters
fn sanitize_filename(filename: &str) -> String {
    // List of characters that are invalid in Windows filenames (most restrictive)
    // Also invalid on other platforms: / \ : * ? " < > |
    let invalid_chars = ['/', '\\', ':', '*', '?', '"', '<', '>', '|'];
    
    let mut sanitized = filename.to_string();
    
    // Replace invalid characters with underscore
    for ch in invalid_chars {
        sanitized = sanitized.replace(ch, "_");
    }
    
    // Remove leading/trailing spaces and dots (Windows doesn't like these)
    sanitized = sanitized.trim().trim_end_matches('.').to_string();
    
    // Remove control characters and other problematic characters
    sanitized = sanitized
        .chars()
        .filter(|c| !c.is_control())
        .collect();
    
    // Limit filename length to 200 characters (leave room for extension and path)
    if sanitized.len() > 200 {
        sanitized.truncate(200);
    }
    
    // If the result is empty, use a default name
    if sanitized.is_empty() {
        sanitized = "download".to_string();
    }
    
    sanitized
}

// Helper function to spawn download task with progress handling
async fn spawn_download_task(
    app_handle: tauri::AppHandle,
    state: &State<'_, AppState>,
    task: DownloadTask,
) -> Result<(), String> {
    let engine = state.engine.clone();
    let cancel_token = tokio_util::sync::CancellationToken::new();
    let cancel_clone = cancel_token.clone();
    let task_id = task.id;

    let (progress_tx, progress_rx) = flume::unbounded::<DownloadProgress>();

    // Progress event emitter
    let app_handle_clone = app_handle.clone();
    tokio::spawn(async move {
        while let Ok(progress) = progress_rx.recv_async().await {
            let _ = app_handle_clone.emit("download-progress", &progress);
        }
    });

    let mut task_clone = task.clone();
    let db = state.db.clone();

    let task_handle = tokio::spawn(async move {
        let result = engine.start_download(&mut task_clone, cancel_clone, progress_tx).await;

        match result {
            Ok(()) => {
                task_clone.status = DownloadStatus::Completed;
                let _ = db.update_download(&task_clone).await;
                let _ = app_handle.emit("download-complete", &task_clone);
            }
            Err(e) => {
                task_clone.status = DownloadStatus::Failed;
                task_clone.error_message = Some(e.to_string());
                let _ = db.update_download(&task_clone).await;
                let _ = app_handle.emit("download-failed", &task_clone);
            }
        }
    });

    // Store handle for pause/cancel
    let mut active = state.active_downloads.write().await;
    active.insert(task_id, ActiveDownload {
        cancel_token,
        task_handle,
        task: std::sync::Arc::new(tokio::sync::RwLock::new(task)),
    });

    Ok(())
}

#[tauri::command]
pub async fn add_download(
    app_handle: tauri::AppHandle,
    state: State<'_, AppState>,
    request: AddDownloadRequest,
) -> Result<DownloadTask, String> {
    // Check if URL is supported by yt-dlp (YouTube, Vimeo, etc.)
    if YouTubeDownloader::is_supported_url(&request.url) {
        return handle_youtube_download(app_handle, state, request).await;
    }

    // Create task via engine helper so logic stays centralized
    let mut task = state
        .engine
        .create_task(&request)
        .await
        .map_err(|e| e.to_string())?;

    // When first created, mark as downloading
    task.status = DownloadStatus::Downloading;

    // Save to database
    state.db.insert_download(&task).await
        .map_err(|e| e.to_string())?;

    // Start download in background using helper
    let task_clone = task.clone();
    spawn_download_task(app_handle, &state, task_clone).await?;

    Ok(task)
}

#[tauri::command]
pub async fn pause_download(
    app_handle: tauri::AppHandle,
    state: State<'_, AppState>,
    id: String,
) -> Result<(), String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;

    let mut active = state.active_downloads.write().await;
    if let Some(handle) = active.remove(&uuid) {
        handle.cancel_token.cancel();
        state.db.update_status(uuid, DownloadStatus::Paused)
            .await.map_err(|e| e.to_string())?;
        
        // Get updated task and emit event
        if let Some(task) = state.db.get_download(uuid).await.map_err(|e| e.to_string())? {
            let _ = app_handle.emit("download-paused", &task);
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn resume_download(
    app_handle: tauri::AppHandle,
    state: State<'_, AppState>,
    id: String,
) -> Result<(), String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;

    let mut task = state.db.get_download(uuid)
        .await
        .map_err(|e| e.to_string())?
        .ok_or("Download not found")?;

    // Update status to downloading
    task.status = DownloadStatus::Downloading;
    state.db.update_download(&task)
        .await
        .map_err(|e| e.to_string())?;
    
    // Emit event so UI updates immediately
    let _ = app_handle.emit("download-resumed", &task);

    // Re-start download with resume using helper
    spawn_download_task(app_handle.clone(), &state, task).await?;

    Ok(())
}

#[tauri::command]
pub async fn cancel_download(
    state: State<'_, AppState>,
    id: String,
) -> Result<(), String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;

    let mut active = state.active_downloads.write().await;
    if let Some(handle) = active.remove(&uuid) {
        handle.cancel_token.cancel();
    }

    state.db.update_status(uuid, DownloadStatus::Cancelled)
        .await.map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn remove_download(
    state: State<'_, AppState>,
    id: String,
    delete_file: bool,
) -> Result<(), String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;

    // Cancel if active
    let mut active = state.active_downloads.write().await;
    if let Some(handle) = active.remove(&uuid) {
        handle.cancel_token.cancel();
    }
    drop(active);

    if delete_file {
        if let Some(task) = state.db.get_download(uuid)
            .await.map_err(|e| e.to_string())? 
        {
            let _ = tokio::fs::remove_file(&task.save_path).await;
        }
    }

    state.db.delete_download(uuid)
        .await.map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn retry_download(
    app_handle: tauri::AppHandle,
    state: State<'_, AppState>,
    id: String,
) -> Result<(), String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;

    let task = state.db.get_download(uuid)
        .await
        .map_err(|e| e.to_string())?
        .ok_or("Download not found")?;

    // Reset status in DB
    state
        .db
        .update_status(uuid, DownloadStatus::Queued)
        .await
        .map_err(|e| e.to_string())?;

    // Re-trigger download using unified AddDownloadRequest
    let request = AddDownloadRequest {
        url: task.url.clone(),
        save_path: task
            .save_path
            .parent()
            .map(|p| p.to_string_lossy().to_string()),
        segments: Some(task.segments),
        max_retries: Some(task.retry_count),
        expected_checksum: task.expected_checksum.clone(),
        checksum_type: task
            .checksum_algorithm
            .as_ref()
            .map(|alg| alg.to_string()),
        file_name: Some(task.file_name.clone()),
        category: task.category.clone(),
        priority: Some(task.priority),
        youtube_format: None,
        youtube_quality: None,
        youtube_video_format: None,
        youtube_audio_format: None,
    };

    add_download(app_handle, state, request).await?;

    Ok(())
}

#[tauri::command]
pub async fn get_all_downloads(
    state: State<'_, AppState>,
) -> Result<Vec<DownloadTask>, String> {
    state.db.get_all_downloads()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_file_info(
    state: State<'_, AppState>,
    url: String,
) -> Result<FileInfo, String> {
    state.engine
        .get_file_info(&url)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn add_batch_downloads(
    app_handle: tauri::AppHandle,
    state: State<'_, AppState>,
    urls: Vec<String>,
    save_path: Option<String>,
) -> Result<Vec<DownloadTask>, String> {
    let mut tasks = Vec::new();
    for url in urls {
        let request = AddDownloadRequest {
            url,
            save_path: save_path.clone(),
            segments: None,
            max_retries: None,
            expected_checksum: None,
            checksum_type: None,
            file_name: None,
            category: None,
            priority: None,
            youtube_format: None,
            youtube_quality: None,
            youtube_video_format: None,
            youtube_audio_format: None,
        };

        let task = add_download(app_handle.clone(), state.clone(), request).await?;
        tasks.push(task);
    }
    Ok(tasks)
}

// Additional command placeholders
#[tauri::command]
pub async fn get_download_progress(
    _state: State<'_, AppState>,
    _id: String,
) -> Result<Option<DownloadProgress>, String> {
    // TODO: Implement
    Ok(None)
}

#[tauri::command]
pub async fn pause_all(
    app_handle: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Result<Vec<String>, String> {
    let mut paused_ids = Vec::new();
    
    // Get all active download IDs
    let active_ids: Vec<Uuid> = {
        let active = state.active_downloads.read().await;
        active.keys().copied().collect()
    };
    
    // Pause each active download
    for uuid in active_ids {
        let mut active = state.active_downloads.write().await;
        if let Some(handle) = active.remove(&uuid) {
            handle.cancel_token.cancel();
            drop(active); // Release lock before database operation
            
            // Update database status
            if let Err(e) = state.db.update_status(uuid, DownloadStatus::Paused).await {
                tracing::error!("Failed to update status for {}: {}", uuid, e);
                continue;
            }
            
            // Get updated task and emit event
            if let Ok(Some(task)) = state.db.get_download(uuid).await {
                let _ = app_handle.emit("download-paused", &task);
            }
            
            paused_ids.push(uuid.to_string());
        }
    }
    
    tracing::info!("Paused {} downloads", paused_ids.len());
    Ok(paused_ids)
}

#[tauri::command]
pub async fn resume_all(
    app_handle: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Result<Vec<String>, String> {
    let mut resumed_ids = Vec::new();
    
    // Get all paused downloads from database
    let all_downloads = state.db.get_all_downloads()
        .await
        .map_err(|e| e.to_string())?;
    
    let paused_downloads: Vec<DownloadTask> = all_downloads
        .into_iter()
        .filter(|task| task.status == DownloadStatus::Paused)
        .collect();
    
    // Resume each paused download
    for mut task in paused_downloads {
        // Update status to downloading
        task.status = DownloadStatus::Downloading;
        
        if let Err(e) = state.db.update_download(&task).await {
            tracing::error!("Failed to update task {}: {}", task.id, e);
            continue;
        }
        
        // Emit event so UI updates immediately
        let _ = app_handle.emit("download-resumed", &task);
        
        // Re-start download with resume
        if let Err(e) = spawn_download_task(app_handle.clone(), &state, task.clone()).await {
            tracing::error!("Failed to spawn download {}: {}", task.id, e);
            continue;
        }
        
        resumed_ids.push(task.id.to_string());
    }
    
    tracing::info!("Resumed {} downloads", resumed_ids.len());
    Ok(resumed_ids)
}

#[tauri::command]
pub async fn cancel_all(
    app_handle: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Result<Vec<String>, String> {
    let mut cancelled_ids = Vec::new();
    
    // Get all active download IDs
    let active_ids: Vec<Uuid> = {
        let active = state.active_downloads.read().await;
        active.keys().copied().collect()
    };
    
    // Cancel each active download
    for uuid in active_ids {
        let mut active = state.active_downloads.write().await;
        if let Some(handle) = active.remove(&uuid) {
            handle.cancel_token.cancel();
            drop(active); // Release lock before database operation
            
            // Update database status
            if let Err(e) = state.db.update_status(uuid, DownloadStatus::Cancelled).await {
                tracing::error!("Failed to update status for {}: {}", uuid, e);
                continue;
            }
            
            // Get updated task and emit event
            if let Ok(Some(task)) = state.db.get_download(uuid).await {
                let _ = app_handle.emit("download-cancelled", &task);
            }
            
            cancelled_ids.push(uuid.to_string());
        }
    }
    
    // Also clear the queue
    {
        let mut queue = state.queue.write().await;
        let queued = queue.get_queue();
        for uuid in queued {
            if let Err(e) = state.db.update_status(uuid, DownloadStatus::Cancelled).await {
                tracing::error!("Failed to cancel queued download {}: {}", uuid, e);
            } else {
                cancelled_ids.push(uuid.to_string());
            }
        }
        // Remove all from queue
        for uuid in queue.get_queue() {
            queue.remove(uuid);
        }
    }
    
    tracing::info!("Cancelled {} downloads", cancelled_ids.len());
    Ok(cancelled_ids)
}

#[tauri::command]
pub async fn open_file(
    state: State<'_, AppState>,
    id: String,
) -> Result<(), String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    
    let task = state.db.get_download(uuid)
        .await
        .map_err(|e| e.to_string())?
        .ok_or("Download not found")?;
    
    // Check if file exists
    if !tokio::fs::metadata(&task.save_path).await.is_ok() {
        return Err("File not found on disk".to_string());
    }
    
    // Open the file with default application
    let result = tokio::task::spawn_blocking(move || {
        opener::open(&task.save_path).map_err(|e| format!("Failed to open file: {}", e))
    })
    .await
    .map_err(|e| format!("Task failed: {}", e))?;
    
    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

#[tauri::command]
pub async fn open_file_location(
    state: State<'_, AppState>,
    id: String,
) -> Result<(), String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    
    let task = state.db.get_download(uuid)
        .await
        .map_err(|e| e.to_string())?
        .ok_or("Download not found")?;
    
    // Validate file path exists (for future use in better error messages)
    let _file_exists = tokio::fs::metadata(&task.save_path).await.is_ok();
    
    // Get the parent directory (folder containing the file)
    let folder_path = task.save_path
        .parent()
        .ok_or("Invalid file path")?
        .to_path_buf();
    
    // Check if folder exists
    if !tokio::fs::metadata(&folder_path).await.is_ok() {
        return Err("Folder not found on disk".to_string());
    }
    
    // Open the folder and select the file if possible
    let file_path = task.save_path.clone();
    #[allow(unused_variables)]
    let folder_path_clone = folder_path.clone();
    
    let result = tokio::task::spawn_blocking(move || -> Result<(), String> {
        // Try to reveal the file in the folder (platform-specific)
        #[cfg(target_os = "windows")]
        {
            use std::process::Command;
            use std::os::windows::process::CommandExt;
            
            // Normalize the path for Windows (convert forward slashes to backslashes)
            let file_path_str = file_path
                .to_string_lossy()
                .replace("/", "\\");
            
            // Windows: Use explorer with /select to highlight the file
            // CREATE_NO_WINDOW flag (0x08000000) to prevent console window from appearing
            const CREATE_NO_WINDOW: u32 = 0x08000000;
            
            let result = Command::new("explorer")
                .creation_flags(CREATE_NO_WINDOW)
                .arg(format!("/select,{}", file_path_str))
                .spawn();
            
            match result {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("Failed to open Windows Explorer: {}", e)),
            }
        }
        
        #[cfg(target_os = "macos")]
        {
            use std::process::Command;
            let file_path_str = file_path.to_string_lossy().to_string();
            let result = Command::new("open")
                .arg("-R")
                .arg(&file_path_str)
                .spawn();
            if let Err(e) = result {
                return Err(format!("Failed to open Finder: {}", e));
            }
            Ok(())
        }
        
        #[cfg(target_os = "linux")]
        {
            // On Linux, just open the folder since file selection varies by DE
            opener::open(&folder_path_clone).map_err(|e| format!("Failed to open folder: {}", e))
        }
        
        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        {
            opener::open(&folder_path_clone).map_err(|e| format!("Failed to open folder: {}", e))
        }
    })
    .await
    .map_err(|e| format!("Task failed: {}", e))?;
    
    result
}

/// Global download statistics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GlobalStats {
    pub total_downloads: u32,
    pub active_downloads: u32,
    pub queued_downloads: u32,
    pub completed_downloads: u32,
    pub failed_downloads: u32,
    pub paused_downloads: u32,
    pub total_downloaded_bytes: u64,
    pub total_size_bytes: u64,
    pub current_speed: f64,
    pub estimated_time_remaining: Option<u64>,
}

#[tauri::command]
pub async fn get_global_stats(
    state: State<'_, AppState>,
) -> Result<GlobalStats, String> {
    // Get all downloads from database
    let all_downloads = state.db.get_all_downloads()
        .await
        .map_err(|e| e.to_string())?;
    
    // Count downloads by status
    let mut stats = GlobalStats {
        total_downloads: all_downloads.len() as u32,
        active_downloads: 0,
        queued_downloads: 0,
        completed_downloads: 0,
        failed_downloads: 0,
        paused_downloads: 0,
        total_downloaded_bytes: 0,
        total_size_bytes: 0,
        current_speed: 0.0,
        estimated_time_remaining: None,
    };
    
    let mut remaining_bytes = 0u64;
    
    for task in &all_downloads {
        // Count by status
        match task.status {
            DownloadStatus::Downloading | DownloadStatus::Connecting => {
                stats.active_downloads += 1;
                stats.current_speed += task.speed;
            }
            DownloadStatus::Queued => stats.queued_downloads += 1,
            DownloadStatus::Completed => stats.completed_downloads += 1,
            DownloadStatus::Failed => stats.failed_downloads += 1,
            DownloadStatus::Paused => stats.paused_downloads += 1,
            _ => {}
        }
        
        // Accumulate bytes
        stats.total_downloaded_bytes += task.downloaded_size;
        if let Some(total) = task.total_size {
            stats.total_size_bytes += total;
            
            // Calculate remaining bytes for active/queued downloads
            if matches!(task.status, DownloadStatus::Downloading | DownloadStatus::Queued | DownloadStatus::Paused) {
                remaining_bytes += total.saturating_sub(task.downloaded_size);
            }
        }
    }
    
    // Calculate ETA if there's active speed
    if stats.current_speed > 0.0 && remaining_bytes > 0 {
        stats.estimated_time_remaining = Some((remaining_bytes as f64 / stats.current_speed) as u64);
    }
    
    Ok(stats)
}

#[tauri::command]
pub async fn set_speed_limit(
    state: State<'_, AppState>,
    limit: Option<u64>,
) -> Result<(), String> {
    // Update the speed limiter in the download engine
    state.engine.speed_limiter.set_limit(limit).await;
    
    tracing::info!(
        "Global speed limit set to: {}",
        limit.map(|l| format!("{} bytes/s", l))
            .unwrap_or_else(|| "Unlimited".to_string())
    );
    
    Ok(())
}

#[tauri::command]
pub async fn get_queue_info(
    state: State<'_, AppState>,
) -> Result<serde_json::Value, String> {
    let queue = state.queue.read().await;
    let info = queue.info();
    Ok(serde_json::to_value(&info).map_err(|e| e.to_string())?)
}

#[tauri::command]
pub async fn set_max_concurrent(
    app_handle: tauri::AppHandle,
    state: State<'_, AppState>,
    max: usize,
) -> Result<(), String> {
    let mut queue = state.queue.write().await;
    let to_start = queue.set_max_concurrent(max as u32);
    drop(queue); // Release lock before spawning tasks
    
    // Start the newly dequeued downloads
    for uuid in to_start {
        if let Ok(Some(mut task)) = state.db.get_download(uuid).await {
            task.status = DownloadStatus::Downloading;
            
            if let Err(e) = state.db.update_download(&task).await {
                tracing::error!("Failed to update task {}: {}", uuid, e);
                continue;
            }
            
            // Emit event
            let _ = app_handle.emit("download-started", &task);
            
            // Start the download
            if let Err(e) = spawn_download_task(app_handle.clone(), &state, task).await {
                tracing::error!("Failed to spawn download {}: {}", uuid, e);
            }
        }
    }
    
    tracing::info!("Max concurrent downloads set to {}", max);
    Ok(())
}

/// Check if yt-dlp is installed
#[tauri::command]
pub async fn check_ytdlp_installed() -> Result<bool, String> {
    YouTubeDownloader::check_installation()
        .await
        .map_err(|e| e.to_string())
}

/// Get video information for a URL
#[tauri::command]
pub async fn get_video_info(url: String) -> Result<VideoInfo, String> {
    let youtube_dl = YouTubeDownloader::new();
    youtube_dl
        .get_video_info(&url)
        .await
        .map_err(|e| e.to_string())
}

/// Get available quality options for a video
#[tauri::command]
pub async fn get_video_qualities(url: String) -> Result<Vec<QualityOption>, String> {
    let youtube_dl = YouTubeDownloader::new();
    youtube_dl
        .get_available_qualities(&url)
        .await
        .map_err(|e| e.to_string())
}

/// Check if URL is a playlist
#[tauri::command]
pub async fn check_is_playlist(url: String) -> Result<bool, String> {
    let youtube_dl = YouTubeDownloader::new();
    youtube_dl
        .is_playlist(&url)
        .await
        .map_err(|e| e.to_string())
}

/// Check if file exists on disk
#[tauri::command]
pub async fn check_file_exists(
    state: State<'_, AppState>,
    id: String,
) -> Result<bool, String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    
    if let Some(task) = state.db.get_download(uuid)
        .await
        .map_err(|e| e.to_string())? 
    {
        let exists = tokio::fs::metadata(&task.save_path).await.is_ok();
        Ok(exists)
    } else {
        Ok(false)
    }
}

/// Get actual file size from disk
#[tauri::command]
pub async fn get_file_size(
    state: State<'_, AppState>,
    id: String,
) -> Result<u64, String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    
    let task = state.db.get_download(uuid)
        .await
        .map_err(|e| e.to_string())?
        .ok_or("Download not found")?;
    
    // Get the actual file size from disk
    let metadata = tokio::fs::metadata(&task.save_path)
        .await
        .map_err(|e| format!("Failed to get file metadata: {}", e))?;
    
    Ok(metadata.len())
}

// YouTube download helper function
async fn handle_youtube_download(
    app_handle: tauri::AppHandle,
    state: State<'_, AppState>,
    request: AddDownloadRequest,
) -> Result<DownloadTask, String> {
    let youtube_dl = YouTubeDownloader::new();

    // Get video info first
    let video_info = youtube_dl
        .get_video_info(&request.url)
        .await
        .map_err(|e| format!("Failed to get video info: {}", e))?;

    // Determine save path and sanitize filename
    let raw_file_name = request.file_name.clone().unwrap_or(video_info.title.clone());
    
    // Sanitize filename - remove invalid characters for filesystem
    let file_name = sanitize_filename(&raw_file_name);
    
    let extension = if request.youtube_format.as_deref() == Some("audio") {
        request.youtube_audio_format.as_deref().unwrap_or("mp3")
    } else {
        request.youtube_video_format.as_deref().unwrap_or("mp4")
    };
    
    let full_file_name = format!("{}.{}", file_name, extension);
    let save_path = PathBuf::from(
        request.save_path.clone().unwrap_or_else(|| state.engine.default_download_dir().to_string_lossy().to_string())
    ).join(&full_file_name);

    // Create download task
    let task_id = Uuid::new_v4();
    let task = DownloadTask {
        id: task_id,
        url: request.url.clone(),
        final_url: None,
        file_name: full_file_name,
        save_path: save_path.clone(),
        total_size: video_info.filesize,
        downloaded_size: 0,
        status: DownloadStatus::Downloading,
        speed: 0.0,
        eta: None,
        segments: 1,
        supports_range: false,
        content_type: Some("video/mp4".to_string()),
        etag: None,
        expected_checksum: None,
        actual_checksum: None,
        checksum_algorithm: None,
        retry_count: 0,
        error_message: None,
        created_at: chrono::Utc::now().naive_utc(),
        completed_at: None,
        priority: request.priority.unwrap_or(0),
        category: Some("youtube".to_string()),
        segment_progress: vec![],
    };

    // Save to database
    state.db.insert_download(&task).await.map_err(|e| e.to_string())?;

    // Download in background
    let options = YouTubeDownloadOptions {
        url: request.url.clone(),
        format_type: request.youtube_format.unwrap_or("video".to_string()),
        video_quality: request.youtube_quality.unwrap_or("best".to_string()),
        video_format: request.youtube_video_format.unwrap_or("mp4".to_string()),
        audio_format: request.youtube_audio_format.unwrap_or("mp3".to_string()),
        save_path: save_path.clone(),
        is_playlist: false,  // Default to single video
        output_filename: Some(file_name.clone()),
    };

    let task_clone = task.clone();
    let db = state.db.clone();
    let app_handle_clone = app_handle.clone();

    // Spawn the download task in background using Tauri's runtime
    // Create a new YouTubeDownloader instance inside the spawn to avoid Send issues
    tauri::async_runtime::spawn(async move {
        let youtube_dl = YouTubeDownloader::new();
        match youtube_dl.download(options).await {
            Ok(final_path) => {
                tracing::info!("YouTube download completed successfully: {:?}", final_path);
                
                // Get actual file size from disk
                let actual_size = tokio::fs::metadata(&final_path)
                    .await
                    .ok()
                    .map(|m| m.len());
                
                // Extract the actual filename from the path
                let actual_filename = final_path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("downloaded_video")
                    .to_string();
                
                let mut completed_task = task_clone;
                completed_task.status = DownloadStatus::Completed;
                completed_task.completed_at = Some(chrono::Utc::now().naive_utc());
                completed_task.save_path = final_path.clone();
                completed_task.file_name = actual_filename; // Update with actual filename including extension
                completed_task.total_size = actual_size; // Update with actual file size
                completed_task.downloaded_size = actual_size.unwrap_or(0); // Set downloaded size
                
                if let Err(e) = db.update_download(&completed_task).await {
                    tracing::error!("Failed to update completed download in DB: {}", e);
                }
                if let Err(e) = app_handle_clone.emit("download-complete", &completed_task) {
                    tracing::error!("Failed to emit download-complete event: {}", e);
                }
            }
            Err(e) => {
                tracing::error!("YouTube download failed: {}", e);
                let mut failed_task = task_clone;
                failed_task.status = DownloadStatus::Failed;
                failed_task.error_message = Some(e.to_string());
                if let Err(e) = db.update_download(&failed_task).await {
                    tracing::error!("Failed to update failed download in DB: {}", e);
                }
                if let Err(e) = app_handle_clone.emit("download-failed", &failed_task) {
                    tracing::error!("Failed to emit download-failed event: {}", e);
                }
            }
        }
    });

    Ok(task)
}
