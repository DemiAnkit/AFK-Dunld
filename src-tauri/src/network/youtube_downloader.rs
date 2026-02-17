use std::process::Stdio;
use tokio::process::Command;
use std::path::PathBuf;
use anyhow::{Result, Context, bail};
use regex::Regex;
use tokio::io::{AsyncBufReadExt, BufReader};
use serde::{Serialize, Deserialize};
use tracing::{debug, error, info, warn};

pub struct YouTubeDownloader {
    ytdlp_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YouTubeDownloadOptions {
    pub url: String,
    pub format_type: String,      // "video" or "audio"
    pub video_quality: String,     // "2160p", "1080p", etc.
    pub video_format: String,      // "mp4", "mkv", "webm"
    pub audio_format: String,      // "mp3", "aac", "flac", "opus", "m4a"
    pub save_path: PathBuf,
    pub is_playlist: bool,         // Whether to download entire playlist
    pub output_filename: Option<String>, // Optional specific filename to use
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoInfo {
    pub title: String,
    pub duration: u64,
    pub filesize: Option<u64>,
    pub thumbnail: Option<String>,
    pub uploader: Option<String>,
    pub upload_date: Option<String>,
    pub view_count: Option<u64>,
    pub is_playlist: bool,
    pub playlist_count: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YouTubeProgress {
    pub percentage: f64,
    pub downloaded_bytes: u64,
    pub total_bytes: u64,
    pub speed: f64,           // bytes per second
    pub eta: u64,             // seconds
    pub status: String,       // "downloading", "processing", "finished"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityOption {
    pub format_id: String,
    pub resolution: String,
    pub ext: String,
    pub filesize: Option<u64>,
    pub fps: Option<u32>,
    pub has_audio: bool,
}

impl YouTubeDownloader {
    pub fn new() -> Self {
        Self {
            ytdlp_path: None,
        }
    }

    /// Create a new YouTubeDownloader with a custom yt-dlp binary path
    pub fn with_binary_path(ytdlp_path: PathBuf) -> Self {
        Self {
            ytdlp_path: Some(ytdlp_path),
        }
    }

    /// Get the yt-dlp command to use (either bundled or system)
    fn get_ytdlp_command(&self) -> String {
        if let Some(ref path) = self.ytdlp_path {
            path.to_string_lossy().to_string()
        } else {
            "yt-dlp".to_string()
        }
    }

    /// Check if yt-dlp is installed and available
    pub async fn check_installation(&self) -> Result<bool> {
        let cmd = self.get_ytdlp_command();
        let result = Command::new(&cmd)
            .arg("--version")
            .output()
            .await;

        match result {
            Ok(output) => Ok(output.status.success()),
            Err(_) => Ok(false),
        }
    }

    /// Static method for backwards compatibility
    pub async fn check_installation_static() -> Result<bool> {
        let downloader = Self::new();
        downloader.check_installation().await
    }

    /// Get available quality options for a video
    pub async fn get_available_qualities(&self, url: &str) -> Result<Vec<QualityOption>> {
        if !Self::is_supported_url(url) {
            bail!("Unsupported URL: {}", url);
        }

        debug!("Fetching available qualities for: {}", url);

        let cmd = self.get_ytdlp_command();
        
        // Build args with browser cookies for authentication
        let mut args = vec![
            "-F".to_string(),  // List all formats
            "--dump-json".to_string(),
            "--js-runtimes".to_string(),
            "node".to_string(),
        ];
        
        // Try to use browser cookies for authentication (helps with age-restricted/sign-in videos)
        let browsers = ["chrome", "firefox", "edge", "brave"];
        let mut cookie_added = false;
        
        for browser in &browsers {
            let browser_available = match *browser {
                "chrome" => {
                    #[cfg(target_os = "windows")]
                    { std::path::Path::new(&std::env::var("LOCALAPPDATA").unwrap_or_default()).join("Google/Chrome").exists() }
                    #[cfg(target_os = "macos")]
                    { std::path::Path::new(&std::env::var("HOME").unwrap_or_default()).join("Library/Application Support/Google/Chrome").exists() }
                    #[cfg(target_os = "linux")]
                    { std::path::Path::new(&std::env::var("HOME").unwrap_or_default()).join(".config/google-chrome").exists() }
                },
                "firefox" => {
                    #[cfg(target_os = "windows")]
                    { std::path::Path::new(&std::env::var("APPDATA").unwrap_or_default()).join("Mozilla/Firefox").exists() }
                    #[cfg(target_os = "macos")]
                    { std::path::Path::new(&std::env::var("HOME").unwrap_or_default()).join("Library/Application Support/Firefox").exists() }
                    #[cfg(target_os = "linux")]
                    { std::path::Path::new(&std::env::var("HOME").unwrap_or_default()).join(".mozilla/firefox").exists() }
                },
                "edge" => {
                    #[cfg(target_os = "windows")]
                    { std::path::Path::new(&std::env::var("LOCALAPPDATA").unwrap_or_default()).join("Microsoft/Edge").exists() }
                    #[cfg(target_os = "macos")]
                    { std::path::Path::new(&std::env::var("HOME").unwrap_or_default()).join("Library/Application Support/Microsoft Edge").exists() }
                    #[cfg(target_os = "linux")]
                    { std::path::Path::new(&std::env::var("HOME").unwrap_or_default()).join(".config/microsoft-edge").exists() }
                },
                "brave" => {
                    #[cfg(target_os = "windows")]
                    { std::path::Path::new(&std::env::var("LOCALAPPDATA").unwrap_or_default()).join("BraveSoftware/Brave-Browser").exists() }
                    #[cfg(target_os = "macos")]
                    { std::path::Path::new(&std::env::var("HOME").unwrap_or_default()).join("Library/Application Support/BraveSoftware/Brave-Browser").exists() }
                    #[cfg(target_os = "linux")]
                    { std::path::Path::new(&std::env::var("HOME").unwrap_or_default()).join(".config/BraveSoftware/Brave-Browser").exists() }
                },
                _ => false,
            };
            
            if browser_available {
                args.push("--cookies-from-browser".to_string());
                args.push(browser.to_string());
                cookie_added = true;
                info!("Using cookies from browser {} for quality info", browser);
                break;
            }
        }
        
        if !cookie_added {
            debug!("No browser cookies available for quality info");
        }
        
        args.push(url.to_string());
        
        let output = Command::new(&cmd)
            .args(&args)
            .output()
            .await
            .context("Failed to fetch available qualities")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            bail!("Failed to fetch formats: {}", stderr);
        }

        let json: serde_json::Value = serde_json::from_slice(&output.stdout)
            .context("Failed to parse formats JSON")?;

        let mut qualities = Vec::new();
        
        if let Some(formats) = json["formats"].as_array() {
            for format in formats {
                if let Some(format_id) = format["format_id"].as_str() {
                    let resolution = format["resolution"]
                        .as_str()
                        .or_else(|| format["format_note"].as_str())
                        .unwrap_or("unknown")
                        .to_string();
                    
                    let ext = format["ext"].as_str().unwrap_or("unknown").to_string();
                    let filesize = format["filesize"].as_u64();
                    let fps = format["fps"].as_u64().map(|f| f as u32);
                    let has_audio = format["acodec"].as_str() != Some("none");

                    // Only include reasonable video formats
                    if resolution != "audio only" && !resolution.contains("unknown") {
                        qualities.push(QualityOption {
                            format_id: format_id.to_string(),
                            resolution,
                            ext,
                            filesize,
                            fps,
                            has_audio,
                        });
                    }
                }
            }
        }

        Ok(qualities)
    }

    /// Check if URL is a playlist
    pub async fn is_playlist(&self, url: &str) -> Result<bool> {
        let info = self.get_video_info(url).await?;
        Ok(info.is_playlist)
    }

    /// Download a video or audio from YouTube or other supported platforms
    pub async fn download(&self, options: YouTubeDownloadOptions) -> Result<PathBuf> {
        // Validate URL
        if !Self::is_supported_url(&options.url) {
            bail!("Unsupported URL: {}", options.url);
        }

        // Check if yt-dlp is installed
        if !self.check_installation().await? {
            bail!("yt-dlp is not available. Please ensure the application is properly installed.");
        }

        // Ensure output directory exists
        if let Some(parent) = options.save_path.parent() {
            tokio::fs::create_dir_all(parent).await
                .context("Failed to create output directory")?;
        }

        let mut args = vec![];

        if options.format_type == "audio" {
            // Audio-only download
            args.extend_from_slice(&[
                "-x",  // Extract audio
                "--audio-format", &options.audio_format,
                "--audio-quality", "0",  // Best quality
            ]);
        } else {
            // Video download with quality selection
            let format_spec = match options.video_quality.as_str() {
                "2160p" | "4k" => "bestvideo[height<=2160][ext=mp4]+bestaudio[ext=m4a]/bestvideo[height<=2160]+bestaudio/best",
                "1440p" | "2k" => "bestvideo[height<=1440][ext=mp4]+bestaudio[ext=m4a]/bestvideo[height<=1440]+bestaudio/best",
                "1080p" | "fullhd" => "bestvideo[height<=1080][ext=mp4]+bestaudio[ext=m4a]/bestvideo[height<=1080]+bestaudio/best",
                "720p" | "hd" => "bestvideo[height<=720][ext=mp4]+bestaudio[ext=m4a]/bestvideo[height<=720]+bestaudio/best",
                "480p" => "bestvideo[height<=480][ext=mp4]+bestaudio[ext=m4a]/bestvideo[height<=480]+bestaudio/best",
                "360p" => "bestvideo[height<=360][ext=mp4]+bestaudio[ext=m4a]/bestvideo[height<=360]+bestaudio/best",
                "best" | _ => "bestvideo[ext=mp4]+bestaudio[ext=m4a]/bestvideo+bestaudio/best",
            };

            args.extend_from_slice(&[
                "-f", format_spec,
                "--merge-output-format", &options.video_format,
            ]);
        }

        // Playlist option
        if options.is_playlist {
            args.push("--yes-playlist");
        } else {
            args.push("--no-playlist");
        }

        // Get output directory
        let output_dir = options.save_path.parent()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| ".".to_string());
        
        // Use provided filename or fallback to yt-dlp's title template
        let output_template = if let Some(ref filename) = options.output_filename {
            // Use the specified filename (without extension, yt-dlp will add it)
            let name_without_ext = filename.rsplit_once('.')
                .map(|(name, _)| name)
                .unwrap_or(filename);
            format!("{}/{:.100}.%(ext)s", output_dir, name_without_ext)
        } else {
            format!("{}/%(title)s.%(ext)s", output_dir)
        };
        
        // Try to use browser cookies for authentication (helps with age-restricted/sign-in videos)
        // Try multiple browsers in order of popularity
        let browsers = ["chrome", "firefox", "edge", "brave"];
        let mut cookie_added = false;
        
        for browser in &browsers {
            // Try to detect if browser is available by checking common paths
            let browser_available = match *browser {
                "chrome" => {
                    #[cfg(target_os = "windows")]
                    { std::path::Path::new(&std::env::var("LOCALAPPDATA").unwrap_or_default()).join("Google/Chrome").exists() }
                    #[cfg(target_os = "macos")]
                    { std::path::Path::new(&std::env::var("HOME").unwrap_or_default()).join("Library/Application Support/Google/Chrome").exists() }
                    #[cfg(target_os = "linux")]
                    { std::path::Path::new(&std::env::var("HOME").unwrap_or_default()).join(".config/google-chrome").exists() }
                },
                "firefox" => {
                    #[cfg(target_os = "windows")]
                    { std::path::Path::new(&std::env::var("APPDATA").unwrap_or_default()).join("Mozilla/Firefox").exists() }
                    #[cfg(target_os = "macos")]
                    { std::path::Path::new(&std::env::var("HOME").unwrap_or_default()).join("Library/Application Support/Firefox").exists() }
                    #[cfg(target_os = "linux")]
                    { std::path::Path::new(&std::env::var("HOME").unwrap_or_default()).join(".mozilla/firefox").exists() }
                },
                "edge" => {
                    #[cfg(target_os = "windows")]
                    { std::path::Path::new(&std::env::var("LOCALAPPDATA").unwrap_or_default()).join("Microsoft/Edge").exists() }
                    #[cfg(target_os = "macos")]
                    { std::path::Path::new(&std::env::var("HOME").unwrap_or_default()).join("Library/Application Support/Microsoft Edge").exists() }
                    #[cfg(target_os = "linux")]
                    { std::path::Path::new(&std::env::var("HOME").unwrap_or_default()).join(".config/microsoft-edge").exists() }
                },
                "brave" => {
                    #[cfg(target_os = "windows")]
                    { std::path::Path::new(&std::env::var("LOCALAPPDATA").unwrap_or_default()).join("BraveSoftware/Brave-Browser").exists() }
                    #[cfg(target_os = "macos")]
                    { std::path::Path::new(&std::env::var("HOME").unwrap_or_default()).join("Library/Application Support/BraveSoftware/Brave-Browser").exists() }
                    #[cfg(target_os = "linux")]
                    { std::path::Path::new(&std::env::var("HOME").unwrap_or_default()).join(".config/BraveSoftware/Brave-Browser").exists() }
                },
                _ => false,
            };
            
            if browser_available {
                args.push("--cookies-from-browser");
                args.push(browser);
                cookie_added = true;
                info!("Using cookies from browser: {}", browser);
                break;
            }
        }
        
        if !cookie_added {
            warn!("No browser cookies available - age-restricted videos may fail");
        }
        
        // Common options for better compatibility and performance
        args.extend_from_slice(&[
            "--progress",              // Show progress
            "--newline",               // New line for each progress update
            "--no-warnings",           // Suppress warnings
            "--ignore-errors",         // Continue on download errors
            "--no-check-certificate",  // Skip certificate validation (for some cases)
            "--prefer-free-formats",   // Prefer free formats
            "--add-metadata",          // Add metadata to file
            "--embed-thumbnail",       // Embed thumbnail in audio files
            "--encoding", "UTF-8",     // Force UTF-8 encoding
            "--retries", "10",         // Retry failed fragments
            "--fragment-retries", "10",
            "--js-runtimes", "node",  // Enable Node.js for YouTube signature decoding
            "-o", &output_template,
            &options.url,
        ]);

        info!("Starting YouTube/video download with yt-dlp");
        debug!("yt-dlp args: {:?}", args);

        let cmd = self.get_ytdlp_command();
        let output = Command::new(&cmd)
            .args(&args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .context("Failed to execute yt-dlp. Make sure yt-dlp is installed and in PATH")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            
            error!("yt-dlp failed with stderr: {}", stderr);
            error!("yt-dlp stdout: {}", stdout);
            
            // Provide user-friendly error messages
            let error_msg = if stderr.contains("HTTP Error 403") {
                "Video is not available or requires authentication"
            } else if stderr.contains("Video unavailable") {
                "Video is unavailable or has been removed"
            } else if stderr.contains("Unsupported URL") {
                "This URL is not supported"
            } else if stderr.contains("Private video") {
                "This video is private"
            } else if stderr.contains("Sign in") || stderr.contains("sign in") {
                "This video requires signing in to view"
            } else if stderr.contains("age-restricted") || stderr.contains("age restricted") {
                "This video is age-restricted"
            } else if stderr.contains("copyright") {
                "This video is unavailable due to copyright"
            } else if stderr.contains("not found") || stderr.contains("404") {
                "Video not found"
            } else {
                "Download failed"
            };
            
            bail!("{}: {}", error_msg, stderr.lines().next().unwrap_or("Unknown error"));
        }

        // Find the actual downloaded file
        let output_dir = options.save_path.parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| PathBuf::from("."));
        
        let expected_stem = options.output_filename.as_ref()
            .map(|f| f.rsplit_once('.').map(|(n, _)| n.to_string()).unwrap_or_else(|| f.clone()))
            .unwrap_or_else(|| "%(title)s".to_string());
        
        // Search for the downloaded file in the output directory
        let mut final_path = options.save_path.clone();
        match tokio::fs::read_dir(&output_dir).await {
            Ok(mut entries) => {
                while let Ok(Some(entry)) = entries.next_entry().await {
                    if let Ok(file_name) = entry.file_name().into_string() {
                        // Check if file starts with our expected stem (truncated to 100 chars by yt-dlp)
                        let truncated_stem = &expected_stem[..expected_stem.len().min(100)];
                        if file_name.starts_with(truncated_stem) {
                            final_path = entry.path();
                            info!("Found downloaded file: {:?}", final_path);
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                warn!("Could not read output directory: {}", e);
            }
        }
        
        info!("Download completed successfully: {:?}", final_path);
        Ok(final_path)
    }

    /// Download with real-time progress tracking
    pub async fn download_with_progress<F>(
        &self, 
        options: YouTubeDownloadOptions,
        mut progress_callback: F,
    ) -> Result<PathBuf> 
    where
        F: FnMut(YouTubeProgress) + Send + 'static,
    {
        // Validate URL
        if !Self::is_supported_url(&options.url) {
            bail!("Unsupported URL: {}", options.url);
        }

        // Check if yt-dlp is installed
        if !self.check_installation().await? {
            bail!("yt-dlp is not available. Please ensure the application is properly installed.");
        }

        // Ensure output directory exists
        if let Some(parent) = options.save_path.parent() {
            tokio::fs::create_dir_all(parent).await
                .context("Failed to create output directory")?;
        }

        let mut args = vec![];

        if options.format_type == "audio" {
            args.extend_from_slice(&[
                "-x",
                "--audio-format", &options.audio_format,
                "--audio-quality", "0",
            ]);
        } else {
            let format_spec = match options.video_quality.as_str() {
                "2160p" | "4k" => "bestvideo[height<=2160][ext=mp4]+bestaudio[ext=m4a]/bestvideo[height<=2160]+bestaudio/best",
                "1440p" | "2k" => "bestvideo[height<=1440][ext=mp4]+bestaudio[ext=m4a]/bestvideo[height<=1440]+bestaudio/best",
                "1080p" | "fullhd" => "bestvideo[height<=1080][ext=mp4]+bestaudio[ext=m4a]/bestvideo[height<=1080]+bestaudio/best",
                "720p" | "hd" => "bestvideo[height<=720][ext=mp4]+bestaudio[ext=m4a]/bestvideo[height<=720]+bestaudio/best",
                "480p" => "bestvideo[height<=480][ext=mp4]+bestaudio[ext=m4a]/bestvideo[height<=480]+bestaudio/best",
                "360p" => "bestvideo[height<=360][ext=mp4]+bestaudio[ext=m4a]/bestvideo[height<=360]+bestaudio/best",
                "best" | _ => "bestvideo[ext=mp4]+bestaudio[ext=m4a]/bestvideo+bestaudio/best",
            };

            args.extend_from_slice(&[
                "-f", format_spec,
                "--merge-output-format", &options.video_format,
            ]);
        }

        if options.is_playlist {
            args.push("--yes-playlist");
        } else {
            args.push("--no-playlist");
        }

        // Get output directory
        let output_dir = options.save_path.parent()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| ".".to_string());
        
        // Use provided filename or fallback to yt-dlp's title template
        let output_template = if let Some(ref filename) = options.output_filename {
            let name_without_ext = filename.rsplit_once('.')
                .map(|(name, _)| name)
                .unwrap_or(filename);
            format!("{}/{:.100}.%(ext)s", output_dir, name_without_ext)
        } else {
            format!("{}/%(title)s.%(ext)s", output_dir)
        };

        // Try to use browser cookies for authentication
        let browsers = ["chrome", "firefox", "edge", "brave"];
        let mut cookie_added = false;
        
        for browser in &browsers {
            let browser_available = match *browser {
                "chrome" => {
                    #[cfg(target_os = "windows")]
                    { std::path::Path::new(&std::env::var("LOCALAPPDATA").unwrap_or_default()).join("Google/Chrome").exists() }
                    #[cfg(target_os = "macos")]
                    { std::path::Path::new(&std::env::var("HOME").unwrap_or_default()).join("Library/Application Support/Google/Chrome").exists() }
                    #[cfg(target_os = "linux")]
                    { std::path::Path::new(&std::env::var("HOME").unwrap_or_default()).join(".config/google-chrome").exists() }
                },
                "firefox" => {
                    #[cfg(target_os = "windows")]
                    { std::path::Path::new(&std::env::var("APPDATA").unwrap_or_default()).join("Mozilla/Firefox").exists() }
                    #[cfg(target_os = "macos")]
                    { std::path::Path::new(&std::env::var("HOME").unwrap_or_default()).join("Library/Application Support/Firefox").exists() }
                    #[cfg(target_os = "linux")]
                    { std::path::Path::new(&std::env::var("HOME").unwrap_or_default()).join(".mozilla/firefox").exists() }
                },
                "edge" => {
                    #[cfg(target_os = "windows")]
                    { std::path::Path::new(&std::env::var("LOCALAPPDATA").unwrap_or_default()).join("Microsoft/Edge").exists() }
                    #[cfg(target_os = "macos")]
                    { std::path::Path::new(&std::env::var("HOME").unwrap_or_default()).join("Library/Application Support/Microsoft Edge").exists() }
                    #[cfg(target_os = "linux")]
                    { std::path::Path::new(&std::env::var("HOME").unwrap_or_default()).join(".config/microsoft-edge").exists() }
                },
                "brave" => {
                    #[cfg(target_os = "windows")]
                    { std::path::Path::new(&std::env::var("LOCALAPPDATA").unwrap_or_default()).join("BraveSoftware/Brave-Browser").exists() }
                    #[cfg(target_os = "macos")]
                    { std::path::Path::new(&std::env::var("HOME").unwrap_or_default()).join("Library/Application Support/BraveSoftware/Brave-Browser").exists() }
                    #[cfg(target_os = "linux")]
                    { std::path::Path::new(&std::env::var("HOME").unwrap_or_default()).join(".config/BraveSoftware/Brave-Browser").exists() }
                },
                _ => false,
            };
            
            if browser_available {
                args.push("--cookies-from-browser");
                args.push(browser);
                cookie_added = true;
                info!("Using cookies from browser: {}", browser);
                break;
            }
        }
        
        args.extend_from_slice(&[
            "--progress",
            "--newline",
            "--no-warnings",
            "--ignore-errors",
            "--no-check-certificate",
            "--prefer-free-formats",
            "--add-metadata",
            "--embed-thumbnail",
            "--encoding", "UTF-8",
            "--retries", "10",
            "--fragment-retries", "10",
            "--js-runtimes", "node",  // Enable Node.js for YouTube signature decoding
            "-o", &output_template,
            &options.url,
        ]);

        info!("Starting download with progress tracking");

        let cmd = self.get_ytdlp_command();
        let mut child = Command::new(&cmd)
            .args(&args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .context("Failed to spawn yt-dlp process")?;

        let stdout = child.stdout.take().expect("Failed to capture stdout");
        let mut reader = BufReader::new(stdout).lines();

        // Parse progress from stdout
        while let Some(line) = reader.next_line().await? {
            if let Some(progress) = Self::parse_progress_line(&line) {
                progress_callback(progress);
            }
        }

        let output = child.wait_with_output().await?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            error!("yt-dlp failed: {}", stderr);
            
            // Provide user-friendly error
            let error_msg = if stderr.contains("HTTP Error 403") {
                "Video is not available or requires authentication"
            } else if stderr.contains("Video unavailable") {
                "Video is unavailable or has been removed"
            } else if stderr.contains("Unsupported URL") {
                "This URL is not supported"
            } else if stderr.contains("Private video") {
                "This video is private"
            } else if stderr.contains("Sign in") || stderr.contains("sign in") {
                "This video requires signing in to view"
            } else if stderr.contains("age-restricted") || stderr.contains("age restricted") {
                "This video is age-restricted"
            } else {
                "Download failed"
            };
            
            bail!("{}: {}", error_msg, stderr.lines().next().unwrap_or("Unknown error"));
        }

        // Send final progress
        progress_callback(YouTubeProgress {
            percentage: 100.0,
            downloaded_bytes: 0,
            total_bytes: 0,
            speed: 0.0,
            eta: 0,
            status: "finished".to_string(),
        });

        // Find the actual downloaded file
        let output_dir_path = options.save_path.parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| PathBuf::from("."));
        
        let expected_stem = options.output_filename.as_ref()
            .map(|f| f.rsplit_once('.').map(|(n, _)| n.to_string()).unwrap_or_else(|| f.clone()))
            .unwrap_or_else(|| "%(title)s".to_string());
        
        let mut final_path = options.save_path.clone();
        match tokio::fs::read_dir(&output_dir_path).await {
            Ok(mut entries) => {
                while let Ok(Some(entry)) = entries.next_entry().await {
                    if let Ok(file_name) = entry.file_name().into_string() {
                        let truncated_stem = &expected_stem[..expected_stem.len().min(100)];
                        if file_name.starts_with(truncated_stem) {
                            final_path = entry.path();
                            info!("Found downloaded file: {:?}", final_path);
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                warn!("Could not read output directory: {}", e);
            }
        }

        Ok(final_path)
    }

    /// Parse progress line from yt-dlp output
    fn parse_progress_line(line: &str) -> Option<YouTubeProgress> {
        // yt-dlp progress format: [download]  45.3% of 123.45MiB at 1.23MiB/s ETA 00:05
        if !line.contains("[download]") {
            return None;
        }

        let percentage_re = Regex::new(r"(\d+\.?\d*)%").ok()?;
        let size_re = Regex::new(r"of\s+(\d+\.?\d*)(.*?iB)").ok()?;
        let speed_re = Regex::new(r"at\s+(\d+\.?\d*)(.*?iB/s)").ok()?;
        let eta_re = Regex::new(r"ETA\s+(\d+):(\d+)").ok()?;

        let percentage = percentage_re
            .captures(line)
            .and_then(|c| c.get(1))
            .and_then(|m| m.as_str().parse::<f64>().ok())
            .unwrap_or(0.0);

        let total_bytes = size_re
            .captures(line)
            .and_then(|c| {
                let value = c.get(1)?.as_str().parse::<f64>().ok()?;
                let unit = c.get(2)?.as_str();
                Some(Self::parse_size(value, unit))
            })
            .unwrap_or(0);

        let speed = speed_re
            .captures(line)
            .and_then(|c| {
                let value = c.get(1)?.as_str().parse::<f64>().ok()?;
                let unit = c.get(2)?.as_str();
                Some(Self::parse_size(value, unit) as f64)
            })
            .unwrap_or(0.0);

        let eta = eta_re
            .captures(line)
            .and_then(|c| {
                let minutes = c.get(1)?.as_str().parse::<u64>().ok()?;
                let seconds = c.get(2)?.as_str().parse::<u64>().ok()?;
                Some(minutes * 60 + seconds)
            })
            .unwrap_or(0);

        let downloaded_bytes = ((percentage / 100.0) * total_bytes as f64) as u64;

        let status = if line.contains("Merging") || line.contains("Post-processing") {
            "processing"
        } else if percentage >= 100.0 {
            "finished"
        } else {
            "downloading"
        }.to_string();

        Some(YouTubeProgress {
            percentage,
            downloaded_bytes,
            total_bytes,
            speed,
            eta,
            status,
        })
    }

    /// Parse size string to bytes
    fn parse_size(value: f64, unit: &str) -> u64 {
        let multiplier = match unit.to_lowercase().as_str() {
            s if s.contains("kib") => 1024.0,
            s if s.contains("mib") => 1024.0 * 1024.0,
            s if s.contains("gib") => 1024.0 * 1024.0 * 1024.0,
            s if s.contains("kb") => 1000.0,
            s if s.contains("mb") => 1000.0 * 1000.0,
            s if s.contains("gb") => 1000.0 * 1000.0 * 1000.0,
            _ => 1.0,
        };
        (value * multiplier) as u64
    }

    /// Get video information without downloading
    pub async fn get_video_info(&self, url: &str) -> Result<VideoInfo> {
        // Validate URL
        if !Self::is_supported_url(url) {
            bail!("Unsupported URL: {}", url);
        }

        debug!("Fetching video info for: {}", url);

        let cmd = self.get_ytdlp_command();
        
        // Build args with browser cookies for authentication
        let mut args = vec![
            "--dump-json".to_string(),
            "--no-playlist".to_string(),
            "--skip-download".to_string(),
            "--js-runtimes".to_string(),
            "node".to_string(),
        ];
        
        // Try to use browser cookies for authentication (helps with age-restricted/sign-in videos)
        let browsers = ["chrome", "firefox", "edge", "brave"];
        let mut cookie_added = false;
        
        for browser in &browsers {
            let browser_available = match *browser {
                "chrome" => {
                    #[cfg(target_os = "windows")]
                    { std::path::Path::new(&std::env::var("LOCALAPPDATA").unwrap_or_default()).join("Google/Chrome").exists() }
                    #[cfg(target_os = "macos")]
                    { std::path::Path::new(&std::env::var("HOME").unwrap_or_default()).join("Library/Application Support/Google/Chrome").exists() }
                    #[cfg(target_os = "linux")]
                    { std::path::Path::new(&std::env::var("HOME").unwrap_or_default()).join(".config/google-chrome").exists() }
                },
                "firefox" => {
                    #[cfg(target_os = "windows")]
                    { std::path::Path::new(&std::env::var("APPDATA").unwrap_or_default()).join("Mozilla/Firefox").exists() }
                    #[cfg(target_os = "macos")]
                    { std::path::Path::new(&std::env::var("HOME").unwrap_or_default()).join("Library/Application Support/Firefox").exists() }
                    #[cfg(target_os = "linux")]
                    { std::path::Path::new(&std::env::var("HOME").unwrap_or_default()).join(".mozilla/firefox").exists() }
                },
                "edge" => {
                    #[cfg(target_os = "windows")]
                    { std::path::Path::new(&std::env::var("LOCALAPPDATA").unwrap_or_default()).join("Microsoft/Edge").exists() }
                    #[cfg(target_os = "macos")]
                    { std::path::Path::new(&std::env::var("HOME").unwrap_or_default()).join("Library/Application Support/Microsoft Edge").exists() }
                    #[cfg(target_os = "linux")]
                    { std::path::Path::new(&std::env::var("HOME").unwrap_or_default()).join(".config/microsoft-edge").exists() }
                },
                "brave" => {
                    #[cfg(target_os = "windows")]
                    { std::path::Path::new(&std::env::var("LOCALAPPDATA").unwrap_or_default()).join("BraveSoftware/Brave-Browser").exists() }
                    #[cfg(target_os = "macos")]
                    { std::path::Path::new(&std::env::var("HOME").unwrap_or_default()).join("Library/Application Support/BraveSoftware/Brave-Browser").exists() }
                    #[cfg(target_os = "linux")]
                    { std::path::Path::new(&std::env::var("HOME").unwrap_or_default()).join(".config/BraveSoftware/Brave-Browser").exists() }
                },
                _ => false,
            };
            
            if browser_available {
                args.push("--cookies-from-browser".to_string());
                args.push(browser.to_string());
                cookie_added = true;
                info!("Using cookies from browser {} for video info", browser);
                break;
            }
        }
        
        if !cookie_added {
            debug!("No browser cookies available for video info");
        }
        
        args.push(url.to_string());
        
        let output = Command::new(&cmd)
            .args(&args)
            .output()
            .await
            .context("Failed to execute yt-dlp for video info")?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            error!("Failed to get video info: {}", error);
            
            // Provide user-friendly error
            let error_msg = if error.contains("HTTP Error 403") {
                "Video is not available"
            } else if error.contains("Video unavailable") {
                "Video is unavailable or has been removed"
            } else if error.contains("Unsupported URL") {
                "This URL is not supported"
            } else if error.contains("Private video") {
                "This video is private"
            } else if error.contains("Sign in") {
                "This video requires signing in"
            } else {
                "Failed to get video info"
            };
            
            bail!("{}", error_msg);
        }

        let json: serde_json::Value = serde_json::from_slice(&output.stdout)
            .context("Failed to parse video info JSON")?;
        
        // Extract info with fallbacks
        let title = json["title"]
            .as_str()
            .or_else(|| json["fulltitle"].as_str())
            .unwrap_or("Unknown Video")
            .to_string();

        let duration = json["duration"]
            .as_f64()
            .or_else(|| json["duration_string"].as_str().and_then(Self::parse_duration))
            .unwrap_or(0.0) as u64;

        let filesize = json["filesize"]
            .as_u64()
            .or_else(|| json["filesize_approx"].as_u64())
            .or_else(|| {
                // Try to estimate from format info
                if let Some(formats) = json["formats"].as_array() {
                    formats.iter()
                        .filter_map(|f| f["filesize"].as_u64())
                        .max()
                } else {
                    None
                }
            });

        let thumbnail = json["thumbnail"]
            .as_str()
            .or_else(|| {
                json["thumbnails"]
                    .as_array()
                    .and_then(|t| t.last())
                    .and_then(|t| t["url"].as_str())
            })
            .map(|s| s.to_string());

        let uploader = json["uploader"]
            .as_str()
            .or_else(|| json["channel"].as_str())
            .map(|s| s.to_string());

        let upload_date = json["upload_date"]
            .as_str()
            .map(|s| s.to_string());

        let view_count = json["view_count"]
            .as_u64();

        // Check if it's a playlist
        let is_playlist = json["_type"].as_str() == Some("playlist")
            || json["playlist_count"].is_number()
            || url.contains("list=");

        let playlist_count = json["playlist_count"]
            .as_u64()
            .map(|c| c as usize)
            .or_else(|| {
                json["entries"]
                    .as_array()
                    .map(|e| e.len())
            });

        info!("Video info retrieved: title='{}', duration={}s, filesize={:?}, playlist={}", 
              title, duration, filesize, is_playlist);

        Ok(VideoInfo {
            title,
            duration,
            filesize,
            thumbnail,
            uploader,
            upload_date,
            view_count,
            is_playlist,
            playlist_count,
        })
    }

    /// Check if a URL is from YouTube
    pub fn is_youtube_url(url: &str) -> bool {
        url.contains("youtube.com") 
            || url.contains("youtu.be")
            || url.contains("youtube-nocookie.com")
    }

    /// Check if a URL is supported by yt-dlp (YouTube and many other sites)
    pub fn is_supported_url(url: &str) -> bool {
        // YouTube
        if Self::is_youtube_url(url) {
            return true;
        }

        // Other popular video platforms supported by yt-dlp
        let supported_domains = [
            "vimeo.com",
            "dailymotion.com",
            "twitch.tv",
            "twitter.com",
            "x.com",
            "facebook.com",
            "instagram.com",
            "tiktok.com",
            "reddit.com",
            "streamable.com",
            "soundcloud.com",
            "mixcloud.com",
            "bandcamp.com",
            "bilibili.com",
            "niconico.jp",
            "vk.com",
        ];

        supported_domains.iter().any(|domain| url.contains(domain))
    }

    /// Parse duration string like "1:23:45" to seconds
    fn parse_duration(duration_str: &str) -> Option<f64> {
        let re = Regex::new(r"(?:(\d+):)?(?:(\d+):)?(\d+)").ok()?;
        let caps = re.captures(duration_str)?;

        let hours = caps.get(1).and_then(|m| m.as_str().parse::<f64>().ok()).unwrap_or(0.0);
        let minutes = caps.get(2).and_then(|m| m.as_str().parse::<f64>().ok()).unwrap_or(0.0);
        let seconds = caps.get(3).and_then(|m| m.as_str().parse::<f64>().ok()).unwrap_or(0.0);

        Some(hours * 3600.0 + minutes * 60.0 + seconds)
    }
}
