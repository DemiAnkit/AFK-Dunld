use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::core::download_engine::DownloadEngine;
use crate::core::download_task::DownloadTask;
use crate::core::queue_manager::QueueManager;
use crate::core::scheduler::{Scheduler, ScheduledTask};
use crate::database::db::Database;
use crate::network::torrent_client::{TorrentClient, TorrentConfig};

/// Handle for an active download (used for cancellation)
pub struct ActiveDownload {
    pub cancel_token: tokio_util::sync::CancellationToken,
    pub task_handle: tokio::task::JoinHandle<()>,
    pub task: Arc<RwLock<DownloadTask>>,
}

/// Download handle type alias for backwards compatibility
pub type DownloadHandle = ActiveDownload;

/// Global application state managed by Tauri
#[derive(Clone)]
pub struct AppState {
    pub db: Database,
    pub engine: Arc<DownloadEngine>,
    pub queue: Arc<RwLock<QueueManager>>,
    pub active_downloads:
        Arc<RwLock<HashMap<Uuid, ActiveDownload>>>,
    pub download_dir: PathBuf,
    pub scheduler: Arc<Scheduler>,
    pub scheduled_task_receiver: Arc<RwLock<Option<tokio::sync::mpsc::Receiver<ScheduledTask>>>>,
    pub torrent_client: Arc<TorrentClient>,
}

impl AppState {
    pub async fn new(
        app_data_dir: PathBuf,
    ) -> Result<Self, crate::utils::error::DownloadError> {
        // Initialize database
        let db = Database::new(&app_data_dir).await?;
        db.run_migrations().await?;

        // Initialize download engine
        let download_dir = dirs::download_dir()
            .unwrap_or_else(|| {
                dirs::home_dir()
                    .unwrap_or_else(|| PathBuf::from("."))
                    .join("Downloads")
            });

        let engine = Arc::new(DownloadEngine::new(
            None,       // No proxy by default
            None,       // No speed limit by default
            Some(download_dir.clone()),
        )?);

        let queue =
            Arc::new(RwLock::new(QueueManager::new(5)));

        // Initialize scheduler
        let (scheduler, receiver) = Scheduler::new();
        let scheduler = Arc::new(scheduler);
        
        // Initialize torrent client
        let torrent_config = TorrentConfig {
            download_dir: download_dir.clone(),
            ..Default::default()
        };
        let torrent_client = Arc::new(TorrentClient::new(torrent_config));

        Ok(Self {
            db,
            engine,
            queue,
            active_downloads: Arc::new(RwLock::new(
                HashMap::new(),
            )),
            download_dir,
            scheduler,
            scheduled_task_receiver: Arc::new(RwLock::new(Some(receiver))),
            torrent_client,
        })
    }
}