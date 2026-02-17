# Changelog

All notable changes to AFK-Dunld will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned
- Browser extension for Chrome and Firefox
- Cloud sync for download history
- Mobile companion app
- RSS feed monitoring

---

## [0.1.0] - 2024-02-17

### 🎉 Initial Release

#### Added
- **Bundled yt-dlp** - Zero-configuration YouTube downloads
  - Automatic binary extraction on first run
  - Support for Windows, macOS, and Linux
  - Auto-update functionality
  - 1000+ supported video platforms

- **Download Management**
  - Multi-threaded downloads with segment splitting
  - Resume support for interrupted downloads
  - Queue management with priority control
  - Batch download support
  - Speed limiting (global and per-download)

- **YouTube Features**
  - Video downloads with quality selection (4K to 360p)
  - Audio extraction (MP3, AAC, FLAC, Opus, M4A)
  - Playlist batch downloads
  - Metadata embedding
  - Thumbnail embedding

- **User Interface**
  - Modern, clean interface with glassmorphism effects
  - Dark and light theme support
  - Real-time download progress with speed graphs
  - Search and filter functionality
  - Keyboard shortcuts for all major actions
  - System tray integration

- **Advanced Features**
  - **Download History** - Complete tracking with statistics
    - Export to JSON
    - Search and filter
    - Performance metrics (speed, duration)
    - 5-stat dashboard
  
  - **Schedule Manager** - Timed and recurring downloads
    - Set specific download times
    - Recurring patterns support
    - Enable/disable schedules
  
  - **Queue Manager** - Advanced queue control
    - Priority management
    - Concurrent download limits
    - Pause/resume all functionality
    - Real-time statistics
  
  - **Error Recovery** - Automatic error handling
    - Auto-retry failed downloads
    - Stall detection and recovery
    - Exponential backoff
    - Global error handling

- **Protocol Support**
  - HTTP/HTTPS downloads
  - FTP/FTPS support
  - SFTP support
  - Torrent support (magnet links and .torrent files)

- **Optimization**
  - Bundle size optimization (60-70% reduction)
  - Platform-specific builds
  - Code splitting and lazy loading
  - Frontend bundle optimization
  - Terser minification

- **Configuration**
  - yt-dlp settings page
  - Download quality defaults
  - Format preferences
  - Retry configuration
  - Network settings
  - Theme customization

#### Technical
- Built with Tauri 2.0 and React 18
- TypeScript for type safety
- Rust backend for performance and security
- SQLite database for download tracking
- Zustand for state management
- Tailwind CSS for styling

#### Documentation
- Comprehensive README
- User guides and quick start
- Developer documentation
- API documentation
- Build and optimization guides
- Troubleshooting guide

#### Performance
- App size: 19-35 MB (platform-specific)
- Cold start: <2 seconds
- Memory usage: ~150 MB average
- Download speed: Network-limited only

#### Security
- Sandboxed runtime via Tauri
- No elevated privileges required
- Local data storage only
- Secure credential handling

---

## Version History

### v0.1.0 - Initial Release
**Release Date:** February 17, 2024

**Highlights:**
- Complete download manager with YouTube support
- Modern UI with dark/light themes
- Advanced queue and schedule management
- Bundled yt-dlp for zero-configuration setup
- Optimized build size (60-70% smaller)

**Files:**
- Windows: `AFK-Dunld-Setup-0.1.0.msi` (19 MB)
- macOS: `AFK-Dunld-0.1.0.dmg` (35 MB)
- Linux: `afk-dunld_0.1.0_amd64.deb` (35 MB)
- Linux: `afk-dunld_0.1.0_amd64.AppImage` (35 MB)

**Known Issues:**
- macOS may require Gatekeeper approval on first run
- Some antivirus software may flag the bundled yt-dlp binary (false positive)

**Upgrade Notes:**
- First release, no upgrade path needed

---

## Versioning Scheme

We use [Semantic Versioning](https://semver.org/):
- **MAJOR** version for incompatible API changes
- **MINOR** version for added functionality (backwards-compatible)
- **PATCH** version for backwards-compatible bug fixes

---

## Release Channels

### Stable
- Recommended for all users
- Thoroughly tested
- Released every 4-8 weeks

### Beta
- Early access to new features
- May contain bugs
- Released every 1-2 weeks

### Nightly
- Latest development code
- Unstable, for testing only
- Released daily

---

## Migration Guides

### Upgrading to v0.2.0 (When Available)
TBD

---

## Deprecation Policy

- Features marked as deprecated will be removed in the next major version
- At least 3 months notice before removal
- Migration guides provided for all breaking changes

---

## Support

For questions about releases:
- 📖 [Documentation](docs/)
- 💬 [Discussions](https://github.com/yourusername/afk-dunld/discussions)
- 🐛 [Issue Tracker](https://github.com/yourusername/afk-dunld/issues)

---

[Unreleased]: https://github.com/yourusername/afk-dunld/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/yourusername/afk-dunld/releases/tag/v0.1.0
