# AFK-Dunld

A powerful cross-platform download manager built with Tauri, React, and Rust. Supports HTTP/HTTPS, FTP, and Torrent downloads with advanced features like pause/resume, speed limiting, clipboard monitoring, and browser integration.

![Version](https://img.shields.io/badge/version-0.1.0-blue.svg)
![License](https://img.shields.io/badge/license-MIT-green.svg)

## Prerequisites

Before you begin, ensure you have the following installed on your local machine:

### Required Software

1. **Node.js** (v18 or higher)
   - Download from: https://nodejs.org
   - Verify installation: `node --version`

2. **Rust** (latest stable version)
   - Install via rustup: https://rustup.rs
   - Verify installation: `rustc --version`

3. **Git**
   - Download from: https://git-scm.com
   - Verify installation: `git --version`

### System Requirements

- **Windows**: Windows 10 or later
- **macOS**: macOS 10.13 or later
- **Linux**: Ubuntu 18.04+ or equivalent

## Installation

### 1. Clone the Repository

```bash
git clone https://github.com/DemiAnkit/AFK-Dunld.git
cd AFK-Dunld
```

### 2. Install Frontend Dependencies

```bash
npm install
```

This installs all required Node.js packages including:
- React 18 + TypeScript
- Tauri API and plugins
- TailwindCSS for styling
- Zustand for state management
- React Query for data fetching
- Framer Motion for animations
- And more (see `package.json` for full list)

### 3. Install Rust Dependencies

Navigate to the Tauri backend directory and install Rust crates:

```bash
cd src-tauri
cargo build
```

This will download and compile all Rust dependencies including:
- Tauri 2.0 framework
- Tokio (async runtime)
- Reqwest (HTTP client)
- SQLx (SQLite database)
- Various Tauri plugins (dialog, fs, notification, etc.)

### 4. Return to Root Directory

```bash
cd ..
```

## Development

### Run in Development Mode

Start the development server with hot reload:

```bash
npm run tauri dev
```

This command:
1. Starts the Vite dev server for the frontend
2. Compiles and runs the Rust backend
3. Opens the Tauri application window
4. Enables hot reload for both frontend and backend

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

### Available Scripts

- `npm run dev` - Start Vite dev server only (frontend only)
- `npm run build` - Build production frontend
- `npm run preview` - Preview production build
- `npm run tauri dev` - Run full Tauri app in development mode
- `npm run tauri build` - Build production Tauri app

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

### Backend (Rust)
- **Tauri 2.0** - Desktop application framework
- **Tokio** - Async runtime
- **Reqwest** - HTTP/HTTPS client with streaming support
- **SQLx** - Async SQLite database
- **Serde** - JSON serialization
- **Tauri Plugins**: dialog, fs, notification, clipboard, shell

### Frontend (React + TypeScript)
- **React 18** - UI framework
- **TypeScript** - Type safety
- **Vite** - Build tool and dev server
- **TailwindCSS** - Utility-first CSS framework
- **Zustand** - State management
- **TanStack Query** - Server state management
- **Framer Motion** - Animations
- **Lucide React** - Icon library
- **Recharts** - Data visualization

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

- Multi-protocol support (HTTP/HTTPS, FTP, Torrent)
- Pause and resume downloads
- Download speed limiting
- Clipboard monitoring for URLs
- Browser integration via extensions
- Download queue management
- File integrity verification (MD5/SHA256)
- System notifications
- Dark/Light theme support
- Cross-platform (Windows, macOS, Linux)
- **Keyboard shortcuts** for efficient download management
- **Bulk operations** with multi-select support
- **Visual status indicators** with color-coded states
- **Smooth animations** and modern UI effects

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

1. **Rust compilation errors**
   - Ensure you have the latest Rust version: `rustup update`
   - Install required build tools for your platform

2. **Node modules issues**
   - Delete `node_modules/` and `package-lock.json`
   - Run `npm install` again

3. **Tauri dev command fails**
   - Ensure no other process is using port 1420
   - Check that all prerequisites are installed

### Getting Help

- Check the [Tauri documentation](https://tauri.app)
- Review [Rust documentation](https://doc.rust-lang.org)
- Open an issue on GitHub

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
