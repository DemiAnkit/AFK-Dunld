# Troubleshooting Guide

Common issues and their solutions for AFK-Dunld.

## Table of Contents
- [Installation Issues](#installation-issues)
- [Download Problems](#download-problems)
- [YouTube/yt-dlp Issues](#youtube-yt-dlp-issues)
- [Performance Issues](#performance-issues)
- [Browser Extension Issues](#browser-extension-issues)
- [Network Problems](#network-problems)
- [Database Issues](#database-issues)
- [Platform-Specific Issues](#platform-specific-issues)
- [Error Messages](#error-messages)

## Installation Issues

### Windows Installation Fails

**Issue:** MSI installer fails or shows errors

**Solutions:**
1. **Run as Administrator**
   - Right-click installer
   - Select "Run as administrator"

2. **Uninstall Previous Version**
   - Settings → Apps → Uninstall old version
   - Reinstall fresh

3. **Check Windows Defender/Antivirus**
   - May block installation
   - Temporarily disable or add exception

4. **Install .NET Runtime** (if required)
   - Download from Microsoft
   - Reinstall AFK-Dunld

### macOS "App is Damaged" Error

**Issue:** "AFK-Dunld is damaged and can't be opened"

**Solutions:**
1. **Allow App from Unidentified Developer**
   ```bash
   xattr -cr /Applications/AFK-Dunld.app
   ```

2. **Open via Right-Click**
   - Right-click app
   - Select "Open"
   - Click "Open" in dialog

3. **Check Security Settings**
   - System Preferences → Security & Privacy
   - Allow apps from: App Store and identified developers

### Linux Permission Denied

**Issue:** Can't run AppImage or install DEB

**Solutions:**
1. **Make AppImage Executable**
   ```bash
   chmod +x afk-dunld_*.AppImage
   ```

2. **Install Missing Dependencies**
   ```bash
   # Ubuntu/Debian
   sudo apt install libwebkit2gtk-4.0-37 libgtk-3-0
   
   # Fedora
   sudo dnf install webkit2gtk3 gtk3
   ```

3. **Use sudo for DEB Install**
   ```bash
   sudo dpkg -i afk-dunld_*.deb
   sudo apt-get install -f  # Fix dependencies
   ```

## Download Problems

### Downloads Fail Immediately

**Issue:** All downloads fail instantly

**Solutions:**
1. **Check Internet Connection**
   - Test other websites
   - Verify network connectivity

2. **Check Firewall/Antivirus**
   - May block downloads
   - Add AFK-Dunld to exceptions

3. **Test Simple HTTP Download**
   - Try downloading a small test file
   - If works, issue is specific to certain URLs

4. **Check Disk Space**
   - Ensure sufficient free space
   - Downloads fail if disk full

### Downloads Start Then Fail

**Issue:** Downloads begin but fail mid-way

**Solutions:**
1. **Retry Download**
   - Right-click → Retry
   - Auto-retry should kick in

2. **Reduce Segments**
   - Settings → Downloads → Segments: 1
   - Some servers don't like multi-threading

3. **Check Server Status**
   - Server may be down or limiting
   - Try again later

4. **Clear Cache**
   - Settings → Advanced → Clear Cache
   - Restart download

### Downloads are Very Slow

**Issue:** Download speed much slower than expected

**Solutions:**
1. **Check Speed Limit Settings**
   - Settings → Network
   - Ensure no limits set

2. **Reduce Concurrent Downloads**
   - Settings → Downloads → Max Concurrent: 1-2
   - Multiple downloads share bandwidth

3. **Disable Other Applications**
   - Close streaming services
   - Close other download managers
   - Close torrent clients

4. **Check Your Internet Speed**
   - Test at speedtest.net
   - AFK-Dunld can't exceed your connection speed

5. **Try Different Time**
   - Server may be busy
   - Peak hours slower

6. **Increase Segments**
   - Settings → Downloads → Segments: 4-8
   - Only if server supports it

### Resume Not Working

**Issue:** Can't resume interrupted downloads

**Solutions:**
1. **Check Server Support**
   - Not all servers support resume
   - HTTP range requests required

2. **Retry with Fresh Download**
   - If resume fails repeatedly
   - Start download from beginning

3. **Check Partial File**
   - May be corrupted
   - Delete `.part` file and restart

4. **Enable Resume in Settings**
   - Settings → Downloads → Auto Resume: On

## YouTube/yt-dlp Issues

### "Unable to Extract Video Information"

**Issue:** YouTube video won't download

**Solutions:**
1. **Update yt-dlp** (Most Common Fix)
   - Settings → YouTube → "Update yt-dlp"
   - YouTube changes frequently
   - Latest yt-dlp has fixes

2. **Check Video URL**
   - Ensure correct URL format
   - Try copying URL again
   - Remove extra parameters

3. **Check Video Availability**
   - Can you watch in browser?
   - Video may be deleted/private

4. **Clear yt-dlp Cache**
   - Settings → Advanced → Clear Cache
   - Restart app

### "Video is Private/Unavailable"

**Issue:** Can't access private or region-locked content

**Solutions:**
1. **Check Video in Browser**
   - Can you watch it while logged in?
   - May require account

2. **Use Cookies** (For Private Content)
   - Export cookies from browser
   - Settings → YouTube → Import Cookies
   - [Cookie export guide](https://github.com/yt-dlp/yt-dlp/wiki/FAQ#how-do-i-pass-cookies-to-yt-dlp)

3. **Use Proxy** (For Geo-Restrictions)
   - Settings → Network → Proxy
   - Set proxy in available country

### yt-dlp Update Fails

**Issue:** Can't update yt-dlp

**Solutions:**
1. **Check Internet Connection**

2. **Manual Update**
   - Download latest yt-dlp
   - Place in: `%APPDATA%\afk-dunld\bin\` (Windows)
   - Place in: `~/Library/Application Support/afk-dunld/bin/` (macOS)
   - Place in: `~/.local/share/afk-dunld/bin/` (Linux)

3. **Run as Administrator/sudo**
   - May need elevated permissions
   - Restart app as admin

4. **Check Firewall**
   - May block update downloads

### Age-Restricted Videos

**Issue:** "Sign in to confirm your age"

**Solutions:**
1. **Use Cookies Method**
   - Export browser cookies
   - Import in AFK-Dunld
   - Bypasses age check

2. **Use Browser Extension**
   - Download via extension
   - Uses your browser session

### Audio/Video Quality Issues

**Issue:** Poor quality or wrong format

**Solutions:**
1. **Select Specific Quality**
   - Don't use "Best"
   - Choose exact quality (1080p, 720p)

2. **Check Available Formats**
   - Click "Get Video Info"
   - See what qualities available
   - Select from available options

3. **Update yt-dlp**
   - Newer version better format selection

## Performance Issues

### High CPU Usage

**Issue:** AFK-Dunld using too much CPU

**Solutions:**
1. **Reduce Concurrent Downloads**
   - Settings → Downloads → Max Concurrent: 2

2. **Reduce Segments**
   - Settings → Downloads → Segments: 4

3. **Close Other Apps**
   - Free up system resources

4. **Update to Latest Version**
   - Performance improvements in updates

### High Memory Usage

**Issue:** Too much RAM usage

**Solutions:**
1. **Reduce Concurrent Downloads**

2. **Clear History**
   - History → Clear Old Downloads
   - Reduces database size

3. **Restart App**
   - Memory leaks cleared on restart

4. **Check for Updates**

### App Freezes/Crashes

**Issue:** Application becomes unresponsive

**Solutions:**
1. **Check Logs**
   - Settings → Advanced → View Logs
   - Look for error messages

2. **Disable Problematic Features**
   - Try disabling browser integration
   - Try disabling clipboard monitoring

3. **Clear Database**
   - Backup first!
   - Settings → Advanced → Reset Database

4. **Reinstall App**
   - Uninstall completely
   - Delete config folder
   - Fresh install

## Browser Extension Issues

### Extension Not Connecting

**Issue:** "Desktop app not running"

**Solutions:**
1. **Ensure AFK-Dunld is Running**
   - App must be open
   - Check system tray

2. **Install Native Messaging**
   - AFK-Dunld → Settings → Browser Integration
   - Click "Install Browser Extension Support"

3. **Restart Browser**
   - Close and reopen browser
   - Extension reconnects

4. **Check Extension Permissions**
   - Browser → Extensions
   - Ensure all permissions granted

### Downloads Not Intercepting

**Issue:** Browser downloads not sent to AFK-Dunld

**Solutions:**
1. **Enable Auto-Intercept**
   - Click extension icon
   - Toggle "Auto-intercept downloads" ON

2. **Check Size Threshold**
   - Extension only intercepts large files
   - Default: >1 MB
   - Adjust in extension settings

3. **Check File Type**
   - Some types may not be intercepted
   - Use right-click menu instead

4. **Reload Page**
   - Refresh page after enabling extension

### Context Menu Not Appearing

**Issue:** No "Download with AFK-Dunld" in right-click menu

**Solutions:**
1. **Refresh Page**
   - Reload page after installing extension

2. **Check Extension is Enabled**
   - Browser → Extensions
   - Ensure AFK-Dunld extension enabled

3. **Reinstall Extension**
   - Remove and reinstall

## Network Problems

### Proxy Connection Fails

**Issue:** Can't connect through proxy

**Solutions:**
1. **Test Proxy Settings**
   - Settings → Network → Test Connection
   - Verify host, port, credentials

2. **Try Different Proxy Type**
   - Switch between HTTP/SOCKS5

3. **Check Proxy Server Status**
   - Test proxy with browser

4. **Disable Proxy Temporarily**
   - Rule out proxy as issue

### SSL/TLS Errors

**Issue:** "SSL certificate verification failed"

**Solutions:**
1. **Update System Certificates**
   - Windows: Windows Update
   - macOS: System Update
   - Linux: `sudo update-ca-certificates`

2. **Temporary Bypass** (Not Recommended for Production)
   - Settings → Network → Ignore SSL Errors
   - Only for trusted sources

3. **Check System Time**
   - Incorrect time causes SSL errors
   - Set time to automatic

### Connection Timeouts

**Issue:** Downloads timeout frequently

**Solutions:**
1. **Increase Timeout**
   - Settings → Network → Timeout: 60 seconds

2. **Check Internet Stability**
   - Test with continuous ping
   - May need better connection

3. **Reduce Segments**
   - Fewer connections = more stable

## Database Issues

### Database Corrupted

**Issue:** "Database error" messages

**Solutions:**
1. **Backup Database**
   - Copy `downloads.db` to safe location

2. **Repair Database**
   - Settings → Advanced → Repair Database

3. **Reset Database** (Last Resort)
   - Settings → Advanced → Reset Database
   - Loses download history
   - Fresh start

### Migration Errors

**Issue:** App won't start after update

**Solutions:**
1. **Check Logs**
   - Look for migration errors

2. **Restore Backup**
   - If you have backup database
   - Replace current with backup

3. **Fresh Install**
   - Backup data
   - Uninstall app
   - Delete config folder
   - Reinstall

## Platform-Specific Issues

### Windows Issues

**Windows Defender Blocks Downloads**
```
Solution: Add exception
Settings → Windows Security → Virus & threat protection
→ Manage settings → Add exclusion
→ Add AFK-Dunld folder
```

**Port Already in Use**
```
Solution: Change port or kill process
Settings → Network → Port: 1421
Or: netstat -ano | findstr :1420
     taskkill /PID [PID] /F
```

### macOS Issues

**"AFK-Dunld can't be opened"**
```bash
# Remove quarantine attribute
xattr -cr /Applications/AFK-Dunld.app

# Allow in System Preferences
System Preferences → Security & Privacy → General
→ Click "Open Anyway"
```

**Permission Denied for Downloads**
```bash
# Give Full Disk Access
System Preferences → Security & Privacy → Privacy
→ Full Disk Access → Add AFK-Dunld
```

### Linux Issues

**Missing System Libraries**
```bash
# Ubuntu/Debian
sudo apt install libwebkit2gtk-4.0-37 libgtk-3-0 libayatana-appindicator3-1

# Fedora
sudo dnf install webkit2gtk3 gtk3

# Arch
sudo pacman -S webkit2gtk gtk3
```

**AppImage Won't Run**
```bash
# Install FUSE
sudo apt install libfuse2  # Ubuntu/Debian
sudo dnf install fuse-libs  # Fedora

# Or extract and run
./afk-dunld_*.AppImage --appimage-extract
./squashfs-root/afk-dunld
```

## Error Messages

### "Failed to create download directory"

**Cause:** No permission to write to folder

**Solution:**
1. Choose different download folder
2. Grant write permissions
3. Run as administrator (not recommended)

### "Insufficient disk space"

**Cause:** Not enough free space

**Solution:**
1. Free up disk space
2. Choose different drive
3. Delete unnecessary files

### "Server does not support resume"

**Cause:** Server doesn't support HTTP range requests

**Solution:**
1. Can't resume this download
2. Restart from beginning
3. Ensure stable connection

### "Network error: Connection refused"

**Cause:** Can't connect to server

**Solution:**
1. Check URL is correct
2. Check internet connection
3. Server may be down
4. Check firewall settings

### "Invalid URL format"

**Cause:** URL is malformed

**Solution:**
1. Check URL syntax
2. Ensure proper protocol (http://, https://)
3. Remove extra characters

## Getting More Help

### Collect Debug Information

1. **Enable Debug Logs**
   - Settings → Advanced → Log Level: Debug
   - Reproduce issue
   - Check logs

2. **Export Logs**
   - Settings → Advanced → Export Logs
   - Attach to bug report

3. **Check System Info**
   - Help → About → System Information
   - Include in bug report

### Report a Bug

Include in your report:
1. **Steps to reproduce**
2. **Expected vs actual behavior**
3. **Error messages**
4. **Log files**
5. **System information**
6. **Screenshots** (if applicable)

### Resources

- 📖 [Documentation](../README.md)
- 🐛 [Issue Tracker](https://github.com/yourusername/afk-dunld/issues)
- 💬 [Discussions](https://github.com/yourusername/afk-dunld/discussions)
- 📧 Email: support@afk-dunld.com

### Known Issues

Check [known issues](https://github.com/yourusername/afk-dunld/issues?q=is%3Aissue+is%3Aopen+label%3A%22known+issue%22) before reporting.

## Still Having Issues?

If your problem isn't listed here:
1. Search [existing issues](https://github.com/yourusername/afk-dunld/issues)
2. Check [discussions](https://github.com/yourusername/afk-dunld/discussions)
3. Create a [new issue](https://github.com/yourusername/afk-dunld/issues/new)

We're here to help! 🚀
