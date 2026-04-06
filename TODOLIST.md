# 🚀 AFK-Dunld Improvement & Feature Roadmap

**Version:** 1.1  
**Last Updated:** 2026-04-06  
**Total Items:** 70

This document outlines all planned improvements, bug fixes, and new features for AFK-Dunld download manager.

---

## 📋 Table of Contents

- [🔴 Critical Bug Fixes (Codebase Audit 2026-04-06)](#-critical-bug-fixes-codebase-audit-2026-04-06)
- [🟠 High-Priority Fixes (Codebase Audit 2026-04-06)](#-high-priority-fixes-codebase-audit-2026-04-06)
- [🟡 Medium-Priority Fixes (Codebase Audit 2026-04-06)](#-medium-priority-fixes-codebase-audit-2026-04-06)
- [Critical Bug Fixes & TODOs](#-critical-bug-fixes--todos)
- [Core Feature Enhancements](#-core-feature-enhancements)
- [Advanced Features](#-advanced-features)
- [Professional Features](#-professional-features)
- [Implementation Priority](#-implementation-priority)
- [Quick Wins](#-quick-wins)
- [High-Impact Features](#-high-impact-features)

---

## 🔴 Critical Bug Fixes (Codebase Audit 2026-04-06)

These are blocking issues discovered during the comprehensive codebase audit. They prevent core functionality from working correctly.

### 51. Implement Empty Stub Components (9 Files)
**Priority:** Critical  
**Complexity:** Low-Medium  
**Status:** Pending  
**Category:** Frontend Bug

Nine component files exist but are completely empty (0 lines), breaking UI features:

- `src/components/common/ProgressBar.tsx` — Core progress display missing
- `src/components/common/SpeedGraph.tsx` — Analytics/speed graph broken
- `src/components/downloads/BatchDownloadDialog.tsx` — Batch downloads non-functional
- `src/components/downloads/DownloadProgress.tsx` — Progress display broken
- `src/components/downloads/DownloadDetails.tsx` — Details panel missing
- `src/components/settings/NetworkSettings.tsx` — Network settings page empty
- `src/hooks/useClipboard.ts` — Clipboard hook missing
- `src/hooks/useSettings.ts` — Settings hook missing
- `src/types/common.ts` — Shared types missing

**Files Affected:**
- `src/components/common/ProgressBar.tsx`
- `src/components/common/SpeedGraph.tsx`
- `src/components/downloads/BatchDownloadDialog.tsx`
- `src/components/downloads/DownloadProgress.tsx`
- `src/components/downloads/DownloadDetails.tsx`
- `src/components/settings/NetworkSettings.tsx`
- `src/hooks/useClipboard.ts`
- `src/hooks/useSettings.ts`
- `src/types/common.ts`

---

### 52. Resume/Retry Functions Are No-Ops
**Priority:** Critical  
**Complexity:** Medium  
**Status:** Pending  
**Category:** Backend Bug

`download_commands.rs:850-868` — `resume_download_internal` and `retry_download_internal` only log messages and return without performing any action. When the scheduler triggers or users click resume/retry, nothing happens.

**Required:**
- Wire up `resume_download_internal` to actual download engine resume logic
- Wire up `retry_download_internal` to actual download engine retry logic
- Pass `app_handle` or `AppState` reference to these functions
- Test end-to-end resume and retry flows

**Files Affected:**
- `src-tauri/src/commands/download_commands.rs` (lines 850-868)
- `src-tauri/src/main.rs` (scheduler calls these functions)
- `src-tauri/src/lib.rs` (scheduler calls these functions)

---

### 53. Torrent P2P Downloads Disabled (librqbit)
**Priority:** Critical  
**Complexity:** High  
**Status:** Pending  
**Category:** Backend Bug

`librqbit` dependency is commented out in `Cargo.toml`. All torrent downloads return `"librqbit is currently disabled"`. The stub implementation compiles but cannot download any torrents.

**Required:**
- Update librqbit from v5.1 to v8.1.1 (latest)
- Migrate API calls to match v8.x interface
- Re-enable the dependency in `Cargo.toml`
- Remove stub implementations in `torrent_client_librqbit.rs`
- Test with real .torrent files and magnet links

**Files Affected:**
- `src-tauri/Cargo.toml` (line 66: commented out)
- `src-tauri/src/network/torrent_client_librqbit.rs` (stub types)
- `src-tauri/src/network/torrent_client.rs`

---

### 54. Duplicate Type Definitions in types/download.ts
**Priority:** Critical  
**Complexity:** Low  
**Status:** Pending  
**Category:** Frontend Bug

`src/types/download.ts` defines `QueueInfo` twice (lines 73-77 and 85-89) with different shapes. `DownloadStats` is also defined twice (lines 62-71 and 91-97). This causes TypeScript compilation errors or unpredictable type resolution.

**Required:**
- Remove duplicate `QueueInfo` definition (keep the correct one)
- Remove duplicate `DownloadStats` definition (keep the correct one)
- Verify all imports reference the correct types

**Files Affected:**
- `src/types/download.ts`

---

### 55. get_download_progress Always Returns None
**Priority:** Critical  
**Complexity:** Medium  
**Status:** Pending  
**Category:** Backend Bug

`download_commands.rs:333-339` — The `get_download_progress` command is a stub that always returns `Ok(None)`. The frontend cannot display real-time progress for individual downloads.

**Required:**
- Implement actual progress retrieval from `DownloadEngine` or `active_downloads` map
- Return progress data including percentage, speed, ETA, downloaded bytes
- Connect to existing progress event system

**Files Affected:**
- `src-tauri/src/commands/download_commands.rs` (lines 333-339)

---

## 🟠 High-Priority Fixes (Codebase Audit 2026-04-06)

### 56. Remove Unused/Dead Code Components
**Priority:** High  
**Complexity:** Low  
**Status:** Pending  
**Category:** Frontend Cleanup

Nine components/hooks exist but are never imported or used in the app:

- `src/components/layout/Sidebar.tsx` — Has hardcoded fake data (Videos: 8, Audio: 5)
- `src/components/layout/Toolbar.tsx` — Alternative toolbar never rendered
- `src/components/layout/MainContent.tsx` — Route wrapper never used
- `src/hooks/useDownloads.ts` — Orphaned hook duplicating store functionality
- `src/assets/vue.svg` — Leftover Vue asset

**Required:**
- Delete unused files OR implement and wire them into the app
- If keeping Sidebar/Toolbar, connect to real data sources
- Remove `vue.svg` asset

**Files Affected:**
- `src/components/layout/Sidebar.tsx`
- `src/components/layout/Toolbar.tsx`
- `src/components/layout/MainContent.tsx`
- `src/hooks/useDownloads.ts`
- `src/assets/vue.svg`

---

### 57. Category Routes Lead to 404
**Priority:** High  
**Complexity:** Low  
**Status:** Pending  
**Category:** Frontend Bug

`TabNavigation.tsx` renders dynamic category tabs that navigate to `/category/${name.toLowerCase()}`, but `App.tsx` has no corresponding `<Route>` definitions. Clicking a custom category tab results in a blank/404 page.

**Required:**
- Add `<Route path="/category/:name" element={...} />` to `App.tsx`
- Create a `CategoryView` component that filters downloads by category
- OR remove the category tab links if not intended to be navigable

**Files Affected:**
- `src/App.tsx`
- `src/components/layout/TabNavigation.tsx`

---

### 58. Duplicate Resume/Retry Buttons in DownloadTableRow
**Priority:** High  
**Complexity:** Low  
**Status:** Pending  
**Category:** Frontend Bug

`DownloadTableRow.tsx:472-526` — Identical Resume and Retry buttons are rendered twice due to a copy-paste error. One set is visible, another set is hidden until hover.

**Required:**
- Remove the duplicate button block
- Keep only one set of Resume/Retry buttons per row

**Files Affected:**
- `src/components/downloads/DownloadTableRow.tsx` (lines 472-526)

---

### 59. Keyboard Shortcut Mismatch
**Priority:** High  
**Complexity:** Low  
**Status:** Pending  
**Category:** Frontend Bug

`MenuBar.tsx` displays `Ctrl+,` as the Settings shortcut but `useKeyboardShortcuts.ts` registers `Ctrl+S` for settings navigation. Users see the wrong shortcut hint.

**Required:**
- Align shortcut display in `MenuBar.tsx` with actual registration in `useKeyboardShortcuts.ts`
- Standardize on one shortcut (recommend `Ctrl+,` as it's the conventional settings shortcut)

**Files Affected:**
- `src/components/common/MenuBar.tsx`
- `src/hooks/useKeyboardShortcuts.ts`

---

### 60. Speed Limit Unit Inconsistency
**Priority:** High  
**Complexity:** Low  
**Status:** Pending  
**Category:** Frontend Bug

`DownloadSettings.tsx` uses a slider from 0 to 104857600 (bytes) labeled as MB/s. `SettingsPage.tsx` uses a different scale (0-10240, presumably KB/s). These two settings pages conflict and save incompatible values.

**Required:**
- Standardize on a single unit (recommend KB/s or MB/s)
- Ensure both settings pages read/write the same setting key
- Add unit conversion helpers if needed

**Files Affected:**
- `src/components/settings/DownloadSettings.tsx`
- `src/components/settings/SettingsPage.tsx`
- `src/types/settings.ts`

---

### 61. errorRecovery.ts Uses Wrong Parameter Names
**Priority:** High  
**Complexity:** Low  
**Status:** Pending  
**Category:** Frontend Bug

`src/utils/errorRecovery.ts` calls `invoke('resume_download', { downloadId })` but the backend expects `{ id }` as the parameter name (matching the pattern used everywhere else in `tauriApi.ts`). Same issue for `pause_download` and `retry_download`.

**Required:**
- Change `downloadId` to `id` in all `invoke` calls in `errorRecovery.ts`
- Verify parameter names match backend command signatures

**Files Affected:**
- `src/utils/errorRecovery.ts`

---

### 62. QueueManager Status Filter Case Mismatch
**Priority:** High  
**Complexity:** Low  
**Status:** Pending  
**Category:** Frontend Bug

`QueueManager.tsx` filters for `'Queued'` and `'Downloading'` (capitalized) but the `DownloadStatus` type uses lowercase `'queued'` and `'downloading'`. Filters will never match any downloads.

**Required:**
- Change filter strings to lowercase: `'queued'`, `'downloading'`
- Verify all status string comparisons use consistent casing

**Files Affected:**
- `src/components/queue/QueueManager.tsx`
- `src/types/download.ts`

---

## 🟡 Medium-Priority Fixes (Codebase Audit 2026-04-06)

### 63. Deep Link Handler Not Connected
**Priority:** Medium  
**Complexity:** Medium  
**Status:** Pending  
**Category:** Backend Bug

`handle_deep_link` function exists in `main.rs` (lines 18-80) but is never called. The `tauri-plugin-deep-link` v2 requires event-based registration, not programmatic handler passing.

**Required:**
- Implement Tauri v2 event-based deep link listener
- Connect `handle_deep_link` logic to the event handler
- Test with `afk-dunld://download?url=...` URLs from browser

**Files Affected:**
- `src-tauri/src/main.rs` (lines 18-80, 196-199)
- `src-tauri/tauri.conf.json`

---

### 64. react-query Imported But Unused
**Priority:** Medium  
**Complexity:** Low  
**Status:** Pending  
**Category:** Frontend Cleanup

`App.tsx` sets up `QueryClient` and `QueryClientProvider` but all data flows through Zustand stores. This adds bundle size and confusion with no benefit.

**Required:**
- Remove `react-query` imports and provider if not needed
- OR integrate `react-query` for server state management alongside Zustand for UI state

**Files Affected:**
- `src/App.tsx`
- `package.json` (remove dependency if unused)

---

### 65. TorrentManager State Type Mismatch
**Priority:** Medium  
**Complexity:** Low  
**Status:** Pending  
**Category:** Frontend Bug

`TorrentManager.tsx` defines its own `TorrentState` interface that doesn't match the `TorrentState` type in `types/torrent.ts` (which uses a tagged union pattern like `{ Downloading: null }`).

**Required:**
- Align `TorrentManager.tsx` state handling with `types/torrent.ts` tagged union
- Use proper pattern matching for torrent states

**Files Affected:**
- `src/components/torrent/TorrentManager.tsx`
- `src/types/torrent.ts`

---

### 66. AddDownloadDialog Save Path Is Read-Only
**Priority:** Medium  
**Complexity:** Low  
**Status:** Pending  
**Category:** Frontend UX

`AddDownloadDialog.tsx:236` — The save path input has `readOnly` attribute. Users cannot type a path manually, they must use the Browse button which only works in Tauri context (not in browser dev mode).

**Required:**
- Remove `readOnly` attribute
- Allow manual path entry in addition to Browse button
- Validate path on blur/submit

**Files Affected:**
- `src/components/downloads/AddDownloadDialog.tsx`

---

### 67. Duplicate Formatting Functions Across Components
**Priority:** Medium  
**Complexity:** Low  
**Status:** Pending  
**Category:** Frontend Cleanup

`formatBytes`, `formatSpeed`, `formatDuration` are reimplemented in multiple components (`DownloadHistory`, `QueueManager`, `CategoryManager`, etc.) instead of using the shared `utils/format.ts`.

**Required:**
- Replace all duplicate formatting functions with imports from `utils/format.ts`
- Ensure `utils/format.ts` covers all needed formatting cases

**Files Affected:**
- `src/components/history/DownloadHistory.tsx`
- `src/components/queue/QueueManager.tsx`
- `src/components/categories/CategoryManager.tsx`
- `src/utils/format.ts`

---

### 68. lib.rs Entry Point Is Incomplete
**Priority:** Medium  
**Complexity:** Medium  
**Status:** Pending  
**Category:** Backend Bug

`lib.rs` is missing torrent commands, browser commands, queue commands, security commands, and ytdlp commands. If the app starts via `lib.rs::run()`, most features are unavailable.

**Required:**
- Add missing command registrations to `lib.rs`
- Ensure `lib.rs` and `main.rs` expose the same command surface
- OR document when each entry point should be used

**Files Affected:**
- `src-tauri/src/lib.rs`

---

### 69. Hardcoded Master Password in CredentialVault
**Priority:** Medium  
**Complexity:** Medium  
**Status:** Pending  
**Category:** Security

`app_state.rs:101` — `CredentialVault::new("default_master_password")` uses a hardcoded string as the encryption key. Any stored credentials can be decrypted by anyone with access to the source code.

**Required:**
- Use OS-level keychain (Windows Credential Manager, macOS Keychain, Linux Secret Service)
- OR derive key from user-provided password
- OR use machine-specific identifier as key source

**Files Affected:**
- `src-tauri/src/state/app_state.rs` (line 101)
- `src-tauri/src/utils/security.rs`

---

### 70. No Tests Exist
**Priority:** Medium  
**Complexity:** High  
**Status:** Pending  
**Category:** Quality

Zero unit or integration tests exist despite TODOLIST marking this as high priority. No CI/CD test automation.

**Required:**
- Add unit tests for core download engine
- Add unit tests for chunk manager
- Add integration tests for download flow
- Add tests for error scenarios
- Set up CI/CD test automation
- Target >80% code coverage

**Files to Create:**
- `src-tauri/tests/core/download_engine_test.rs`
- `src-tauri/tests/core/chunk_manager_test.rs`
- `src-tauri/tests/core/scheduler_test.rs`
- `src-tauri/tests/integration/download_flow_test.rs`
- `src-tauri/tests/integration/torrent_flow_test.rs`

---

## 🐛 Critical Bug Fixes & TODOs

These address existing issues found in the codebase that need immediate attention.

### 1. Complete Torrent Support (Re-enable librqbit)
**Priority:** High  
**Complexity:** High  
**Status:** Pending

- Re-enable librqbit dependency (currently commented out in Cargo.toml)
- Update to librqbit v8.1.1 (v5.1 is incompatible with current Rust toolchain)
- Implement proper BitTorrent protocol
- Fix compilation errors that caused librqbit to be disabled
- Complete torrent_client_librqbit.rs implementation
- Remove stub types and implement real session management
- Add torrent file parsing using bencode

**Files Affected:**
- `src-tauri/Cargo.toml`
- `src-tauri/src/network/torrent_client_librqbit.rs`
- `src-tauri/src/network/torrent_client.rs`

---

### 2. Implement Scheduled Download Execution Logic
**Priority:** High  
**Complexity:** Medium  
**Status:** Pending

- Complete the TODO at `main.rs:235` and `lib.rs:90`
- Implement actual download restart logic when scheduler triggers
- Add proper download loading from database
- Call add_download or resume_download for scheduled tasks
- Fix `resume_download_internal` and `retry_download_internal` which are currently no-ops
- Test scheduled downloads end-to-end

**Files Affected:**
- `src-tauri/src/main.rs` (lines 235-240)
- `src-tauri/src/lib.rs` (lines 88-94)
- `src-tauri/src/commands/download_commands.rs` (lines 850-868)

---

### 3. Fix Deep Link Handler for Tauri v2 API
**Priority:** High  
**Complexity:** Medium  
**Status:** Pending

- Update deep link handler to use Tauri v2 plugin API (event-based)
- Replace commented-out code at `main.rs:197`
- Connect the existing `handle_deep_link` function (lines 18-80) to the event system
- Investigate new tauri-plugin-deep-link v2 event system
- Test browser extension protocol handling
- Document the new implementation

**Files Affected:**
- `src-tauri/src/main.rs` (lines 18-80, 196-199)
- `src-tauri/tauri.conf.json`

---

### 4. Replace String Interpolation with Parameterized Queries
**Priority:** High  
**Complexity:** Low  
**Status:** ✅ Fixed

- Security improvement: prevent SQL injection
- Updated database queries at `database/queries.rs`
- All queries now use SQLx parameterized syntax
- Security review completed

---

### 5. Implement History Deletion Functionality
**Priority:** Medium  
**Complexity:** Low  
**Status:** ✅ Fixed

- Added `clear_download_history()`, `delete_download_from_history()`, `delete_downloads_bulk()`, `clear_old_history()`
- Database methods for history deletion implemented
- UI confirmation dialog added
- Bulk delete option available
- Data integrity verified after deletion

---

### 6. Complete FTP Test Suite Implementation
**Priority:** Medium  
**Complexity:** Medium  
**Status:** Pending

- Implement TODOs in `tests/test_phase3.rs`
- Set up test FTP server
- Test FTP download with resume capability
- Test listing files on FTP server
- Add integration tests for FTP client

**Files Affected:**
- `src-tauri/tests/test_phase3.rs`

---

### 7. Add Comprehensive Unit and Integration Tests
**Priority:** High  
**Complexity:** High  
**Status:** Pending

- Create unit tests for core download engine
- Add integration tests for download workflows
- Test error scenarios and edge cases
- Add mock HTTP server for testing
- Achieve >80% code coverage
- Set up CI/CD test automation

**Files to Create:**
- `src-tauri/tests/core/download_engine_test.rs`
- `src-tauri/tests/core/chunk_manager_test.rs`
- `src-tauri/tests/core/scheduler_test.rs`
- `src-tauri/tests/integration/download_flow_test.rs`
- `src-tauri/tests/integration/torrent_flow_test.rs`

---

## 🔧 Core Feature Enhancements

Improvements to existing download functionality.

### 8. Implement Download Mirroring Support
**Priority:** High  
**Complexity:** High  
**Status:** Pending

- Allow multiple source URLs for the same file
- Download from fastest/most reliable source
- Automatic failover between mirrors
- Combine chunks from multiple sources
- UI to manage mirror URLs

**Benefits:**
- Faster download speeds
- Better reliability
- Automatic failover

---

### 9. Add Cloud Storage Integration
**Priority:** High  
**Complexity:** Very High  
**Status:** Pending

**Supported Platforms:**
- Google Drive API integration
- Dropbox API integration
- OneDrive API integration
- Box.com support (optional)

**Features:**
- OAuth authentication
- Direct download from cloud storage
- Upload completed downloads to cloud
- Sync settings across devices via cloud

**New Dependencies:**
- `google-drive3` crate
- `dropbox-sdk` crate
- OAuth2 handling

---

### 10. Implement Auto-Categorization
**Priority:** Medium  
**Complexity:** Medium  
**Status:** Pending

- Detect file type from MIME type and extension
- Auto-assign category based on content
- Machine learning for pattern recognition (optional)
- User-defined categorization rules
- Category suggestions in UI

**Categories:**
- Video (mp4, mkv, avi, etc.)
- Audio (mp3, flac, aac, etc.)
- Documents (pdf, doc, xlsx, etc.)
- Software (exe, dmg, deb, etc.)
- Archives (zip, rar, 7z, etc.)
- Images (jpg, png, svg, etc.)

---

### 11. Add Batch File Renaming Capabilities
**Priority:** Low  
**Complexity:** Medium  
**Status:** Pending

- Pattern-based renaming with variables
- Preview before applying
- Regex support
- Variables: {name}, {ext}, {date}, {size}, {index}
- Undo functionality
- Save rename patterns as templates

**Example Patterns:**
- `{name}_{date}.{ext}`
- `Episode_{index:03d}.{ext}`
- `[{size}]_{name}.{ext}`

---

### 12. Create Download Templates/Presets
**Priority:** Medium  
**Complexity:** Low  
**Status:** Pending

- Save download configuration as templates
- Quick apply presets for common scenarios
- Share templates with other users
- Import/export templates

**Example Templates:**
- "YouTube 1080p MP4"
- "Audio Only MP3 320kbps"
- "4K Video Best Quality"
- "Fast Download (Max Segments)"

---

### 13. Implement Bandwidth Scheduling
**Priority:** Medium  
**Complexity:** Medium  
**Status:** Pending

- Different speed limits at different times
- Day-of-week scheduling
- Multiple time slots per day
- Visual schedule editor
- Apply globally or per-download

**Use Cases:**
- Limit speed during work hours
- Unlimited at night
- Different limits for weekends

---

### 14. Add MEGA.nz Support
**Priority:** Medium  
**Complexity:** High  
**Status:** Pending

- Implement MEGA.nz API integration
- Handle MEGA encryption/decryption
- Support for password-protected links
- Folder download support
- Bypass MEGA bandwidth limits (where possible)

**Dependencies:**
- MEGA SDK or custom implementation
- Crypto libraries for MEGA encryption

---

### 15. Implement Download Acceleration
**Priority:** High  
**Complexity:** Medium  
**Status:** Pending

- Multiple connections per file (already partially implemented)
- Optimize chunk sizes dynamically
- Connection pooling
- Adaptive segment count based on speed
- HTTP/2 multiplexing support

**Performance Target:**
- 2-5x speed improvement on supported servers

---

### 16. Add Password-Protected Archive Support
**Priority:** Low  
**Complexity:** Medium  
**Status:** Pending

- Auto-detect archive files (zip, rar, 7z)
- Prompt for password or use stored passwords
- Auto-extract after download
- Verify extracted files
- Delete archive after extraction (optional)

**Supported Formats:**
- ZIP (with password)
- RAR
- 7-Zip
- TAR.GZ (encrypted)

---

### 17. Create Mobile Companion App
**Priority:** Low  
**Complexity:** Very High  
**Status:** Pending

**Platforms:**
- iOS (Swift/SwiftUI)
- Android (Kotlin/Jetpack Compose)

**Features:**
- Remote control desktop app
- Add downloads remotely
- Monitor progress
- Receive notifications
- Start/pause/cancel downloads
- View download history

**Technical Approach:**
- REST API or WebSocket for communication
- End-to-end encryption
- Local network discovery

---

### 18. Implement Analytics Dashboard
**Priority:** Medium  
**Complexity:** Medium  
**Status:** Pending

**Charts & Metrics:**
- Download speed over time (line chart)
- Downloads by category (pie chart)
- Daily/weekly/monthly statistics
- Bandwidth usage trends
- Success/failure rates
- Average download speed
- Most downloaded file types
- Peak usage hours

**Export Options:**
- PDF reports
- CSV data export
- Image export for sharing

---

### 19. Add Streaming Protocol Support
**Priority:** Low  
**Complexity:** Very High  
**Status:** Pending

**Supported Protocols:**
- RTMP (Real-Time Messaging Protocol)
- HLS (HTTP Live Streaming)
- DASH (Dynamic Adaptive Streaming)
- RTSP (Real Time Streaming Protocol)

**Use Cases:**
- Live stream recording
- VOD download
- Adaptive bitrate stream capture

**Dependencies:**
- `ffmpeg` integration
- Stream parser libraries

---

### 20. Implement Smart Retry Logic
**Priority:** High  
**Complexity:** Medium  
**Status:** Pending

- Exponential backoff algorithm
- Error analysis and classification
- Different retry strategies per error type
- Max retry limits per error type
- Retry statistics and logging

**Error Types:**
- Network timeout → Immediate retry
- Server error 5xx → Exponential backoff
- Rate limit 429 → Wait for retry-after header
- DNS error → Longer backoff
- Connection refused → Check server availability

---

## 🎯 Advanced Features

Innovative functionality to set AFK-Dunld apart from competitors.

### 21. Add URL Import from Text Files
**Priority:** Low  
**Complexity:** Low  
**Status:** Pending

- Import URLs from .txt files
- Support multiple formats (one URL per line, CSV)
- Parse additional metadata (filename, category)
- Batch import with validation
- Duplicate URL detection

**Format Examples:**
```
# Simple format
https://example.com/file1.zip
https://example.com/file2.pdf

# CSV format
url,filename,category
https://example.com/file1.zip,myfile.zip,Software
```

---

### 22. Create Plugin/Extension System
**Priority:** Low  
**Complexity:** Very High  
**Status:** Pending

- Plugin architecture for custom protocols
- WebAssembly (WASM) plugins for safety
- Plugin API documentation
- Plugin marketplace/repository
- Sandboxed plugin execution
- Version compatibility checking

**Plugin Types:**
- Protocol handlers (custom download methods)
- Post-processors (file conversion)
- Notification channels
- Authentication providers

---

### 23. Implement Virus Scanning Integration
**Priority:** Medium  
**Complexity:** Medium  
**Status:** Pending

**Options:**
- ClamAV integration (local scanning)
- VirusTotal API (cloud scanning)
- Windows Defender API (Windows only)

**Features:**
- Automatic scan after download
- Quarantine infected files
- Scan history and reports
- Whitelist trusted sources
- Configurable scan sensitivity

---

### 24. Add RSS Feed Monitoring
**Priority:** Low  
**Complexity:** Medium  
**Status:** Pending

- Monitor RSS/Atom feeds
- Auto-download new items matching filters
- Filter by keywords, file types, size
- Episode tracking for podcasts/shows
- Mark downloaded items
- Support for torrent RSS feeds

**Use Cases:**
- Podcast auto-download
- Software release monitoring
- Blog post downloads
- Torrent RSS feeds

---

### 25. Implement Download Packaging
**Priority:** Low  
**Complexity:** Medium  
**Status:** Pending

- Combine multiple downloads into single archive
- Create ZIP, TAR, 7Z archives
- Custom compression levels
- Password protection
- Split archives by size
- Auto-package by category or date

---

### 26. Add WebDAV Protocol Support
**Priority:** Low  
**Complexity:** Medium  
**Status:** Pending

- WebDAV client implementation
- Authentication (Basic, Digest)
- Browse WebDAV folders
- Upload/download files
- Resume support for WebDAV
- SSL/TLS support

**Use Cases:**
- Nextcloud/ownCloud integration
- Corporate file servers
- Web hosting file management

---

### 27. Create Advanced Filtering System
**Priority:** Medium  
**Complexity:** Medium  
**Status:** Pending

**Filter Types:**
- Regex pattern matching
- File size ranges (min/max)
- Date ranges (created, completed)
- Status combinations
- Speed ranges
- Multiple category selection
- Custom SQL-like queries

**UI Features:**
- Filter builder with visual editor
- Save favorite filters
- Quick filter presets
- Filter export/import

---

### 28. Implement Download Preview
**Priority:** Low  
**Complexity:** High  
**Status:** Pending

- Preview media files before completion
- Progressive download for streaming
- Video player integration
- Audio player with waveform
- Image viewer with thumbnails
- PDF preview

**Supported Formats:**
- Video: MP4, MKV, WebM
- Audio: MP3, AAC, FLAC
- Images: JPG, PNG, GIF, WebP
- Documents: PDF, TXT

---

### 29. Add Subtitle Download Support
**Priority:** Low  
**Complexity:** Low  
**Status:** Pending

- Auto-download subtitles with videos
- yt-dlp already supports this (easy integration)
- Multiple subtitle languages
- Subtitle format selection (SRT, VTT, ASS)
- Embed subtitles in video (optional)
- Auto-detect preferred languages

---

### 30. Implement Download Sharing
**Priority:** Low  
**Complexity:** Medium  
**Status:** Pending

**Sharing Methods:**
- Generate QR code for download link
- Create shareable links with expiration
- Share download status/progress
- Share download templates
- Export download configuration

**Privacy:**
- Optional password protection
- Expiring links
- Limited uses
- Private/public toggle

---

### 31. Add Usenet (NZB) Support
**Priority:** Low  
**Complexity:** Very High  
**Status:** Pending

- NZB file parsing
- NNTP protocol implementation
- Multi-server support
- PAR2 repair integration
- Automatic unrar/unzip
- SSL/TLS connection support

**Dependencies:**
- NNTP client library
- PAR2 library
- RAR extraction

---

### 32. Create Download Completion Actions
**Priority:** Medium  
**Complexity:** Low  
**Status:** Pending

**Action Types:**
- Run custom script/command
- Shutdown computer
- Sleep/hibernate
- Send notification (email, webhook)
- Open file
- Move to specific folder
- Play sound
- Launch application

**Conditions:**
- All downloads complete
- Specific download complete
- Queue empty
- Category complete

---

### 33. Implement Duplicate Detection
**Priority:** Medium  
**Complexity:** Medium  
**Status:** Pending

- Detect duplicate URLs before download
- File hash comparison
- Size + filename matching
- Handle duplicates automatically (skip, rename, prompt)
- Duplicate detection in existing files
- Show duplicate sources

---

### 34. Add Authenticated Download Support
**Priority:** Medium  
**Complexity:** Medium  
**Status:** Pending

- Form-based authentication
- HTTP Basic/Digest auth (already supported)
- Session cookie handling
- OAuth2 flows
- Credential storage in vault
- Auto-login and retry

---

### 35. Implement P2P File Sharing
**Priority:** Low  
**Complexity:** Very High  
**Status:** Pending

- Share downloads between AFK-Dunld instances
- Local network discovery
- Encrypted transfers
- Resume support
- NAT traversal
- Public/private sharing modes

**Use Cases:**
- Share large files within organization
- Distributed downloads
- Local network acceleration

---

## 🌟 Professional Features

Enterprise-grade capabilities for power users and organizations.

### 36. Add Download Metadata Preservation
**Priority:** Low  
**Complexity:** Medium  
**Status:** Pending

- Preserve file tags and attributes
- Store custom metadata
- Comments and notes per download
- File origin tracking
- Extended attributes support
- Metadata export/import

---

### 37. Create Advanced Queue Rules
**Priority:** Medium  
**Complexity:** High  
**Status:** Pending

**Rule System:**
- Condition-based auto-start
- If-then rules engine
- Time-based conditions
- Resource-based conditions (disk space, bandwidth)
- Priority adjustments
- Category-based rules

**Example Rules:**
- "If disk space > 10GB, start queued downloads"
- "If time is 2AM-6AM, set max concurrent to 10"
- "If bandwidth usage < 50%, start high-priority downloads"

---

### 38. Implement Download Version Tracking
**Priority:** Low  
**Complexity:** Medium  
**Status:** Pending

- Track file versions
- Auto-detect newer versions
- Version comparison
- Update notifications
- Keep multiple versions
- Changelog tracking

**Use Cases:**
- Software updates
- Document revisions
- Media remastered versions

---

### 39. Add Multi-Language UI (i18n)
**Priority:** Medium  
**Complexity:** Medium  
**Status:** Pending

**Supported Languages (Phase 1):**
- English (default)
- Spanish
- French
- German
- Japanese
- Chinese (Simplified & Traditional)
- Russian
- Portuguese
- Arabic
- Hindi

**Implementation:**
- React i18next integration
- Language selector in settings
- RTL support for Arabic/Hebrew
- Crowdin for community translations
- Dynamic language loading

---

### 40. Create CLI Interface
**Priority:** Medium  
**Complexity:** High  
**Status:** Pending

**Commands:**
```bash
afkdunld add <url> [options]
afkdunld pause <id>
afkdunld resume <id>
afkdunld cancel <id>
afkdunld list [--status=downloading]
afkdunld stats
afkdunld config --set key=value
afkdunld daemon [--start|--stop|--status]
```

**Features:**
- Headless operation
- JSON output for scripting
- Daemon mode
- Progress bars in terminal
- Colored output
- Tab completion

---

### 41. Implement Device Synchronization
**Priority:** Low  
**Complexity:** Very High  
**Status:** Pending

- Sync downloads across devices
- Cloud-based sync backend
- End-to-end encryption
- Conflict resolution
- Selective sync (by category)
- Sync settings and templates

**Sync Data:**
- Download history
- Settings and preferences
- Categories
- Templates
- Queue state

---

### 42. Add Browser Cookie Import
**Priority:** Medium  
**Complexity:** Medium  
**Status:** Pending

- Import cookies from browsers
- Support Chrome, Firefox, Edge, Safari
- Use cookies for authenticated downloads
- Automatic cookie refresh
- Cookie jar management
- Privacy protection

**Use Cases:**
- Download from sites requiring login
- Premium content access
- Session-based downloads

---

### 43. Create Hash Verification System
**Priority:** Low  
**Complexity:** Medium  
**Status:** Pending

- Verify downloads against online databases
- Support for MD5, SHA1, SHA256, SHA512
- Query hash databases (VirusTotal, etc.)
- Store verified hashes
- Auto-verify on completion
- Hash mismatch alerts

---

### 44. Implement Smart Bandwidth Distribution
**Priority:** Medium  
**Complexity:** High  
**Status:** Pending

- Intelligent bandwidth allocation
- Priority-based distribution
- Dynamic adjustment based on:
  - Download priority
  - File size
  - Time remaining
  - Network conditions
- Fair-share algorithm
- Reserve bandwidth for interactive use

---

### 45. Add Segment Recovery
**Priority:** Medium  
**Complexity:** High  
**Status:** Pending

- Detect corrupted segments
- Re-download only corrupted parts
- CRC checking per segment
- Auto-repair damaged files
- Segment-level resume
- Integrity verification

---

### 46. Create Multi-Format History Export
**Priority:** Low  
**Complexity:** Low  
**Status:** Pending

**Export Formats:**
- CSV (spreadsheet compatible)
- JSON (for developers)
- XML (structured data)
- HTML (readable report)
- PDF (formatted report)
- SQLite (database dump)

**Export Options:**
- Date range filtering
- Category filtering
- Status filtering
- Custom field selection

---

### 47. Implement Multi-Channel Notifications
**Priority:** Low  
**Complexity:** Medium  
**Status:** Pending

**Notification Channels:**
- Email (SMTP)
- Webhook (HTTP POST)
- Push notifications (mobile)
- Slack integration
- Discord webhook
- Telegram bot
- Microsoft Teams
- Desktop notification (existing)

**Notification Events:**
- Download complete
- Download failed
- Queue empty
- Disk space low
- Speed threshold reached

---

### 48. Add Network Drive Support
**Priority:** Low  
**Complexity:** Medium  
**Status:** Pending

- Map network drives
- UNC path support (Windows)
- NFS mount support (Linux/Mac)
- SMB/CIFS support
- Remote path validation
- Connection monitoring
- Auto-reconnect

---

### 49. Create Performance Profiling
**Priority:** Low  
**Complexity:** High  
**Status:** Pending

**Profiling Metrics:**
- Download speed by time of day
- Server performance tracking
- Connection success rates
- Optimal segment count
- Bottleneck detection
- Resource usage monitoring

**Optimization Suggestions:**
- "Increase segments for this server"
- "Use proxy for better speed"
- "Download during off-peak hours"
- "Server is slow, try mirror"

---

### 50. Implement Rules-Based File Organization
**Priority:** Medium  
**Complexity:** High  
**Status:** Pending

**Rule Engine:**
- Move files based on rules
- Pattern matching (filename, size, type)
- Date-based organization
- Category-based folders
- Custom folder structures
- Variables: {category}, {date}, {year}, {month}, {ext}

**Example Rules:**
```
If category = "Video" AND size > 1GB:
  Move to "D:/Videos/{year}/{month}/"

If filename contains "Episode":
  Move to "D:/TV Shows/{series}/"
```

---

## 📊 Implementation Priority

### Phase 0: Codebase Audit Fixes (Sprint 0-1) — NEW
**Timeline:** 1-2 weeks  
**Focus:** Fix bugs discovered during codebase audit

- [ ] **Item 51**: Implement empty stub components (9 files)
- [ ] **Item 54**: Fix duplicate type definitions in types/download.ts
- [ ] **Item 58**: Remove duplicate Resume/Retry buttons in DownloadTableRow
- [ ] **Item 59**: Fix keyboard shortcut mismatch (MenuBar vs useKeyboardShortcuts)
- [ ] **Item 60**: Fix speed limit unit inconsistency between settings pages
- [ ] **Item 61**: Fix errorRecovery.ts parameter name mismatch
- [ ] **Item 62**: Fix QueueManager status filter case mismatch
- [ ] **Item 64**: Remove unused react-query or integrate it properly
- [ ] **Item 65**: Fix TorrentManager state type mismatch
- [ ] **Item 66**: Make AddDownloadDialog save path editable
- [ ] **Item 67**: Consolidate duplicate formatting functions

**Success Criteria:**
- Zero empty/stub component files
- No TypeScript type conflicts
- Consistent parameter naming across frontend/backend
- All UI shortcuts match actual behavior

---

### Phase 1: Critical Backend Fixes (Sprint 1-3)
**Timeline:** 3-5 weeks  
**Focus:** Stability, security, and core functionality

- [ ] **Item 52**: Implement resume/retry functions (currently no-ops)
- [ ] **Item 55**: Implement get_download_progress (currently returns None)
- [ ] **Item 2**: Complete scheduled download execution logic
- [ ] **Item 53**: Re-enable librqbit / implement torrent P2P
- [ ] **Item 3**: Connect deep link handler (Tauri v2 event-based)
- [ ] **Item 68**: Complete lib.rs entry point (missing commands)
- [ ] **Item 69**: Fix hardcoded master password (security)
- [ ] **Item 4**: ~~Parameterized database queries~~ ✅ Done
- [ ] **Item 5**: ~~History deletion~~ ✅ Done

**Success Criteria:**
- Resume/retry actually works
- Progress tracking displays real-time data
- Scheduled downloads auto-start
- Torrent downloads functional
- Deep links work from browser
- No hardcoded secrets
- All existing TODOs resolved

---

### Phase 2: Frontend Cleanup & Routing (Sprint 3-4)
**Timeline:** 2-3 weeks  
**Focus:** Remove dead code, fix routing, improve UX

- [ ] **Item 56**: Remove or implement unused components (Sidebar, Toolbar, etc.)
- [ ] **Item 57**: Fix category routes (currently 404)
- [ ] **Item 6**: Complete FTP test suite
- [ ] **Item 7**: Add comprehensive unit and integration tests

**Success Criteria:**
- No dead code in codebase
- All routes functional
- >80% test coverage
- FTP integration tested

---

### Phase 3: Core Enhancements (Sprint 5-8)
**Timeline:** 1-3 months  
**Focus:** Performance and reliability

- [ ] **Item 15**: Download acceleration (Performance)
- [ ] **Item 8**: Download mirroring (Reliability)
- [ ] **Item 20**: Smart retry logic (Reliability)
- [ ] **Item 13**: Bandwidth scheduling (User control)
- [ ] **Item 18**: Analytics dashboard (User insight)

**Success Criteria:**
- 2x faster download speeds
- <1% download failure rate
- 80%+ test coverage

---

### Phase 4: User Experience (Sprint 9-12)
**Timeline:** 2-3 months  
**Focus:** Usability and convenience

- [ ] **Item 10**: Auto-categorization (Convenience)
- [ ] **Item 11**: Batch renaming (Utility)
- [ ] **Item 12**: Download templates (Efficiency)
- [ ] **Item 32**: Completion actions (Automation)
- [ ] **Item 33**: Duplicate detection (Intelligence)
- [ ] **Item 27**: Advanced filtering (Organization)
- [ ] **Item 39**: Multi-language UI (Accessibility)

**Success Criteria:**
- 50% reduction in user actions
- Positive user feedback
- 10+ languages supported

---

### Phase 5: Advanced Features (Sprint 13-18)
**Timeline:** 3-6 months  
**Focus:** Differentiation and innovation

- [ ] **Item 9**: Cloud storage integration (Major feature)
- [ ] **Item 23**: Virus scanning (Security)
- [ ] **Item 42**: Browser cookie import (Convenience)
- [ ] **Item 21**: URL import (Batch operations)
- [ ] **Item 24**: RSS monitoring (Automation)
- [ ] **Item 29**: Subtitle downloads (Media)
- [ ] **Item 28**: Download preview (UX)

**Success Criteria:**
- Cloud storage working smoothly
- Virus detection active
- Positive feature reviews

---

### Phase 6: Enterprise Features (Sprint 19-26)
**Timeline:** 6-12 months  
**Focus:** Professional and enterprise needs

- [ ] **Item 40**: CLI interface (Automation)
- [ ] **Item 17**: Mobile companion app (Platform expansion)
- [ ] **Item 41**: Device synchronization (Multi-device)
- [ ] **Item 22**: Plugin system (Extensibility)
- [ ] **Item 47**: Multi-channel notifications (Integration)
- [ ] **Item 50**: Rules-based organization (Automation)
- [ ] **Item 49**: Performance profiling (Optimization)

**Success Criteria:**
- Enterprise customers onboarded
- Plugin ecosystem established
- Mobile app in stores

---

## ⚡ Quick Wins

These can be implemented quickly (1-3 days each) for immediate value:

### Tier 1: Easiest (< 1 day)
- [ ] **Item 54**: Fix duplicate type definitions (simple dedup)
- [ ] **Item 58**: Remove duplicate buttons (delete copy-paste error)
- [ ] **Item 59**: Fix keyboard shortcut mismatch (update string)
- [ ] **Item 61**: Fix errorRecovery parameter names (rename variable)
- [ ] **Item 62**: Fix QueueManager case mismatch (lowercase strings)
- [ ] **Item 64**: Remove unused react-query (delete imports)
- [ ] **Item 66**: Make save path editable (remove readOnly)

### Tier 2: Easy (1-2 days)
- [ ] **Item 51**: Implement stub components (9 files, straightforward)
- [ ] **Item 60**: Fix speed limit unit inconsistency (standardize units)
- [ ] **Item 65**: Fix TorrentManager type mismatch (align types)
- [ ] **Item 67**: Consolidate formatting functions (import from utils)
- [ ] **Item 56**: Remove dead code components (delete unused files)
- [ ] **Item 5**: ~~History deletion~~ ✅ Done

### Tier 3: Medium (2-5 days)
- [ ] **Item 52**: Implement resume/retry logic (wire up engine)
- [ ] **Item 55**: Implement get_download_progress (connect to engine)
- [ ] **Item 57**: Fix category routes (add route definitions)
- [ ] **Item 68**: Complete lib.rs entry point (add missing commands)
- [ ] **Item 12**: Download templates (JSON storage)
- [ ] **Item 32**: Completion actions (event handlers)
- [ ] **Item 33**: Duplicate detection (hash comparison)

---

## 🔥 High-Impact Features

Features that would significantly differentiate AFK-Dunld from competitors:

### Must-Have Differentiators
1. 🎯 **Item 8: Download Mirroring** (Faster, more reliable)
2. 🎯 **Item 9: Cloud Storage Integration** (Huge use case, unique)
3. 🎯 **Item 18: Analytics Dashboard** (Data-driven insights)
4. 🎯 **Item 23: Virus Scanning** (Security and trust)

### Strong Differentiators
5. ⭐ **Item 17: Mobile Companion App** (Modern workflow)
6. ⭐ **Item 40: CLI Interface** (Developer/DevOps appeal)
7. ⭐ **Item 24: RSS Monitoring** (Automation power users)
8. ⭐ **Item 22: Plugin System** (Community extensibility)

### Nice-to-Have Differentiators
9. 💡 **Item 41: Device Sync** (Multi-device users)
10. 💡 **Item 50: Rules Engine** (Power automation)
11. 💡 **Item 35: P2P Sharing** (Unique feature)
12. 💡 **Item 42: Cookie Import** (Authenticated downloads)

---

## 📈 Metrics & Success Tracking

### Key Performance Indicators (KPIs)

**Development Velocity:**
- Items completed per sprint
- Average time per item
- Bug fix rate

**Quality Metrics:**
- Test coverage percentage
- Bug count (open vs closed)
- Performance benchmarks

**User Satisfaction:**
- Feature request votes
- User feedback scores
- Support ticket trends

**Adoption Metrics:**
- Active users
- Download volume
- Feature usage statistics

---

## 🤝 Contributing

Want to contribute? Here's how:

1. **Pick an item** from the Quick Wins or any category
2. **Check GitHub Issues** to see if someone is already working on it
3. **Create an issue** to claim the item
4. **Fork and implement** following the contribution guidelines
5. **Submit a PR** with tests and documentation

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines.

---

## 📝 Notes

- All items are subject to change based on community feedback
- Priority levels may be adjusted based on user demand
- Complexity estimates are rough and may vary
- Some features may be combined or split during implementation
- Security and stability always take precedence over new features
- Items 51-70 were added during the comprehensive codebase audit on 2026-04-06

---

## 🔄 Updates Log

| Date | Change | Notes |
|------|--------|-------|
| 2026-02-20 | Initial creation | 50 items added across 4 categories |
| 2026-04-06 | Codebase audit | Added 20 new items (51-70) from comprehensive analysis. Updated status of Items 4, 5 to ✅ Fixed. Added Phase 0 to implementation priority. Reorganized into 🔴 Critical, 🟠 High, 🟡 Medium priority sections for audit findings. |

---

**Last Updated:** 2026-04-06  
**Maintained by:** AFK-Dunld Development Team  
**Version:** 1.1
