// src-tauri/src/commands/download_commands.rs
use tauri::State;
use tauri::Emitter;
use uuid::Uuid;
use std::path::PathBuf;

use crate::state::app_state::{AppState, ActiveDownload};
use crate::core::download_engine::AddDownloadRequest;
use crate::core::download_task::{
    DownloadTask, DownloadStatus, DownloadProgress, FileInfo
};

#[tauri::command]
pub async fn add_download(
    app_handle: tauri::AppHandle,
    state: State<'_, AppState>,
    request: AddDownloadRequest,
) -> Result<DownloadTask, String> {
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

    // Start download in background
    let engine = state.engine.clone();
    let cancel_token = tokio_util::sync::CancellationToken::new();
    let cancel_clone = cancel_token.clone();
    let task_id = task.id;

    let (progress_tx, progress_rx) = flume::unbounded::<DownloadProgress>();

    // Progress event emitter
    let app_handle_clone = app_handle.clone();
    tokio::spawn(async move {
        while let Ok(progress) = progress_rx.recv_async().await {
            let _ = app_handle_clone.emit(
                "download-progress",
                &progress,
            );
        }
    });

    let mut task_clone = task.clone();
    let db = state.db.clone();

    let task_handle = tokio::spawn(async move {
        let result = engine.start_download(
            &mut task_clone,
            cancel_clone,
            progress_tx,
        ).await;

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
        task: std::sync::Arc::new(tokio::sync::RwLock::new(task.clone())),
    });

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

    // Re-start download with resume
    let engine = state.engine.clone();
    let cancel_token = tokio_util::sync::CancellationToken::new();
    let cancel_clone = cancel_token.clone();
    let (progress_tx, progress_rx) = flume::unbounded();

    let app_handle_clone = app_handle.clone();
    tokio::spawn(async move {
        while let Ok(progress) = progress_rx.recv_async().await {
            let _ = app_handle_clone.emit("download-progress", &progress);
        }
    });

    let mut task_clone = task.clone();
    let db = state.db.clone();

    let task_handle = tokio::spawn(async move {
        let result = engine.start_download(
            &mut task_clone,
            cancel_clone,
            progress_tx,
        ).await;

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
            }
        }
    });

    let mut active = state.active_downloads.write().await;
    active.insert(uuid, ActiveDownload {
        cancel_token,
        task_handle,
        task: std::sync::Arc::new(tokio::sync::RwLock::new(task)),
    });

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
