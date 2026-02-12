use sqlx::{Row, SqlitePool};
use std::path::PathBuf;
use uuid::Uuid;

use crate::core::download_task::{
    DownloadStatus, DownloadTask,
};
use crate::database::models::DownloadRow;
use crate::utils::error::DownloadError;

#[derive(Clone)]
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    /// Create a new database connection
    pub async fn new(
        app_data_dir: &PathBuf,
    ) -> Result<Self, DownloadError> {
        std::fs::create_dir_all(app_data_dir).map_err(|e| {
            DownloadError::FileError(format!(
                "Cannot create data dir: {}",
                e
            ))
        })?;

        let db_path = app_data_dir.join("downloads.db");
        let db_url =
            format!("sqlite:{}?mode=rwc", db_path.display());

        let pool =
            SqlitePool::connect(&db_url).await.map_err(|e| {
                DownloadError::Unknown(format!(
                    "DB connection failed: {}",
                    e
                ))
            })?;

        Ok(Self { pool })
    }

    /// Run database migrations
    pub async fn run_migrations(
        &self,
    ) -> Result<(), DownloadError> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS downloads (
                id TEXT PRIMARY KEY,
                url TEXT NOT NULL,
                final_url TEXT,
                file_name TEXT NOT NULL,
                save_path TEXT NOT NULL,
                total_size INTEGER,
                downloaded_size INTEGER NOT NULL DEFAULT 0,
                status TEXT NOT NULL DEFAULT 'Queued',
                segments INTEGER NOT NULL DEFAULT 4,
                supports_range BOOLEAN NOT NULL DEFAULT FALSE,
                content_type TEXT,
                etag TEXT,
                expected_checksum TEXT,
                actual_checksum TEXT,
                checksum_algorithm TEXT,
                retry_count INTEGER NOT NULL DEFAULT 0,
                error_message TEXT,
                created_at TEXT NOT NULL,
                completed_at TEXT,
                priority INTEGER NOT NULL DEFAULT 100,
                category TEXT,
                segment_progress TEXT
            );

            CREATE INDEX IF NOT EXISTS idx_downloads_status
                ON downloads(status);
            CREATE INDEX IF NOT EXISTS idx_downloads_created
                ON downloads(created_at DESC);
            CREATE INDEX IF NOT EXISTS idx_downloads_category
                ON downloads(category);
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| {
            DownloadError::Unknown(format!(
                "Migration failed: {}",
                e
            ))
        })?;

        Ok(())
    }

    /// Insert a new download
    pub async fn insert_download(
        &self,
        task: &DownloadTask,
    ) -> Result<(), DownloadError> {
        let segment_progress_json = serde_json::to_string(&task.segment_progress)
            .unwrap_or_else(|_| "[]".to_string());

        sqlx::query(
            r#"
            INSERT INTO downloads (
                id, url, final_url, file_name, save_path, total_size,
                downloaded_size, status, segments, supports_range,
                content_type, etag, expected_checksum, actual_checksum,
                checksum_algorithm, retry_count, error_message, created_at,
                completed_at, priority, category, segment_progress
            ) VALUES (
                ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10,
                ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19,
                ?20, ?21, ?22
            )
            "#,
        )
        .bind(task.id.to_string())
        .bind(&task.url)
        .bind(&task.final_url)
        .bind(&task.file_name)
        .bind(task.save_path.to_string_lossy().to_string())
        .bind(task.total_size.map(|s| s as i64))
        .bind(task.downloaded_size as i64)
        .bind(task.status.as_str())
        .bind(task.segments as i32)
        .bind(task.supports_range)
        .bind(&task.content_type)
        .bind(&task.etag)
        .bind(&task.expected_checksum)
        .bind(&task.actual_checksum)
        .bind(task.checksum_algorithm.as_ref().map(|a| format!("{:?}", a)))
        .bind(task.retry_count as i32)
        .bind(&task.error_message)
        .bind(task.created_at.to_string())
        .bind(task.completed_at.map(|c| c.to_string()))
        .bind(task.priority as i32)
        .bind(&task.category)
        .bind(segment_progress_json)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            DownloadError::Unknown(format!(
                "Insert failed: {}",
                e
            ))
        })?;

        Ok(())
    }

    /// Update download status and progress
    pub async fn update_download(
        &self,
        task: &DownloadTask,
    ) -> Result<(), DownloadError> {
        let segment_progress_json = serde_json::to_string(&task.segment_progress)
            .unwrap_or_else(|_| "[]".to_string());

        sqlx::query(
            r#"
            UPDATE downloads SET
                file_name = ?1,
                save_path = ?2,
                total_size = ?3,
                downloaded_size = ?4,
                status = ?5,
                retry_count = ?6,
                error_message = ?7,
                completed_at = ?8,
                actual_checksum = ?9,
                segment_progress = ?10
            WHERE id = ?11
            "#,
        )
        .bind(&task.file_name)
        .bind(task.save_path.to_string_lossy().to_string())
        .bind(task.total_size.map(|s| s as i64))
        .bind(task.downloaded_size as i64)
        .bind(task.status.as_str())
        .bind(task.retry_count as i32)
        .bind(&task.error_message)
        .bind(task.completed_at.map(|c| c.to_string()))
        .bind(&task.actual_checksum)
        .bind(segment_progress_json)
        .bind(task.id.to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| {
            DownloadError::Unknown(format!(
                "Update failed: {}",
                e
            ))
        })?;

        Ok(())
    }

    /// Update just the status
    pub async fn update_status(
        &self,
        id: Uuid,
        status: DownloadStatus,
    ) -> Result<(), DownloadError> {
        sqlx::query(
            "UPDATE downloads SET status = ?1 WHERE id = ?2",
        )
        .bind(status.as_str())
        .bind(id.to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| {
            DownloadError::Unknown(format!(
                "Status update failed: {}",
                e
            ))
        })?;

        Ok(())
    }

    /// Get a single download by ID
    pub async fn get_download(
        &self,
        id: Uuid,
    ) -> Result<Option<DownloadTask>, DownloadError> {
        let row: Option<DownloadRow> = sqlx::query_as::<_, DownloadRow>(
            "SELECT * FROM downloads WHERE id = ?1",
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            DownloadError::Unknown(format!(
                "Query failed: {}",
                e
            ))
        })?;

        Ok(row.map(|r| Self::row_to_task(r)))
    }

    /// Get all downloads ordered by creation date
    pub async fn get_all_downloads(
        &self,
    ) -> Result<Vec<DownloadTask>, DownloadError> {
        let rows: Vec<DownloadRow> = sqlx::query_as::<_, DownloadRow>(
            "SELECT * FROM downloads ORDER BY created_at DESC",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            DownloadError::Unknown(format!(
                "Query failed: {}",
                e
            ))
        })?;

        Ok(rows.into_iter().map(Self::row_to_task).collect())
    }

    /// Delete a download record
    pub async fn delete_download(
        &self,
        id: Uuid,
    ) -> Result<(), DownloadError> {
        sqlx::query("DELETE FROM downloads WHERE id = ?1")
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| {
                DownloadError::Unknown(format!(
                    "Delete failed: {}",
                    e
                ))
            })?;

        Ok(())
    }

    /// Convert database row to DownloadTask
    fn row_to_task(row: DownloadRow) -> DownloadTask {
        let status = match row.status.as_str() {
            "Queued" => DownloadStatus::Queued,
            "Connecting" => DownloadStatus::Connecting,
            "Downloading" => DownloadStatus::Downloading,
            "Paused" => DownloadStatus::Paused,
            "Merging" => DownloadStatus::Merging,
            "Verifying" => DownloadStatus::Verifying,
            "Completed" => DownloadStatus::Completed,
            "Failed" => DownloadStatus::Failed,
            "Cancelled" => DownloadStatus::Cancelled,
            _ => DownloadStatus::Queued,
        };

        DownloadTask {
            id: Uuid::parse_str(&row.id).unwrap_or(Uuid::new_v4()),
            url: row.url,
            final_url: row.final_url,
            file_name: row.file_name,
            save_path: PathBuf::from(row.save_path),
            total_size: row.total_size.map(|s| s as u64),
            downloaded_size: row.downloaded_size as u64,
            status,
            speed: 0.0,
            eta: None,
            segments: row.segments as u8,
            supports_range: row.supports_range,
            content_type: row.content_type,
            etag: row.etag,
            expected_checksum: row.expected_checksum,
            actual_checksum: row.actual_checksum,
            checksum_algorithm: row.checksum_algorithm
                .and_then(|s| crate::core::checksum::ChecksumAlgorithm::from_str(&s)),
            retry_count: row.retry_count as u32,
            error_message: row.error_message,
            created_at: chrono::NaiveDateTime::parse_from_str(
                &row.created_at,
                "%Y-%m-%d %H:%M:%S%.f",
            )
            .unwrap_or_else(|_| chrono::Local::now().naive_local()),
            completed_at: row.completed_at.and_then(|c| {
                chrono::NaiveDateTime::parse_from_str(
                    &c,
                    "%Y-%m-%d %H:%M:%S%.f",
                )
                .ok()
            }),
            priority: row.priority as u32,
            category: row.category,
            segment_progress: row.segment_progress
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or_default(),
        }
    }
}

