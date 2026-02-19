# Architecture Overview

Technical architecture documentation for AFK-Dunld download manager.

## Table of Contents
- [High-Level Architecture](#high-level-architecture)
- [Technology Stack](#technology-stack)
- [Project Structure](#project-structure)
- [Core Components](#core-components)
- [Data Flow](#data-flow)
- [Backend Architecture](#backend-architecture)
- [Frontend Architecture](#frontend-architecture)
- [Database Schema](#database-schema)
- [Communication Layer](#communication-layer)
- [Security Architecture](#security-architecture)

## High-Level Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                        Frontend (React)                      │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐   │
│  │   UI     │  │  Stores  │  │  Hooks   │  │ Services │   │
│  │Components│  │  (Zustand)│  │          │  │          │   │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬─────┘   │
│       └─────────────┴─────────────┴─────────────┘          │
└───────────────────────┬─────────────────────────────────────┘
                        │ Tauri IPC
┌───────────────────────┴─────────────────────────────────────┐
│                    Backend (Rust/Tauri)                      │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐   │
│  │ Commands │  │   Core   │  │ Network  │  │ Services │   │
│  │          │  │  Engine  │  │  Layer   │  │          │   │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬─────┘   │
│       └─────────────┴─────────────┴─────────────┘          │
│                          ↕                                   │
│                   ┌──────────────┐                          │
│                   │   Database   │                          │
│                   │   (SQLite)   │                          │
│                   └──────────────┘                          │
└─────────────────────────────────────────────────────────────┘
```

## Technology Stack

### Frontend
- **Framework**: React 18
- **Language**: TypeScript 5.6
- **State Management**: Zustand
- **Styling**: Tailwind CSS 3.4
- **Build Tool**: Vite
- **UI Libraries**: Custom components with Headless UI

### Backend
- **Framework**: Tauri 2.x
- **Language**: Rust (latest stable)
- **Async Runtime**: Tokio
- **HTTP Client**: Reqwest
- **Database**: SQLite (rusqlite)
- **Serialization**: Serde

### External Tools
- **yt-dlp**: Video downloading (bundled)
- **FFmpeg**: Media processing (system install)
- **Torrent**: Custom BitTorrent implementation

## Project Structure

```
afk-dunld/
├── src/                      # Frontend source
│   ├── components/           # React components
│   │   ├── common/          # Reusable UI components
│   │   ├── downloads/       # Download-related components
│   │   ├── settings/        # Settings UI
│   │   └── ...
│   ├── stores/              # Zustand stores
│   ├── hooks/               # Custom React hooks
│   ├── services/            # Frontend API services
│   ├── types/               # TypeScript type definitions
│   └── styles/              # Global styles
│
├── src-tauri/               # Backend source
│   ├── src/
│   │   ├── commands/        # Tauri command handlers
│   │   ├── core/            # Core download engine
│   │   ├── network/         # Network protocols
│   │   ├── database/        # Database layer
│   │   ├── services/        # Background services
│   │   ├── state/           # Application state
│   │   └── utils/           # Utilities
│   ├── Cargo.toml           # Rust dependencies
│   └── tauri.conf.json      # Tauri configuration
│
├── browser-extension/       # Browser extensions
│   ├── chrome/              # Chrome extension
│   └── firefox/             # Firefox extension
│
└── docs/                    # Documentation
```

## Core Components

### 1. Download Engine (`src-tauri/src/core/`)

**Purpose**: Core download logic and management

**Key Files**:
- `download_engine.rs` - Main download orchestration
- `download_task.rs` - Download task representation
- `segment_downloader.rs` - Multi-threaded segment downloads
- `chunk_manager.rs` - Chunk management and merging
- `resume_manager.rs` - Resume/pause functionality
- `retry.rs` - Error handling and retry logic

**Responsibilities**:
- Create and manage download tasks
- Split files into segments
- Download segments in parallel
- Merge completed segments
- Handle pause/resume
- Retry on failures

### 2. Network Layer (`src-tauri/src/network/`)

**Purpose**: Protocol implementations and network communication

**Key Files**:
- `http_client.rs` - HTTP/HTTPS downloads
- `ftp_client.rs` - FTP/FTPS protocol
- `sftp_client.rs` - SFTP protocol
- `torrent_client.rs` - BitTorrent protocol
- `youtube_downloader.rs` - yt-dlp integration
- `proxy_manager.rs` - Proxy configuration
- `url_parser.rs` - URL parsing and validation

**Responsibilities**:
- Handle different protocols
- Manage connections
- Implement proxy support
- Handle authentication
- Parse and validate URLs

### 3. Database Layer (`src-tauri/src/database/`)

**Purpose**: Data persistence and queries

**Key Files**:
- `db.rs` - Database connection and initialization
- `models.rs` - Data models (DownloadTask, Category, etc.)
- `queries.rs` - SQL queries
- `migrations/` - Database migrations

**Responsibilities**:
- Store download metadata
- Track download history
- Manage categories
- Store settings
- Handle migrations

### 4. Command Layer (`src-tauri/src/commands/`)

**Purpose**: Tauri command handlers (IPC endpoints)

**Key Files**:
- `download_commands.rs` - Download operations
- `queue_commands.rs` - Queue management
- `settings_commands.rs` - Settings CRUD
- `category_commands.rs` - Category management
- `browser_commands.rs` - Browser extension integration
- `ytdlp_commands.rs` - yt-dlp management

**Responsibilities**:
- Expose Rust functions to frontend
- Handle IPC requests
- Validate input
- Return formatted responses

### 5. Services (`src-tauri/src/services/`)

**Purpose**: Background services and utilities

**Key Files**:
- `clipboard_service.rs` - Clipboard monitoring
- `tray_service.rs` - System tray integration
- `notification_service.rs` - Desktop notifications
- `native_messaging.rs` - Browser extension communication
- `file_watcher.rs` - File system monitoring

**Responsibilities**:
- Run background tasks
- Monitor system events
- Integrate with OS features
- Handle browser communication

## Data Flow

### Download Creation Flow

```
┌──────────┐      1. User Input      ┌──────────┐
│ Frontend ├────────────────────────>│ UI Store │
│   UI     │                          └────┬─────┘
└──────────┘                               │
                                           │ 2. Create Download
                                           ↓
┌──────────┐    3. IPC Call (invoke)  ┌────────────┐
│  Tauri   │<───────────────────────  │  Download  │
│   API    │                          │  Service   │
└────┬─────┘                          └────────────┘
     │
     │ 4. Command Handler
     ↓
┌─────────────────┐
│ add_download()  │
│   command       │
└────┬────────────┘
     │
     │ 5. Create Task
     ↓
┌─────────────────┐
│ DownloadEngine  │
│ .create_task()  │
└────┬────────────┘
     │
     │ 6. Save to DB
     ↓
┌─────────────────┐
│   Database      │
│ .insert_download│
└────┬────────────┘
     │
     │ 7. Start Download
     ↓
┌─────────────────┐
│ SegmentDownloader│
│  .download()    │
└────┬────────────┘
     │
     │ 8. Progress Events
     ↓
┌──────────┐     9. Event Emission    ┌──────────┐
│ Frontend │<──────────────────────── │  Events  │
│  Update  │                          │  System  │
└──────────┘                          └──────────┘
```

### Download Progress Flow

```
┌──────────────────┐
│ SegmentDownloader│ Downloading segments in parallel
└────┬─────────────┘
     │ Progress Updates (every 500ms)
     ↓
┌─────────────────┐
│  Speed Tracker  │ Calculate speed, ETA
└────┬────────────┘
     │
     ↓
┌─────────────────┐
│  Update Task    │ Update in-memory state
│  in Engine      │
└────┬────────────┘
     │
     ├─> Database Update (every 5s)
     │
     └─> Event Emission
         ↓
    ┌────────────┐
    │  Frontend  │ Update UI
    │   Update   │
    └────────────┘
```

## Backend Architecture

### State Management

**AppState** (`src-tauri/src/state/app_state.rs`):
```rust
pub struct AppState {
    pub db: Database,
    pub engine: DownloadEngine,
    pub queue_manager: QueueManager,
    pub scheduler: Scheduler,
    pub ytdlp_manager: YtDlpManager,
    // ... other managers
}
```

**Managed as Tauri State**:
- Single instance shared across app
- Thread-safe (Arc<Mutex<T>>)
- Accessible in all commands

### Download Engine Details

**Task Lifecycle**:
```
Created → Queued → Downloading → Paused/Completed/Failed
                      ↓
                  [Segments]
                   ↓ ↓ ↓
                  Merging
                     ↓
                 Completed
```

**Segment Download**:
1. **Check Range Support**: HEAD request
2. **Calculate Segments**: Based on file size and settings
3. **Create Segment Tasks**: Each segment is independent
4. **Download in Parallel**: Tokio async tasks
5. **Save to Temp Files**: `.part.N` files
6. **Merge on Completion**: Combine into final file
7. **Cleanup**: Remove temp files

### Error Handling Strategy

**Retry Logic**:
```rust
pub struct RetryConfig {
    max_retries: u32,      // Default: 5
    base_delay: Duration,  // Default: 10s
    max_delay: Duration,   // Default: 5min
    exponential: bool,     // Default: true
}
```

**Error Recovery**:
1. Catch error
2. Check retry count
3. Calculate delay (exponential backoff)
4. Wait
5. Retry download
6. If max retries exceeded → Fail

## Frontend Architecture

### Component Hierarchy

```
App
├── ErrorBoundary
│   └── MainContent
│       ├── Header
│       ├── Sidebar
│       ├── TabNavigation
│       │   ├── DownloadList (Active Downloads)
│       │   ├── QueueManager
│       │   ├── ScheduleManager
│       │   ├── DownloadHistory
│       │   ├── CategoryManager
│       │   └── SettingsPage
│       └── StatusBar
```

### State Management (Zustand)

**Download Store** (`src/stores/downloadStore.ts`):
```typescript
interface DownloadStore {
  downloads: Download[];
  addDownload: (download: Download) => void;
  updateDownload: (id: string, updates: Partial<Download>) => void;
  removeDownload: (id: string) => void;
  // ... more actions
}
```

**Settings Store** (`src/stores/settingsStore.ts`):
```typescript
interface SettingsStore {
  settings: Settings;
  updateSettings: (settings: Partial<Settings>) => void;
  loadSettings: () => Promise<void>;
  saveSettings: () => Promise<void>;
}
```

**UI Store** (`src/stores/uiStore.ts`):
```typescript
interface UIStore {
  activeTab: string;
  theme: 'light' | 'dark';
  setActiveTab: (tab: string) => void;
  toggleTheme: () => void;
}
```

### Custom Hooks

**useDownloads** - Download management
```typescript
const { downloads, addDownload, pauseDownload } = useDownloads();
```

**useTauriEvents** - Listen to backend events
```typescript
useTauriEvents('download-progress', (data) => {
  updateDownload(data.id, data);
});
```

**useKeyboardShortcuts** - Keyboard shortcuts
```typescript
useKeyboardShortcuts({
  'Ctrl+N': () => openAddDownloadDialog(),
  'Ctrl+P': () => pauseAll(),
});
```

## Database Schema

### Tables

**downloads**
```sql
CREATE TABLE downloads (
  id TEXT PRIMARY KEY,
  url TEXT NOT NULL,
  file_name TEXT NOT NULL,
  save_path TEXT NOT NULL,
  total_size INTEGER,
  downloaded_size INTEGER,
  status TEXT NOT NULL,
  speed REAL,
  segments INTEGER,
  category TEXT,
  created_at TIMESTAMP,
  completed_at TIMESTAMP
);
```

**categories**
```sql
CREATE TABLE categories (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL UNIQUE,
  color TEXT,
  save_path TEXT,
  auto_match TEXT,  -- JSON array of extensions
  created_at TIMESTAMP
);
```

**schedules**
```sql
CREATE TABLE schedules (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  url TEXT NOT NULL,
  schedule_type TEXT,  -- 'once' or 'recurring'
  start_time TIMESTAMP,
  recurrence TEXT,     -- JSON config
  enabled BOOLEAN,
  created_at TIMESTAMP
);
```

**settings**
```sql
CREATE TABLE settings (
  key TEXT PRIMARY KEY,
  value TEXT NOT NULL,  -- JSON serialized
  updated_at TIMESTAMP
);
```

## Communication Layer

### Tauri IPC

**Command Definition** (Rust):
```rust
#[tauri::command]
async fn add_download(
    url: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    // Implementation
}
```

**Frontend Call** (TypeScript):
```typescript
import { invoke } from '@tauri-apps/api/tauri';

const downloadId = await invoke<string>('add_download', {
  url: 'https://example.com/file.zip'
});
```

### Event System

**Emit Event** (Rust):
```rust
app_handle.emit("download-progress", &progress_data)?;
```

**Listen to Event** (TypeScript):
```typescript
import { listen } from '@tauri-apps/api/event';

await listen('download-progress', (event) => {
  console.log('Progress:', event.payload);
});
```

### Browser Extension Communication

**Native Messaging Protocol**:
1. Extension sends message via `chrome.runtime.sendNativeMessage()`
2. Message received on stdin (length-prefixed JSON)
3. App processes message
4. Response sent to stdout (length-prefixed JSON)
5. Extension receives response

## Security Architecture

### Input Validation
- All user inputs validated
- URL parsing and sanitization
- Path traversal prevention
- SQL injection prevention (parameterized queries)

### Credential Storage
- Encrypted using platform keyring
- FTP/SFTP passwords encrypted
- Proxy credentials encrypted
- Browser cookies encrypted

### File System Security
- Sandboxed file access
- Permission checks before write
- Temp file cleanup
- Safe file naming (no path traversal)

### Network Security
- HTTPS by default
- SSL certificate verification
- Proxy support (HTTP/SOCKS5)
- Configurable timeout and retry

## Performance Optimizations

### Backend
- **Async I/O**: Tokio async runtime
- **Connection Pooling**: Reuse HTTP connections
- **Buffered I/O**: Efficient file writes
- **Database Connection Pool**: Shared connections
- **Lazy Loading**: Load data on demand

### Frontend
- **Code Splitting**: Route-based splitting
- **Virtual Lists**: For large download lists
- **Debounced Updates**: Reduce re-renders
- **Memoization**: Cache expensive computations
- **Lazy Components**: Load on demand

### Download Engine
- **Multi-threading**: Parallel segment downloads
- **Adaptive Segments**: Adjust based on speed
- **Smart Merging**: Stream-based merge
- **Resume Support**: Continue from last position
- **Compression**: Support gzip/deflate

## Build & Deployment

### Development Build
```bash
npm run tauri dev
```

### Production Build
```bash
npm run tauri build
```

### Bundling yt-dlp
- Platform-specific binaries bundled
- Downloaded during build from official releases
- Version tracking in `ytdlp-version.txt`
- Auto-update mechanism in app

## Next Steps

- [API Documentation](API.md)
- [Build Guide](BUILD_GUIDE.md)
- [Contributing Guide](../CONTRIBUTING.md)

## Resources

- [Tauri Documentation](https://tauri.app)
- [Rust Async Book](https://rust-lang.github.io/async-book/)
- [React Documentation](https://react.dev)
