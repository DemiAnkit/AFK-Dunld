# Build Guide

Complete guide to building AFK-Dunld from source on all platforms.

## Table of Contents
- [Prerequisites](#prerequisites)
- [Getting the Source Code](#getting-the-source-code)
- [Development Setup](#development-setup)
- [Development Build](#development-build)
- [Production Build](#production-build)
- [Platform-Specific Instructions](#platform-specific-instructions)
- [Troubleshooting Build Issues](#troubleshooting-build-issues)
- [CI/CD](#cicd)

## Prerequisites

### All Platforms

#### Node.js
- **Version**: 18.x or higher
- **Download**: [nodejs.org](https://nodejs.org)
- **Verify**:
  ```bash
  node --version  # Should be v18.x or higher
  npm --version
  ```

#### Rust
- **Version**: 1.70 or higher
- **Install**: [rustup.rs](https://rustup.rs)
- **Verify**:
  ```bash
  rustc --version  # Should be 1.70 or higher
  cargo --version
  ```

#### Tauri Prerequisites
Follow platform-specific guides: [Tauri Prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites)

### Windows

**Required:**
- **Microsoft Visual Studio C++ Build Tools**
  - Download: [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
  - Install "Desktop development with C++" workload
  
- **WebView2**
  - Usually pre-installed on Windows 10/11
  - Download: [WebView2 Runtime](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)

**Optional:**
- **Windows SDK** - Usually included with VS Build Tools
- **Git for Windows** - [git-scm.com](https://git-scm.com)

**Verify**:
```powershell
# Check if C++ compiler is available
where cl.exe

# Check WebView2
Get-AppxPackage -Name Microsoft.WebView2
```

### macOS

**Required:**
- **Xcode Command Line Tools**
  ```bash
  xcode-select --install
  ```

- **Homebrew** (recommended)
  ```bash
  /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
  ```

**Verify**:
```bash
# Check Xcode CLI tools
xcode-select -p

# Check compiler
clang --version
```

### Linux (Ubuntu/Debian)

**Required Packages**:
```bash
sudo apt update
sudo apt install -y \
    libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    libsqlite3-dev
```

**For Other Distros**:

**Fedora**:
```bash
sudo dnf install \
    webkit2gtk3-devel \
    openssl-devel \
    gtk3-devel \
    librsvg2-devel \
    sqlite-devel
```

**Arch Linux**:
```bash
sudo pacman -S \
    webkit2gtk \
    base-devel \
    curl \
    wget \
    openssl \
    gtk3 \
    librsvg \
    sqlite
```

## Getting the Source Code

### Clone Repository

```bash
# Via HTTPS
git clone https://github.com/yourusername/afk-dunld.git
cd afk-dunld

# Or via SSH
git clone git@github.com:yourusername/afk-dunld.git
cd afk-dunld
```

### Directory Structure

```
afk-dunld/
├── src/              # Frontend (React + TypeScript)
├── src-tauri/        # Backend (Rust + Tauri)
├── browser-extension/ # Browser extensions
├── docs/             # Documentation
└── public/           # Static assets
```

## Development Setup

### Install Dependencies

```bash
# Install Node.js dependencies
npm install

# This will also:
# - Install frontend dependencies
# - Install Tauri CLI
# - Set up development environment
```

### Environment Configuration

**Optional**: Create `.env` file in project root:
```env
# Development server port
VITE_PORT=1420

# Tauri development server
TAURI_DEV_HOST=localhost
```

### Verify Setup

```bash
# Check if everything is ready
npm run tauri info

# Should show:
# - Node.js version
# - Rust version
# - Operating system
# - Tauri version
```

## Development Build

### Start Development Server

```bash
npm run tauri dev
```

This will:
1. Start Vite development server (frontend)
2. Compile Rust backend
3. Launch the application
4. Enable hot-reload for frontend changes
5. Auto-rebuild backend on Rust file changes

**Development Features**:
- ✅ Hot Module Replacement (HMR)
- ✅ React Fast Refresh
- ✅ DevTools enabled
- ✅ Debug logging
- ✅ Source maps

### Development Shortcuts

**Reload Frontend**: `Ctrl+R` or `Cmd+R`

**Open DevTools**: `F12` or `Ctrl+Shift+I`

**Restart Backend**: Stop (Ctrl+C) and run `npm run tauri dev` again

## Production Build

### Platform-Specific Build

Builds for your current platform only (smaller bundle):

```bash
npm run tauri build
```

### What Gets Built

**Windows**:
- `.msi` installer
- `.exe` executable
- Located in: `src-tauri/target/release/bundle/msi/`

**macOS**:
- `.dmg` disk image
- `.app` application bundle
- Located in: `src-tauri/target/release/bundle/dmg/`

**Linux**:
- `.deb` package (Debian/Ubuntu)
- `.AppImage` portable app
- Located in: `src-tauri/target/release/bundle/deb/` and `appimage/`

### Build Output Locations

```
src-tauri/target/release/
├── bundle/
│   ├── msi/           # Windows installers
│   ├── dmg/           # macOS disk images
│   ├── deb/           # Debian packages
│   └── appimage/      # Linux AppImages
└── afk-dunld[.exe]    # Standalone executable
```

### yt-dlp Bundling

**Automatic Bundling** (Default):
```bash
# Bundles yt-dlp for your platform only
npm run tauri build
```

**All Platforms** (Larger Bundle):
```bash
# Windows
set YTDLP_BUNDLE_ALL_PLATFORMS=1
npm run tauri build

# Linux/macOS
export YTDLP_BUNDLE_ALL_PLATFORMS=1
npm run tauri build
```

**Skip Bundling** (Use system yt-dlp):
```bash
# Windows
set YTDLP_SKIP_BUNDLE=1
npm run tauri build

# Linux/macOS
export YTDLP_SKIP_BUNDLE=1
npm run tauri build
```

### Build Configuration

Edit `src-tauri/tauri.conf.json` for build settings:

```json
{
  "bundle": {
    "identifier": "com.ankit.afk-dunld",
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ],
    "windows": {
      "wix": {
        "language": "en-US"
      }
    },
    "macOS": {
      "minimumSystemVersion": "10.13"
    }
  }
}
```

## Platform-Specific Instructions

### Windows Build

**Prerequisites Check**:
```powershell
# Verify Visual Studio Build Tools
where cl.exe

# Verify Rust
rustc --version

# Verify Node.js
node --version
```

**Build Commands**:
```powershell
# Standard build
npm run tauri build

# Debug build (faster, larger)
cargo build --manifest-path=src-tauri/Cargo.toml

# Release build (optimized)
cargo build --release --manifest-path=src-tauri/Cargo.toml
```

**Installer Options**:
- MSI installer created automatically
- Includes auto-update support
- Installs to `C:\Program Files\AFK-Dunld\`

**Code Signing** (Optional):
```powershell
# Set certificate
$env:TAURI_PRIVATE_KEY = "path/to/cert.pfx"
$env:TAURI_KEY_PASSWORD = "password"

# Build with signing
npm run tauri build
```

### macOS Build

**Prerequisites Check**:
```bash
# Verify Xcode
xcode-select -p

# Verify Rust
rustc --version

# Verify Node.js
node --version
```

**Build Commands**:
```bash
# Standard build
npm run tauri build

# Universal binary (Intel + Apple Silicon)
npm run tauri build -- --target universal-apple-darwin
```

**Code Signing** (Required for distribution):
```bash
# Set up developer certificate
export APPLE_CERTIFICATE="Developer ID Application: Your Name (TEAM_ID)"
export APPLE_CERTIFICATE_PASSWORD="cert_password"
export APPLE_ID="your@apple.id"
export APPLE_PASSWORD="app-specific-password"

# Build with signing
npm run tauri build
```

**Notarization** (Required for distribution):
```bash
# Automatic with Tauri
export APPLE_ID="your@apple.id"
export APPLE_PASSWORD="app-specific-password"
export APPLE_TEAM_ID="TEAM_ID"

npm run tauri build
```

### Linux Build

**Prerequisites Check**:
```bash
# Verify dependencies
pkg-config --modversion gtk+-3.0
pkg-config --modversion webkit2gtk-4.0

# Verify Rust
rustc --version

# Verify Node.js
node --version
```

**Build Commands**:
```bash
# Standard build (creates .deb and .AppImage)
npm run tauri build

# DEB only
npm run tauri build -- --bundles deb

# AppImage only
npm run tauri build -- --bundles appimage
```

**DEB Package**:
- Installs to `/usr/bin/afk-dunld`
- Desktop entry created
- Icon installed
- Can be installed with: `sudo dpkg -i afk-dunld_*.deb`

**AppImage**:
- Portable, no installation needed
- Make executable: `chmod +x afk-dunld_*.AppImage`
- Run: `./afk-dunld_*.AppImage`

## Troubleshooting Build Issues

### Common Issues

#### Node.js Version Mismatch
```bash
# Error: Unsupported Node.js version
# Solution: Install Node.js 18+
nvm install 18
nvm use 18
```

#### Rust Toolchain Issues
```bash
# Error: rustc version too old
# Solution: Update Rust
rustup update stable
rustup default stable
```

#### Missing System Dependencies (Linux)
```bash
# Error: package 'xyz' not found
# Solution: Install missing package
sudo apt install libxyz-dev
```

#### WebView2 Missing (Windows)
```bash
# Error: WebView2 not found
# Solution: Download and install
# https://developer.microsoft.com/microsoft-edge/webview2/
```

#### Xcode CLI Tools (macOS)
```bash
# Error: xcrun: error: invalid active developer path
# Solution: Install Xcode CLI tools
xcode-select --install
```

### Build Errors

#### Cargo Build Fails
```bash
# Clean and rebuild
cd src-tauri
cargo clean
cargo build --release
```

#### Frontend Build Fails
```bash
# Clean and reinstall
rm -rf node_modules package-lock.json
npm install
npm run build
```

#### yt-dlp Download Fails
```bash
# Manual download
cd src-tauri/resources/bin
wget https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp
chmod +x yt-dlp  # Linux/macOS
```

### Performance Issues

#### Slow Build
```bash
# Use incremental compilation (development)
export CARGO_INCREMENTAL=1

# Parallel compilation
export CARGO_BUILD_JOBS=8

# Use faster linker (Linux)
sudo apt install mold
export RUSTFLAGS="-C link-arg=-fuse-ld=mold"
```

#### Large Bundle Size
```bash
# Strip debug symbols
cargo build --release
strip target/release/afk-dunld  # Linux/macOS

# Optimize for size
# Add to Cargo.toml:
[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
strip = true
```

## CI/CD

### GitHub Actions

**Example workflow** (`.github/workflows/build.yml`):

```yaml
name: Build

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  build:
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    
    runs-on: ${{ matrix.os }}
    
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Node.js
        uses: actions/setup-node@v3
        with:
          node-version: 18
      
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Install dependencies (Linux)
        if: runner.os == 'Linux'
        run: |
          sudo apt update
          sudo apt install -y libwebkit2gtk-4.0-dev \
            libgtk-3-dev libayatana-appindicator3-dev \
            librsvg2-dev
      
      - name: Install dependencies
        run: npm install
      
      - name: Build
        run: npm run tauri build
      
      - name: Upload artifacts
        uses: actions/upload-artifact@v3
        with:
          name: ${{ runner.os }}-build
          path: src-tauri/target/release/bundle/
```

### Release Workflow

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  release:
    # ... similar to build workflow
    
    steps:
      # ... build steps
      
      - name: Create Release
        uses: softprops/action-gh-release@v1
        with:
          files: |
            src-tauri/target/release/bundle/**/*.msi
            src-tauri/target/release/bundle/**/*.dmg
            src-tauri/target/release/bundle/**/*.deb
            src-tauri/target/release/bundle/**/*.AppImage
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

## Advanced Build Options

### Custom Build Scripts

**Build backend only**:
```bash
cd src-tauri
cargo build --release
```

**Build frontend only**:
```bash
npm run build
```

### Cross-Compilation

**Windows → Linux** (Docker):
```bash
docker run --rm -v $(pwd):/app -w /app \
  rust:latest \
  cargo build --release --target x86_64-unknown-linux-gnu
```

**Linux → Windows** (Cross):
```bash
# Install cross
cargo install cross

# Build for Windows
cross build --release --target x86_64-pc-windows-gnu
```

### Debug vs Release

**Debug Build** (faster compilation, slower runtime):
```bash
cargo build
# Output: target/debug/afk-dunld
```

**Release Build** (slower compilation, optimized):
```bash
cargo build --release
# Output: target/release/afk-dunld
```

## Resources

- [Tauri Build Documentation](https://tauri.app/v1/guides/building/)
- [Rust Cargo Book](https://doc.rust-lang.org/cargo/)
- [Vite Build Guide](https://vitejs.dev/guide/build.html)

## Need Help?

- 📖 [Architecture Overview](ARCHITECTURE.md)
- 🐛 [Report Issue](https://github.com/yourusername/afk-dunld/issues)
- 💬 [Discussions](https://github.com/yourusername/afk-dunld/discussions)
