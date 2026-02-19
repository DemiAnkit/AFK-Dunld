# Bundled yt-dlp Implementation Details

Technical implementation details for yt-dlp integration in AFK-Dunld.

## Table of Contents
- [Architecture](#architecture)
- [YtDlpManager](#ytdlpmanager)
- [Video Download Flow](#video-download-flow)
- [Format Selection](#format-selection)
- [Error Handling](#error-handling)
- [Progress Tracking](#progress-tracking)
- [Metadata Extraction](#metadata-extraction)
- [Playlist Support](#playlist-support)

## Architecture

### Component Overview

```
┌─────────────────────────────────────────────────┐
│              Frontend (React)                    │
│  ┌──────────────┐  ┌───────────────────────┐   │
│  │  YouTube     │  │  Playlist Download    │   │
│  │  Dialog      │  │  Dialog               │   │
│  └──────┬───────┘  └──────┬────────────────┘   │
└─────────┼──────────────────┼─────────────────────┘
          │ IPC              │ IPC
          ▼                  ▼
┌─────────────────────────────────────────────────┐
│           Tauri Commands Layer                   │
│  ┌──────────────┐  ┌───────────────────────┐   │
│  │ get_video_   │  │ add_download          │   │
│  │ info()       │  │ (YouTube URL)         │   │
│  └──────┬───────┘  └──────┬────────────────┘   │
└─────────┼──────────────────┼─────────────────────┘
          │                  │
          ▼                  ▼
┌─────────────────────────────────────────────────┐
│            YtDlpManager                          │
│  ┌──────────────┐  ┌───────────────────────┐   │
│  │ Binary Path  │  │  YouTubeDownloader    │   │
│  │ Management   │  │  Instance             │   │
│  └──────────────┘  └───────────────────────┘   │
└─────────────────────────────────────────────────┘
          │
          ▼
┌─────────────────────────────────────────────────┐
│        yt-dlp Binary (Subprocess)                │
│  • Extract metadata                              │
│  • Download video/audio                          │
│  • Format selection                              │
│  • Progress reporting                            │
└─────────────────────────────────────────────────┘
```

## YtDlpManager

### Implementation (`src-tauri/src/utils/ytdlp_manager.rs`)

```rust
use std::path::PathBuf;
use std::process::Command;
use serde::{Deserialize, Serialize};

pub struct YtDlpManager {
    binary_path: PathBuf,
    version: Option<String>,
}

impl YtDlpManager {
    pub fn new() -> Self {
        let binary_path = Self::find_binary_path();
        let version = Self::detect_version(&binary_path);
        
        Self {
            binary_path,
            version,
        }
    }
    
    fn find_binary_path() -> PathBuf {
        // 1. Check bundled location
        if let Some(bundled) = Self::get_bundled_path() {
            if bundled.exists() {
                return bundled;
            }
        }
        
        // 2. Check system PATH
        if let Ok(output) = Command::new("which")
            .arg("yt-dlp")
            .output()
        {
            if output.status.success() {
                return PathBuf::from(
                    String::from_utf8_lossy(&output.stdout).trim()
                );
            }
        }
        
        // 3. Fallback
        PathBuf::from("yt-dlp")
    }
    
    fn get_bundled_path() -> Option<PathBuf> {
        let app_data = dirs::data_local_dir()?;
        let bin_path = app_data.join("afk-dunld").join("bin");
        
        #[cfg(windows)]
        let binary = bin_path.join("yt-dlp.exe");
        
        #[cfg(not(windows))]
        let binary = bin_path.join("yt-dlp");
        
        Some(binary)
    }
    
    pub fn get_binary_path(&self) -> PathBuf {
        self.binary_path.clone()
    }
    
    pub fn get_version(&self) -> Option<&str> {
        self.version.as_deref()
    }
}
```

### Binary Path Resolution

**Priority Order**:
1. Bundled binary in app data directory
2. System PATH
3. Common installation paths
4. Fallback to "yt-dlp" command

## Video Download Flow

### Complete Flow

```
User Pastes YouTube URL
    ↓
Frontend validates URL
    ↓
Call get_video_info() command
    ↓
YtDlpManager extracts metadata
    ↓
Display info to user (title, duration, formats)
    ↓
User selects quality/format
    ↓
Call add_download() command
    ↓
Create DownloadTask
    ↓
Start YouTubeDownloader
    ↓
yt-dlp downloads video
    ↓
Progress updates emitted
    ↓
Download completes
    ↓
Merge video+audio (if needed)
    ↓
Save to destination
    ↓
Update database
    ↓
Emit download-complete event
```

### YouTubeDownloader Implementation

```rust
pub struct YouTubeDownloader {
    ytdlp_path: PathBuf,
}

impl YouTubeDownloader {
    pub fn new(ytdlp_path: PathBuf) -> Self {
        Self { ytdlp_path }
    }
    
    pub async fn get_video_info(&self, url: &str) 
        -> Result<VideoInfo> 
    {
        let output = Command::new(&self.ytdlp_path)
            .args(&[
                "--dump-json",
                "--no-playlist",
                url
            ])
            .output()?;
        
        let json = String::from_utf8(output.stdout)?;
        let info: VideoInfo = serde_json::from_str(&json)?;
        
        Ok(info)
    }
    
    pub async fn download(&self, options: DownloadOptions) 
        -> Result<PathBuf> 
    {
        let mut cmd = Command::new(&self.ytdlp_path);
        
        // Add arguments based on options
        self.add_format_args(&mut cmd, &options);
        self.add_output_args(&mut cmd, &options);
        
        // Execute
        let output = cmd.output()?;
        
        if !output.status.success() {
            return Err(/* error */);
        }
        
        Ok(options.output_path)
    }
}
```

## Format Selection

### Available Formats

yt-dlp provides multiple format codes:

| Code | Description |
|------|-------------|
| `bestvideo+bestaudio` | Best video + best audio (merged) |
| `best` | Best single file |
| `worst` | Worst quality |
| `bestvideo` | Best video only |
| `bestaudio` | Best audio only |

### Format Selection Logic

```rust
fn select_format(quality: &str, format_type: &str) -> String {
    match (format_type, quality) {
        ("video", "best") => "bestvideo+bestaudio/best",
        ("video", "1080p") => "bestvideo[height<=1080]+bestaudio/best[height<=1080]",
        ("video", "720p") => "bestvideo[height<=720]+bestaudio/best[height<=720]",
        ("audio", _) => "bestaudio/best",
        _ => "best"
    }.to_string()
}
```

### Example Command

```bash
yt-dlp \
  --format "bestvideo[height<=1080]+bestaudio/best[height<=1080]" \
  --merge-output-format mp4 \
  --output "/path/to/output.mp4" \
  "https://www.youtube.com/watch?v=VIDEO_ID"
```

## Error Handling

### Common Errors

```rust
#[derive(Debug, thiserror::Error)]
pub enum YtDlpError {
    #[error("Video not available: {0}")]
    VideoUnavailable(String),
    
    #[error("Private/age-restricted video")]
    AccessRestricted,
    
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("Invalid URL")]
    InvalidUrl,
    
    #[error("yt-dlp not found")]
    BinaryNotFound,
}
```

### Error Recovery

```rust
async fn download_with_retry(
    url: &str, 
    max_retries: u32
) -> Result<PathBuf> {
    let mut retries = 0;
    
    loop {
        match download_video(url).await {
            Ok(path) => return Ok(path),
            Err(e) if retries < max_retries => {
                retries += 1;
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
            Err(e) => return Err(e),
        }
    }
}
```

## Progress Tracking

### Progress Hook

```rust
pub async fn download_with_progress<F>(
    url: &str,
    progress_callback: F,
) -> Result<PathBuf>
where
    F: Fn(DownloadProgress) + Send + 'static,
{
    // Use yt-dlp's progress hooks
    let mut child = Command::new(&self.ytdlp_path)
        .args(&["--newline", "--progress", url])
        .stdout(Stdio::piped())
        .spawn()?;
    
    // Parse progress from stdout
    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        
        for line in reader.lines() {
            if let Ok(line) = line {
                if let Some(progress) = parse_progress(&line) {
                    progress_callback(progress);
                }
            }
        }
    }
    
    child.wait()?;
    Ok(output_path)
}
```

### Progress Format

yt-dlp outputs progress in this format:
```
[download]  45.2% of 10.50MiB at 1.20MiB/s ETA 00:04
```

Parsed into:
```rust
pub struct DownloadProgress {
    pub percentage: f64,      // 45.2
    pub downloaded: u64,      // bytes
    pub total: u64,          // bytes  
    pub speed: f64,          // bytes/s
    pub eta: Option<u64>,    // seconds
}
```

## Metadata Extraction

### Video Information

```rust
#[derive(Debug, Deserialize)]
pub struct VideoInfo {
    pub id: String,
    pub title: String,
    pub duration: Option<u64>,
    pub uploader: String,
    pub thumbnail: String,
    pub description: Option<String>,
    pub formats: Vec<Format>,
    pub filesize: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct Format {
    pub format_id: String,
    pub ext: String,
    pub resolution: Option<String>,
    pub filesize: Option<u64>,
    pub vcodec: Option<String>,
    pub acodec: Option<String>,
}
```

### Extraction Command

```bash
yt-dlp --dump-json --no-playlist URL
```

Returns full JSON with all metadata.

## Playlist Support

### Playlist Detection

```rust
pub async fn is_playlist(url: &str) -> Result<bool> {
    let output = Command::new(&self.ytdlp_path)
        .args(&["--flat-playlist", "--dump-json", url])
        .output()?;
    
    // If output contains multiple entries, it's a playlist
    let json = String::from_utf8(output.stdout)?;
    let entries: Vec<serde_json::Value> = json
        .lines()
        .filter_map(|line| serde_json::from_str(line).ok())
        .collect();
    
    Ok(entries.len() > 1)
}
```

### Playlist Download

```rust
pub async fn download_playlist(
    url: &str,
    options: PlaylistOptions,
) -> Result<Vec<PathBuf>> {
    // Get playlist info
    let info = self.get_playlist_info(url).await?;
    
    let mut downloaded = Vec::new();
    
    // Download each video
    for video in info.entries {
        let path = self.download_video(&video.url, &options).await?;
        downloaded.push(path);
    }
    
    Ok(downloaded)
}
```

## Related Documentation

- [YouTube Download Guide](docs/YOUTUBE_GUIDE.md)
- [yt-dlp Bundling Guide](YTDLP_BUNDLING_GUIDE.md)
- [Build Optimization](BUILD_OPTIMIZATION_GUIDE.md)
- [API Documentation](docs/API.md)
