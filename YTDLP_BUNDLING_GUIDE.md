# yt-dlp Bundling Guide

Guide to bundling yt-dlp with AFK-Dunld for zero-configuration downloads.

## Table of Contents
- [Overview](#overview)
- [Why Bundle yt-dlp](#why-bundle-yt-dlp)
- [How Bundling Works](#how-bundling-works)
- [Build-Time Bundling](#build-time-bundling)
- [Runtime Detection](#runtime-detection)
- [Update Mechanism](#update-mechanism)
- [Platform-Specific Details](#platform-specific-details)
- [Troubleshooting](#troubleshooting)

## Overview

AFK-Dunld bundles yt-dlp binaries to provide zero-configuration YouTube and video downloads. Users don't need to install yt-dlp separately - it's included with the app.

## Why Bundle yt-dlp

### Benefits
✅ **Zero Configuration** - Works out of the box
✅ **No Manual Installation** - yt-dlp included
✅ **Version Control** - Known working version
✅ **Consistent Experience** - Same across all platforms
✅ **Auto-Updates** - Update yt-dlp from within app
✅ **Portable** - No system dependencies

### Challenges
❌ **Bundle Size** - Adds ~10-30 MB per platform
❌ **Maintenance** - Must keep yt-dlp updated
❌ **Platform Support** - Need binaries for each OS

## How Bundling Works

### Build Process Flow

```
Build Start
    ↓
Check Environment Variables
    ↓
YTDLP_SKIP_BUNDLE? → Yes → Skip bundling
    ↓ No
    ↓
YTDLP_BUNDLE_ALL_PLATFORMS? → Yes → Download all platforms
    ↓ No
    ↓
Download platform-specific binary
    ↓
Place in src-tauri/resources/bin/
    ↓
Tauri bundles resources into app
    ↓
Build Complete
```

### Directory Structure

```
src-tauri/
├── resources/
│   └── bin/
│       ├── yt-dlp           # Linux/macOS binary
│       ├── yt-dlp.exe       # Windows binary
│       └── ytdlp-version.txt # Version tracker
├── build.rs                  # Build script
└── Cargo.toml
```

## Build-Time Bundling

### Build Script (`src-tauri/build.rs`)

The build script automatically downloads yt-dlp during compilation:

```rust
// Simplified version
fn main() {
    // Check if bundling is disabled
    if env::var("YTDLP_SKIP_BUNDLE").is_ok() {
        return;
    }
    
    // Determine which platforms to bundle
    let bundle_all = env::var("YTDLP_BUNDLE_ALL_PLATFORMS").is_ok();
    
    if bundle_all {
        download_ytdlp_for_all_platforms();
    } else {
        download_ytdlp_for_current_platform();
    }
}
```

### Download Sources

**Official yt-dlp Releases:**
- Linux: `https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp_linux`
- macOS: `https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp_macos`
- Windows: `https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp.exe`

### Build Environment Variables

#### YTDLP_SKIP_BUNDLE
Skip bundling entirely (use system yt-dlp).

```bash
# Linux/macOS
export YTDLP_SKIP_BUNDLE=1
npm run tauri build

# Windows
set YTDLP_SKIP_BUNDLE=1
npm run tauri build
```

#### YTDLP_BUNDLE_ALL_PLATFORMS
Bundle binaries for all platforms (larger bundle).

```bash
# Linux/macOS
export YTDLP_BUNDLE_ALL_PLATFORMS=1
npm run tauri build

# Windows
set YTDLP_BUNDLE_ALL_PLATFORMS=1
npm run tauri build
```

#### YTDLP_VERSION
Specify yt-dlp version to bundle.

```bash
# Linux/macOS
export YTDLP_VERSION=2023.12.30
npm run tauri build

# Windows
set YTDLP_VERSION=2023.12.30
npm run tauri build
```

## Runtime Detection

### YtDlpManager (`src-tauri/src/utils/ytdlp_manager.rs`)

The manager handles yt-dlp detection and execution:

```rust
pub struct YtDlpManager {
    binary_path: PathBuf,
    version: Option<String>,
}

impl YtDlpManager {
    pub fn new() -> Self {
        let binary_path = Self::find_binary();
        let version = Self::get_version(&binary_path);
        
        Self {
            binary_path,
            version,
        }
    }
    
    fn find_binary() -> PathBuf {
        // 1. Check bundled location
        if let Some(path) = Self::bundled_binary() {
            return path;
        }
        
        // 2. Check system PATH
        if let Some(path) = Self::system_binary() {
            return path;
        }
        
        // 3. Check common install locations
        Self::common_locations()
            .into_iter()
            .find(|p| p.exists())
            .unwrap_or_else(|| PathBuf::from("yt-dlp"))
    }
}
```

### Search Order

1. **Bundled Binary** (Highest Priority)
   - Windows: `%APPDATA%\afk-dunld\bin\yt-dlp.exe`
   - macOS: `~/Library/Application Support/afk-dunld/bin/yt-dlp`
   - Linux: `~/.local/share/afk-dunld/bin/yt-dlp`

2. **System PATH**
   - Checks if `yt-dlp` is in PATH

3. **Common Locations**
   - `/usr/local/bin/yt-dlp`
   - `/usr/bin/yt-dlp`
   - `~/.local/bin/yt-dlp`

4. **Fallback**
   - Assumes `yt-dlp` in PATH

## Update Mechanism

### In-App Updates

Users can update yt-dlp from Settings:

**Settings → YouTube → Update yt-dlp**

### Update Process

```
User Clicks "Update yt-dlp"
    ↓
Fetch latest version from GitHub API
    ↓
Compare with current version
    ↓
Download new binary
    ↓
Verify download (size, executable)
    ↓
Backup current binary
    ↓
Replace with new binary
    ↓
Set executable permissions
    ↓
Update version.txt
    ↓
Verify new version works
    ↓
Update Complete
```

### Update Command (`src-tauri/src/commands/ytdlp_commands.rs`)

```rust
#[tauri::command]
pub async fn update_ytdlp() -> Result<String, String> {
    // 1. Get latest version
    let latest_version = fetch_latest_version().await?;
    
    // 2. Download binary
    let binary_data = download_ytdlp_binary().await?;
    
    // 3. Get binary path
    let binary_path = get_ytdlp_path();
    
    // 4. Backup current
    backup_current_binary(&binary_path)?;
    
    // 5. Write new binary
    fs::write(&binary_path, binary_data)?;
    
    // 6. Set permissions (Unix)
    #[cfg(unix)]
    set_executable(&binary_path)?;
    
    // 7. Verify
    verify_ytdlp_works(&binary_path)?;
    
    Ok(latest_version)
}
```

## Platform-Specific Details

### Windows

**Binary Name**: `yt-dlp.exe`

**Bundled Location**: `%APPDATA%\afk-dunld\bin\yt-dlp.exe`

**Installation**:
- Copied to AppData during first run
- No special permissions needed
- Executable by default

**Antivirus**:
- May be flagged as false positive
- Add exclusion if needed

### macOS

**Binary Name**: `yt-dlp` (universal binary)

**Bundled Location**: `~/Library/Application Support/afk-dunld/bin/yt-dlp`

**Installation**:
- Extracted from app bundle
- Copied to Application Support
- `chmod +x` applied

**Gatekeeper**:
- May require approval on first run
- `xattr -d com.apple.quarantine` if needed

**Architecture**:
- Universal binary (Intel + Apple Silicon)
- Works on both architectures

### Linux

**Binary Name**: `yt-dlp`

**Bundled Location**: `~/.local/share/afk-dunld/bin/yt-dlp`

**Installation**:
- Extracted from AppImage/DEB
- Copied to .local/share
- `chmod +x` applied

**Dependencies**:
- Python not required (standalone binary)
- FFmpeg recommended for merging

**Permissions**:
- Must be executable
- No root required

## Bundle Size Comparison

### Single Platform

| Platform | Binary Size | Compressed |
|----------|------------|------------|
| Windows  | ~10 MB     | ~4 MB      |
| macOS    | ~30 MB     | ~12 MB     |
| Linux    | ~12 MB     | ~5 MB      |

### All Platforms

| Bundle     | Total Size | Compressed |
|------------|------------|------------|
| All 3      | ~52 MB     | ~21 MB     |

### Size Optimization

**Enable Compression**:
```toml
# Cargo.toml
[profile.release]
lto = true
opt-level = "z"
strip = true
```

**Result**: Reduces bundle size by ~30-40%

## Troubleshooting

### yt-dlp Not Found

**Issue**: App can't find yt-dlp binary

**Solutions**:
1. Check bundled location exists
2. Reinstall app (re-extracts binary)
3. Manually download and place in bin folder
4. Install system yt-dlp as fallback

### Update Fails

**Issue**: Can't update yt-dlp

**Solutions**:
1. Check internet connection
2. Check GitHub is accessible
3. Check write permissions
4. Download manually:
   ```bash
   # Download latest
   curl -L https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp -o yt-dlp
   chmod +x yt-dlp
   
   # Place in correct location
   # See platform-specific paths above
   ```

### Permission Denied (Linux/macOS)

**Issue**: Binary not executable

**Solution**:
```bash
# Make executable
chmod +x ~/.local/share/afk-dunld/bin/yt-dlp

# Or for macOS
chmod +x ~/Library/Application\ Support/afk-dunld/bin/yt-dlp
```

### Antivirus False Positive (Windows)

**Issue**: Windows Defender blocks yt-dlp.exe

**Solution**:
1. Add exclusion in Windows Security
2. Path: `%APPDATA%\afk-dunld\bin\`
3. Or download from official source and verify hash

## Development

### Testing Bundled vs System

**Use Bundled**:
```bash
npm run tauri dev
```

**Use System**:
```bash
export YTDLP_SKIP_BUNDLE=1
npm run tauri dev
```

### Manual Binary Placement

For development/testing:

```bash
# Create resources directory
mkdir -p src-tauri/resources/bin

# Download yt-dlp
curl -L https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp \
  -o src-tauri/resources/bin/yt-dlp

# Make executable
chmod +x src-tauri/resources/bin/yt-dlp

# Build
npm run tauri build
```

## CI/CD Integration

### GitHub Actions Example

```yaml
- name: Download yt-dlp
  run: |
    mkdir -p src-tauri/resources/bin
    
    # Linux
    curl -L https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp_linux \
      -o src-tauri/resources/bin/yt-dlp
    chmod +x src-tauri/resources/bin/yt-dlp
    
    # Windows
    curl -L https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp.exe \
      -o src-tauri/resources/bin/yt-dlp.exe
    
    # macOS
    curl -L https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp_macos \
      -o src-tauri/resources/bin/yt-dlp_macos
    chmod +x src-tauri/resources/bin/yt-dlp_macos
```

## Resources

- [yt-dlp Releases](https://github.com/yt-dlp/yt-dlp/releases)
- [yt-dlp Documentation](https://github.com/yt-dlp/yt-dlp#readme)
- [Tauri Resource Bundling](https://tauri.app/v1/guides/building/resources)

## Related Documentation

- [Build Optimization Guide](BUILD_OPTIMIZATION_GUIDE.md)
- [Implementation Details](BUNDLED_YTDLP_IMPLEMENTATION.md)
- [Build Guide](docs/BUILD_GUIDE.md)
