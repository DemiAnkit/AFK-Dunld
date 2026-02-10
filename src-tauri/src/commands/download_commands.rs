// src-tauri/src/commands/download_commands.rs
use tauri::State;
use uuid::Uuid;
use std::path::PathBuf;

use crate::state::app_state::{AppState, DownloadHandle};
use crate::core::download_engine::{
    DownloadTask, DownloadStatus, DownloadProgress, FileInfo
};

#[derive(serde::Deserialize)]
pub struct AddDownloadRequest {
    pub url: String,
    pub save_path: Option<String>,
    pub segments: Option<u8>,
}

#[tauri::command]
pub async fn add_download(
    app_handle: tauri::AppHandle,
    state: State<'_, AppState>,
    request: AddDownloadRequest,
) -> Result<DownloadTask, String> {
    // Get file info
    let file_info = state.download_engine
        .get_file_info(&request.url)
        .await
        .map_err(|e| e.to_string())?;

    let config = state.config.read().await;
    let save_dir = request.save_path
        .map(PathBuf::from)
        .unwrap_or_else(|| config.default_download_path.clone());
    let segments = request.segments.unwrap_or(config.max_segments);
    drop(config);

    let save_path = save_dir.join(&file_info.file_name);

    let mut task = DownloadTask {
        id: Uuid::new_v4(),
        url: request.url.clone(),
        file_name: file_info.file_name,
        save_path,
        total_size: file_info.total_size,
        downloaded_size: 0,
        status: DownloadStatus::Downloading,
        speed: 0.0,
        eta: None,
        segments,
        created_at: chrono::Local::now().naive_local(),
        completed_at: None,
        checksum: None,
        error_message: None,
    };

    // Save to database
    state.db.insert_download(&task).await
        .map_err(|e| e.to_string())?;

    // Start download in background
    let engine = state.download_engine.clone();
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
    active.insert(task_id, DownloadHandle {
        cancel_token,
        task_handle,
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
    let engine = state.download_engine.clone();
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
    active.insert(uuid, DownloadHandle {
        cancel_token,
        task_handle,
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

    // Reset and restart
    state.db.update_status(uuid, DownloadStatus::Queued)
        .await.map_err(|e| e.to_string())?;

    // Re-trigger download
    add_download(
        app_handle,
        state,
        AddDownloadRequest {
            url: task.url,
            save_path: task.save_path.parent()
                .map(|p| p.to_string_lossy().to_string()),
            segments: Some(task.segments),
        },
    ).await?;

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
pub async fn get_download_info(
    state: State<'_, AppState>,
    url: String,
) -> Result<FileInfo, String> {
    state.download_engine
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
        let task = add_download(
            app_handle.clone(),
            state.clone(),
            AddDownloadRequest {
                url,
                save_path: save_path.clone(),
                segments: None,
            },
        ).await?;
        tasks.push(task);
    }
    Ok(tasks)
}