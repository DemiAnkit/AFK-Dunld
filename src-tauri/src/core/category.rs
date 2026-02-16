// src-tauri/src/core/category.rs

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub save_path: Option<PathBuf>,
    pub created_at: i64,
    pub updated_at: i64,
}

impl Category {
    pub fn new(name: String, color: Option<String>, icon: Option<String>, save_path: Option<PathBuf>) -> Self {
        let now = chrono::Utc::now().timestamp();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            color,
            icon,
            save_path,
            created_at: now,
            updated_at: now,
        }
    }

    /// Get default category
    pub fn default() -> Self {
        Self {
            id: "default".to_string(),
            name: "Default".to_string(),
            color: Some("#6B7280".to_string()),
            icon: Some("folder".to_string()),
            save_path: None,
            created_at: 0,
            updated_at: 0,
        }
    }

    /// Detect category from file extension
    pub fn detect_from_extension(ext: &str) -> String {
        let ext_lower = ext.to_lowercase();
        match ext_lower.as_str() {
            // Documents
            "pdf" | "doc" | "docx" | "txt" | "rtf" | "odt" | "xls" | "xlsx" | "ppt" | "pptx" => "documents",
            
            // Videos
            "mp4" | "avi" | "mkv" | "mov" | "wmv" | "flv" | "webm" | "m4v" | "mpg" | "mpeg" => "videos",
            
            // Music
            "mp3" | "wav" | "flac" | "aac" | "ogg" | "wma" | "m4a" | "opus" => "music",
            
            // Images
            "jpg" | "jpeg" | "png" | "gif" | "bmp" | "svg" | "webp" | "ico" | "tiff" => "images",
            
            // Software
            "exe" | "msi" | "dmg" | "pkg" | "deb" | "rpm" | "apk" | "app" => "software",
            
            // Compressed
            "zip" | "rar" | "7z" | "tar" | "gz" | "bz2" | "xz" | "iso" => "compressed",
            
            // Default
            _ => "default",
        }.to_string()
    }

    /// Detect category from MIME type
    pub fn detect_from_mime(mime: &str) -> String {
        if mime.starts_with("video/") {
            "videos".to_string()
        } else if mime.starts_with("audio/") {
            "music".to_string()
        } else if mime.starts_with("image/") {
            "images".to_string()
        } else if mime.starts_with("application/pdf") || mime.starts_with("application/msword") {
            "documents".to_string()
        } else if mime.contains("zip") || mime.contains("compressed") || mime.contains("archive") {
            "compressed".to_string()
        } else {
            "default".to_string()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryStats {
    pub category_id: String,
    pub total_downloads: usize,
    pub completed_downloads: usize,
    pub total_size: u64,
    pub downloaded_size: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_from_extension() {
        assert_eq!(Category::detect_from_extension("mp4"), "videos");
        assert_eq!(Category::detect_from_extension("MP4"), "videos");
        assert_eq!(Category::detect_from_extension("pdf"), "documents");
        assert_eq!(Category::detect_from_extension("mp3"), "music");
        assert_eq!(Category::detect_from_extension("zip"), "compressed");
        assert_eq!(Category::detect_from_extension("unknown"), "default");
    }

    #[test]
    fn test_detect_from_mime() {
        assert_eq!(Category::detect_from_mime("video/mp4"), "videos");
        assert_eq!(Category::detect_from_mime("audio/mpeg"), "music");
        assert_eq!(Category::detect_from_mime("image/png"), "images");
        assert_eq!(Category::detect_from_mime("application/pdf"), "documents");
        assert_eq!(Category::detect_from_mime("application/zip"), "compressed");
    }
}
