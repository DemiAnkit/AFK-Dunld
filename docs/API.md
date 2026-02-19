# API Documentation

Complete API reference for AFK-Dunld Tauri commands and event system.

## Table of Contents
- [Introduction](#introduction)
- [Download Commands](#download-commands)
- [Queue Commands](#queue-commands)
- [Settings Commands](#settings-commands)
- [Category Commands](#category-commands)
- [YouTube Commands](#youtube-commands)
- [Browser Integration Commands](#browser-integration-commands)
- [System Commands](#system-commands)
- [Event System](#event-system)
- [Data Types](#data-types)
- [Error Handling](#error-handling)

## Introduction

AFK-Dunld uses Tauri's IPC (Inter-Process Communication) system for communication between the frontend (JavaScript/TypeScript) and backend (Rust).

### Basic Usage

```typescript
import { invoke } from '@tauri-apps/api/tauri';

// Call a command
const result = await invoke<ReturnType>('command_name', {
  param1: 'value1',
  param2: 'value2'
});
```

### Error Handling

```typescript
try {
  const result = await invoke('command_name', { params });
} catch (error) {
  console.error('Command failed:', error);
}
```

## Download Commands

### add_download

Add a new download to the queue.

**Command**: `add_download`

**Parameters**:
```typescript
{
  url: string;                    // Download URL
  save_path?: string;             // Optional save location
  file_name?: string;             // Optional custom filename
  segments?: number;              // Number of segments (default: auto)
  max_retries?: number;           // Max retry attempts (default: 5)
  expected_checksum?: string;     // Optional checksum for verification
  checksum_type?: 'md5' | 'sha256' | 'sha512';
  category?: string;              // Category ID
  priority?: number;              // Priority 1-10 (default: 5)
}
```

**Returns**: `Promise<string>` - Download ID (UUID)

**Example**:
```typescript
const downloadId = await invoke<string>('add_download', {
  url: 'https://example.com/file.zip',
  save_path: 'C:\\Downloads',
  segments: 8
});
```

### pause_download

Pause an active download.

**Command**: `pause_download`

**Parameters**:
```typescript
{
  id: string;  // Download ID
}
```

**Returns**: `Promise<void>`

**Example**:
```typescript
await invoke('pause_download', { id: downloadId });
```

### resume_download

Resume a paused download.

**Command**: `resume_download`

**Parameters**:
```typescript
{
  id: string;  // Download ID
}
```

**Returns**: `Promise<void>`

**Example**:
```typescript
await invoke('resume_download', { id: downloadId });
```

### cancel_download

Cancel a download and remove it from the queue.

**Command**: `cancel_download`

**Parameters**:
```typescript
{
  id: string;  // Download ID
}
```

**Returns**: `Promise<void>`

### remove_download

Remove a download from history (completed/failed only).

**Command**: `remove_download`

**Parameters**:
```typescript
{
  id: string;  // Download ID
}
```

**Returns**: `Promise<void>`

### retry_download

Retry a failed download.

**Command**: `retry_download`

**Parameters**:
```typescript
{
  id: string;  // Download ID
}
```

**Returns**: `Promise<void>`

### get_all_downloads

Get all downloads.

**Command**: `get_all_downloads`

**Parameters**: None

**Returns**: `Promise<Download[]>`

**Example**:
```typescript
const downloads = await invoke<Download[]>('get_all_downloads');
```

### get_download_progress

Get real-time progress for a download.

**Command**: `get_download_progress`

**Parameters**:
```typescript
{
  id: string;  // Download ID
}
```

**Returns**: `Promise<DownloadProgress>`

```typescript
interface DownloadProgress {
  id: string;
  downloaded_size: number;
  total_size: number;
  speed: number;
  eta: number | null;
  percentage: number;
}
```

### pause_all

Pause all active downloads.

**Command**: `pause_all`

**Parameters**: None

**Returns**: `Promise<void>`

### resume_all

Resume all paused downloads.

**Command**: `resume_all`

**Parameters**: None

**Returns**: `Promise<void>`

### cancel_all

Cancel all active/paused downloads.

**Command**: `cancel_all`

**Parameters**: None

**Returns**: `Promise<void>`

### open_file

Open a completed download file with default application.

**Command**: `open_file`

**Parameters**:
```typescript
{
  id: string;  // Download ID
}
```

**Returns**: `Promise<void>`

### open_file_location

Open the folder containing the downloaded file.

**Command**: `open_file_location`

**Parameters**:
```typescript
{
  id: string;  // Download ID
}
```

**Returns**: `Promise<void>`

### add_batch_downloads

Add multiple downloads at once.

**Command**: `add_batch_downloads`

**Parameters**:
```typescript
{
  urls: string[];
  save_path?: string;
  category?: string;
}
```

**Returns**: `Promise<string[]>` - Array of download IDs

### get_file_info

Get file information without downloading (HEAD request).

**Command**: `get_file_info`

**Parameters**:
```typescript
{
  url: string;
}
```

**Returns**: `Promise<FileInfo>`

```typescript
interface FileInfo {
  filename: string;
  size: number | null;
  content_type: string | null;
  supports_resume: boolean;
}
```

## Queue Commands

### get_queue_info

Get queue status and statistics.

**Command**: `get_queue_info`

**Parameters**: None

**Returns**: `Promise<QueueInfo>`

```typescript
interface QueueInfo {
  total: number;
  active: number;
  queued: number;
  paused: number;
  completed: number;
  failed: number;
}
```

### set_max_concurrent

Set maximum concurrent downloads.

**Command**: `set_max_concurrent`

**Parameters**:
```typescript
{
  max: number;  // 1-10, or 0 for unlimited
}
```

**Returns**: `Promise<void>`

## Settings Commands

### get_settings

Get all application settings.

**Command**: `get_settings`

**Parameters**: None

**Returns**: `Promise<Settings>`

```typescript
interface Settings {
  default_download_path: string;
  max_concurrent_downloads: number;
  default_segments: number;
  speed_limit: number | null;  // KB/s, null for unlimited
  auto_start: boolean;
  minimize_to_tray: boolean;
  theme: 'light' | 'dark' | 'system';
  notifications_enabled: boolean;
  // ... more settings
}
```

### update_settings

Update application settings.

**Command**: `update_settings`

**Parameters**:
```typescript
{
  settings: Partial<Settings>;
}
```

**Returns**: `Promise<void>`

### set_speed_limit

Set global download speed limit.

**Command**: `set_speed_limit`

**Parameters**:
```typescript
{
  limit: number | null;  // KB/s, null for unlimited
}
```

**Returns**: `Promise<void>`

## Category Commands

### get_categories

Get all download categories.

**Command**: `get_categories`

**Parameters**: None

**Returns**: `Promise<Category[]>`

```typescript
interface Category {
  id: string;
  name: string;
  color: string;
  save_path: string | null;
  auto_match: string[];  // File extensions
  icon: string | null;
}
```

### create_category

Create a new category.

**Command**: `create_category`

**Parameters**:
```typescript
{
  name: string;
  color?: string;
  save_path?: string;
  auto_match?: string[];
  icon?: string;
}
```

**Returns**: `Promise<string>` - Category ID

### update_category

Update an existing category.

**Command**: `update_category`

**Parameters**:
```typescript
{
  id: string;
  name?: string;
  color?: string;
  save_path?: string;
  auto_match?: string[];
  icon?: string;
}
```

**Returns**: `Promise<void>`

### delete_category

Delete a category.

**Command**: `delete_category`

**Parameters**:
```typescript
{
  id: string;
}
```

**Returns**: `Promise<void>`

### assign_download_category

Assign a download to a category.

**Command**: `assign_download_category`

**Parameters**:
```typescript
{
  download_id: string;
  category_id: string;
}
```

**Returns**: `Promise<void>`

## YouTube Commands

### check_ytdlp_installed

Check if yt-dlp is installed and get version.

**Command**: `check_ytdlp_installed`

**Parameters**: None

**Returns**: `Promise<YtDlpStatus>`

```typescript
interface YtDlpStatus {
  installed: boolean;
  version: string | null;
  bundled: boolean;
}
```

### get_video_info

Get YouTube video information.

**Command**: `get_video_info`

**Parameters**:
```typescript
{
  url: string;
}
```

**Returns**: `Promise<VideoInfo>`

```typescript
interface VideoInfo {
  title: string;
  duration: number;  // seconds
  uploader: string;
  thumbnail: string;
  formats: VideoFormat[];
}

interface VideoFormat {
  format_id: string;
  ext: string;
  resolution: string;
  filesize: number | null;
  vcodec: string;
  acodec: string;
}
```

### get_video_qualities

Get available quality options for a video.

**Command**: `get_video_qualities`

**Parameters**:
```typescript
{
  url: string;
}
```

**Returns**: `Promise<string[]>` - Array of quality strings (e.g., ["1080p", "720p", "480p"])

### check_is_playlist

Check if URL is a playlist.

**Command**: `check_is_playlist`

**Parameters**:
```typescript
{
  url: string;
}
```

**Returns**: `Promise<boolean>`

### update_ytdlp

Update yt-dlp to the latest version.

**Command**: `update_ytdlp`

**Parameters**: None

**Returns**: `Promise<string>` - New version number

### get_ytdlp_version

Get current yt-dlp version.

**Command**: `get_ytdlp_version`

**Parameters**: None

**Returns**: `Promise<string>`

## Browser Integration Commands

### install_browser_extension_support

Install native messaging manifests for browser extensions.

**Command**: `install_browser_extension_support`

**Parameters**: None

**Returns**: `Promise<string>` - Installation status message

### uninstall_browser_extension_support

Remove native messaging manifests.

**Command**: `uninstall_browser_extension_support`

**Parameters**: None

**Returns**: `Promise<string>` - Uninstallation status message

### is_browser_extension_available

Check if browser extension support is installed.

**Command**: `is_browser_extension_available`

**Parameters**: None

**Returns**: `Promise<boolean>`

### add_download_from_browser

Add download from browser extension.

**Command**: `add_download_from_browser`

**Parameters**:
```typescript
{
  url: string;
  referrer?: string;
  filename?: string;
}
```

**Returns**: `Promise<string>` - Download ID

## System Commands

### get_system_info

Get system information.

**Command**: `get_system_info`

**Parameters**: None

**Returns**: `Promise<SystemInfo>`

```typescript
interface SystemInfo {
  os: string;
  os_version: string;
  arch: string;
  app_version: string;
  disk_space: {
    total: number;
    free: number;
    used: number;
  };
}
```

### check_file_exists

Check if a file exists at the given path.

**Command**: `check_file_exists`

**Parameters**:
```typescript
{
  path: string;
}
```

**Returns**: `Promise<boolean>`

### get_file_size

Get size of a file in bytes.

**Command**: `get_file_size`

**Parameters**:
```typescript
{
  path: string;
}
```

**Returns**: `Promise<number>`

### get_global_stats

Get global download statistics.

**Command**: `get_global_stats`

**Parameters**: None

**Returns**: `Promise<GlobalStats>`

```typescript
interface GlobalStats {
  total_downloads: number;
  total_bytes_downloaded: number;
  active_downloads: number;
  current_speed: number;
  average_speed: number;
}
```

## Event System

### Listening to Events

```typescript
import { listen } from '@tauri-apps/api/event';

const unlisten = await listen<EventPayload>('event-name', (event) => {
  console.log('Event received:', event.payload);
});

// Later: unlisten();
```

### Download Events

#### download-added
Emitted when a new download is added.

**Payload**:
```typescript
{
  id: string;
  url: string;
  filename: string;
}
```

#### download-progress
Emitted during download progress (every 500ms).

**Payload**:
```typescript
{
  id: string;
  downloaded_size: number;
  total_size: number;
  speed: number;
  eta: number | null;
  percentage: number;
}
```

#### download-complete
Emitted when a download completes successfully.

**Payload**:
```typescript
{
  id: string;
  filename: string;
  save_path: string;
  file_size: number;
}
```

#### download-failed
Emitted when a download fails.

**Payload**:
```typescript
{
  id: string;
  error: string;
}
```

#### download-paused
Emitted when a download is paused.

**Payload**:
```typescript
{
  id: string;
}
```

#### download-resumed
Emitted when a download is resumed.

**Payload**:
```typescript
{
  id: string;
}
```

### Example: Progress Tracking

```typescript
import { listen } from '@tauri-apps/api/event';

await listen<DownloadProgress>('download-progress', (event) => {
  const progress = event.payload;
  console.log(`Download ${progress.id}: ${progress.percentage}%`);
  console.log(`Speed: ${formatSpeed(progress.speed)}`);
  console.log(`ETA: ${formatTime(progress.eta)}`);
});
```

## Data Types

### Download

```typescript
interface Download {
  id: string;
  url: string;
  final_url: string | null;
  file_name: string;
  save_path: string;
  total_size: number | null;
  downloaded_size: number;
  status: DownloadStatus;
  speed: number;
  eta: number | null;
  segments: number;
  supports_range: boolean;
  content_type: string | null;
  category: string | null;
  priority: number;
  created_at: string;
  completed_at: string | null;
  error_message: string | null;
}

type DownloadStatus = 
  | 'Queued'
  | 'Downloading'
  | 'Paused'
  | 'Completed'
  | 'Failed'
  | 'Cancelled';
```

### Category

```typescript
interface Category {
  id: string;
  name: string;
  color: string;
  save_path: string | null;
  auto_match: string[];
  icon: string | null;
  created_at: string;
}
```

### Settings

```typescript
interface Settings {
  // Download Settings
  default_download_path: string;
  max_concurrent_downloads: number;
  default_segments: number;
  speed_limit: number | null;
  auto_categorize: boolean;
  
  // UI Settings
  theme: 'light' | 'dark' | 'system';
  language: string;
  
  // Behavior Settings
  auto_start: boolean;
  minimize_to_tray: boolean;
  close_to_tray: boolean;
  start_minimized: boolean;
  
  // Notification Settings
  notifications_enabled: boolean;
  notify_on_complete: boolean;
  notify_on_error: boolean;
  
  // Advanced Settings
  proxy_enabled: boolean;
  proxy_type: 'http' | 'socks5' | null;
  proxy_host: string | null;
  proxy_port: number | null;
}
```

## Error Handling

### Error Response Format

All commands that can fail will return errors in this format:

```typescript
{
  error: string;  // Error message
  code?: string;  // Optional error code
}
```

### Common Error Codes

- `NETWORK_ERROR` - Network connectivity issues
- `FILE_NOT_FOUND` - File does not exist
- `PERMISSION_DENIED` - Insufficient permissions
- `INVALID_URL` - Malformed URL
- `DISK_FULL` - Insufficient disk space
- `DATABASE_ERROR` - Database operation failed
- `YTDLP_ERROR` - yt-dlp operation failed

### Example Error Handling

```typescript
try {
  await invoke('add_download', { url: invalidUrl });
} catch (error) {
  if (error === 'INVALID_URL') {
    showError('Please enter a valid URL');
  } else if (error === 'DISK_FULL') {
    showError('Not enough disk space');
  } else {
    showError(`Download failed: ${error}`);
  }
}
```

## Best Practices

### 1. Always Handle Errors

```typescript
try {
  const result = await invoke('command_name', params);
  // Handle success
} catch (error) {
  // Handle error
  console.error('Command failed:', error);
}
```

### 2. Use TypeScript Types

```typescript
import { invoke } from '@tauri-apps/api/tauri';
import type { Download } from '@/types';

const downloads = await invoke<Download[]>('get_all_downloads');
```

### 3. Clean Up Event Listeners

```typescript
const unlisten = await listen('event-name', handler);

// When component unmounts:
unlisten();
```

### 4. Validate Input

```typescript
if (!url || !isValidUrl(url)) {
  throw new Error('Invalid URL');
}

await invoke('add_download', { url });
```

### 5. Use Batch Operations

```typescript
// Instead of multiple calls:
// for (const url of urls) await invoke('add_download', { url });

// Use batch:
await invoke('add_batch_downloads', { urls });
```

## Resources

- [Tauri IPC Documentation](https://tauri.app/v1/guides/features/command)
- [Tauri Events Documentation](https://tauri.app/v1/guides/features/events)
- [TypeScript Documentation](https://www.typescriptlang.org/docs/)

## Need Help?

- 📖 [Architecture Overview](ARCHITECTURE.md)
- 🐛 [Report Issue](https://github.com/yourusername/afk-dunld/issues)
- 💬 [Discussions](https://github.com/yourusername/afk-dunld/discussions)
