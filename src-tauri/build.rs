use std::fs;
use std::path::PathBuf;
use std::io::Write;

fn main() {
    // Download yt-dlp binaries for bundling
    download_ytdlp_binaries();
    
    tauri_build::build()
}

fn download_ytdlp_binaries() {
    // Only rerun if build.rs changes, not on every build
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=resources/bin/ytdlp-version.txt");
    
    let out_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("resources")
        .join("bin");
    
    // Create resources/bin directory if it doesn't exist
    if let Err(e) = fs::create_dir_all(&out_dir) {
        eprintln!("Warning: Failed to create resources/bin directory: {}", e);
        return;
    }
    
    // Check if all binaries already exist
    let version_file = out_dir.join("ytdlp-version.txt");
    let all_exist = ["yt-dlp.exe", "yt-dlp_macos", "yt-dlp_linux"]
        .iter()
        .all(|name| {
            let path = out_dir.join(name);
            path.exists() && fs::metadata(&path).map(|m| m.len() > 1000000).unwrap_or(false)
        });
    
    if all_exist && version_file.exists() {
        // All binaries exist, skip download
        return;
    }
    
    println!("cargo:warning=Downloading yt-dlp binaries to {:?}", out_dir);
    
    // yt-dlp latest release URLs
    let ytdlp_version = "2024.08.06"; // Update periodically or fetch latest
    let base_url = "https://github.com/yt-dlp/yt-dlp/releases/download";
    
    // Only download binary for current platform to reduce build size
    let current_platform = if cfg!(target_os = "windows") {
        Some(("yt-dlp.exe", format!("{}/{}/yt-dlp.exe", base_url, ytdlp_version)))
    } else if cfg!(target_os = "macos") {
        Some(("yt-dlp_macos", format!("{}/{}/yt-dlp_macos", base_url, ytdlp_version)))
    } else if cfg!(target_os = "linux") {
        Some(("yt-dlp_linux", format!("{}/{}/yt-dlp_linux", base_url, ytdlp_version)))
    } else {
        None
    };
    
    // Option to download all platforms (for universal builds)
    let download_all_platforms = std::env::var("YTDLP_BUNDLE_ALL_PLATFORMS").is_ok();
    
    let binaries = if download_all_platforms {
        vec![
            ("yt-dlp.exe", format!("{}/{}/yt-dlp.exe", base_url, ytdlp_version)),
            ("yt-dlp_macos", format!("{}/{}/yt-dlp_macos", base_url, ytdlp_version)),
            ("yt-dlp_linux", format!("{}/{}/yt-dlp_linux", base_url, ytdlp_version)),
        ]
    } else if let Some(platform_binary) = current_platform {
        vec![platform_binary]
    } else {
        println!("cargo:warning=Unsupported platform, skipping yt-dlp download");
        return;
    };
    
    for (filename, url) in binaries {
        let dest_path = out_dir.join(filename);
        
        // Skip if already downloaded
        if dest_path.exists() {
            // Silently skip if file already exists
            continue;
        }
        
        println!("cargo:warning=Downloading {} from {}", filename, url);
        
        match download_file(&url, &dest_path) {
            Ok(_) => {
                println!("cargo:warning=Successfully downloaded {}", filename);
                
                // Make executable on Unix-like systems
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    if let Ok(metadata) = fs::metadata(&dest_path) {
                        let mut perms = metadata.permissions();
                        perms.set_mode(0o755);
                        let _ = fs::set_permissions(&dest_path, perms);
                    }
                }
            }
            Err(e) => {
                eprintln!("cargo:warning=Failed to download {}: {}", filename, e);
            }
        }
    }
    
    // Create version file
    let version_file = out_dir.join("ytdlp-version.txt");
    if let Ok(mut file) = fs::File::create(version_file) {
        let _ = write!(file, "{}", ytdlp_version);
    }
}

fn download_file(url: &str, dest: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    use std::io::copy;
    
    // Use ureq for simple blocking HTTP requests (we'll add this dependency)
    let response = ureq::get(url)
        .timeout(std::time::Duration::from_secs(300))
        .call()?;
    
    let mut file = fs::File::create(dest)?;
    let mut reader = response.into_reader();
    copy(&mut reader, &mut file)?;
    
    Ok(())
}
