# Browser Extension Testing Guide

This guide will help you test the browser extension integration with AFK-Dunld.

## Prerequisites

1. AFK-Dunld desktop app built and running
2. Browser extension loaded in Chrome or Firefox
3. Native messaging support installed (or custom protocol handler registered)

## Test Checklist

### 1. Extension Installation Test

#### Chrome
- [ ] Load extension from `browser-extension/chrome` folder
- [ ] Extension icon appears in toolbar
- [ ] No errors in `chrome://extensions/` page
- [ ] Note the Extension ID for native messaging setup

#### Firefox
- [ ] Load extension from `browser-extension/firefox` folder
- [ ] Extension icon appears in toolbar
- [ ] No errors in `about:debugging` page

### 2. Native Messaging Test

- [ ] Open AFK-Dunld desktop app
- [ ] Go to Settings → Browser Integration
- [ ] Click "Install Browser Support"
- [ ] Verify status shows "Installed"
- [ ] Click extension icon in browser
- [ ] Status should show "Connected to AFK-Dunld" (green dot)

**Alternative Test (Manual):**
If automatic installation doesn't work:

**Windows:**
```cmd
# Check registry entries
reg query "HKCU\Software\Google\Chrome\NativeMessagingHosts\com.ankit.afkdunld"
reg query "HKCU\Software\Mozilla\NativeMessagingHosts\com.ankit.afkdunld"
```

**macOS/Linux:**
```bash
# Check manifest files exist
ls -la ~/Library/Application\ Support/Google/Chrome/NativeMessagingHosts/com.ankit.afkdunld.json  # macOS Chrome
ls -la ~/.config/google-chrome/NativeMessagingHosts/com.ankit.afkdunld.json  # Linux Chrome
ls -la ~/.mozilla/native-messaging-hosts/com.ankit.afkdunld.json  # Linux Firefox
```

### 3. Download Interception Test

- [ ] Enable "Auto-intercept downloads" in extension popup
- [ ] Try to download a file > 1MB from any website
- [ ] Browser download should be cancelled
- [ ] Download should appear in AFK-Dunld app
- [ ] Browser shows notification "Sending to AFK-Dunld"

**Test URLs:**
- `https://speed.hetzner.de/10MB.bin` (10MB test file)
- `https://proof.ovh.net/files/100Mb.dat` (100MB test file)

### 4. Right-Click Context Menu Test

- [ ] Right-click on any link → "Download with AFK-Dunld" appears
- [ ] Click menu item → download appears in AFK-Dunld
- [ ] Right-click on image → context menu appears
- [ ] Right-click on video → context menu appears
- [ ] Right-click on selected URL text → context menu appears

**Test Pages:**
- Any Wikipedia article (has images and links)
- GitHub repository (has download links)
- YouTube video page

### 5. YouTube Integration Test

- [ ] Navigate to any YouTube video
- [ ] "↓ AFK-Dunld" button appears in video player controls
- [ ] Click button → download appears in AFK-Dunld
- [ ] Button shows "✓ Sent" briefly then returns to normal

**Test Videos:**
- `https://www.youtube.com/watch?v=dQw4w9WgXcQ` (Classic test video)

### 6. Keyboard Shortcut Test

- [ ] Navigate to page with download links
- [ ] Hold Ctrl+Shift (or Cmd+Shift on Mac)
- [ ] Click on a download link
- [ ] Download should go to AFK-Dunld instead of browser
- [ ] Link shows "✓ Sent to AFK-Dunld" feedback

### 7. Custom Protocol Fallback Test

This test is for when native messaging is not available.

- [ ] Uninstall native messaging support
- [ ] Try to download using context menu
- [ ] Browser attempts to open `afkdunld://download?url=...`
- [ ] AFK-Dunld app receives the download request
- [ ] Download is added successfully

### 8. Extension Popup Test

- [ ] Click extension icon
- [ ] Popup shows connection status
- [ ] Recent downloads list is visible (if any)
- [ ] Toggle "Auto-intercept downloads" on/off
- [ ] Click "Open AFK-Dunld" → app window appears
- [ ] Click "Clear List" → recent downloads cleared

### 9. Multi-Browser Test

- [ ] Test in Chrome
- [ ] Test in Firefox
- [ ] Test in Edge (uses Chrome extension)
- [ ] Test in Brave (uses Chrome extension)
- [ ] Verify all features work in each browser

### 10. Stability Test

- [ ] Restart AFK-Dunld app while browser is open
- [ ] Extension reconnects automatically
- [ ] Restart browser while app is running
- [ ] Extension connects on startup
- [ ] Send 5+ downloads in quick succession
- [ ] All downloads are queued properly

## Common Issues and Solutions

### Extension shows "Desktop app not running"

**Solutions:**
1. Make sure AFK-Dunld is running
2. Check native messaging is installed (`Settings → Browser Integration`)
3. Try restarting both app and browser
4. Check browser console for errors

### Downloads not being intercepted

**Solutions:**
1. Enable "Auto-intercept downloads" in extension popup
2. Check file size threshold (default: 1MB minimum)
3. Some sites use special download mechanisms that can't be intercepted
4. Try using right-click menu instead

### Context menu not appearing

**Solutions:**
1. Refresh the page after installing extension
2. Check extension is enabled in browser
3. Try right-clicking on different elements (link, image, video)

### Native messaging errors

**Solutions:**
1. Reinstall browser support from Settings
2. Check executable path in manifest file is correct
3. On Windows, verify registry entries exist
4. On macOS/Linux, check file permissions on manifest

### YouTube button not appearing

**Solutions:**
1. Refresh the YouTube page
2. Wait a few seconds for page to fully load
3. Try a different video
4. Check browser console for JavaScript errors

## Debugging

### Chrome DevTools

1. Open extension popup
2. Right-click in popup → "Inspect"
3. Check Console tab for errors
4. Check Network tab for native messaging communication

### Firefox DevTools

1. Navigate to `about:debugging#/runtime/this-firefox`
2. Find the extension
3. Click "Inspect"
4. Check Console tab for errors

### Desktop App Logs

Check the AFK-Dunld app logs for native messaging activity:
- Windows: `%APPDATA%\AFK-Dunld\logs\`
- macOS: `~/Library/Application Support/AFK-Dunld/logs/`
- Linux: `~/.local/share/AFK-Dunld/logs/`

Look for messages like:
- "Native messaging host started"
- "Received message from browser"
- "Download added from browser extension"

## Performance Testing

### Test Scenarios:

1. **Rapid Fire Downloads**
   - Send 10 downloads within 5 seconds
   - Verify all are queued
   - Check for memory leaks

2. **Large File Handling**
   - Test with files > 1GB
   - Verify download starts correctly
   - Check progress updates

3. **Concurrent Downloads**
   - Send downloads from multiple tabs
   - Verify all are handled
   - Check for race conditions

## Security Testing

1. **URL Validation**
   - Try malformed URLs
   - Test with file:// URLs (should be blocked)
   - Test with javascript: URLs (should be blocked)

2. **XSS Prevention**
   - Verify all displayed URLs are escaped
   - Check for injection vulnerabilities
   - Test with special characters in filenames

## Reporting Issues

If you find bugs, please report with:
1. Browser name and version
2. Operating system
3. Steps to reproduce
4. Extension console errors
5. AFK-Dunld app logs
6. Screenshots if applicable

## Success Criteria

Phase 4 is complete when:
- ✅ Browser extensions intercept downloads
- ✅ Right-click integration works
- ✅ Native messaging is stable and reliable
- ✅ All tests in this guide pass
- ✅ No critical bugs found
