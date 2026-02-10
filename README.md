AFK-Dunld
this is AFK download which will support may be all platforms. It helps download every file.



# Tauri + Vue + TypeScript

This template should help get you started developing with Vue 3 and TypeScript in Vite. The template uses Vue 3 `<script setup>` SFCs, check out the [script setup docs](https://v3.vuejs.org/api/sfc-script-setup.html#sfc-script-setup) to learn more.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Vue - Official](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)


here is complete stack:
📦 RUST + TAURI TECH STACK
│
├── 🦀 Backend: Rust
│   ├── Tauri 2.0 (app framework)
│   ├── Tokio (async runtime)
│   ├── reqwest (HTTP/HTTPS client)
│   ├── rust-ftp (FTP support)
│   ├── librqbit (Torrent support)
│   ├── SQLx (async SQLite database)
│   ├── serde (serialization)
│   ├── tokio-util (rate limiting)
│   ├── notify-rust (notifications)
│   ├── arboard (clipboard monitoring)
│   ├── sha2 / md5 (checksum verification)
│   └── tracing (logging)
│
├── 🎨 Frontend: React + TypeScript
│   ├── Vite (build tool)
│   ├── TailwindCSS (styling)
│   ├── Shadcn/UI (component library)
│   ├── Zustand (state management)
│   ├── React Query (data fetching)
│   ├── Framer Motion (animations)
│   └── Lucide React (icons)
│
├── 💾 Database: SQLite (via SQLx)
│
├── 🌍 Browser Extension:
│   ├── Chrome Extension (Manifest V3)
│   ├── Firefox Add-on
│   └── Native Messaging Host (Rust)
│
└── 📦 Packaging:
    ├── .msi / .exe (Windows)
    ├── .dmg / .app (macOS)
    └── .deb / .AppImage / .rpm (Linux)


📁 Complete Project Structure
super-downloader/
│
├── src-tauri/                          # Rust Backend
│   ├── Cargo.toml                      # Rust dependencies
│   ├── tauri.conf.json                 # Tauri configuration
│   ├── capabilities/                   # Tauri 2.0 permissions
│   │   └── default.json
│   ├── icons/                          # App icons
│   │
│   └── src/
│       ├── main.rs                     # Entry point
│       ├── lib.rs                      # Library root
│       │
│       ├── core/                       # Core download engine
│       │   ├── mod.rs
│       │   ├── download_engine.rs      # Main download orchestrator
│       │   ├── chunk_manager.rs        # Multi-segment splitting
│       │   ├── download_task.rs        # Single download task
│       │   ├── resume_manager.rs       # Pause/Resume logic
│       │   ├── speed_limiter.rs        # Bandwidth control
│       │   ├── scheduler.rs            # Download scheduling
│       │   ├── queue_manager.rs        # Download queue
│       │   └── checksum.rs             # File integrity (MD5/SHA256)
│       │
│       ├── network/                    # Network layer
│       │   ├── mod.rs
│       │   ├── http_client.rs          # HTTP/HTTPS handler
│       │   ├── ftp_client.rs           # FTP handler
│       │   ├── torrent_client.rs       # Torrent handler
│       │   ├── proxy_manager.rs        # Proxy support
│       │   ├── url_parser.rs           # URL validation & info
│       │   └── connection.rs           # Connection management
│       │
│       ├── database/                   # Data persistence
│       │   ├── mod.rs
│       │   ├── db.rs                   # Database connection
│       │   ├── models.rs               # Data models
│       │   ├── queries.rs              # SQL queries
│       │   └── migrations/             # DB migrations
│       │       └── 001_initial.sql
│       │
│       ├── commands/                   # Tauri IPC commands
│       │   ├── mod.rs
│       │   ├── download_commands.rs    # Download operations
│       │   ├── settings_commands.rs    # Settings operations
│       │   ├── queue_commands.rs       # Queue operations
│       │   └── system_commands.rs      # System operations
│       │
│       ├── services/                   # Business logic
│       │   ├── mod.rs
│       │   ├── clipboard_service.rs    # Clipboard monitoring
│       │   ├── notification_service.rs # System notifications
│       │   ├── tray_service.rs         # System tray
│       │   ├── config_service.rs       # App configuration
│       │   └── browser_service.rs      # Browser integration
│       │
│       ├── state/                      # App state management
│       │   ├── mod.rs
│       │   └── app_state.rs            # Global app state
│       │
│       └── utils/                      # Utilities
│           ├── mod.rs
│           ├── file_utils.rs           # File operations
│           ├── format_utils.rs         # Size/speed formatting
│           ├── error.rs                # Error types
│           └── constants.rs            # Constants
│
├── src/                                # Frontend (React + TS)
│   ├── index.html
│   ├── main.tsx                        # React entry point
│   ├── App.tsx                         # Root component
│   ├── vite-env.d.ts
│   │
│   ├── components/                     # UI Components
│   │   ├── layout/
│   │   │   ├── Sidebar.tsx
│   │   │   ├── Header.tsx
│   │   │   ├── MainContent.tsx
│   │   │   └── StatusBar.tsx
│   │   │
│   │   ├── downloads/
│   │   │   ├── DownloadList.tsx        # List of downloads
│   │   │   ├── DownloadItem.tsx        # Single download row
│   │   │   ├── DownloadProgress.tsx    # Progress bar
│   │   │   ├── AddDownloadDialog.tsx   # New download modal
│   │   │   ├── BatchDownloadDialog.tsx # Batch download modal
│   │   │   └── DownloadDetails.tsx     # Download info panel
│   │   │
│   │   ├── settings/
│   │   │   ├── SettingsPage.tsx        # Settings page
│   │   │   ├── GeneralSettings.tsx     # General settings
│   │   │   ├── NetworkSettings.tsx     # Network/proxy settings
│   │   │   ├── DownloadSettings.tsx    # Download preferences
│   │   │   └── ThemeSettings.tsx       # Theme selector
│   │   │
│   │   └── common/
│   │       ├── Button.tsx
│   │       ├── Modal.tsx
│   │       ├── ProgressBar.tsx
│   │       ├── SpeedGraph.tsx
│   │       ├── ContextMenu.tsx
│   │       └── Tooltip.tsx
│   │
│   ├── hooks/                          # Custom React hooks
│   │   ├── useDownloads.ts
│   │   ├── useSettings.ts
│   │   ├── useClipboard.ts
│   │   ├── useTauriEvents.ts
│   │   └── useTheme.ts
│   │
│   ├── stores/                         # Zustand stores
│   │   ├── downloadStore.ts
│   │   ├── settingsStore.ts
│   │   ├── queueStore.ts
│   │   └── uiStore.ts
│   │
│   ├── services/                       # Frontend services
│   │   ├── tauriApi.ts                 # Tauri IPC calls
│   │   ├── downloadService.ts
│   │   └── settingsService.ts
│   │
│   ├── types/                          # TypeScript types
│   │   ├── download.ts
│   │   ├── settings.ts
│   │   └── common.ts
│   │
│   ├── styles/                         # Styles
│   │   ├── globals.css
│   │   ├── themes/
│   │   │   ├── dark.css
│   │   │   └── light.css
│   │   └── animations.css
│   │
│   └── assets/                         # Static assets
│       ├── icons/
│       └── images/
│
├── browser-extension/                  # Browser Extension
│   ├── chrome/
│   │   ├── manifest.json
│   │   ├── background.js
│   │   ├── content.js
│   │   ├── popup.html
│   │   ├── popup.js
│   │   └── icons/
│   │
│   └── firefox/
│       ├── manifest.json
│       ├── background.js
│       ├── content.js
│       ├── popup.html
│       ├── popup.js
│       └── icons/
│
├── migrations/                         # SQLite migrations
│   └── 001_create_tables.sql
│
├── package.json                        # Node dependencies
├── tsconfig.json                       # TypeScript config
├── tailwind.config.js                  # Tailwind config
├── vite.config.ts                      # Vite config
├── postcss.config.js
├── README.md
└── LICENSE

