use std::path::PathBuf;
use std::fs;
use anyhow::{Result, Context, bail};
use tracing::{info, warn, error, debug};
use tokio::process::Command;

/// Manages the bundled yt-dlp binary
pub struct YtdlpManager {
    binary_path: PathBuf,
    app_data_dir: PathBuf,
}

impl YtdlpManager {
    /// Create a new YtdlpManager instance
    pub fn new(app_handle: &tauri::AppHandle) -> Result<Self> {
        let app_data_dir = app_handle
            .path()
            .app_data_dir()
            .context("Failed to get app data directory")?;
        
        // Create bin directory in app data
        let bin_dir = app_data_dir.join("bin");
        fs::create_dir_all(&bin_dir)
            .context("Failed to create bin directory")?;
        
        // Determine binary name based on platform
        let binary_name = if cfg!(target_os = "windows") {
            "yt-dlp.exe"
        } else {
            "yt-dlp"
        };
        
        let binary_path = bin_dir.join(binary_name);
        
        Ok(Self {
            binary_path,
            app_data_dir,
        })
    }
    
    /// Initialize yt-dlp: extract bundled binary if not present
    pub async fn initialize(&self, app_handle: &tauri::AppHandle) -> Result<()> {
        info!("Initializing yt-dlp manager");
        
        // Check if binary already exists
        if self.binary_path.exists() {
            debug!("yt-dlp binary already exists at {:?}", self.binary_path);
            
            // Verify it's executable
            if self.verify_binary().await {
                info!("Existing yt-dlp binary is valid");
                return Ok(());
            } else {
                warn!("Existing yt-dlp binary is invalid, will re-extract");
                let _ = fs::remove_file(&self.binary_path);
            }
        }
        
        // Extract bundled binary
        self.extract_bundled_binary(app_handle).await?;
        
        // Verify the extracted binary
        if !self.verify_binary().await {
            bail!("Extracted yt-dlp binary is not working correctly");
        }
        
        info!("yt-dlp initialized successfully at {:?}", self.binary_path);
        Ok(())
    }
    
    /// Extract the bundled yt-dlp binary from resources
    async fn extract_bundled_binary(&self, app_handle: &tauri::AppHandle) -> Result<()> {
        info!("Extracting bundled yt-dlp binary");
        
        // Determine which bundled binary to use based on platform
        let resource_name = if cfg!(target_os = "windows") {
            "resources/bin/yt-dlp.exe"
        } else if cfg!(target_os = "macos") {
            "resources/bin/yt-dlp_macos"
        } else if cfg!(target_os = "linux") {
            "resources/bin/yt-dlp_linux"
        } else {
            bail!("Unsupported platform for bundled yt-dlp");
        };
        
        // Get resource path
        let resource_path = app_handle
            .path()
            .resource_dir()
            .context("Failed to get resource directory")?
            .join(resource_name.trim_start_matches("resources/"));
        
        debug!("Resource path: {:?}", resource_path);
        
        if !resource_path.exists() {
            // Try alternative path (for development)
            let dev_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join(resource_name);
            
            if dev_path.exists() {
                info!("Using development binary from {:?}", dev_path);
                fs::copy(&dev_path, &self.binary_path)
                    .context("Failed to copy development binary")?;
            } else {
                bail!("Bundled yt-dlp binary not found at {:?} or {:?}", resource_path, dev_path);
            }
        } else {
            // Copy from resources to app data
            fs::copy(&resource_path, &self.binary_path)
                .context("Failed to copy bundled binary")?;
        }
        
        // Make executable on Unix-like systems
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&self.binary_path)?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&self.binary_path, perms)
                .context("Failed to set executable permissions")?;
        }
        
        info!("Successfully extracted yt-dlp binary to {:?}", self.binary_path);
        Ok(())
    }
    
    /// Verify that the binary is executable and working
    async fn verify_binary(&self) -> bool {
        debug!("Verifying yt-dlp binary at {:?}", self.binary_path);
        
        let result = Command::new(&self.binary_path)
            .arg("--version")
            .output()
            .await;
        
        match result {
            Ok(output) => {
                let success = output.status.success();
                if success {
                    let version = String::from_utf8_lossy(&output.stdout);
                    info!("yt-dlp version: {}", version.trim());
                }
                success
            }
            Err(e) => {
                error!("Failed to execute yt-dlp: {}", e);
                false
            }
        }
    }
    
    /// Get the path to the yt-dlp binary
    pub fn get_binary_path(&self) -> &PathBuf {
        &self.binary_path
    }
    
    /// Check if yt-dlp is available (either bundled or system)
    pub async fn is_available(&self) -> bool {
        // First check bundled binary
        if self.binary_path.exists() && self.verify_binary().await {
            return true;
        }
        
        // Fallback to system yt-dlp
        let result = Command::new("yt-dlp")
            .arg("--version")
            .output()
            .await;
        
        result.is_ok() && result.unwrap().status.success()
    }
    
    /// Update yt-dlp to the latest version
    pub async fn update(&self) -> Result<()> {
        info!("Updating yt-dlp");
        
        if !self.binary_path.exists() {
            bail!("yt-dlp binary not found, cannot update");
        }
        
        // Use yt-dlp's self-update feature
        let output = Command::new(&self.binary_path)
            .arg("-U")
            .output()
            .await
            .context("Failed to execute yt-dlp update")?;
        
        if output.status.success() {
            info!("yt-dlp updated successfully");
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            error!("yt-dlp update failed: {}", stderr);
            bail!("Failed to update yt-dlp: {}", stderr)
        }
    }
    
    /// Get the version of the installed yt-dlp
    pub async fn get_version(&self) -> Result<String> {
        let output = Command::new(&self.binary_path)
            .arg("--version")
            .output()
            .await
            .context("Failed to get yt-dlp version")?;
        
        if output.status.success() {
            let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
            Ok(version)
        } else {
            bail!("Failed to get yt-dlp version")
        }
    }
    
    /// Get the bundled version from the version file
    pub fn get_bundled_version(&self) -> Option<String> {
        let version_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("resources")
            .join("bin")
            .join("ytdlp-version.txt");
        
        fs::read_to_string(version_file).ok()
    }
}
