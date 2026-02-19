# Browser Extension Installation Instructions

## ✅ Both Chrome and Firefox Extensions Are Complete!

The AFK-Dunld browser extensions are fully implemented and ready to use for both Chrome and Firefox browsers.

---

## 🔵 Chrome/Edge/Brave Installation

### Step 1: Load Extension

1. Open your browser and navigate to:
   - **Chrome:** `chrome://extensions/`
   - **Edge:** `edge://extensions/`
   - **Brave:** `brave://extensions/`

2. Enable **Developer Mode** (toggle in top-right corner)

3. Click **"Load unpacked"** button

4. Navigate to and select the folder:
   ```
   browser-extension/chrome
   ```

5. ✅ Extension is now loaded!

### Step 2: Note Your Extension ID

After loading, you'll see something like:
```
ID: abcdefghijklmnopqrstuvwxyz123456
```

**Important:** Save this ID - you may need it for native messaging setup.

---

## 🦊 Firefox Installation

### Step 1: Load Extension (Temporary)

1. Open Firefox and navigate to:
   ```
   about:debugging#/runtime/this-firefox
   ```

2. Click **"Load Temporary Add-on..."** button

3. Navigate to `browser-extension/firefox` folder

4. Select **ANY file** in the folder (e.g., `manifest.json`)

5. ✅ Extension is now loaded!

**Note:** Temporary add-ons are removed when Firefox restarts. For permanent installation, the extension needs to be signed by Mozilla.

### Step 2: For Permanent Installation (Optional)

To keep the extension permanently:

1. Create a Firefox account at https://addons.mozilla.org
2. Submit the extension for review
3. Once approved, install from AMO

For development/personal use, temporary installation works fine.

---

## 🔗 Native Messaging Setup

### Option 1: Automatic (Recommended)

1. Run AFK-Dunld desktop application
2. Go to **Settings** → **Browser Integration**
3. Click **"Install Browser Support"**
4. Status should change to **"Installed"** ✓

### Option 2: Manual Setup

If automatic installation doesn't work, see the platform-specific instructions below.

#### Windows

1. Update the manifest file:
   ```
   browser-extension/native-messaging-hosts/chrome/com.ankit.afkdunld.json
   ```
   
2. Replace `REPLACE_WITH_EXECUTABLE_PATH` with your AFK-Dunld.exe path

3. Register in registry:
   ```cmd
   reg add "HKCU\Software\Google\Chrome\NativeMessagingHosts\com.ankit.afkdunld" /ve /t REG_SZ /d "C:\path\to\com.ankit.afkdunld.json" /f
   ```

For Firefox:
   ```cmd
   reg add "HKCU\Software\Mozilla\NativeMessagingHosts\com.ankit.afkdunld" /ve /t REG_SZ /d "C:\path\to\com.ankit.afkdunld.json" /f
   ```

#### macOS

1. Update manifest files with executable path

2. Copy manifests:
   ```bash
   # Chrome
   mkdir -p ~/Library/Application\ Support/Google/Chrome/NativeMessagingHosts
   cp browser-extension/native-messaging-hosts/chrome/com.ankit.afkdunld.json \
      ~/Library/Application\ Support/Google/Chrome/NativeMessagingHosts/

   # Firefox
   mkdir -p ~/Library/Application\ Support/Mozilla/NativeMessagingHosts
   cp browser-extension/native-messaging-hosts/firefox/com.ankit.afkdunld.json \
      ~/Library/Application\ Support/Mozilla/NativeMessagingHosts/
   ```

#### Linux

1. Update manifest files with executable path

2. Copy manifests:
   ```bash
   # Chrome
   mkdir -p ~/.config/google-chrome/NativeMessagingHosts
   cp browser-extension/native-messaging-hosts/chrome/com.ankit.afkdunld.json \
      ~/.config/google-chrome/NativeMessagingHosts/

   # Chromium
   mkdir -p ~/.config/chromium/NativeMessagingHosts
   cp browser-extension/native-messaging-hosts/chrome/com.ankit.afkdunld.json \
      ~/.config/chromium/NativeMessagingHosts/

   # Firefox
   mkdir -p ~/.mozilla/native-messaging-hosts
   cp browser-extension/native-messaging-hosts/firefox/com.ankit.afkdunld.json \
      ~/.mozilla/native-messaging-hosts/
   ```

---

## ✅ Verify Installation

### Check Extension Status

1. Click the extension icon in your browser toolbar
2. The popup should display:
   - 🟢 **"Connected to AFK-Dunld"** (if app is running and native messaging works)
   - 🔴 **"Desktop app not running"** (if app isn't running or native messaging not set up)

### Test Basic Functionality

1. **Context Menu Test:**
   - Right-click any link on a webpage
   - You should see **"Download with AFK-Dunld"** option

2. **Download Interception Test:**
   - Enable **"Auto-intercept downloads"** in extension popup
   - Try downloading a file larger than 1MB
   - It should appear in AFK-Dunld instead of browser downloads

3. **YouTube Test:**
   - Visit any YouTube video
   - Look for **"↓ AFK-Dunld"** button in video player controls
   - Click it to send video to download manager

---

## 🔧 Troubleshooting

### Extension not appearing in toolbar
- Check if it's in the extensions menu (puzzle piece icon)
- Pin it to toolbar for easy access

### "Desktop app not running" message
- Ensure AFK-Dunld application is running
- Install native messaging from Settings → Browser Integration
- Try restarting both browser and app

### Context menu not showing
- Refresh the webpage after installing extension
- Check if extension is enabled

### Downloads not being intercepted
- Enable "Auto-intercept downloads" in extension popup
- Check that file size is > 1MB (default threshold)
- Some sites use special download methods that can't be intercepted

### Firefox "This add-on could not be installed"
- Make sure you're loading from `about:debugging` (not about:addons)
- Select any file in the firefox folder, not the folder itself

---

## 📊 Extension Information

### Chrome Extension
- **Manifest Version:** 3 (Latest)
- **API:** chrome.*
- **Background:** Service Worker
- **Files:** 10 total (5 code + 5 icons)

### Firefox Extension
- **Manifest Version:** 2 (Firefox standard)
- **API:** browser.*
- **Background:** Background Script
- **Files:** 10 total (5 code + 5 icons)

### Permissions Used
- `contextMenus` - Right-click menu integration
- `downloads` - Download interception
- `storage` - Save user preferences
- `tabs` - Open protocol URLs
- `notifications` - Show notifications
- `nativeMessaging` - Communicate with desktop app
- `<all_urls>` - Detect download links on all sites

---

## 🔒 Privacy & Security

The extensions:
- ✅ Do NOT collect any personal data
- ✅ Do NOT send data to external servers
- ✅ Only communicate with local AFK-Dunld app
- ✅ All processing happens locally
- ✅ Open source - you can review the code

---

## 📚 Additional Resources

- **Quick Start:** `QUICK_START_BROWSER_EXTENSION.md`
- **Testing Guide:** `browser-extension/TEST_GUIDE.md`
- **Full Documentation:** `browser-extension/README.md`
- **Technical Details:** `PHASE4_IMPLEMENTATION.md`

---

## ✅ Installation Complete!

Once you see the **"Connected to AFK-Dunld"** status in the extension popup, you're all set!

**Enjoy seamless download management from your browser! 🎉**
