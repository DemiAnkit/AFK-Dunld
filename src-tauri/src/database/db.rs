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

        // Create settings table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| {
            DownloadError::Unknown(format!(
                "Settings table creation failed: {}",
                e
            ))
        })?;

        // Run torrent migrations
        self.run_torrent_migrations().await?;

        Ok(())
    }

    /// Run torrent-specific migrations
    async fn run_torrent_migrations(&self) -> Result<(), DownloadError> {
        // Read and execute the torrent migration SQL
        let migration_sql = include_str!("migrations/003_add_torrents.sql");
        
        sqlx::query(migration_sql)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                DownloadError::Unknown(format!(
                    "Torrent migration failed: {}",
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
    pub fn row_to_task(row: DownloadRow) -> DownloadTask {
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

    // ========== Settings Operations ==========

    /// Get a setting value by key
    pub async fn get_setting(&self, key: &str) -> Result<Option<String>, DownloadError> {
        let result: Option<(String,)> = sqlx::query_as(
            "SELECT value FROM settings WHERE key = ?1"
        )
        .bind(key)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DownloadError::Unknown(format!("Failed to get setting: {}", e)))?;

        Ok(result.map(|r| r.0))
    }

    /// Set a setting value
    pub async fn set_setting(&self, key: &str, value: &str) -> Result<(), DownloadError> {
        sqlx::query(
            r#"
            INSERT INTO settings (key, value, updated_at)
            VALUES (?1, ?2, datetime('now'))
            ON CONFLICT(key) DO UPDATE SET
                value = ?2,
                updated_at = datetime('now')
            "#
        )
        .bind(key)
        .bind(value)
        .execute(&self.pool)
        .await
        .map_err(|e| DownloadError::Unknown(format!("Failed to set setting: {}", e)))?;

        Ok(())
    }

    /// Get all settings as a key-value map
    pub async fn get_all_settings(&self) -> Result<std::collections::HashMap<String, String>, DownloadError> {
        let rows: Vec<(String, String)> = sqlx::query_as(
            "SELECT key, value FROM settings"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DownloadError::Unknown(format!("Failed to get all settings: {}", e)))?;

        Ok(rows.into_iter().collect())
    }

    /// Delete a setting
    pub async fn delete_setting(&self, key: &str) -> Result<(), DownloadError> {
        sqlx::query("DELETE FROM settings WHERE key = ?1")
            .bind(key)
            .execute(&self.pool)
            .await
            .map_err(|e| DownloadError::Unknown(format!("Failed to delete setting: {}", e)))?;

        Ok(())
    }

    // ========== Category Operations ==========

    /// Get all categories
    pub async fn get_all_categories(&self) -> Result<Vec<crate::core::category::Category>, DownloadError> {
        let rows: Vec<(String, String, Option<String>, Option<String>, Option<String>, i64, i64)> = sqlx::query_as(
            "SELECT id, name, color, icon, save_path, created_at, updated_at FROM categories ORDER BY name"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DownloadError::Unknown(format!("Failed to get categories: {}", e)))?;

        Ok(rows.into_iter().map(|(id, name, color, icon, save_path, created_at, updated_at)| {
            crate::core::category::Category {
                id,
                name,
                color,
                icon,
                save_path: save_path.map(PathBuf::from),
                created_at,
                updated_at,
            }
        }).collect())
    }

    /// Get a single category by ID
    pub async fn get_category(&self, category_id: &str) -> Result<crate::core::category::Category, DownloadError> {
        let row: (String, String, Option<String>, Option<String>, Option<String>, i64, i64) = sqlx::query_as(
            "SELECT id, name, color, icon, save_path, created_at, updated_at FROM categories WHERE id = ?1"
        )
        .bind(category_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DownloadError::NotFound(format!("Category not found: {}", e)))?;

        Ok(crate::core::category::Category {
            id: row.0,
            name: row.1,
            color: row.2,
            icon: row.3,
            save_path: row.4.map(PathBuf::from),
            created_at: row.5,
            updated_at: row.6,
        })
    }

    /// Create a new category
    pub async fn create_category(&self, category: &crate::core::category::Category) -> Result<(), DownloadError> {
        sqlx::query(
            r#"
            INSERT INTO categories (id, name, color, icon, save_path, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            "#
        )
        .bind(&category.id)
        .bind(&category.name)
        .bind(&category.color)
        .bind(&category.icon)
        .bind(category.save_path.as_ref().map(|p| p.to_string_lossy().to_string()))
        .bind(category.created_at)
        .bind(category.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| DownloadError::Unknown(format!("Failed to create category: {}", e)))?;

        Ok(())
    }

    /// Update a category
    pub async fn update_category(&self, category: &crate::core::category::Category) -> Result<(), DownloadError> {
        sqlx::query(
            r#"
            UPDATE categories SET 
                name = ?1, 
                color = ?2, 
                icon = ?3, 
                save_path = ?4, 
                updated_at = ?5
            WHERE id = ?6
            "#
        )
        .bind(&category.name)
        .bind(&category.color)
        .bind(&category.icon)
        .bind(category.save_path.as_ref().map(|p| p.to_string_lossy().to_string()))
        .bind(category.updated_at)
        .bind(&category.id)
        .execute(&self.pool)
        .await
        .map_err(|e| DownloadError::Unknown(format!("Failed to update category: {}", e)))?;

        Ok(())
    }

    /// Delete a category
    pub async fn delete_category(&self, category_id: &str) -> Result<(), DownloadError> {
        // First, reassign downloads to default category
        sqlx::query("UPDATE downloads SET category_id = 'default' WHERE category_id = ?1")
            .bind(category_id)
            .execute(&self.pool)
            .await
            .map_err(|e| DownloadError::Unknown(format!("Failed to reassign downloads: {}", e)))?;

        // Then delete the category
        sqlx::query("DELETE FROM categories WHERE id = ?1")
            .bind(category_id)
            .execute(&self.pool)
            .await
            .map_err(|e| DownloadError::Unknown(format!("Failed to delete category: {}", e)))?;

        Ok(())
    }

    /// Get category statistics
    pub async fn get_category_stats(&self, category_id: &str) -> Result<crate::core::category::CategoryStats, DownloadError> {
        let row: (i64, i64, i64, i64) = sqlx::query_as(
            r#"
            SELECT 
                COUNT(*) as total,
                SUM(CASE WHEN status = 'Completed' THEN 1 ELSE 0 END) as completed,
                COALESCE(SUM(total_size), 0) as total_size,
                COALESCE(SUM(downloaded_size), 0) as downloaded_size
            FROM downloads 
            WHERE category_id = ?1
            "#
        )
        .bind(category_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DownloadError::Unknown(format!("Failed to get category stats: {}", e)))?;

        Ok(crate::core::category::CategoryStats {
            category_id: category_id.to_string(),
            total_downloads: row.0 as usize,
            completed_downloads: row.1 as usize,
            total_size: row.2 as u64,
            downloaded_size: row.3 as u64,
        })
    }

    /// Assign a download to a category
    pub async fn assign_download_category(&self, download_id: &str, category_id: &str) -> Result<(), DownloadError> {
        sqlx::query("UPDATE downloads SET category_id = ?1 WHERE id = ?2")
            .bind(category_id)
            .bind(download_id)
            .execute(&self.pool)
            .await
            .map_err(|e| DownloadError::Unknown(format!("Failed to assign category: {}", e)))?;

        Ok(())
    }

    /// Get the underlying pool for torrent queries
    pub fn pool(&self) -> &SqlitePool {
        &self.pool
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
