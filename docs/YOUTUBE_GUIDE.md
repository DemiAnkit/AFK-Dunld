# YouTube Download Guide

Complete guide to downloading videos and audio from YouTube and 1000+ other video platforms using AFK-Dunld.

## Table of Contents
- [Quick Start](#quick-start)
- [Downloading Videos](#downloading-videos)
- [Downloading Audio](#downloading-audio)
- [Playlist Downloads](#playlist-downloads)
- [Advanced Options](#advanced-options)
- [Quality Selection](#quality-selection)
- [Supported Platforms](#supported-platforms)
- [Troubleshooting](#troubleshooting)

## Quick Start

### Download a Single Video

1. **Copy the video URL** from your browser
2. Open AFK-Dunld
3. Click **"+ Add Download"** or press `Ctrl+N` / `Cmd+N`
4. **Paste the URL** in the URL field
5. The app automatically detects it's a YouTube video
6. **Select format**:
   - **Video** - Download video with audio
   - **Audio Only** - Extract audio only
7. **Choose quality/format**
8. Click **"Add"** to start downloading

### Download Audio Only (MP3)

1. Paste YouTube URL
2. Select **"Audio Only"**
3. Choose format: **MP3** (most compatible)
4. Choose quality: **Best** (highest bitrate)
5. Click **"Add"**
6. Audio will be extracted and saved as MP3

## Downloading Videos

### Video Quality Options

AFK-Dunld offers multiple quality options:

| Quality | Resolution | Use Case |
|---------|-----------|----------|
| **Best** | Highest available (up to 8K) | Maximum quality |
| **4K** | 3840x2160 | Ultra HD displays |
| **1440p** | 2560x1440 | QHD/2K displays |
| **1080p** | 1920x1080 | Full HD (recommended) |
| **720p** | 1280x720 | HD, smaller file size |
| **480p** | 854x480 | SD, mobile devices |
| **360p** | 640x360 | Low bandwidth |

### Video Format Options

- **MP4** (recommended) - Universal compatibility
- **WebM** - Open format, good quality
- **MKV** - High quality, large files
- **FLV** - Legacy format

### Step-by-Step Video Download

1. **Get Video Info First** (optional)
   ```
   Click "Get Video Info" to see:
   - Title
   - Duration
   - Available qualities
   - File size estimates
   ```

2. **Select Video Quality**
   - Choose from dropdown
   - File size shown for each quality
   - Higher quality = larger file

3. **Choose Video Format**
   - MP4 works on all devices
   - WebM for web usage
   - MKV for archival

4. **Set Save Location** (optional)
   - Default: Downloads folder
   - Click folder icon to change
   - Per-category defaults available

5. **Start Download**
   - Click "Add" button
   - Download starts immediately
   - Progress shown in main window

## Downloading Audio

### Audio Format Options

| Format | Quality | Compatibility | Use Case |
|--------|---------|---------------|----------|
| **MP3** | Good | Universal | Most devices |
| **AAC** | Better | Modern devices | Apple devices |
| **FLAC** | Lossless | Audio players | Archival |
| **Opus** | Best | Modern browsers | Smallest files |
| **M4A** | Better | Apple ecosystem | iTunes/iPhone |

### Audio Quality Settings

- **Best** - Highest bitrate available (usually 320kbps)
- **High** - 256kbps
- **Medium** - 192kbps
- **Low** - 128kbps

### Step-by-Step Audio Download

1. **Paste Video URL**
2. **Select "Audio Only"**
3. **Choose Audio Format**
   - **MP3** - Most compatible (recommended)
   - **AAC** - Better quality, same size
   - **FLAC** - Lossless, large files
4. **Select Quality**
   - **Best** for music
   - **Medium** for podcasts/speech
5. **Click "Add"**

### Audio Extraction Process

AFK-Dunld uses yt-dlp to:
1. Download the best audio stream
2. Convert to your chosen format
3. Embed metadata (title, artist, album art)
4. Save with proper file extension

## Playlist Downloads

### Download Entire Playlists

1. **Copy Playlist URL**
   ```
   https://www.youtube.com/playlist?list=PLAYLIST_ID
   ```

2. **Paste in AFK-Dunld**
   - Playlist detected automatically
   - Playlist dialog opens

3. **Configure Playlist Settings**
   - Format: Video or Audio Only
   - Quality: Apply to all videos
   - Video Format: MP4, WebM, etc.
   - Audio Format: MP3, AAC, etc.

4. **Preview Playlist**
   - See all videos in playlist
   - Total count and estimated size
   - Option to select specific videos

5. **Start Playlist Download**
   - Click "Download Playlist"
   - Videos download sequentially
   - Each video appears in download list

### Playlist Options

- **Download All** - All videos in playlist
- **Select Specific** - Choose which videos
- **Skip Downloaded** - Don't re-download existing
- **Update Playlist** - Check for new videos later

### Managing Large Playlists

For playlists with 50+ videos:

1. **Use Queue Management**
   - Set max concurrent downloads: 2-3
   - Prevents overwhelming system

2. **Schedule Downloads**
   - Download during off-peak hours
   - Use scheduler feature

3. **Monitor Progress**
   - Check Queue tab
   - See completed/remaining count

## Advanced Options

### Custom File Naming

Configure filename template in Settings:
```
%(title)s-%(id)s.%(ext)s
%(uploader)s - %(title)s.%(ext)s
%(upload_date)s - %(title)s.%(ext)s
```

### Subtitle Downloads

1. Enable in Settings → YouTube
2. Subtitle options:
   - **Auto-generated** - Available for most videos
   - **Manual** - If uploaded by creator
   - **All languages** - Download all available
   - **Specific language** - English, Spanish, etc.

### Metadata Embedding

Automatically embed:
- Title
- Artist/Uploader
- Album (playlist name)
- Thumbnail as cover art
- Upload date
- Description

### Speed Limiting

Limit download speed for YouTube:
1. Settings → Network
2. Set speed limit per download
3. Or global speed limit

## Quality Selection

### How Quality Selection Works

1. **yt-dlp fetches available formats**
2. **AFK-Dunld filters by quality setting**
3. **Best matching format selected**
4. **Download starts**

### Format Selection Priority

For **Video**:
1. Resolution match (1080p, 720p, etc.)
2. Format preference (MP4 > WebM > other)
3. Codec preference (H.264 > VP9 > others)
4. Smallest file size for same quality

For **Audio**:
1. Bitrate match (320kbps, 256kbps, etc.)
2. Format preference (Opus > AAC > MP3)
3. Smallest file size for same quality

### Adaptive Formats

YouTube uses adaptive formats:
- **Video-only streams** (no audio)
- **Audio-only streams** (no video)

yt-dlp automatically:
1. Downloads best video stream
2. Downloads best audio stream
3. Merges them together
4. Outputs single file

## Supported Platforms

### Video Platforms (1000+ sites)

**Popular Platforms:**
- ✅ YouTube, YouTube Music
- ✅ Vimeo
- ✅ Dailymotion
- ✅ Twitch (VODs and clips)
- ✅ Twitter/X
- ✅ Facebook
- ✅ Instagram
- ✅ TikTok
- ✅ Reddit (v.redd.it)
- ✅ Imgur

**Social Media:**
- ✅ LinkedIn
- ✅ Pinterest
- ✅ Snapchat (public stories)
- ✅ Tumblr

**Streaming:**
- ✅ Bilibili
- ✅ Niconico
- ✅ Viki
- ✅ VLive

**Music:**
- ✅ SoundCloud
- ✅ Bandcamp
- ✅ Mixcloud
- ✅ Audiomack

**Education:**
- ✅ Coursera
- ✅ Udemy
- ✅ Khan Academy
- ✅ TED

[Full list of 1000+ supported sites](https://github.com/yt-dlp/yt-dlp/blob/master/supportedsites.md)

## Troubleshooting

### Video Not Downloading

**Issue:** "Unable to extract video information"

**Solutions:**
1. **Update yt-dlp**
   - Settings → YouTube → "Update yt-dlp"
   - Fixes compatibility with site changes

2. **Check URL**
   - Ensure full URL is copied
   - No extra characters or spaces

3. **Try Different Quality**
   - Some qualities may not be available
   - Try "Best" or lower quality

### Download Stalls/Fails

**Issue:** Download starts but fails

**Solutions:**
1. **Check Internet Connection**
2. **Retry Download**
   - Right-click → Retry
3. **Clear Cache**
   - Settings → Advanced → Clear Cache
4. **Update yt-dlp**

### Slow Download Speed

**Issue:** YouTube download very slow

**Solutions:**
1. **Disable Speed Limit**
   - Settings → Network → Remove limits
2. **Try Different Time**
   - Peak hours may be slower
3. **Check Your Internet Speed**
4. **Use Fewer Segments**
   - Settings → Downloads → Segments: 1

### Audio/Video Out of Sync

**Issue:** Merged video has sync issues

**Solutions:**
1. **Update yt-dlp and FFmpeg**
2. **Try Different Format**
   - Use MP4 instead of WebM
3. **Download Pre-merged Format**
   - May have lower quality

### Age-Restricted Content

**Issue:** "Sign in to confirm your age"

**Solutions:**
1. **Add Cookies** (Advanced)
   - Export cookies from browser
   - Settings → YouTube → Import Cookies
2. **Use Browser Extension** (Easier)
   - Download via browser extension
   - Automatically uses your login

### Private/Unlisted Videos

**Issue:** "Video is private"

**Solutions:**
1. **Ensure Video is Accessible**
   - Can you watch it in browser?
2. **Use Cookies for Private Playlists**
   - Import browser cookies
3. **Check Sharing Settings**

### Playlist Not Detected

**Issue:** Playlist URL treated as single video

**Solutions:**
1. **Use Full Playlist URL**
   ```
   ✓ https://www.youtube.com/playlist?list=PLxxxxx
   ✗ https://www.youtube.com/watch?v=xxxxx&list=PLxxxxx
   ```
2. **Ensure "list=" in URL**

### Geographic Restrictions

**Issue:** "Video not available in your country"

**Solutions:**
1. **Use Proxy** (if legal in your area)
   - Settings → Network → Proxy Settings
2. **Use VPN** (external to app)

## Tips & Tricks

### Batch Download Videos

Download multiple videos at once:
1. Paste URLs separated by newlines
2. Or use "Batch Download" feature
3. All videos added to queue

### Download Video Descriptions

1. Settings → YouTube
2. Enable "Save Description"
3. Saved as .txt file alongside video

### Thumbnail as Cover Art

Audio downloads automatically:
- Download video thumbnail
- Embed as cover art in MP3/M4A
- Visible in music players

### Monitor YouTube Channels

Coming in future version:
- Subscribe to channels
- Auto-download new videos
- RSS feed integration

### Keyboard Shortcuts

- `Ctrl+N` / `Cmd+N` - Add new download
- `Ctrl+V` - Paste URL from clipboard
- `Enter` - Start download
- `Esc` - Cancel dialog

## Best Practices

### For Video Quality
1. Use **1080p MP4** for best compatibility
2. Use **4K** only if you have 4K display
3. Use **720p** for mobile devices

### For Audio Quality
1. Use **MP3 320kbps** for music
2. Use **AAC 256kbps** for Apple devices
3. Use **Opus** for smallest files
4. Use **FLAC** for archival

### For Playlists
1. Download during **off-peak hours**
2. Use **queue management**
3. Enable **skip downloaded** to avoid duplicates
4. Set **max concurrent** to 2-3 downloads

### For Storage
1. Choose **save location** before downloading
2. Use **categories** to organize
3. Enable **auto-delete** for temp files
4. Monitor **disk space** in status bar

## Resources

- [yt-dlp Documentation](https://github.com/yt-dlp/yt-dlp#readme)
- [Supported Sites List](https://github.com/yt-dlp/yt-dlp/blob/master/supportedsites.md)
- [Format Selection](https://github.com/yt-dlp/yt-dlp#format-selection)
- [AFK-Dunld FAQ](TROUBLESHOOTING.md)

## Need Help?

- 📖 [Troubleshooting Guide](TROUBLESHOOTING.md)
- 🐛 [Report Issue](https://github.com/yourusername/afk-dunld/issues)
- 💬 [Discussions](https://github.com/yourusername/afk-dunld/discussions)
