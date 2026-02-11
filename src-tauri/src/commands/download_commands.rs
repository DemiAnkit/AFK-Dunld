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
    state: State<'_, AppState>,
    id: String,
) -> Result<(), String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;

    let mut active = state.active_downloads.write().await;
    if let Some(handle) = active.remove(&uuid) {
        handle.cancel_token.cancel();
        state.db.update_status(uuid, DownloadStatus::Paused)
            .await.map_err(|e| e.to_string())?;
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

    let task = state.db.get_download(uuid)
        .await
        .map_err(|e| e.to_string())?
        .ok_or("Download not found")?;

    // Re-start download with resume using helper
    spawn_download_task(app_handle, &state, task).await?;

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
    _state: State<'_, AppState>,
) -> Result<(), String> {
    // TODO: Implement
    Ok(())
}

#[tauri::command]
pub async fn resume_all(
    _state: State<'_, AppState>,
) -> Result<(), String> {
    // TODO: Implement
    Ok(())
}

#[tauri::command]
pub async fn cancel_all(
    _state: State<'_, AppState>,
) -> Result<(), String> {
    // TODO: Implement
    Ok(())
}

#[tauri::command]
pub async fn open_file(
    _state: State<'_, AppState>,
    _id: String,
) -> Result<(), String> {
    // TODO: Implement
    Ok(())
}

#[tauri::command]
pub async fn open_file_location(
    _state: State<'_, AppState>,
    _id: String,
) -> Result<(), String> {
    // TODO: Implement
    Ok(())
}

#[tauri::command]
pub async fn get_global_stats(
    _state: State<'_, AppState>,
) -> Result<serde_json::Value, String> {
    // TODO: Implement
    Ok(serde_json::json!({}))
}

#[tauri::command]
pub async fn set_speed_limit(
    _state: State<'_, AppState>,
    _limit: Option<u64>,
) -> Result<(), String> {
    // TODO: Implement
    Ok(())
}

#[tauri::command]
pub async fn get_queue_info(
    _state: State<'_, AppState>,
) -> Result<serde_json::Value, String> {
    // TODO: Implement
    Ok(serde_json::json!({}))
}

#[tauri::command]
pub async fn set_max_concurrent(
    _state: State<'_, AppState>,
    _max: usize,
) -> Result<(), String> {
    // TODO: Implement
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

    // Determine save path
    let file_name = request.file_name.clone().unwrap_or(video_info.title.clone());
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
    };

    let task_clone = task.clone();
    let db = state.db.clone();
    let app_handle_clone = app_handle.clone();

    // Spawn the download task in background
    // Create a new YouTubeDownloader instance inside the spawn to avoid Send issues
    tokio::spawn(async move {
        let youtube_dl = YouTubeDownloader::new();
        match youtube_dl.download(options).await {
            Ok(_) => {
                let mut completed_task = task_clone;
                completed_task.status = DownloadStatus::Completed;
                completed_task.completed_at = Some(chrono::Utc::now().naive_utc());
                let _ = db.update_download(&completed_task).await;
                let _ = app_handle_clone.emit("download-complete", &completed_task);
            }
            Err(e) => {
                let mut failed_task = task_clone;
                failed_task.status = DownloadStatus::Failed;
                failed_task.error_message = Some(e.to_string());
                let _ = db.update_download(&failed_task).await;
                let _ = app_handle_clone.emit("download-failed", &failed_task);
            }
        }
    });

    Ok(task)
}
