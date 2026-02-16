use std::collections::HashMap;
use std::sync::Arc;
use chrono::{DateTime, Utc, Duration};
use tokio::sync::{RwLock, mpsc};
use tokio::time::interval;
use serde::{Deserialize, Serialize};
use crate::utils::error::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledTask {
    pub id: String,
    pub download_id: String,
    pub scheduled_time: DateTime<Utc>,
    pub repeat_interval: Option<RepeatInterval>,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RepeatInterval {
    Hourly,
    Daily,
    Weekly,
    Monthly,
    Custom(i64), // seconds
}

impl RepeatInterval {
    pub fn to_duration(&self) -> Duration {
        match self {
            RepeatInterval::Hourly => Duration::hours(1),
            RepeatInterval::Daily => Duration::days(1),
            RepeatInterval::Weekly => Duration::weeks(1),
            RepeatInterval::Monthly => Duration::days(30),
            RepeatInterval::Custom(seconds) => Duration::seconds(*seconds),
        }
    }
}

pub struct Scheduler {
    tasks: Arc<RwLock<HashMap<String, ScheduledTask>>>,
    sender: mpsc::Sender<ScheduledTask>,
    running: Arc<RwLock<bool>>,
}

impl Scheduler {
    pub fn new() -> (Self, mpsc::Receiver<ScheduledTask>) {
        let (sender, receiver) = mpsc::channel(100);
        
        (
            Self {
                tasks: Arc::new(RwLock::new(HashMap::new())),
                sender,
                running: Arc::new(RwLock::new(false)),
            },
            receiver,
        )
    }

    pub async fn add_task(&self, task: ScheduledTask) -> Result<(), AppError> {
        let mut tasks = self.tasks.write().await;
        tasks.insert(task.id.clone(), task);
        Ok(())
    }

    pub async fn remove_task(&self, task_id: &str) -> Result<(), AppError> {
        let mut tasks = self.tasks.write().await;
        tasks.remove(task_id);
        Ok(())
    }

    pub async fn update_task(&self, task: ScheduledTask) -> Result<(), AppError> {
        let mut tasks = self.tasks.write().await;
        tasks.insert(task.id.clone(), task);
        Ok(())
    }

    pub async fn get_task(&self, task_id: &str) -> Option<ScheduledTask> {
        let tasks = self.tasks.read().await;
        tasks.get(task_id).cloned()
    }

    pub async fn get_all_tasks(&self) -> Vec<ScheduledTask> {
        let tasks = self.tasks.read().await;
        tasks.values().cloned().collect()
    }

    pub async fn start(&self) -> Result<(), AppError> {
        let mut running = self.running.write().await;
        if *running {
            return Ok(());
        }
        *running = true;
        drop(running);

        let tasks = self.tasks.clone();
        let sender = self.sender.clone();
        let running = self.running.clone();

        tokio::spawn(async move {
            // Optimized: 1-second interval instead of 10 seconds for better precision
            let mut check_interval = interval(std::time::Duration::from_secs(1));
            
            loop {
                check_interval.tick().await;

                let is_running = *running.read().await;
                if !is_running {
                    break;
                }

                let now = Utc::now();
                let tasks_snapshot = {
                    let tasks_guard = tasks.read().await;
                    tasks_guard.values().cloned().collect::<Vec<_>>()
                };

                for task in tasks_snapshot {
                    if !task.enabled {
                        continue;
                    }

                    if task.scheduled_time <= now {
                        // Send task for execution
                        if sender.send(task.clone()).await.is_ok() {
                            // Update task for next execution if it's repeating
                            if let Some(interval) = &task.repeat_interval {
                                let mut updated_task = task.clone();
                                updated_task.scheduled_time = now + interval.to_duration();
                                
                                let mut tasks_guard = tasks.write().await;
                                tasks_guard.insert(updated_task.id.clone(), updated_task);
                            } else {
                                // Remove one-time task
                                let mut tasks_guard = tasks.write().await;
                                tasks_guard.remove(&task.id);
                            }
                        }
                    }
                }
            }
        });

        Ok(())
    }

    pub async fn stop(&self) -> Result<(), AppError> {
        let mut running = self.running.write().await;
        *running = false;
        Ok(())
    }

    pub async fn is_running(&self) -> bool {
        *self.running.read().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::timeout;
    use std::time::Duration as StdDuration;

    #[tokio::test]
    async fn test_scheduler_add_and_get_task() {
        let (scheduler, _receiver) = Scheduler::new();
        
        let task = ScheduledTask {
            id: "test-1".to_string(),
            download_id: "dl-1".to_string(),
            scheduled_time: Utc::now() + Duration::hours(1),
            repeat_interval: None,
            enabled: true,
        };

        scheduler.add_task(task.clone()).await.unwrap();
        
        let retrieved = scheduler.get_task("test-1").await;
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().download_id, "dl-1");
    }

    #[tokio::test]
    async fn test_scheduler_execution() {
        let (scheduler, mut receiver) = Scheduler::new();
        
        // Schedule a task for immediate execution
        let task = ScheduledTask {
            id: "test-2".to_string(),
            download_id: "dl-2".to_string(),
            scheduled_time: Utc::now() - Duration::seconds(1),
            repeat_interval: None,
            enabled: true,
        };

        scheduler.add_task(task).await.unwrap();
        scheduler.start().await.unwrap();

        // Wait for task to be executed
        let result = timeout(StdDuration::from_secs(15), receiver.recv()).await;
        
        assert!(result.is_ok());
        let executed_task = result.unwrap();
        assert!(executed_task.is_some());
        assert_eq!(executed_task.unwrap().download_id, "dl-2");

        scheduler.stop().await.unwrap();
    }

    #[tokio::test]
    async fn test_scheduler_repeating_task() {
        let (scheduler, mut receiver) = Scheduler::new();
        
        // Schedule a repeating task
        let task = ScheduledTask {
            id: "test-3".to_string(),
            download_id: "dl-3".to_string(),
            scheduled_time: Utc::now() - Duration::seconds(1),
            repeat_interval: Some(RepeatInterval::Custom(2)), // Repeat every 2 seconds
            enabled: true,
        };

        scheduler.add_task(task).await.unwrap();
        scheduler.start().await.unwrap();

        // Wait for first execution
        let result1 = timeout(StdDuration::from_secs(15), receiver.recv()).await;
        assert!(result1.is_ok());

        // Wait for second execution
        let result2 = timeout(StdDuration::from_secs(15), receiver.recv()).await;
        assert!(result2.is_ok());

        scheduler.stop().await.unwrap();
    }
}
