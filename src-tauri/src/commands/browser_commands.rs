// Browser Extension Integration Commands

use serde::{Deserialize, Serialize};
use tauri::State;
use crate::state::app_state::AppState;
use crate::utils::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct BrowserDownloadRequest {
    pub url: String,
    pub referrer: Option<String>,
    pub filename: Option<String>,
}

/// Add download from browser extension
#[tauri::command]
pub async fn add_download_from_browser(
    request: BrowserDownloadRequest,
    state: State<'_, AppState>,
) -> Result<String, String> {
    tracing::info!("Adding download from browser: {}", request.url);
    
    crate::commands::download_commands::add_download_internal(
        request.url,
        None, // Use default save path
        request.filename,
        request.referrer,
        state.inner().clone(),
    )
    .await
    .map_err(|e| e.to_string())
}

/// Check if browser extension is supported
#[tauri::command]
pub async fn is_browser_extension_available() -> Result<bool, String> {
    // Check if native messaging manifests are installed
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        
        // Check Chrome registry
        let chrome_check = Command::new("reg")
            .args(&[
                "query",
                "HKCU\\Software\\Google\\Chrome\\NativeMessagingHosts\\com.ankit.afkdunld",
            ])
            .output();
        
        Ok(chrome_check.is_ok())
    }
    
    #[cfg(target_os = "macos")]
    {
        use std::path::PathBuf;
        
        // Check Chrome manifest location
        let home = std::env::var("HOME").unwrap_or_default();
        let chrome_manifest = PathBuf::from(format!(
            "{}/Library/Application Support/Google/Chrome/NativeMessagingHosts/com.ankit.afkdunld.json",
            home
        ));
        
        Ok(chrome_manifest.exists())
    }
    
    #[cfg(target_os = "linux")]
    {
        use std::path::PathBuf;
        
        // Check Chrome manifest location
        let home = std::env::var("HOME").unwrap_or_default();
        let chrome_manifest = PathBuf::from(format!(
            "{}/.config/google-chrome/NativeMessagingHosts/com.ankit.afkdunld.json",
            home
        ));
        
        Ok(chrome_manifest.exists())
    }
}

