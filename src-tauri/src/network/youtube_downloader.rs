use std::process::Stdio;
use tokio::process::Command;
use std::path::PathBuf;
use anyhow::{Result, Context};

pub struct YouTubeDownloader;

#[derive(Debug, Clone)]
pub struct YouTubeDownloadOptions {
    pub url: String,
    pub format_type: String,      // "video" or "audio"
    pub video_quality: String,     // "2160p", "1080p", etc.
    pub video_format: String,      // "mp4", "mkv", "webm"
    pub audio_format: String,      // "mp3", "aac", "flac", "opus", "m4a"
    pub save_path: PathBuf,
}

impl YouTubeDownloader {
    pub fn new() -> Self {
        Self
    }

    pub async fn download(&self, options: YouTubeDownloadOptions) -> Result<PathBuf> {
        let mut args = vec![];

        if options.format_type == "audio" {
            // Audio-only download
            args.extend_from_slice(&[
                "-x",  // Extract audio
                "--audio-format", &options.audio_format,
                "--audio-quality", "0",  // Best quality
            ]);
        } else {
            // Video download
            let format_spec = match options.video_quality.as_str() {
                "2160p" => "bestvideo[height<=2160]+bestaudio/best",
                "1440p" => "bestvideo[height<=1440]+bestaudio/best",
                "1080p" => "bestvideo[height<=1080]+bestaudio/best",
                "720p" => "bestvideo[height<=720]+bestaudio/best",
                "480p" => "bestvideo[height<=480]+bestaudio/best",
                "360p" => "bestvideo[height<=360]+bestaudio/best",
                _ => "bestvideo+bestaudio/best",
            };

            args.extend_from_slice(&[
                "-f", format_spec,
                "--merge-output-format", &options.video_format,
            ]);
        }

        // Common options
        args.extend_from_slice(&[
            "--no-playlist",  // Download single video
            "--progress",     // Show progress
            "--newline",      // New line for each progress update
            "-o", options.save_path.to_str().unwrap(),
            &options.url,
        ]);

        tracing::info!("Starting YouTube download with yt-dlp: {:?}", args);

        let output = Command::new("yt-dlp")
            .args(&args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .context("Failed to execute yt-dlp")?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("yt-dlp failed: {}", error);
        }

        Ok(options.save_path)
    }

    pub async fn get_video_info(&self, url: &str) -> Result<VideoInfo> {
        let output = Command::new("yt-dlp")
            .args(&[
                "--dump-json",
                "--no-playlist",
                url,
            ])
            .output()
            .await
            .context("Failed to get video info")?;

        if !output.status.success() {
            anyhow::bail!("Failed to get video info");
        }

        let json: serde_json::Value = serde_json::from_slice(&output.stdout)?;
        
        Ok(VideoInfo {
            title: json["title"].as_str().unwrap_or("Unknown").to_string(),
            duration: json["duration"].as_f64().unwrap_or(0.0) as u64,
            filesize: json["filesize"].as_u64(),
        })
    }

    pub fn is_youtube_url(url: &str) -> bool {
        url.contains("youtube.com") || url.contains("youtu.be")
    }
}

#[derive(Debug, Clone)]
pub struct VideoInfo {
    pub title: String,
    pub duration: u64,
    pub filesize: Option<u64>,
}