// AFK-Dunld Browser Extension - Background Service Worker
// Handles download interception and communication with desktop app

const NATIVE_APP_NAME = 'com.ankit.afkdunld';
const DOWNLOAD_THRESHOLD_MB = 1;
const PROTOCOL_NAME = 'afk-dunld';

// State management
let isAppConnected = false;
let activeDownloads = new Map();
let nativeMessagingPort = null;

// Initialize extension
chrome.runtime.onInstalled.addListener(() => {
  console.log('AFK-Dunld extension installed');
  
  // Create context menu items
  chrome.contextMenus.create({
    id: 'download-with-afkdunld',
    title: 'Download with AFK-Dunld',
    contexts: ['link', 'video', 'audio', 'image']
  });
  
  chrome.contextMenus.create({
    id: 'download-selection-with-afkdunld',
    title: 'Download with AFK-Dunld',
    contexts: ['selection']
  });
  
  // Check if desktop app is available
  checkAppConnection();
});

// Handle context menu clicks
chrome.contextMenus.onClicked.addListener((info, tab) => {
  if (info.menuItemId === 'download-with-afkdunld') {
    const url = info.linkUrl || info.srcUrl;
    if (url) {
      sendToDesktopApp(url, info.pageUrl, tab.title);
    }
  } else if (info.menuItemId === 'download-selection-with-afkdunld') {
    const selectedText = info.selectionText.trim();
    if (selectedText.match(/^https?:\/\//)) {
      sendToDesktopApp(selectedText, info.pageUrl, tab.title);
    }
  }
});

// Intercept downloads
chrome.downloads.onCreated.addListener(async (downloadItem) => {
  // Skip if URL is our own protocol
  if (downloadItem.url.startsWith(PROTOCOL_NAME + '://')) {
    return;
  }

  // Get user preferences
  const settings = await chrome.storage.sync.get({
    interceptDownloads: true,
    sizeThreshold: DOWNLOAD_THRESHOLD_MB
  });
  
  if (!settings.interceptDownloads) {
    return;
  }
  
  // Check if file size exceeds threshold
  const sizeMB = (downloadItem.fileSize || 0) / (1024 * 1024);
  if (downloadItem.fileSize > 0 && sizeMB < settings.sizeThreshold) {
    return;
  }
  
  // Cancel the browser download
  chrome.downloads.cancel(downloadItem.id, () => {
    chrome.downloads.erase({ id: downloadItem.id });
  });
  
  // Send to desktop app
  sendToDesktopApp(downloadItem.url, downloadItem.referrer, downloadItem.filename);
  
  // Show notification
  chrome.notifications.create({
    type: 'basic',
    iconUrl: 'icons/icon48.png',
    title: 'Sending to AFK-Dunld',
    message: `Download: ${downloadItem.filename || 'file'}`
  });
});

// Send download to desktop app
function sendToDesktopApp(url, referrer, filename) {
  // Always try custom protocol first (most reliable)
  sendViaCustomProtocol(url, referrer, filename);
  
  // Also try native messaging if connected
  if (isAppConnected) {
    sendViaNativeMessaging(url, referrer, filename);
  }
  
  // Store in active downloads
  const downloadId = Date.now().toString();
  activeDownloads.set(downloadId, {
    url,
    referrer,
    filename,
    timestamp: Date.now()
  });
  
  // Update badge
  updateBadge();
}

// Send via custom protocol (PRIMARY method)
function sendViaCustomProtocol(url, referrer, filename) {
  const params = new URLSearchParams();
  params.set('url', url);
  if (referrer) params.set('referrer', referrer);
  if (filename) params.set('filename', filename);
  
  const protocolUrl = `${PROTOCOL_NAME}://download?${params.toString()}`;
  
  // Open protocol URL in a hidden tab
  chrome.tabs.create({ url: protocolUrl, active: false }, (tab) => {
    // Close the tab after a short delay
    setTimeout(() => {
      if (tab && tab.id) {
        chrome.tabs.remove(tab.id, () => {
          // Ignore errors (tab may already be closed)
          if (chrome.runtime.lastError) {
            // Tab was already closed by the protocol handler
          }
        });
      }
    }, 1000);
  });
}

// Send via native messaging (secondary)
function sendViaNativeMessaging(url, referrer, filename) {
  try {
    chrome.runtime.sendNativeMessage(
      NATIVE_APP_NAME,
      {
        type: 'add_download',
        url: url,
        referrer: referrer || '',
        filename: filename || '',
        timestamp: Date.now()
      },
      (response) => {
        if (chrome.runtime.lastError) {
          isAppConnected = false;
          nativeMessagingPort = null;
        } else {
          isAppConnected = true;
          console.log('Download sent via native messaging:', response);
        }
      }
    );
  } catch (error) {
    isAppConnected = false;
    nativeMessagingPort = null;
  }
}

// Check if desktop app is connected
function checkAppConnection() {
  // Try custom protocol first - open afk-dunld://ping and see if it works
  // Since we can't directly detect protocol success, we use a heuristic:
  // Try native messaging, if that fails, assume protocol might work
  
  try {
    chrome.runtime.sendNativeMessage(
      NATIVE_APP_NAME,
      { type: 'ping' },
      (response) => {
        if (chrome.runtime.lastError) {
          // Native messaging failed, but custom protocol might still work
          // We'll optimistically assume the app is connected if it's installed
          console.log('Native messaging not available, using custom protocol');
          isAppConnected = true; // Assume connected via protocol
        } else {
          isAppConnected = true;
          console.log('Desktop app connected via native messaging:', response);
        }
        // Notify all tabs of connection status
        broadcastConnectionStatus();
      }
    );
  } catch (error) {
    // Native messaging not available, but custom protocol might work
    isAppConnected = true; // Optimistic - assume protocol works
    broadcastConnectionStatus();
  }
}

// Broadcast connection status to all tabs
function broadcastConnectionStatus() {
  chrome.runtime.sendMessage({
    type: 'connection_status',
    connected: isAppConnected
  }).catch(() => {
    // No listeners, ignore
  });
}

// Update extension badge
function updateBadge() {
  const count = activeDownloads.size;
  if (count > 0) {
    chrome.action.setBadgeText({ text: count.toString() });
    chrome.action.setBadgeBackgroundColor({ color: '#4CAF50' });
  } else {
    chrome.action.setBadgeText({ text: '' });
  }
}

// Handle messages from popup or content scripts
chrome.runtime.onMessage.addListener((request, sender, sendResponse) => {
  if (request.type === 'get_downloads') {
    sendResponse({
      downloads: Array.from(activeDownloads.values()),
      connected: isAppConnected
    });
  } else if (request.type === 'clear_downloads') {
    activeDownloads.clear();
    updateBadge();
    sendResponse({ success: true });
  } else if (request.type === 'check_connection') {
    checkAppConnection();
    setTimeout(() => {
      sendResponse({ connected: isAppConnected });
    }, 200);
    return true; // Keep channel open for async response
  } else if (request.type === 'send_download') {
    sendToDesktopApp(request.url, request.referrer, request.filename);
    sendResponse({ success: true });
  } else if (request.type === 'open_app') {
    // Open the desktop app via custom protocol
    chrome.tabs.create({ url: `${PROTOCOL_NAME}://open`, active: false }, (tab) => {
      setTimeout(() => {
        if (tab && tab.id) {
          chrome.tabs.remove(tab.id);
        }
      }, 1000);
    });
    sendResponse({ success: true });
  }
  
  return true;
});

// Periodic connection check
setInterval(() => {
  checkAppConnection();
}, 30000);
