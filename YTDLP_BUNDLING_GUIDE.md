# yt-dlp Bundling Implementation Guide

## Overview
This application now includes **bundled yt-dlp binaries** for all platforms (Windows, macOS, Linux), eliminating the need for users to manually install yt-dlp on their systems.

## Implementation Summary

### 1. Build-Time Binary Download (`src-tauri/build.rs`)
- Automatically downloads platform-specific yt-dlp binaries during compilation
- Downloads from official yt-dlp GitHub releases
- Stores binaries in `src-tauri/resources/bin/`
- Creates version tracking file for update management
- Sets executable permissions on Unix systems

**Binaries Downloaded:**
- Windows: `yt-dlp.exe`
- macOS: `yt-dlp_macos`
- Linux: `yt-dlp_linux`

### 2. Resource Management (`src-tauri/src/utils/ytdlp_manager.rs`)
**YtdlpManager** handles:
- Extracting bundled binary to app data directory on first run
- Binary path management across platforms
- Version checking and validation
- Auto-update capability via yt-dlp's self-update feature
- Fallback to system yt-dlp if bundled version fails

### 3. YouTubeDownloader Integration (`src-tauri/src/network/youtube_downloader.rs`)
- Modified to accept custom binary path
- Uses bundled binary instead of system PATH
- Backwards compatible with system installations

### 4. State Management (`src-tauri/src/state/app_state.rs`)
- Initializes YtdlpManager during app startup
- Automatically extracts and validates bundled binary
- Makes binary path available to all download commands

### 5. Command Interface (`src-tauri/src/commands/`)
**Updated Commands:**
- `check_ytdlp_installed` - Uses bundled binary
- `get_video_info` - Uses bundled binary
- `get_video_qualities` - Uses bundled binary
- `check_is_playlist` - Uses bundled binary

**New Commands:**
- `update_ytdlp` - Update bundled yt-dlp to latest version
- `get_ytdlp_version` - Get current yt-dlp version
- `get_bundled_ytdlp_version` - Get originally bundled version

### 6. Tauri Configuration (`src-tauri/tauri.conf.json`)
- Added `resources/bin/*` to bundle configuration
- Ensures binaries are included in final application package

### 7. Frontend Updates (`src/App.tsx`)
- Updated warning messages for bundled version
- Changed from "install yt-dlp" to "restart application" guidance

## File Structure
```
src-tauri/
├── build.rs                          # Downloads binaries during build
├── resources/
│   └── bin/
│       ├── .gitkeep
│       ├── yt-dlp.exe               # Windows (downloaded)
│       ├── yt-dlp_macos             # macOS (downloaded)
│       ├── yt-dlp_linux             # Linux (downloaded)
│       └── ytdlp-version.txt        # Version tracking (downloaded)
├── src/
│   ├── utils/
│   │   └── ytdlp_manager.rs         # Binary management
│   └── commands/
│       └── ytdlp_commands.rs        # Update/version commands
└── Cargo.toml                        # Added ureq for downloads
```

## Build Process

### First Build
1. `cargo build` runs `build.rs`
2. Creates `resources/bin/` directory
3. Downloads all platform binaries from GitHub
4. Sets executable permissions
5. Creates version tracking file

### Subsequent Builds
- Skips download if binaries already exist
- Only re-downloads if files are deleted

## Runtime Behavior

### First App Launch
1. YtdlpManager initializes
2. Detects platform (Windows/macOS/Linux)
3. Extracts appropriate binary to `~/.afk-dunld/bin/`
4. Sets executable permissions
5. Verifies binary works (`--version` check)

### Subsequent Launches
- Uses existing extracted binary
- Validates it's still working
- Re-extracts if validation fails

## User Experience Improvements

### Before (Manual Installation Required)
❌ Users had to separately install yt-dlp  
❌ Platform-specific installation instructions needed  
❌ PATH configuration required  
❌ Version inconsistencies across users  
❌ Extra installation step

### After (Bundled Version)
✅ Zero configuration required  
✅ Works immediately after app installation  
✅ Consistent version across all users  
✅ Single installation process  
✅ Automatic updates available  

## Update Mechanism

Users can update yt-dlp via:
```typescript
// Frontend call
await invoke('update_ytdlp');
```

This uses yt-dlp's built-in self-update feature:
```bash
yt-dlp -U
```

## Platform-Specific Details

### Windows
- Binary: `yt-dlp.exe` (~10-12 MB)
- Location: `%APPDATA%\afk-dunld\bin\yt-dlp.exe`
- No permission issues

### macOS
- Binary: `yt-dlp_macos` (~12-15 MB, universal binary)
- Location: `~/Library/Application Support/afk-dunld/bin/yt-dlp`
- Executable permissions set automatically
- May require Gatekeeper approval on first run

### Linux
- Binary: `yt-dlp_linux` (~12-15 MB, static binary)
- Location: `~/.local/share/afk-dunld/bin/yt-dlp`
- Executable permissions set automatically
- Works on all major distributions

## Dependencies Added

### Build Dependencies (`Cargo.toml`)
```toml
[build-dependencies]
ureq = { version = "2.9", features = ["json"] }
```

## Troubleshooting

### Binary Not Found
- Check `resources/bin/` exists after build
- Verify build script ran successfully
- Look for build warnings about download failures

### Permission Denied (Unix)
- Binary permissions should be `755`
- Re-initialization will reset permissions
- Check app data directory is writable

### Update Fails
- Requires internet connection
- May need admin/sudo on some systems
- Check firewall isn't blocking GitHub

### Version Mismatch
- Compare via `get_ytdlp_version` and `get_bundled_ytdlp_version`
- Run `update_ytdlp` to sync

## Testing Checklist

- [ ] Build completes successfully
- [ ] Binaries downloaded to `resources/bin/`
- [ ] App launches without errors
- [ ] `check_ytdlp_installed` returns true
- [ ] YouTube video info fetches successfully
- [ ] Video download works
- [ ] Audio extraction works
- [ ] Update command succeeds
- [ ] Version commands return valid data

## Future Enhancements

1. **Auto-update on startup** - Check for yt-dlp updates periodically
2. **Progress indicator** - Show download progress during updates
3. **Rollback mechanism** - Keep backup of previous version
4. **Custom version selection** - Allow users to choose specific yt-dlp versions
5. **Offline mode** - Graceful degradation without updates

## Supported Platforms

Downloads from **1000+ websites** including:
- YouTube, Vimeo, Dailymotion
- Twitch, Twitter/X, Facebook
- Instagram, TikTok, Reddit
- SoundCloud, Mixcloud, Bandcamp
- And many more...

## Binary Size Impact

Total addition to app size: ~30-40 MB (includes all 3 platform binaries)
- Only one binary is actually used per platform
- Future: Could optimize by downloading only current platform binary

## Security Considerations

- Binaries downloaded from official yt-dlp GitHub releases
- Version tracking for integrity verification
- Executable permissions properly managed
- No network access required after installation (except for updates)

## Conclusion

This implementation provides a seamless, zero-configuration experience for YouTube and video downloads, significantly improving the user experience and reducing support burden.
