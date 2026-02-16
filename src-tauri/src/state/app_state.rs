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
use crate::network::torrent_client_librqbit::{LibrqbitTorrentClient, TorrentConfig};
use crate::utils::logging::Logger;
use crate::utils::security::{CredentialVault, RateLimiter};
use std::time::Duration;

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
    pub torrent_client: Arc<LibrqbitTorrentClient>,
    pub logger: Arc<Logger>,
    pub credential_vault: Arc<CredentialVault>,
    pub rate_limiter: Arc<RateLimiter>,
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
        
        // Initialize torrent client with librqbit
        let torrent_config = TorrentConfig {
            download_dir: download_dir.clone(),
            ..Default::default()
        };
        let torrent_client = LibrqbitTorrentClient::new(torrent_config)
            .await
            .map_err(|e| crate::utils::error::DownloadError::NetworkError(format!("Failed to create torrent client: {}", e)))?;
        let torrent_client = Arc::new(torrent_client);

        // Initialize logger
        let logger = Arc::new(Logger::new());

        // Initialize credential vault with a master password
        // In production, this should be stored securely or derived from user input
        let credential_vault = Arc::new(
            CredentialVault::new("default_master_password")
                .map_err(|e| crate::utils::error::DownloadError::Unknown(e))?
        );

        // Initialize rate limiter (10 requests per 60 seconds per key)
        let rate_limiter = Arc::new(RateLimiter::new(10, Duration::from_secs(60)));

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
            logger,
            credential_vault,
            rate_limiter,
        })
    }
}