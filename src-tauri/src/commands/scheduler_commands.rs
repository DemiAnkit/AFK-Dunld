use tauri::State;
use crate::core::scheduler::{ScheduledTask, RepeatInterval};
use crate::state::app_state::AppState;
use chrono::{DateTime, Utc};

#[tauri::command]
pub async fn schedule_download(
    state: State<'_, AppState>,
    download_id: String,
    scheduled_time: String, // ISO 8601 format
    repeat_interval: Option<String>,
) -> Result<String, String> {
    // Parse the scheduled time
    let scheduled_time: DateTime<Utc> = scheduled_time
        .parse()
        .map_err(|e| format!("Invalid datetime format: {}", e))?;

    // Parse repeat interval if provided
    let repeat = match repeat_interval.as_deref() {
        Some("hourly") => Some(RepeatInterval::Hourly),
        Some("daily") => Some(RepeatInterval::Daily),
        Some("weekly") => Some(RepeatInterval::Weekly),
        Some("monthly") => Some(RepeatInterval::Monthly),
        Some(custom) if custom.starts_with("custom:") => {
            let seconds: i64 = custom
                .trim_start_matches("custom:")
                .parse()
                .map_err(|e| format!("Invalid custom interval: {}", e))?;
            Some(RepeatInterval::Custom(seconds))
        }
        Some(_) => return Err("Invalid repeat interval".to_string()),
        None => None,
    };

    // Create scheduled task
    let task_id = uuid::Uuid::new_v4().to_string();
    let task = ScheduledTask {
        id: task_id.clone(),
        download_id,
        scheduled_time,
        repeat_interval: repeat,
        enabled: true,
    };

    // Add to scheduler
    state
        .scheduler
        .add_task(task)
        .await
        .map_err(|e| e.to_string())?;

    Ok(task_id)
}

#[tauri::command]
pub async fn cancel_scheduled_download(
    state: State<'_, AppState>,
    task_id: String,
) -> Result<(), String> {
    state
        .scheduler
        .remove_task(&task_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_scheduled_download(
    state: State<'_, AppState>,
    task_id: String,
    scheduled_time: Option<String>,
    repeat_interval: Option<String>,
    enabled: Option<bool>,
) -> Result<(), String> {
    // Get existing task
    let mut task = state
        .scheduler
        .get_task(&task_id)
        .await
        .ok_or_else(|| format!("Scheduled task {} not found", task_id))?;

    // Update fields
    if let Some(time_str) = scheduled_time {
        task.scheduled_time = time_str
            .parse()
            .map_err(|e| format!("Invalid datetime format: {}", e))?;
    }

    if let Some(interval) = repeat_interval {
        task.repeat_interval = match interval.as_str() {
            "none" => None,
            "hourly" => Some(RepeatInterval::Hourly),
            "daily" => Some(RepeatInterval::Daily),
            "weekly" => Some(RepeatInterval::Weekly),
            "monthly" => Some(RepeatInterval::Monthly),
            custom if custom.starts_with("custom:") => {
                let seconds: i64 = custom
                    .trim_start_matches("custom:")
                    .parse()
                    .map_err(|e| format!("Invalid custom interval: {}", e))?;
                Some(RepeatInterval::Custom(seconds))
            }
            _ => return Err("Invalid repeat interval".to_string()),
        };
    }

    if let Some(en) = enabled {
        task.enabled = en;
    }

    // Update in scheduler
    state
        .scheduler
        .update_task(task)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_scheduled_downloads(
    state: State<'_, AppState>,
) -> Result<Vec<ScheduledTask>, String> {
    Ok(state.scheduler.get_all_tasks().await)
}

#[tauri::command]
pub async fn start_scheduler(
    state: State<'_, AppState>,
) -> Result<(), String> {
    state.scheduler.start().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn stop_scheduler(
    state: State<'_, AppState>,
) -> Result<(), String> {
    state.scheduler.stop().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn is_scheduler_running(
    state: State<'_, AppState>,
) -> Result<bool, String> {
    Ok(state.scheduler.is_running().await)
}