// Implement sqlx::FromRow for DownloadRow
impl<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow>
    for DownloadRow
{
    fn from_row(
        row: &'r sqlx::sqlite::SqliteRow,
    ) -> Result<Self, sqlx::Error> {
        Ok(DownloadRow {
            id: row.try_get("id")?,
            url: row.try_get("url")?,
            final_url: row.try_get("final_url")?,
            file_name: row.try_get("file_name")?,
            save_path: row.try_get("save_path")?,
            total_size: row.try_get("total_size")?,
            downloaded_size: row.try_get("downloaded_size")?,
            status: row.try_get("status")?,
            segments: row.try_get("segments")?,
            supports_range: row
                .try_get("supports_range")?,
            content_type: row.try_get("content_type")?,
            etag: row.try_get("etag")?,
            expected_checksum: row
                .try_get("expected_checksum")?,
            actual_checksum: row
                .try_get("actual_checksum")?,
            checksum_algorithm: row.try_get("checksum_algorithm")?,
            retry_count: row.try_get("retry_count")?,
            error_message: row.try_get("error_message")?,
            created_at: row.try_get("created_at")?,
            completed_at: row.try_get("completed_at")?,
            priority: row.try_get("priority")?,
            category: row.try_get("category")?,
            segment_progress: row.try_get("segment_progress")?,
        })
    }
}
