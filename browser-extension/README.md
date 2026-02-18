# AFK-Dunld Browser Extensions

Browser extensions for Chrome and Firefox that integrate with the AFK-Dunld desktop download manager.

## Features

- **Download Interception**: Automatically intercept browser downloads and send them to AFK-Dunld
- **Right-Click Menu**: Download any link, image, video, or audio file via context menu
- **YouTube Integration**: Add download button directly to YouTube videos
- **Native Messaging**: Fast, reliable communication with desktop app
- **Custom Protocol Fallback**: Works even without native messaging setup

## Installation

### Chrome Extension

1. Open Chrome and navigate to `chrome://extensions/`
2. Enable "Developer mode" in the top-right corner
3. Click "Load unpacked"
4. Select the `browser-extension/chrome` folder
5. Note the Extension ID (needed for native messaging setup)

### Firefox Extension

1. Open Firefox and navigate to `about:debugging#/runtime/this-firefox`
2. Click "Load Temporary Add-on"
3. Select any file in the `browser-extension/firefox` folder (e.g., `manifest.json`)
4. The extension will be loaded temporarily

For permanent installation, the extension needs to be signed by Mozilla.

## Native Messaging Setup

Native messaging allows the browser extension to communicate directly with the AFK-Dunld desktop app.

### Automatic Setup (Recommended)

The desktop app can automatically install native messaging support:

1. Open AFK-Dunld desktop app
2. Go to Settings → Browser Integration
3. Click "Install Browser Extension Support"

### Manual Setup

#### Windows

1. Build the AFK-Dunld app in release mode
2. Copy the executable path
3. Update the manifest file at:
   - Chrome: `browser-extension/native-messaging-hosts/chrome/com.ankit.afkdunld.json`
   - Firefox: `browser-extension/native-messaging-hosts/firefox/com.ankit.afkdunld.json`
4. Replace `REPLACE_WITH_EXECUTABLE_PATH` with your executable path
5. For Chrome, replace `EXTENSION_ID_HERE` with your extension ID
6. Register in Windows Registry:

```cmd
# For Chrome
reg add "HKCU\Software\Google\Chrome\NativeMessagingHosts\com.ankit.afkdunld" /ve /t REG_SZ /d "C:\path\to\com.ankit.afkdunld.json" /f

# For Firefox
reg add "HKCU\Software\Mozilla\NativeMessagingHosts\com.ankit.afkdunld" /ve /t REG_SZ /d "C:\path\to\com.ankit.afkdunld.json" /f
```

#### macOS

1. Build the AFK-Dunld app
2. Update the manifest files with the executable path
3. Copy manifests to the appropriate locations:

```bash
# Chrome
mkdir -p ~/Library/Application\ Support/Google/Chrome/NativeMessagingHosts
cp browser-extension/native-messaging-hosts/chrome/com.ankit.afkdunld.json ~/Library/Application\ Support/Google/Chrome/NativeMessagingHosts/

# Firefox
mkdir -p ~/Library/Application\ Support/Mozilla/NativeMessagingHosts
cp browser-extension/native-messaging-hosts/firefox/com.ankit.afkdunld.json ~/Library/Application\ Support/Mozilla/NativeMessagingHosts/
```

#### Linux

1. Build the AFK-Dunld app
2. Update the manifest files with the executable path
3. Copy manifests to the appropriate locations:

```bash
# Chrome
mkdir -p ~/.config/google-chrome/NativeMessagingHosts
cp browser-extension/native-messaging-hosts/chrome/com.ankit.afkdunld.json ~/.config/google-chrome/NativeMessagingHosts/

# Chromium
mkdir -p ~/.config/chromium/NativeMessagingHosts
cp browser-extension/native-messaging-hosts/chrome/com.ankit.afkdunld.json ~/.config/chromium/NativeMessagingHosts/

# Firefox
mkdir -p ~/.mozilla/native-messaging-hosts
cp browser-extension/native-messaging-hosts/firefox/com.ankit.afkdunld.json ~/.mozilla/native-messaging-hosts/
```

## Usage

### Auto-Intercept Downloads

1. Click the extension icon in your browser
2. Enable "Auto-intercept downloads"
3. All downloads larger than 1 MB will be sent to AFK-Dunld automatically

### Right-Click Downloads

1. Right-click on any link, image, video, or audio file
2. Select "Download with AFK-Dunld"
3. The download will be added to AFK-Dunld

### YouTube Downloads

1. Navigate to any YouTube video
2. Click the "↓ AFK-Dunld" button in the video player controls
3. The video will be sent to AFK-Dunld for download

### Keyboard Shortcuts

- **Ctrl+Shift+Click** (or **Cmd+Shift+Click** on Mac): Send any download link to AFK-Dunld

## Troubleshooting

### Extension says "Desktop app not running"

- Make sure AFK-Dunld desktop app is running
- Verify native messaging is set up correctly
- Check if the custom protocol handler is registered

### Downloads not being intercepted

- Enable "Auto-intercept downloads" in the extension popup
- Check the size threshold setting (default: 1 MB minimum)
- Some downloads may be blocked by the browser's security policy

### Context menu not appearing

- Refresh the page after installing the extension
- Check browser extension permissions
- Make sure the extension is enabled

## Development

### Building from Source

```bash
# No build step required - extensions are plain JavaScript
# Just load the extension folders directly in your browser
```

### Testing

1. Load the extension in developer mode
2. Make changes to the code
3. Click "Reload" in the extensions page
4. Test functionality

## Privacy

The extension:
- Does NOT collect or transmit any personal data
- Only communicates with the local AFK-Dunld desktop app
- Does NOT send data to external servers
- Only requests necessary permissions for download management

## License

Same license as AFK-Dunld main project.
