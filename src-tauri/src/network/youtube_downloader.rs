use std::process::Stdio;
use tokio::process::Command;
use std::path::PathBuf;
use anyhow::{Result, Context, bail};
use regex::Regex;
use tokio::io::{AsyncBufReadExt, BufReader};
use serde::{Serialize, Deserialize};

pub struct YouTubeDownloader;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YouTubeDownloadOptions {
    pub url: String,
    pub format_type: String,      // "video" or "audio"
    pub video_quality: String,     // "2160p", "1080p", etc.
    pub video_format: String,      // "mp4", "mkv", "webm"
    pub audio_format: String,      // "mp3", "aac", "flac", "opus", "m4a"
    pub save_path: PathBuf,
    pub is_playlist: bool,         // Whether to download entire playlist
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
        Self
    }

    /// Check if yt-dlp is installed and available
    pub async fn check_installation() -> Result<bool> {
        let result = Command::new("yt-dlp")
            .arg("--version")
            .output()
            .await;

        match result {
            Ok(output) => Ok(output.status.success()),
            Err(_) => Ok(false),
        }
    }

    /// Get available quality options for a video
    pub async fn get_available_qualities(&self, url: &str) -> Result<Vec<QualityOption>> {
        if !Self::is_supported_url(url) {
            bail!("Unsupported URL: {}", url);
        }

        tracing::debug!("Fetching available qualities for: {}", url);

        let output = Command::new("yt-dlp")
            .args(&[
                "-F",  // List all formats
                "--dump-json",
                url,
            ])
            .output()
            .await
            .context("Failed to fetch available qualities")?;

        if !output.status.success() {
            bail!("Failed to fetch formats");
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
        if !Self::check_installation().await? {
            bail!("yt-dlp is not installed. Please install it from https://github.com/yt-dlp/yt-dlp");
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
            "-o", options.save_path.to_str().unwrap_or("download.mp4"),
            &options.url,
        ]);

        tracing::info!("Starting YouTube/video download with yt-dlp");
        tracing::debug!("yt-dlp args: {:?}", args);

        let output = Command::new("yt-dlp")
            .args(&args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .context("Failed to execute yt-dlp. Make sure yt-dlp is installed and in PATH")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            
            tracing::error!("yt-dlp failed with stderr: {}", stderr);
            tracing::error!("yt-dlp stdout: {}", stdout);
            
            // Provide user-friendly error messages
            let error_msg = if stderr.contains("HTTP Error 403") {
                "Video is not available or requires authentication"
            } else if stderr.contains("Video unavailable") {
                "Video is unavailable or has been removed"
            } else if stderr.contains("Unsupported URL") {
                "This URL is not supported"
            } else if stderr.contains("Private video") {
                "This video is private"
            } else if stderr.contains("Sign in") {
                "This video requires signing in to view"
            } else {
                "Download failed. Check the URL and try again"
            };
            
            bail!("{}: {}", error_msg, stderr);
        }

        tracing::info!("Download completed successfully: {:?}", options.save_path);
        Ok(options.save_path)
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
        if !Self::check_installation().await? {
            bail!("yt-dlp is not installed. Please install it from https://github.com/yt-dlp/yt-dlp");
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
            "-o", options.save_path.to_str().unwrap_or("download.mp4"),
            &options.url,
        ]);

        tracing::info!("Starting download with progress tracking");

        let mut child = Command::new("yt-dlp")
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
            tracing::error!("yt-dlp failed: {}", stderr);
            bail!("Download failed: {}", stderr);
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

        Ok(options.save_path)
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

        tracing::debug!("Fetching video info for: {}", url);

        let output = Command::new("yt-dlp")
            .args(&[
                "--dump-json",
                "--no-playlist",
                "--skip-download",
                url,
            ])
            .output()
            .await
            .context("Failed to execute yt-dlp for video info")?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            tracing::error!("Failed to get video info: {}", error);
            bail!("Failed to get video info: {}", error);
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

        tracing::info!("Video info retrieved: title='{}', duration={}s, filesize={:?}, playlist={}", 
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
            "pornhub.com",
            "xvideos.com",
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