use crate::state::app_state::AppState;
use crate::core::download_task::DownloadTask;
use crate::database::models::DownloadStatus;
use tauri::State;
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadHistoryItem {
    pub id: String,
    pub url: String,
    pub file_name: String,
    pub total_size: Option<u64>,
    pub status: String,
    pub completed_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub category: Option<String>,
    pub download_speed_avg: f64, // Average speed in bytes/sec
    pub download_time: Option<i64>, // Time taken in seconds
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryStats {
    pub total_downloads: usize,
    pub completed_downloads: usize,
    pub failed_downloads: usize,
    pub total_bytes_downloaded: u64,
    pub average_speed: f64,
    pub most_downloaded_category: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryFilter {
    pub status: Option<String>,
    pub category: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub search_query: Option<String>,
    pub limit: Option<usize>,
}

/// Get download history with optional filters
#[tauri::command]
pub async fn get_download_history(
    state: State<'_, AppState>,
    filter: Option<HistoryFilter>,
) -> Result<Vec<DownloadHistoryItem>, String> {
    let downloads = state.db.get_all_downloads()
        .await
        .map_err(|e| e.to_string())?;
    
    let mut history: Vec<DownloadHistoryItem> = downloads
        .into_iter()
        .map(|task| {
            let download_time = if let (Some(completed), created) = (task.completed_at, task.created_at) {
                Some((completed.and_utc().timestamp() - created.and_utc().timestamp()).max(0))
            } else {
                None
            };
            
            let download_speed_avg = if let (Some(size), Some(time)) = (task.total_size, download_time) {
                if time > 0 {
                    size as f64 / time as f64
                } else {
                    0.0
                }
            } else {
                0.0
            };
            
            DownloadHistoryItem {
                id: task.id.to_string(),
                url: task.url.clone(),
                file_name: task.file_name.clone(),
                total_size: task.total_size,
                status: format!("{:?}", task.status),
                completed_at: task.completed_at,
                created_at: task.created_at,
                category: task.category.clone(),
                download_speed_avg,
                download_time,
            }
        })
        .collect();
    
    // Apply filters
    if let Some(f) = filter {
        // Filter by status
        if let Some(status) = f.status {
            history.retain(|h| h.status.to_lowercase().contains(&status.to_lowercase()));
        }
        
        // Filter by category
        if let Some(category) = f.category {
            history.retain(|h| {
                h.category.as_ref().map(|c| c == &category).unwrap_or(false)
            });
        }
        
        // Filter by date range
        if let Some(date_from) = f.date_from {
            if let Ok(from_date) = chrono::NaiveDateTime::parse_from_str(&date_from, "%Y-%m-%d %H:%M:%S") {
                history.retain(|h| h.created_at >= from_date);
            }
        }
        
        if let Some(date_to) = f.date_to {
            if let Ok(to_date) = chrono::NaiveDateTime::parse_from_str(&date_to, "%Y-%m-%d %H:%M:%S") {
                history.retain(|h| h.created_at <= to_date);
            }
        }
        
        // Filter by search query
        if let Some(query) = f.search_query {
            let query_lower = query.to_lowercase();
            history.retain(|h| {
                h.file_name.to_lowercase().contains(&query_lower) ||
                h.url.to_lowercase().contains(&query_lower)
            });
        }
        
        // Apply limit
        if let Some(limit) = f.limit {
            history.truncate(limit);
        }
    }
    
    // Sort by created_at descending (newest first)
    history.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    
    Ok(history)
}

/// Get history statistics
#[tauri::command]
pub async fn get_history_stats(
    state: State<'_, AppState>,
) -> Result<HistoryStats, String> {
    let downloads = state.db.get_all_downloads()
        .await
        .map_err(|e| e.to_string())?;
    
    let total_downloads = downloads.len();
    let completed_downloads = downloads.iter()
        .filter(|d| matches!(d.status, DownloadStatus::Completed))
        .count();
    let failed_downloads = downloads.iter()
        .filter(|d| matches!(d.status, DownloadStatus::Failed))
        .count();
    
    let total_bytes_downloaded: u64 = downloads.iter()
        .filter(|d| matches!(d.status, DownloadStatus::Completed))
        .filter_map(|d| d.total_size)
        .sum();
    
    // Calculate average speed
    let speeds: Vec<f64> = downloads.iter()
        .filter(|d| matches!(d.status, DownloadStatus::Completed))
        .filter_map(|task| {
            if let (Some(completed), created, Some(size)) = (task.completed_at, task.created_at, task.total_size) {
                let time = (completed.and_utc().timestamp() - created.and_utc().timestamp()).max(1);
                Some(size as f64 / time as f64)
            } else {
                None
            }
        })
        .collect();
    
    let average_speed = if !speeds.is_empty() {
        speeds.iter().sum::<f64>() / speeds.len() as f64
    } else {
        0.0
    };
    
    // Find most downloaded category
    let mut category_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for task in downloads.iter() {
        if let Some(category) = &task.category {
            *category_counts.entry(category.clone()).or_insert(0) += 1;
        }
    }
    
    let most_downloaded_category = category_counts.iter()
        .max_by_key(|(_, count)| *count)
        .map(|(cat, _)| cat.clone());
    
    Ok(HistoryStats {
        total_downloads,
        completed_downloads,
        failed_downloads,
        total_bytes_downloaded,
        average_speed,
        most_downloaded_category,
    })
}

/// Clear download history (completed and failed only)
#[tauri::command]
pub async fn clear_download_history(
    state: State<'_, AppState>,
    clear_completed: bool,
    clear_failed: bool,
) -> Result<usize, String> {
    let downloads = state.db.get_all_downloads()
        .await
        .map_err(|e| e.to_string())?;
    
    let mut cleared_count = 0;
    
    for task in downloads {
        let should_delete = match task.status {
            DownloadStatus::Completed if clear_completed => true,
            DownloadStatus::Failed if clear_failed => true,
            _ => false,
        };
        
        if should_delete {
            state.db.delete_download(task.id)
                .await
                .map_err(|e| e.to_string())?;
            cleared_count += 1;
        }
    }
    
    Ok(cleared_count)
}

/// Export history to JSON
#[tauri::command]
pub async fn export_history(
    state: State<'_, AppState>,
    file_path: String,
) -> Result<String, String> {
    let history = get_download_history(state.clone(), None).await?;
    let stats = get_history_stats(state).await?;
    
    let export_data = serde_json::json!({
        "exported_at": chrono::Utc::now().to_rfc3339(),
        "stats": stats,
        "downloads": history,
    });
    
    let json_str = serde_json::to_string_pretty(&export_data)
        .map_err(|e| e.to_string())?;
    
    tokio::fs::write(&file_path, json_str)
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(format!("History exported to {}", file_path))
}
