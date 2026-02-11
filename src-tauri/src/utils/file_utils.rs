// src-tauri/src/utils/file_utils.rs

use std::path::{Path, PathBuf};
use tokio::fs;

/// Generate a unique file name if the file already exists
/// e.g., file.zip → file (1).zip → file (2).zip
pub async fn get_unique_filename(path: &Path) -> PathBuf {
    if !path.exists() {
        return path.to_path_buf();
    }

    let stem = path
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    let extension = path
        .extension()
        .map(|e| format!(".{}", e.to_string_lossy()))
        .unwrap_or_default();
    let parent = path.parent().unwrap_or(Path::new("."));

    let mut counter = 1u32;
    loop {
        let new_name = format!("{} ({}){}", stem, counter, extension);
        let new_path = parent.join(&new_name);
        if !new_path.exists() {
            return new_path;
        }
        counter += 1;
        if counter > 10000 {
            // Safety valve
            return parent.join(format!(
                "{}_{}{}", stem,
                uuid::Uuid::new_v4().to_string()[..8].to_string(),
                extension
            ));
        }
    }
}

/// Get available disk space at the given path
pub async fn get_available_space(path: &Path) -> std::io::Result<u64> {
    // Use the parent directory if the file doesn't exist yet
    let check_path = if path.is_dir() {
        path.to_path_buf()
    } else {
        path.parent()
            .unwrap_or(Path::new("."))
            .to_path_buf()
    };

    #[cfg(target_os = "windows")]
    {
        // On Windows, use GetDiskFreeSpaceEx via std
        let _meta = fs::metadata(&check_path).await?;
        // Simplified - in production use winapi
        Ok(u64::MAX) // placeholder
    }

    #[cfg(not(target_os = "windows"))]
    {
        use std::process::Command;
        let output = Command::new("df")
            .arg("-k")
            .arg(&check_path)
            .output()?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        let available = stdout
            .lines()
            .nth(1)
            .and_then(|line| line.split_whitespace().nth(3))
            .and_then(|s| s.parse::<u64>().ok())
            .map(|kb| kb * 1024)
            .unwrap_or(u64::MAX);
        Ok(available)
    }
}

/// Ensure directory exists
pub async fn ensure_dir(path: &Path) -> std::io::Result<()> {
    if !path.exists() {
        fs::create_dir_all(path).await?;
    }
    Ok(())
}

/// Get temp directory for download parts
pub fn get_temp_dir(save_path: &Path, download_id: &uuid::Uuid) -> PathBuf {
    let parent = save_path.parent().unwrap_or(Path::new("."));
    parent.join(format!(
        "{}{}", 
        crate::utils::constants::TEMP_DIR_PREFIX, 
        download_id
    ))
}

/// Clean up temp directory
pub async fn cleanup_temp_dir(temp_dir: &Path) -> std::io::Result<()> {
    if temp_dir.exists() {
        fs::remove_dir_all(temp_dir).await?;
    }
    Ok(())
}

/// Get default download directory
pub fn get_default_download_dir() -> PathBuf {
    dirs::download_dir()
        .or_else(|| dirs::home_dir().map(|h| h.join("Downloads")))
        .unwrap_or_else(|| PathBuf::from("."))
}