/// Install native messaging manifests for browser extensions
#[tauri::command]
pub async fn install_browser_extension_support(
    app_handle: tauri::AppHandle,
) -> Result<String, String> {
    use std::fs;
    use std::path::PathBuf;
    
    let exe_path = std::env::current_exe()
        .map_err(|e| format!("Failed to get executable path: {}", e))?;
    
    let exe_path_str = exe_path.to_string_lossy().to_string();
    
    // Create manifest content
    let manifest = serde_json::json!({
        "name": "com.ankit.afkdunld",
        "description": "AFK-Dunld Download Manager",
        "path": exe_path_str,
        "type": "stdio",
        "allowed_origins": [
            "chrome-extension://EXTENSION_ID_PLACEHOLDER/"
        ]
    });
    
    let manifest_content = serde_json::to_string_pretty(&manifest)
        .map_err(|e| format!("Failed to serialize manifest: {}", e))?;
    
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        
        // Create manifest file in temp location
        let manifest_path = std::env::temp_dir().join("com.ankit.afkdunld.json");
        fs::write(&manifest_path, &manifest_content)
            .map_err(|e| format!("Failed to write manifest: {}", e))?;
        
        // Register in Windows registry for Chrome
        let _ = Command::new("reg")
            .args(&[
                "add",
                "HKCU\\Software\\Google\\Chrome\\NativeMessagingHosts\\com.ankit.afkdunld",
                "/ve",
                "/t",
                "REG_SZ",
                "/d",
                &manifest_path.to_string_lossy(),
                "/f",
            ])
            .output();
        
        // Register for Firefox
        let _ = Command::new("reg")
            .args(&[
                "add",
                "HKCU\\Software\\Mozilla\\NativeMessagingHosts\\com.ankit.afkdunld",
                "/ve",
                "/t",
                "REG_SZ",
                "/d",
                &manifest_path.to_string_lossy(),
                "/f",
            ])
            .output();
        
        Ok("Installed for Chrome and Firefox on Windows".to_string())
    }
    
    #[cfg(target_os = "macos")]
    {
        let home = std::env::var("HOME").map_err(|e| format!("Failed to get HOME: {}", e))?;
        
        // Chrome
        let chrome_dir = PathBuf::from(format!(
            "{}/Library/Application Support/Google/Chrome/NativeMessagingHosts",
            home
        ));
        fs::create_dir_all(&chrome_dir)
            .map_err(|e| format!("Failed to create Chrome directory: {}", e))?;
        fs::write(chrome_dir.join("com.ankit.afkdunld.json"), &manifest_content)
            .map_err(|e| format!("Failed to write Chrome manifest: {}", e))?;
        
        // Firefox
        let firefox_dir = PathBuf::from(format!(
            "{}/Library/Application Support/Mozilla/NativeMessagingHosts",
            home
        ));
        fs::create_dir_all(&firefox_dir)
            .map_err(|e| format!("Failed to create Firefox directory: {}", e))?;
        fs::write(firefox_dir.join("com.ankit.afkdunld.json"), &manifest_content)
            .map_err(|e| format!("Failed to write Firefox manifest: {}", e))?;
        
        Ok("Installed for Chrome and Firefox on macOS".to_string())
    }
    
    #[cfg(target_os = "linux")]
    {
        let home = std::env::var("HOME").map_err(|e| format!("Failed to get HOME: {}", e))?;
        
        // Chrome
        let chrome_dir = PathBuf::from(format!(
            "{}/.config/google-chrome/NativeMessagingHosts",
            home
        ));
        fs::create_dir_all(&chrome_dir)
            .map_err(|e| format!("Failed to create Chrome directory: {}", e))?;
        fs::write(chrome_dir.join("com.ankit.afkdunld.json"), &manifest_content)
            .map_err(|e| format!("Failed to write Chrome manifest: {}", e))?;
        
        // Chromium
        let chromium_dir = PathBuf::from(format!(
            "{}/.config/chromium/NativeMessagingHosts",
            home
        ));
        fs::create_dir_all(&chromium_dir)
            .map_err(|e| format!("Failed to create Chromium directory: {}", e))?;
        fs::write(chromium_dir.join("com.ankit.afkdunld.json"), &manifest_content)
            .map_err(|e| format!("Failed to write Chromium manifest: {}", e))?;
        
        // Firefox
        let firefox_dir = PathBuf::from(format!(
            "{}/.mozilla/native-messaging-hosts",
            home
        ));
        fs::create_dir_all(&firefox_dir)
            .map_err(|e| format!("Failed to create Firefox directory: {}", e))?;
        fs::write(firefox_dir.join("com.ankit.afkdunld.json"), &manifest_content)
            .map_err(|e| format!("Failed to write Firefox manifest: {}", e))?;
        
        Ok("Installed for Chrome, Chromium, and Firefox on Linux".to_string())
    }
}

/// Uninstall native messaging manifests
#[tauri::command]
pub async fn uninstall_browser_extension_support() -> Result<String, String> {
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        
        let _ = Command::new("reg")
            .args(&[
                "delete",
                "HKCU\\Software\\Google\\Chrome\\NativeMessagingHosts\\com.ankit.afkdunld",
                "/f",
            ])
            .output();
        
        let _ = Command::new("reg")
            .args(&[
                "delete",
                "HKCU\\Software\\Mozilla\\NativeMessagingHosts\\com.ankit.afkdunld",
                "/f",
            ])
            .output();
        
        Ok("Uninstalled from Windows registry".to_string())
    }
    
    #[cfg(any(target_os = "macos", target_os = "linux"))]
    {
        use std::fs;
        use std::path::PathBuf;
        
        let home = std::env::var("HOME").map_err(|e| format!("Failed to get HOME: {}", e))?;
        
        #[cfg(target_os = "macos")]
        let paths = vec![
            format!("{}/Library/Application Support/Google/Chrome/NativeMessagingHosts/com.ankit.afkdunld.json", home),
            format!("{}/Library/Application Support/Mozilla/NativeMessagingHosts/com.ankit.afkdunld.json", home),
        ];
        
        #[cfg(target_os = "linux")]
        let paths = vec![
            format!("{}/.config/google-chrome/NativeMessagingHosts/com.ankit.afkdunld.json", home),
            format!("{}/.config/chromium/NativeMessagingHosts/com.ankit.afkdunld.json", home),
            format!("{}/.mozilla/native-messaging-hosts/com.ankit.afkdunld.json", home),
        ];
        
        for path in paths {
            let _ = fs::remove_file(PathBuf::from(path));
        }
        
        Ok("Uninstalled manifest files".to_string())
    }
}
