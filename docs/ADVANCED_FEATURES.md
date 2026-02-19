# Advanced Features Guide

Comprehensive guide to advanced features in AFK-Dunld for power users.

## Table of Contents
- [Multi-threaded Downloads](#multi-threaded-downloads)
- [Queue Management](#queue-management)
- [Download Scheduler](#download-scheduler)
- [Category Management](#category-management)
- [FTP/SFTP Support](#ftpsftp-support)
- [Torrent Downloads](#torrent-downloads)
- [Browser Integration](#browser-integration)
- [Proxy Configuration](#proxy-configuration)
- [Checksum Verification](#checksum-verification)
- [Batch Operations](#batch-operations)
- [Speed Limiting](#speed-limiting)
- [Auto Error Recovery](#auto-error-recovery)

## Multi-threaded Downloads

### Overview
AFK-Dunld splits large files into multiple segments and downloads them simultaneously for faster speeds.

### How It Works
1. **Initial Request** - Check if server supports range requests
2. **Segment Creation** - Split file into N segments
3. **Parallel Download** - Download all segments simultaneously
4. **Merge** - Combine segments into final file

### Configuration

**Settings → Downloads → Segments**
- **Auto** - Automatically determine optimal segments (recommended)
- **1** - Single-threaded (disable multi-threading)
- **4** - 4 segments (good for most files)
- **8** - 8 segments (fast connections)
- **16** - 16 segments (very fast connections, may be throttled)

### When to Use

✅ **Use Multi-threading For:**
- Large files (>100 MB)
- Fast internet connections (>50 Mbps)
- Servers with good bandwidth
- HTTP/HTTPS downloads

❌ **Don't Use For:**
- Small files (<10 MB)
- Slow connections (<10 Mbps)
- Servers that throttle
- FTP downloads (use single thread)
- YouTube/video sites (use yt-dlp's settings)

### Performance Tips
1. **Test different segment counts** - More isn't always faster
2. **Monitor speed graphs** - See if segments help
3. **Respect server limits** - Some servers throttle multi-threading
4. **Use on reliable connections** - Unstable connections may fail more

## Queue Management

### Queue Overview
Control download order, priorities, and concurrent download limits.

### Accessing Queue Manager
- Click **"Queue"** tab in main window
- Or press `Ctrl+Q` / `Cmd+Q`

### Queue Features

#### Priority System
- **High Priority** (1-3) - Downloads first
- **Normal Priority** (4-6) - Default
- **Low Priority** (7-10) - Downloads last

**Set Priority:**
1. Right-click download
2. Select "Set Priority"
3. Choose priority level

#### Concurrent Download Limit
Control how many downloads run simultaneously.

**Settings → Downloads → Max Concurrent**
- **1** - One at a time (safest)
- **3** - Default, balanced
- **5** - For fast connections
- **Unlimited** - All at once (not recommended)

#### Queue Actions
- **Pause Queue** - Pause all downloads
- **Resume Queue** - Resume all paused
- **Clear Queue** - Remove all queued items
- **Reorder** - Drag and drop to change order

### Queue Strategies

**For Slow Connections:**
```
Max Concurrent: 1-2
Priority: Use wisely for important files
```

**For Fast Connections:**
```
Max Concurrent: 3-5
Priority: Balance large and small files
```

**For Background Downloads:**
```
Max Concurrent: 2
Speed Limit: 50% of bandwidth
```

## Download Scheduler

### Overview
Schedule downloads to start at specific times or recurring intervals.

### Accessing Scheduler
- Click **"Schedule"** tab
- Or Settings → Scheduler

### Creating a Schedule

1. **Click "Add Schedule"**
2. **Configure Schedule:**
   - **Name** - Descriptive name
   - **URL** - Download URL
   - **Type** - One-time or Recurring
   - **Start Time** - When to start
   - **Recurrence** - Daily, Weekly, Monthly
   - **End Condition** - Never, After N times, End date

### Schedule Types

#### One-Time Schedule
Download once at specific time.
```
Name: Important File
URL: https://example.com/file.zip
Type: One-time
Start: 2024-12-25 00:00
```

#### Recurring Schedule
Download at regular intervals.
```
Name: Daily Backup
URL: https://backup.example.com/backup.tar.gz
Type: Recurring
Start: Every day at 2:00 AM
End: Never
```

### Use Cases

**Download During Off-Peak Hours:**
```
Schedule large files for 2-6 AM
Benefit: Faster speeds, less network congestion
```

**Regular Content Updates:**
```
Schedule podcast/video series downloads
Recurrence: Daily at 3:00 PM
```

**Timed Releases:**
```
Schedule downloads for software releases
One-time: Release date/time
```

### Calendar View
- View all scheduled downloads
- See upcoming downloads
- Drag to reschedule
- Color-coded by status

### Schedule Management
- **Enable/Disable** - Toggle without deleting
- **Edit** - Modify schedule settings
- **Delete** - Remove schedule
- **History** - View past executions

## Category Management

### Overview
Organize downloads by type, project, or custom categories.

### Default Categories
- **Documents** - PDFs, DOCX, XLSX, etc.
- **Videos** - MP4, MKV, AVI, etc.
- **Music** - MP3, FLAC, AAC, etc.
- **Software** - EXE, DMG, DEB, etc.
- **Archives** - ZIP, RAR, 7Z, etc.
- **Images** - JPG, PNG, GIF, etc.

### Creating Custom Categories

1. **Settings → Categories**
2. **Click "Add Category"**
3. **Configure:**
   - **Name** - Category name
   - **Color** - Visual identifier
   - **Save Path** - Default download location
   - **Auto-match** - File extensions to auto-assign
   - **Icon** - Optional icon

**Example Custom Category:**
```
Name: Work Projects
Color: Blue
Save Path: C:\Work\Downloads
Auto-match: .psd, .ai, .sketch
Icon: briefcase
```

### Auto-Categorization

**By File Extension:**
```
.mp4, .mkv, .avi → Videos
.mp3, .flac, .m4a → Music
.zip, .rar, .7z → Archives
```

**By URL Pattern:**
```
youtube.com/* → YouTube Videos
github.com/*/releases/* → Software
```

**Manual Assignment:**
1. Right-click download
2. Select "Assign Category"
3. Choose category

### Category Features
- **Per-category save paths**
- **Per-category speed limits**
- **Filter downloads by category**
- **Statistics per category**
- **Batch operations per category**

## FTP/SFTP Support

### FTP Downloads

**Supported:**
- FTP (File Transfer Protocol)
- FTPS (FTP over SSL/TLS)
- Anonymous and authenticated FTP

**Adding FTP Download:**
1. Paste FTP URL
   ```
   ftp://ftp.example.com/path/to/file.zip
   ftp://username:password@ftp.example.com/file.zip
   ```
2. Enter credentials if required
3. Click "Add"

**FTP Browser:**
1. Click "FTP Browser" in toolbar
2. Connect to FTP server
3. Browse files and folders
4. Select files to download
5. Click "Download Selected"

### SFTP Downloads

**Supported:**
- SFTP (SSH File Transfer Protocol)
- Key-based authentication
- Password authentication

**Adding SFTP Download:**
1. Click "Add Download" → "SFTP"
2. Enter connection details:
   ```
   Host: sftp.example.com
   Port: 22
   Username: user
   Password/Key: [credentials]
   Remote Path: /path/to/file
   ```
3. Click "Connect and Download"

**SFTP Browser:**
1. Settings → SFTP → Saved Connections
2. Add new connection
3. Save credentials securely
4. Browse and download files

### Connection Management
- **Save connections** for quick access
- **Encrypted credential storage**
- **Connection pooling** for multiple files
- **Automatic reconnection** on disconnect

## Torrent Downloads

### Overview
Download torrent files and magnet links with built-in torrent client.

### Supported Formats
- **.torrent files**
- **Magnet links** (`magnet:?xt=urn:btih:...`)

### Adding Torrent Download

**Method 1: Torrent File**
1. Click "Add Download"
2. Click "Browse" and select .torrent file
3. Choose files to download
4. Select save location
5. Click "Add"

**Method 2: Magnet Link**
1. Copy magnet link
2. Click "Add Download"
3. Paste magnet link
4. Choose files (after metadata fetch)
5. Click "Add"

### Torrent Features
- **Peer selection** - DHT, PEX, Trackers
- **Speed limits** - Per-torrent or global
- **Bandwidth scheduling** - Time-based limits
- **Sequential download** - Download in order
- **Selective download** - Choose specific files
- **Seeding** - Upload to peers after download

### Torrent Settings

**Settings → Torrents:**
- **Max connections** - Per torrent
- **Max peers** - Global limit
- **Upload limit** - Seeding speed
- **Download location** - Default save path
- **Auto-seeding** - Continue seeding after download
- **Seed ratio** - Stop seeding after ratio

### Torrent Management
- **View peers** - See connected peers
- **View trackers** - See tracker status
- **Add trackers** - Manually add trackers
- **Force recheck** - Verify downloaded data
- **Pause/Resume seeding**

## Browser Integration

### Overview
Download files directly from browser with one click.

### Chrome Extension

**Installation:**
1. Download from Chrome Web Store
2. Or load unpacked: `browser-extension/chrome/`
3. Grant permissions

**Features:**
- **Auto-intercept downloads** - Large files sent to AFK-Dunld
- **Right-click menu** - "Download with AFK-Dunld"
- **YouTube button** - Download button on videos
- **Keyboard shortcut** - Ctrl+Shift+Click

**Settings:**
- **Auto-intercept threshold** - File size minimum
- **File types** - Which extensions to intercept
- **Connection method** - Native messaging or protocol

### Firefox Extension

**Installation:**
1. Download from Firefox Add-ons
2. Or load temporarily: `browser-extension/firefox/`
3. Grant permissions

**Features:**
- Same as Chrome extension
- Uses native messaging for communication

### Native Messaging Setup

**Automatic (Recommended):**
1. Open AFK-Dunld
2. Settings → Browser Integration
3. Click "Install Browser Extension Support"
4. Extension will connect automatically

**Manual:**
See [Browser Extension README](../browser-extension/README.md)

### Usage

**Auto-Intercept:**
1. Enable in extension popup
2. Click any download link
3. Large files sent to AFK-Dunld automatically

**Right-Click:**
1. Right-click link/image/video
2. Select "Download with AFK-Dunld"

**YouTube:**
1. Go to YouTube video
2. Click "↓ AFK-Dunld" button
3. Video downloaded with chosen quality

## Proxy Configuration

### Supported Proxy Types
- **HTTP** - Basic HTTP proxy
- **HTTPS** - Secure HTTP proxy
- **SOCKS5** - Universal proxy protocol

### Configuring Proxy

**Settings → Network → Proxy:**

**HTTP/HTTPS Proxy:**
```
Type: HTTP/HTTPS
Host: proxy.example.com
Port: 8080
Username: (optional)
Password: (optional)
```

**SOCKS5 Proxy:**
```
Type: SOCKS5
Host: socks-proxy.example.com
Port: 1080
Username: (optional)
Password: (optional)
```

### Proxy Settings
- **Use proxy for all downloads**
- **Use proxy for specific downloads**
- **Bypass proxy for local URLs**
- **Test proxy connection**

### Per-Download Proxy
1. Right-click download
2. Select "Proxy Settings"
3. Choose proxy or direct connection

## Checksum Verification

### Supported Algorithms
- **MD5** - Fast, less secure
- **SHA-1** - Deprecated, compatibility
- **SHA-256** - Recommended
- **SHA-512** - Maximum security

### Adding Checksum Verification

**During Download:**
1. Click "Add Download"
2. Expand "Advanced Options"
3. Enter expected checksum
4. Select algorithm
5. Click "Add"

**After Download:**
1. Right-click completed download
2. Select "Verify Checksum"
3. Enter expected checksum
4. Click "Verify"

### Auto-Verification
Some websites provide checksums:
- AFK-Dunld can auto-fetch from common formats
- `.md5`, `.sha256` files
- Checksum in download page

### Verification Results
- ✅ **Match** - File is intact
- ❌ **Mismatch** - File may be corrupted
- ⚠️ **No checksum** - Verification skipped

## Batch Operations

### Batch Download
Add multiple URLs at once.

**Method 1: Paste Multiple URLs**
1. Click "Add Download"
2. Paste URLs (one per line)
   ```
   https://example.com/file1.zip
   https://example.com/file2.zip
   https://example.com/file3.zip
   ```
3. Click "Add All"

**Method 2: Import from File**
1. Create text file with URLs (one per line)
2. Click "Batch Download" → "Import from File"
3. Select file
4. Configure common settings
5. Click "Add All"

### Batch Actions
Select multiple downloads and:
- **Pause All** - Pause selected
- **Resume All** - Resume selected
- **Cancel All** - Cancel selected
- **Set Category** - Assign category to all
- **Set Priority** - Change priority
- **Move to Folder** - Change save location

### Export/Import
- **Export downloads** - Save list as JSON
- **Import downloads** - Restore from JSON
- **Export history** - Backup download history

## Speed Limiting

### Global Speed Limit
Limit total download speed for all downloads.

**Settings → Network → Global Speed Limit:**
- **Unlimited** - No limit
- **50%** - Half of bandwidth
- **Custom** - Specify in KB/s or MB/s

### Per-Download Speed Limit
1. Right-click download
2. Select "Speed Limit"
3. Set limit for this download

### Bandwidth Scheduling
Schedule speed limits by time:
```
9 AM - 5 PM: 1 MB/s (work hours)
5 PM - 9 AM: Unlimited (off-peak)
```

**Settings → Network → Bandwidth Schedule:**
1. Click "Add Schedule"
2. Set time range
3. Set speed limit
4. Save

### Smart Throttling
Automatically adjust speed based on:
- **Network usage** - Slow down if network busy
- **System resources** - Limit if CPU/RAM high
- **Active applications** - Detect streaming, gaming

## Auto Error Recovery

### Overview
Automatically handle and recover from download errors.

### Error Recovery Features

**Automatic Retry:**
- Retry failed downloads automatically
- Configurable retry count (1-10)
- Exponential backoff between retries
- Max retry interval

**Resume Support:**
- Continue interrupted downloads
- Works with HTTP range requests
- Saves progress every 5 seconds
- Automatic resume on app restart

**Connection Recovery:**
- Detect connection loss
- Auto-reconnect when available
- Resume from last position
- No data loss

### Configuration

**Settings → Downloads → Error Recovery:**
```
Auto Retry: Enabled
Max Retries: 5
Retry Delay: 10 seconds
Exponential Backoff: Enabled
Max Delay: 5 minutes
```

### Error Types Handled
- **Network errors** - Connection timeout, DNS failure
- **Server errors** - 500, 502, 503 errors
- **File system errors** - Disk full, permission denied
- **Partial content errors** - Resume support

### Manual Retry
For downloads that exceed retry limit:
1. Right-click failed download
2. Select "Retry Download"
3. Download starts from beginning or resumes

## Tips & Tricks

### Optimize Download Speed
1. Use **4-8 segments** for large files
2. **Disable** speed limits
3. **Pause** other downloads
4. **Close** bandwidth-heavy apps
5. Use **wired connection** over Wi-Fi

### Organize Downloads
1. Create **categories** for projects
2. Use **separate folders** per category
3. Enable **auto-categorization**
4. **Archive** old downloads

### Save Bandwidth
1. **Schedule** large downloads for off-peak
2. Set **global speed limit**
3. Use **queue management**
4. Enable **compression** for supported files

### Troubleshoot Issues
1. Check **logs** in Settings → Logs
2. Enable **debug mode** for detailed info
3. **Update** to latest version
4. **Clear cache** if corruption suspected

## Advanced Configuration

### Config File Location
- Windows: `%APPDATA%\com.ankit.afk-dunld\config.json`
- macOS: `~/Library/Application Support/com.ankit.afk-dunld/config.json`
- Linux: `~/.local/share/com.ankit.afk-dunld/config.json`

### Database
- SQLite database: `downloads.db`
- Backup before major updates
- Can be exported/imported

### Logs
- Location: `logs/` in config directory
- Rotation: Daily, keep 7 days
- Levels: ERROR, WARN, INFO, DEBUG

## Next Steps

- [Troubleshooting Guide](TROUBLESHOOTING.md)
- [API Documentation](API.md)
- [Architecture Overview](ARCHITECTURE.md)

## Need Help?

- 🐛 [Report Issue](https://github.com/yourusername/afk-dunld/issues)
- 💬 [Discussions](https://github.com/yourusername/afk-dunld/discussions)
- 📧 Email: support@afk-dunld.com
