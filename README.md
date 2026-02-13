# AFK-Dunld

A powerful cross-platform download manager built with Tauri, React, and Rust. Supports HTTP/HTTPS, FTP, and Torrent downloads with advanced features like pause/resume, speed limiting, clipboard monitoring, and browser integration.

![Version](https://img.shields.io/badge/version-0.1.0-blue.svg)
![License](https://img.shields.io/badge/license-MIT-green.svg)

---

## Table of Contents

- [Prerequisites](#prerequisites)
- [Installation Guide](#installation-guide)
  - [Step 1: Install Rust](#step-1-install-rust)
  - [Step 2: Install Node.js](#step-2-install-nodejs)
  - [Step 3: Clone the Repository](#step-3-clone-the-repository)
  - [Step 4: Install Dependencies](#step-4-install-dependencies)
  - [Step 5: Run the Application](#step-5-run-the-application)
- [Platform-Specific Requirements](#platform-specific-requirements)
- [Development](#development)
- [Building for Production](#building-for-production)
- [Project Structure](#project-structure)
- [Browser Extension](#browser-extension)
- [Troubleshooting](#troubleshooting)

---

## Prerequisites

Before you begin, ensure you have the following installed on your system:

### System Requirements

- **Windows**: Windows 10 (version 1809 or later) or Windows 11
- **macOS**: macOS 10.13 (High Sierra) or later
- **Linux**: Ubuntu 18.04+, Debian 10+, Fedora 32+, or equivalent distributions

### Required Software

1. **Rust** (latest stable version)
2. **Node.js** (v18 or higher) with npm
3. **Git** (for cloning the repository)

---

## Installation Guide

Follow these steps to set up AFK-Dunld on your machine:

### Step 1: Install Rust

Rust is required for building the Tauri backend.

#### Windows

1. Download and run **rustup-init.exe** from [https://rustup.rs](https://rustup.rs)
2. Follow the on-screen instructions (select default installation when prompted)
3. Restart your terminal or command prompt
4. Verify installation:
   ```powershell
   rustc --version
   cargo --version
   ```

**Additional Windows Requirements:**

- **Microsoft C++ Build Tools**: Download from [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
  - During installation, select "Desktop development with C++"
  - This includes MSVC compiler and Windows SDK

- **WebView2**: Usually pre-installed on Windows 11. For Windows 10:
  - Download from [Microsoft Edge WebView2 Runtime](https://developer.microsoft.com/microsoft-edge/webview2/)

#### macOS

1. Open Terminal and run:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. Follow the prompts (press Enter for default installation)
3. Reload your shell configuration:
   ```bash
   source $HOME/.cargo/env
   ```
4. Verify installation:
   ```bash
   rustc --version
   cargo --version
   ```

**Additional macOS Requirements:**

- **Xcode Command Line Tools**:
  ```bash
  xcode-select --install
  ```

#### Linux (Ubuntu/Debian)

1. Install dependencies first:
   ```bash
   sudo apt update
   sudo apt install -y libwebkit2gtk-4.1-dev \
     build-essential \
     curl \
     wget \
     file \
     libssl-dev \
     libgtk-3-dev \
     libayatana-appindicator3-dev \
     librsvg2-dev
   ```

2. Install Rust:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
3. Reload shell:
   ```bash
   source $HOME/.cargo/env
   ```
4. Verify:
   ```bash
   rustc --version
   cargo --version
   ```

#### Linux (Fedora)

1. Install dependencies:
   ```bash
   sudo dnf install webkit2gtk4.1-devel \
     openssl-devel \
     curl \
     wget \
     file \
     libappindicator-gtk3-devel \
     librsvg2-devel
   sudo dnf group install "C Development Tools and Libraries"
   ```

2. Install Rust (same as Ubuntu steps 2-4)

---

### Step 2: Install Node.js

Node.js is required for the React frontend.

#### Windows & macOS

1. Download the LTS installer from [https://nodejs.org](https://nodejs.org)
2. Run the installer and follow the prompts
3. Verify installation:
   ```bash
   node --version
   npm --version
   ```

#### Linux (Ubuntu/Debian)

Using NodeSource repository (recommended):
```bash
curl -fsSL https://deb.nodesource.com/setup_lts.x | sudo -E bash -
sudo apt install -y nodejs
```

Verify:
```bash
node --version
npm --version
```

#### Alternative: Using Node Version Manager (nvm)

For better version management across all platforms:

1. Install nvm:
   ```bash
   # macOS/Linux
   curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
   
   # Windows: Download nvm-windows from
   # https://github.com/coreybutler/nvm-windows/releases
   ```

2. Install Node.js:
   ```bash
   nvm install --lts
   nvm use --lts
   ```

---

### Step 3: Clone the Repository

```bash
git clone <your-repository-url>
cd AFK-Dunld
```

Or download the source code as a ZIP file and extract it.

---

### Step 4: Install Dependencies

#### Frontend Dependencies

Install all Node.js packages:

```bash
npm install
```

This will install:
- **Core Framework**: React 18.2, TypeScript 5.6
- **State Management**: Zustand 4.5
- **UI Components**: Framer Motion, Lucide React
- **Styling**: TailwindCSS 4.1, Tailwind Animate
- **Tauri Plugins**: Dialog, Filesystem, Notifications, Clipboard, Shell
- **Utilities**: React Query, React Router, Date-fns, Recharts
- **Build Tools**: Vite 6.0, TypeScript compiler

#### Rust Dependencies

Rust dependencies are managed by Cargo and will be automatically downloaded during the build process. The project uses:

**Core Libraries:**
- `tauri` (v2) - Application framework
- `tokio` - Async runtime with full features
- `serde` & `serde_json` - Serialization
- `reqwest` - HTTP client with streaming, gzip, and proxy support
- `sqlx` - SQLite database with async support

**Download Features:**
- `futures-util` - Async utilities
- `governor` - Rate limiting/speed control
- `sha2`, `md-5` - Checksum verification
- `crc32fast` - CRC32 checksums

**System Integration:**
- `arboard` - Clipboard access
- `notify-rust` - System notifications
- `dirs` - Standard directory paths
- `opener` - File opening

**Tauri Plugins:**
- `tauri-plugin-shell` - Shell command execution
- `tauri-plugin-dialog` - Native dialogs
- `tauri-plugin-fs` - Filesystem operations
- `tauri-plugin-notification` - System notifications
- `tauri-plugin-clipboard-manager` - Clipboard monitoring
- `tauri-plugin-autostart` - Auto-start on boot
- `tauri-plugin-single-instance` - Single instance enforcement

---

### Step 5: Run the Application

#### Development Mode

Start the development server with hot-reload:

```bash
npm run tauri dev
```

This will:
1. Start the Vite dev server on `http://localhost:1420`
2. Compile the Rust backend
3. Launch the application window
4. Enable hot-reload for frontend changes

**First Run Note:** The initial compilation may take 5-10 minutes as Cargo downloads and compiles all dependencies. Subsequent runs will be much faster.

#### Production Build

See [Building for Production](#building-for-production) section below.

---

## Platform-Specific Requirements

### Windows

**Build Tools:**
```powershell
# Install Visual Studio Build Tools 2019 or later
# Download from: https://visualstudio.microsoft.com/downloads/

# Or install via winget:
winget install Microsoft.VisualStudio.2022.BuildTools
```

**WebView2 Runtime:**
- Pre-installed on Windows 11
- For Windows 10: [Download WebView2](https://go.microsoft.com/fwlink/p/?LinkId=2124703)

### macOS

**Xcode Command Line Tools:**
```bash
xcode-select --install
```

**Rosetta 2 (Apple Silicon Macs):**
```bash
softwareupdate --install-rosetta
```

### Linux

**Ubuntu/Debian Dependencies:**
```bash
sudo apt install libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  file \
  libssl-dev \
  libgtk-3-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev
```

**Fedora Dependencies:**
```bash
sudo dnf install webkit2gtk4.1-devel \
  openssl-devel \
  curl \
  wget \
  file \
  libappindicator-gtk3-devel \
  librsvg2-devel
sudo dnf group install "C Development Tools and Libraries"
```

**Arch Linux Dependencies:**
```bash
sudo pacman -S webkit2gtk \
  base-devel \
  curl \
  wget \
  file \
  openssl \
  gtk3 \
  libappindicator-gtk3 \
  librsvg
```

---

## Quick Start (After Prerequisites)

If you've already installed Rust and Node.js following the guide above:

```bash
# 1. Clone the repository
git clone <your-repository-url>
cd AFK-Dunld

# 2. Install Node.js dependencies
npm install

# 3. Run in development mode
npm run tauri dev
```

**First Run Note:** The initial compilation may take 5-10 minutes as Cargo downloads and compiles all Rust dependencies. Subsequent runs will be much faster.

---

## Development

### Available Commands

```bash
# Start development server with hot-reload
npm run tauri dev

# Build frontend only
npm run build

# Preview production build
npm run preview

# Run Tauri CLI directly
npm run tauri -- <command>
```

### Development Workflow

1. **Frontend Development**
   - Edit files in `src/` directory
   - Changes auto-reload in development mode
   - TypeScript compilation errors appear in terminal

2. **Backend Development**
   - Edit Rust files in `src-tauri/src/`
   - Save to trigger recompilation
   - Check console for compilation errors

3. **Debugging**
   - Open DevTools: Right-click → Inspect Element (or F12)
   - Backend logs appear in terminal
   - Frontend logs in browser DevTools console

### Quick Start Guide

Once the app is running, try these features:

1. **Add a download** - Click the "Add Download" button or use the plus icon
2. **Test keyboard shortcuts**:
   - Add multiple downloads
   - Press **P** to pause them
   - Press **R** to resume
   - Press **Ctrl/Cmd + A** to select all
3. **Try multi-select**:
   - Hold Shift and click to select a range
   - Hold Ctrl/Cmd and click to toggle individual items
   - Use the bulk actions toolbar that appears
4. **View shortcuts** - Click the keyboard icon (⌨️) in the header
5. **Watch status indicators** - Notice the orange color when downloads are paused

### Environment & Configuration

The application uses sensible defaults and doesn't require environment variables for basic operation. All configuration is done through the settings UI after first launch.

### Database Setup

The SQLite database is **automatically created** on first run at:
- **Windows**: `%APPDATA%\com.ankit.afk-dunld\`
- **macOS**: `~/Library/Application Support/com.ankit.afk-dunld/`
- **Linux**: `~/.local/share/com.ankit.afk-dunld/`

No manual database setup is required. The application will:
1. Initialize the database with schema
2. Create default download directory
3. Set up default settings

### Rebuild Scripts

For clean builds, use the provided rebuild scripts:

**Windows:**
```powershell
.\rebuild.bat
```

**macOS/Linux:**
```bash
chmod +x rebuild.sh
./rebuild.sh
```

These scripts will:
- Clean build cache and temporary files
- Rebuild the frontend
- Rebuild the Rust backend in release mode

### Code Structure

**Frontend (React + TypeScript):**
- `src/components/` - UI components (dialogs, downloads, layout, settings)
- `src/hooks/` - Custom React hooks (clipboard, downloads, keyboard shortcuts)
- `src/stores/` - Zustand state management (downloads, queue, settings, UI)
- `src/services/` - API layer (Tauri communication)
- `src/types/` - TypeScript definitions
- `src/styles/` - CSS and theme files

**Backend (Rust):**
- `src-tauri/src/commands/` - Tauri commands (API endpoints)
- `src-tauri/src/core/` - Download engine (chunking, merging, resume, retry)
- `src-tauri/src/network/` - Network protocols (HTTP, FTP, Torrent, YouTube)
- `src-tauri/src/database/` - SQLite integration
- `src-tauri/src/services/` - Background services (clipboard, notifications, tray)
- `src-tauri/src/state/` - Application state management
- `src-tauri/src/utils/` - Utility functions

---

## Building for Production

### Build Application

```bash
npm run tauri build
```

This creates platform-specific installers in `src-tauri/target/release/bundle/`:
- **Windows**: `.msi` and `.exe` installers
- **macOS**: `.dmg` and `.app` bundles
- **Linux**: `.deb`, `.AppImage`, and `.rpm` packages

### Build Only Frontend

```bash
npm run build
```

Output will be in the `dist/` directory.

## Project Structure

```
AFK-Dunld/
│
├── src/                          # Frontend (React + TypeScript)
│   ├── main.tsx                  # React entry point
│   ├── App.tsx                   # Root component
│   ├── components/               # UI Components
│   ├── hooks/                    # Custom React hooks
│   ├── stores/                   # Zustand state stores
│   ├── services/                 # API and service layer
│   ├── types/                    # TypeScript type definitions
│   ├── styles/                   # CSS and Tailwind styles
│   └── assets/                   # Static assets (icons, images)
│
├── src-tauri/                    # Rust Backend
│   ├── Cargo.toml                # Rust dependencies
│   ├── tauri.conf.json           # Tauri configuration
│   ├── icons/                    # Application icons
│   └── src/                      # Rust source code
│       ├── main.rs               # Application entry point
│       └── lib.rs                # Library root
│
├── browser-extension/            # Browser Extensions
│   ├── chrome/                   # Chrome extension files
│   │   ├── manifest.json
│   │   ├── background.js
│   │   ├── content.js
│   │   ├── popup.html
│   │   └── popup.js
│   └── firefox/                  # Firefox extension files
│       ├── manifest.json
│       ├── background.js
│       ├── content.js
│       ├── popup.html
│       └── popup.js
│
├── dist/                         # Built frontend files
├── node_modules/                 # Node.js dependencies
├── package.json                  # Node.js project configuration
├── vite.config.ts                # Vite build configuration
├── tailwind.config.js            # TailwindCSS configuration
├── tsconfig.json                 # TypeScript configuration
├── .gitignore                    # Git ignore rules
├── install-code.md               # Quick install reference
└── README.md                     # This file
```

## Tech Stack

### Frontend
- **Framework**: React 18.2 with TypeScript 5.6
- **Build Tool**: Vite 6.0 with HMR (Hot Module Replacement)
- **Styling**: TailwindCSS 4.1 with PostCSS and Autoprefixer
- **State Management**: Zustand 4.5 (lightweight, performant state)
- **Data Fetching**: TanStack Query (React Query) 5.8
- **Routing**: React Router DOM 6.20
- **Animations**: Framer Motion 10.16 (smooth, declarative animations)
- **Icons**: Lucide React 0.294 (modern icon library)
- **Charts**: Recharts 2.10 (download speed graphs)
- **Notifications**: React Hot Toast 2.4
- **Utilities**: 
  - clsx & tailwind-merge (className management)
  - date-fns 2.30 (date formatting)

### Backend (Rust)
- **Framework**: Tauri 2.0 (secure, lightweight desktop framework)
- **Async Runtime**: Tokio 1.49 with full features
- **HTTP Client**: Reqwest 0.13 (streaming, gzip, SOCKS proxy support)
- **Database**: SQLx 0.8 with SQLite + Chrono integration
- **Serialization**: Serde 1.0 + Serde JSON
- **Error Handling**: 
  - thiserror 2.0 (custom error types)
  - anyhow 1.0 (flexible error handling)
- **Cryptography**: 
  - SHA2 0.10 (SHA-256 checksums)
  - MD5 0.10 (MD5 checksums)
  - CRC32Fast 1.3 (fast CRC32)
- **System Integration**:
  - arboard 3.6 (clipboard access)
  - notify-rust 4.12 (desktop notifications)
  - dirs 6.0 (platform-specific directories)
  - opener 0.7 (open files with default apps)
- **Concurrency**: 
  - futures-util 0.3 (async stream utilities)
  - governor 0.10 (rate limiting for speed control)
  - parking_lot 0.12 (efficient sync primitives)
  - tokio-util 0.7 (Tokio utilities)
- **Utilities**: 
  - regex 1.12 (URL parsing, pattern matching)
  - humansize 2.1 (human-readable file sizes)
  - uuid 1.20 (unique identifiers)
  - chrono 0.4 (date/time handling)
  - tracing 0.1 & tracing-subscriber 0.3 (logging)

### Tauri Plugins (Official v2)
- **tauri-plugin-shell** - Execute shell commands securely
- **tauri-plugin-dialog** - Native file/message dialogs
- **tauri-plugin-fs** - Filesystem operations with permissions
- **tauri-plugin-notification** - System notifications
- **tauri-plugin-clipboard-manager** - Clipboard monitoring & access
- **tauri-plugin-autostart** - Auto-launch on system startup
- **tauri-plugin-single-instance** - Prevent multiple app instances
- **tauri-plugin-opener** - Open files/URLs with default applications

## Browser Extension

The project includes browser extensions for Chrome and Firefox that integrate with the desktop application.

### Installing Extensions

**Chrome:**
1. Open Chrome and navigate to `chrome://extensions/`
2. Enable "Developer mode"
3. Click "Load unpacked"
4. Select the `browser-extension/chrome/` folder

**Firefox:**
1. Open Firefox and navigate to `about:debugging`
2. Click "This Firefox"
3. Click "Load Temporary Add-on"
4. Select the `manifest.json` from `browser-extension/firefox/`

## Configuration

### Tauri Configuration

Edit `src-tauri/tauri.conf.json` to modify:
- Window size and behavior
- Application metadata
- Security policies
- Bundle settings

### Frontend Configuration

- **Vite**: `vite.config.ts`
- **TypeScript**: `tsconfig.json`
- **Tailwind**: `tailwind.config.js`

## Features

### Core Download Features
- ✅ **Multi-protocol support**: HTTP/HTTPS, FTP, Torrent, YouTube downloads
- ✅ **Pause and resume**: Full resume support with chunked downloads
- ✅ **Speed limiting**: Per-download and global speed limits with real-time adjustment
- ✅ **Queue management**: Organize downloads with categories and priorities
- ✅ **Batch downloads**: Add multiple URLs at once from clipboard or text file
- ✅ **File integrity**: MD5, SHA256, and CRC32 checksum verification
- ✅ **Retry logic**: Automatic retry with exponential backoff for failed downloads
- ✅ **Segmented downloading**: Split large files into chunks for faster downloads

### User Interface
- 🎨 **Modern UI**: Clean, responsive interface with dark/light themes
- ⌨️ **Keyboard shortcuts**: Full keyboard navigation and control (Ctrl+N, P, R, Delete, etc.)
- 🖱️ **Bulk operations**: Multi-select with Shift/Ctrl click and batch actions
- 🎯 **Visual indicators**: Color-coded status (green=active, orange=paused, red=error)
- ✨ **Smooth animations**: Framer Motion powered transitions and effects
- 📊 **Progress tracking**: Real-time speed graphs and detailed progress bars
- 🔍 **Download details**: Detailed view with file info, segments, and download history
- 🗂️ **Category management**: Organize downloads into custom categories

### System Integration
- 📋 **Clipboard monitoring**: Automatically detect and capture download URLs
- 🌐 **Browser extensions**: Chrome and Firefox integration for seamless downloading
- 🔔 **System tray**: Minimize to tray with quick actions and status
- 🔔 **Notifications**: Desktop notifications for completed/failed downloads
- 🚀 **Auto-start**: Option to launch on system boot
- 🔒 **Single instance**: Prevents multiple app instances from running

### Advanced Features
- ⚡ **Segmented downloading**: Multi-threaded downloads for maximum speed
- 🔄 **Connection management**: Automatic connection pooling and retry
- 🌐 **Proxy support**: HTTP and SOCKS proxy configuration
- 📹 **YouTube downloads**: Video downloads with quality/format selection
- 🔍 **Search and filter**: Quick find and filter downloads by status/category
- 💾 **Export/Import**: Backup and restore download lists and settings
- 🎯 **Smart queue**: Automatic queue management based on priorities
- 📁 **Custom paths**: Per-category and per-download destination folders

## User Interface

### Keyboard Shortcuts

AFK-Dunld includes powerful keyboard shortcuts for efficient download management:

| Shortcut | Action | Category |
|----------|--------|----------|
| **Ctrl/Cmd + N** | Open new download dialog | General |
| **Ctrl/Cmd + S** | Open settings | General |
| **Ctrl/Cmd + F** | Focus search box | General |
| **1-8** | Navigate between tabs (All, Missing, Active, Completed, YouTube, Torrent, Video, Music) | Navigation |
| **P** | Pause selected/active downloads | Downloads |
| **R** | Resume selected/paused downloads | Downloads |
| **Delete** | Remove selected downloads (with confirmation) | Downloads |
| **Ctrl/Cmd + A** | Select all downloads | Selection |
| **Shift + Click** | Select range of downloads | Selection |
| **Ctrl/Cmd + Click** | Toggle individual selection | Selection |
| **Esc** | Clear selection | Selection |

**Tip:** Press the keyboard icon (⌨️) in the header to view all shortcuts anytime!

### Download Status Indicators

Downloads are visually distinguished by color-coded status indicators:

- 🔵 **Blue** - Downloading (active)
- ⏸️ **Orange** - Paused
- ✅ **Green** - Completed
- 🔴 **Red** - Failed/Error
- ⏳ **Gray** - Queued/Waiting

### Multi-Select & Bulk Operations

Select multiple downloads to perform bulk actions:

1. **Select downloads** using:
   - Click checkboxes
   - Shift + Click for range selection
   - Ctrl/Cmd + Click for individual toggle
   - Ctrl/Cmd + A to select all

2. **Bulk actions toolbar** appears when items are selected:
   - Pause all selected
   - Resume all selected
   - Remove all selected
   - Clear selection

### UI Features

- **Animated buttons** with hover effects and glow
- **Smooth transitions** for status changes
- **Progress visualization** with real-time speed graphs
- **Modal animations** with backdrop blur
- **Responsive design** that adapts to window size
- **Custom scrollbars** for better aesthetics
- **Glass morphism effects** on cards and dialogs

## Troubleshooting

### Common Issues

#### 1. **Rust Compilation Errors**

**Problem**: Build fails with Rust compiler errors.

**Solutions**:
```bash
# Update Rust to the latest stable version
rustup update stable

# Update all Rust toolchains
rustup update

# Verify Rust installation
rustc --version
cargo --version
```

**Platform-specific**:
- **Windows**: Ensure Visual Studio Build Tools are installed
- **Linux**: Install all system dependencies (see [Platform-Specific Requirements](#platform-specific-requirements))
- **macOS**: Install Xcode Command Line Tools

#### 2. **Node.js/npm Issues**

**Problem**: `npm install` fails or shows dependency conflicts.

**Solutions**:
```bash
# Clear npm cache
npm cache clean --force

# Delete node_modules and lock file
rm -rf node_modules package-lock.json  # macOS/Linux
# OR
rmdir /s /q node_modules && del package-lock.json  # Windows

# Reinstall dependencies
npm install

# If using older Node.js version, use legacy peer deps
npm install --legacy-peer-deps
```

**Verify Node.js version**:
```bash
node --version  # Should be v18 or higher
npm --version
```

#### 3. **Tauri Dev Command Fails**

**Problem**: `npm run tauri dev` fails to start.

**Solutions**:

**Port 1420 already in use**:
```bash
# Find process using port 1420
# Windows:
netstat -ano | findstr :1420

# macOS/Linux:
lsof -i :1420

# Kill the process or change port in vite.config.ts
```

**WebView2 missing (Windows 10)**:
- Download and install [WebView2 Runtime](https://go.microsoft.com/fwlink/p/?LinkId=2124703)

**First run taking too long**:
- This is normal! First compilation can take 5-10 minutes
- Subsequent runs will be much faster (< 1 minute)

#### 4. **SQLite Database Errors**

**Problem**: Database initialization or migration fails.

**Solutions**:
```bash
# Delete the database and let it recreate
# Windows:
del %APPDATA%\com.ankit.afk-dunld\downloads.db

# macOS:
rm ~/Library/Application\ Support/com.ankit.afk-dunld/downloads.db

# Linux:
rm ~/.local/share/com.ankit.afk-dunld/downloads.db
```

Then restart the application.

#### 5. **Build Fails on Linux**

**Problem**: Missing system dependencies.

**Solutions**:

**Ubuntu/Debian**:
```bash
sudo apt update
sudo apt install -y libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  file \
  libssl-dev \
  libgtk-3-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev
```

**Fedora**:
```bash
sudo dnf install webkit2gtk4.1-devel \
  openssl-devel \
  curl \
  wget \
  file \
  libappindicator-gtk3-devel \
  librsvg2-devel
sudo dnf group install "C Development Tools and Libraries"
```

#### 6. **Hot Reload Not Working**

**Problem**: Frontend changes don't update automatically.

**Solutions**:
```bash
# Stop the dev server (Ctrl+C)
# Clean Vite cache
rm -rf node_modules/.vite  # macOS/Linux
rmdir /s /q node_modules\.vite  # Windows

# Restart dev server
npm run tauri dev
```

#### 7. **TypeScript Errors**

**Problem**: TypeScript compilation errors in the terminal.

**Solutions**:
```bash
# Check TypeScript version
npx tsc --version

# Run TypeScript compiler manually
npm run build

# If errors persist, check tsconfig.json is properly configured
```

#### 8. **Application Won't Start After Build**

**Problem**: Production build doesn't launch.

**Solutions**:

**Check bundle location**:
```bash
# Installers are in:
src-tauri/target/release/bundle/

# Direct executable (for testing):
# Windows: src-tauri/target/release/afk-dunld.exe
# macOS: src-tauri/target/release/bundle/macos/AFK-Dunld.app
# Linux: src-tauri/target/release/afk-dunld
```

**Clean build**:
```bash
# Use the rebuild script
./rebuild.sh  # macOS/Linux
.\rebuild.bat  # Windows
```

#### 9. **Downloads Not Starting**

**Problem**: Downloads get added but don't start.

**Check**:
- Download directory exists and has write permissions
- Internet connection is active
- Firewall/antivirus isn't blocking the application
- Check application logs in terminal for error messages

#### 10. **Browser Extension Not Working**

**Problem**: Extension doesn't communicate with desktop app.

**Solutions**:
- Ensure the desktop app is running
- Reload the extension in browser
- Check browser console for errors (F12 → Console)
- Verify extension has necessary permissions

### Performance Optimization

**Slow downloads**:
- Check speed limit settings (Settings → Network)
- Increase number of connections per download
- Disable antivirus scanning for download directory temporarily

**High memory usage**:
- Reduce concurrent downloads
- Lower number of segments per download
- Close download details panel when not needed

**UI feels sluggish**:
- Disable animations in settings (if available)
- Reduce number of displayed downloads (use filters)
- Close unnecessary background applications

### Debug Mode

Enable detailed logging for troubleshooting:

**Check application logs**:
- **Windows**: `%APPDATA%\com.ankit.afk-dunld\logs\`
- **macOS**: `~/Library/Logs/com.ankit.afk-dunld/`
- **Linux**: `~/.local/share/com.ankit.afk-dunld/logs/`

**Backend logs**: Available in terminal when running `npm run tauri dev`

**Frontend logs**: Available in DevTools console (F12 → Console)

### Getting Help

If you're still experiencing issues:

1. **Check existing issues**: Search [GitHub Issues](https://github.com/your-repo/issues)
2. **Tauri Documentation**: [https://tauri.app](https://tauri.app)
3. **Rust Documentation**: [https://doc.rust-lang.org](https://doc.rust-lang.org)
4. **Create an issue**: Include:
   - Operating system and version
   - Node.js and Rust versions
   - Error messages and logs
   - Steps to reproduce the issue

### Clean Reinstall

If all else fails, perform a clean reinstall:

```bash
# 1. Remove all build artifacts
rm -rf node_modules dist src-tauri/target

# 2. Remove application data
# Windows: Delete %APPDATA%\com.ankit.afk-dunld
# macOS: Delete ~/Library/Application Support/com.ankit.afk-dunld
# Linux: Delete ~/.local/share/com.ankit.afk-dunld

# 3. Reinstall dependencies
npm install

# 4. Rebuild
npm run tauri dev
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Tauri](https://tauri.app)
- Frontend powered by [React](https://reactjs.org)
- Styled with [TailwindCSS](https://tailwindcss.com)
