// src-tauri/src/core/resume_manager.rs

use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::utils::error::DownloadError;

/// Resume data saved to disk for crash recovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumeData {
    pub download_id: Uuid,
    pub url: String,
    pub file_name: String,
    pub save_path: PathBuf,
    pub total_size: Option<u64>,
    pub segments: Vec<SegmentResumeData>,
    pub etag: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentResumeData {
    pub segment_id: u32,
    pub start_byte: u64,
    pub end_byte: u64,
    pub downloaded_bytes: u64,
    pub completed: bool,
}

pub struct ResumeManager;

impl ResumeManager {
    /// Save resume data to disk
    pub async fn save(
        temp_dir: &Path,
        data: &ResumeData,
    ) -> Result<(), DownloadError> {
        let resume_path = Self::resume_file_path(temp_dir);
        let json = serde_json::to_string_pretty(data)
            .map_err(|e| DownloadError::FileError(
                format!("Failed to serialize resume data: {}", e)
            ))?;

        tokio::fs::write(&resume_path, json)
            .await
            .map_err(|e| DownloadError::FileError(
                format!("Failed to write resume file: {}", e)
            ))?;

        Ok(())
    }

    /// Load resume data from disk
    pub async fn load(
        temp_dir: &Path,
    ) -> Result<Option<ResumeData>, DownloadError> {
        let resume_path = Self::resume_file_path(temp_dir);

        if !resume_path.exists() {
            return Ok(None);
        }

        let json = tokio::fs::read_to_string(&resume_path)
            .await
            .map_err(|e| DownloadError::FileError(
                format!("Failed to read resume file: {}", e)
            ))?;

        let data: ResumeData = serde_json::from_str(&json)
            .map_err(|e| DownloadError::FileError(
                format!("Failed to parse resume data: {}", e)
            ))?;

        Ok(Some(data))
    }

    /// Check if resume data exists for a download
    pub async fn has_resume_data(temp_dir: &Path) -> bool {
        Self::resume_file_path(temp_dir).exists()
    }

    /// Get the actual downloaded bytes for each segment by checking file sizes
    pub async fn get_segment_progress(
        temp_dir: &Path,
        num_segments: u32,
    ) -> Vec<u64> {
        let mut progress = Vec::with_capacity(num_segments as usize);

        for i in 0..num_segments {
            let part_path = temp_dir.join(format!("part_{}", i));
            let bytes = if part_path.exists() {
                tokio::fs::metadata(&part_path)
                    .await
                    .map(|m| m.len())
                    .unwrap_or(0)
            } else {
                0
            };
            progress.push(bytes);
        }

        progress
    }

    /// Delete resume data
    pub async fn delete(temp_dir: &Path) -> Result<(), DownloadError> {
        let resume_path = Self::resume_file_path(temp_dir);
        if resume_path.exists() {
            tokio::fs::remove_file(&resume_path)
                .await
                .map_err(|e| DownloadError::FileError(e.to_string()))?;
        }
        Ok(())
    }

    fn resume_file_path(temp_dir: &Path) -> PathBuf {
        temp_dir.join("resume.json")
    }
}