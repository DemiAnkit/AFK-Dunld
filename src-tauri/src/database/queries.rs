// src-tauri/src/database/queries.rs

use sqlx::SqlitePool;
use uuid::Uuid;

use crate::core::download_task::{DownloadStatus, DownloadTask};
use crate::database::db::Database;
use crate::database::models::DownloadRow;
use crate::utils::error::DownloadError;

/// Query builder for downloads with filtering, sorting, and pagination
pub struct DownloadQuery {
    status_filter: Option<Vec<DownloadStatus>>,
    category_filter: Option<String>,
    search_term: Option<String>,
    sort_by: SortField,
    sort_order: SortOrder,
    limit: Option<i64>,
    offset: Option<i64>,
}

#[derive(Debug, Clone)]
pub enum SortField {
    CreatedAt,
    FileName,
    FileSize,
    Progress,
    Status,
}

#[derive(Debug, Clone)]
pub enum SortOrder {
    Asc,
    Desc,
}

impl DownloadQuery {
    pub fn new() -> Self {
        Self {
            status_filter: None,
            category_filter: None,
            search_term: None,
            sort_by: SortField::CreatedAt,
            sort_order: SortOrder::Desc,
            limit: None,
            offset: None,
        }
    }

    pub fn with_status(mut self, statuses: Vec<DownloadStatus>) -> Self {
        self.status_filter = Some(statuses);
        self
    }

    pub fn with_category(mut self, category: String) -> Self {
        self.category_filter = Some(category);
        self
    }

    pub fn with_search(mut self, term: String) -> Self {
        self.search_term = Some(term);
        self
    }

    pub fn sort_by(mut self, field: SortField, order: SortOrder) -> Self {
        self.sort_by = field;
        self.sort_order = order;
        self
    }

    pub fn paginate(mut self, limit: i64, offset: i64) -> Self {
        self.limit = Some(limit);
        self.offset = Some(offset);
        self
    }

    /// Build and execute the query using proper parameterized queries
    pub async fn execute(&self, db: &Database) -> Result<Vec<DownloadTask>, DownloadError> {
        // Build base query
        let mut query_str = String::from("SELECT * FROM downloads WHERE 1=1");
        
        // Build WHERE clause with proper parameterization
        let mut where_clauses = Vec::new();
        
        // Status filter
        if let Some(ref statuses) = self.status_filter {
            if !statuses.is_empty() {
                let placeholders = vec!["?"; statuses.len()].join(", ");
                where_clauses.push(format!(" AND status IN ({})", placeholders));
            }
        }

        // Category filter
        if self.category_filter.is_some() {
            where_clauses.push(" AND category = ?".to_string());
        }

        // Search filter
        if self.search_term.is_some() {
            where_clauses.push(" AND (file_name LIKE ? OR url LIKE ?)".to_string());
        }

        // Append WHERE clauses
        for clause in where_clauses {
            query_str.push_str(&clause);
        }

        // Sorting - use whitelisted fields only (prevent SQL injection)
        let sort_field = match self.sort_by {
            SortField::CreatedAt => "created_at",
            SortField::FileName => "file_name",
            SortField::FileSize => "total_size",
            SortField::Progress => "downloaded_size",
            SortField::Status => "status",
        };

        let sort_order = match self.sort_order {
            SortOrder::Asc => "ASC",
            SortOrder::Desc => "DESC",
        };

        query_str.push_str(&format!(" ORDER BY {} {}", sort_field, sort_order));

        // Pagination - use numeric values directly (safe)
        if let Some(limit) = self.limit {
            query_str.push_str(&format!(" LIMIT {}", limit));
        }
        if let Some(offset) = self.offset {
            query_str.push_str(&format!(" OFFSET {}", offset));
        }

        // Build parameterized query using sqlx
        let mut query = sqlx::query_as::<_, DownloadRow>(&query_str);

        // Bind parameters in order
        if let Some(ref statuses) = self.status_filter {
            for status in statuses {
                query = query.bind(status.as_str());
            }
        }

        if let Some(ref category) = self.category_filter {
            query = query.bind(category);
        }

        if let Some(ref term) = self.search_term {
            let search_pattern = format!("%{}%", term);
            query = query.bind(&search_pattern);
            query = query.bind(&search_pattern);
        }

        // Execute query
        let rows = query
            .fetch_all(db.pool())
            .await
            .map_err(|e| DownloadError::Unknown(format!("Query failed: {}", e)))?;

        // Convert rows to tasks
        Ok(rows.into_iter().map(|r| Database::row_to_task(r)).collect())
    }
}

impl Default for DownloadQuery {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper functions for common queries
impl Database {
    /// Get downloads by status
    pub async fn get_downloads_by_status(
        &self,
        status: DownloadStatus,
    ) -> Result<Vec<DownloadTask>, DownloadError> {
        let all = self.get_all_downloads().await?;
        Ok(all.into_iter().filter(|t| t.status == status).collect())
    }

    /// Get downloads by category
    pub async fn get_downloads_by_category(
        &self,
        category: &str,
    ) -> Result<Vec<DownloadTask>, DownloadError> {
        let all = self.get_all_downloads().await?;
        Ok(all
            .into_iter()
            .filter(|t| t.category.as_deref() == Some(category))
            .collect())
    }

    /// Count downloads by status
    pub async fn count_by_status(
        &self,
        status: DownloadStatus,
    ) -> Result<u32, DownloadError> {
        let tasks = self.get_downloads_by_status(status).await?;
        Ok(tasks.len() as u32)
    }

    /// Get active downloads (downloading or connecting)
    pub async fn get_active_downloads(&self) -> Result<Vec<DownloadTask>, DownloadError> {
        let all = self.get_all_downloads().await?;
        Ok(all
            .into_iter()
            .filter(|t| {
                matches!(
                    t.status,
                    DownloadStatus::Downloading | DownloadStatus::Connecting
                )
            })
            .collect())
    }

    /// Get total downloaded bytes across all downloads
    pub async fn get_total_downloaded_bytes(&self) -> Result<u64, DownloadError> {
        let all = self.get_all_downloads().await?;
        Ok(all.iter().map(|t| t.downloaded_size).sum())
    }

    /// Search downloads by filename or URL
    pub async fn search_downloads(&self, query: &str) -> Result<Vec<DownloadTask>, DownloadError> {
        let all = self.get_all_downloads().await?;
        let query_lower = query.to_lowercase();
        Ok(all
            .into_iter()
            .filter(|t| {
                t.file_name.to_lowercase().contains(&query_lower)
                    || t.url.to_lowercase().contains(&query_lower)
            })
            .collect())
    }
}
