// AFK-Dunld Browser Extension - Background Script (Firefox)
// Handles download interception and communication with desktop app

const NATIVE_APP_NAME = 'com.ankit.afkdunld';
const DOWNLOAD_THRESHOLD_MB = 1;
const PROTOCOL_NAME = 'afk-dunld';

// State management
let isAppConnected = false;
let activeDownloads = new Map();

// Initialize extension
browser.runtime.onInstalled.addListener(() => {
  console.log('AFK-Dunld extension installed');
  
  // Create context menu items
  browser.contextMenus.create({
    id: 'download-with-afkdunld',
    title: 'Download with AFK-Dunld',
    contexts: ['link', 'video', 'audio', 'image']
  });
  
  browser.contextMenus.create({
    id: 'download-selection-with-afkdunld',
    title: 'Download with AFK-Dunld',
    contexts: ['selection']
  });
  
  // Check if desktop app is available
  checkAppConnection();
});

// Handle context menu clicks
browser.contextMenus.onClicked.addListener((info, tab) => {
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
browser.downloads.onCreated.addListener(async (downloadItem) => {
  // Skip if URL is our own protocol
  if (downloadItem.url.startsWith(PROTOCOL_NAME + '://')) {
    return;
  }

  // Get user preferences
  const settings = await browser.storage.sync.get({
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
  browser.downloads.cancel(downloadItem.id).then(() => {
    browser.downloads.erase({ id: downloadItem.id });
  });
  
  // Send to desktop app
  sendToDesktopApp(downloadItem.url, downloadItem.referrer, downloadItem.filename);
  
  // Show notification
  browser.notifications.create({
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
  browser.tabs.create({ url: protocolUrl, active: false }).then((tab) => {
    setTimeout(() => {
      if (tab && tab.id) {
        browser.tabs.remove(tab.id);
      }
    }, 1000);
  });
}

// Send via native messaging (secondary)
function sendViaNativeMessaging(url, referrer, filename) {
  browser.runtime.sendNativeMessage(
    NATIVE_APP_NAME,
    {
      type: 'add_download',
      url: url,
      referrer: referrer || '',
      filename: filename || '',
      timestamp: Date.now()
    }
  ).then(
    (response) => {
      isAppConnected = true;
      console.log('Download sent via native messaging:', response);
    },
    (error) => {
      isAppConnected = false;
      console.log('Native messaging not available');
    }
  );
}

// Check if desktop app is connected
function checkAppConnection() {
  browser.runtime.sendNativeMessage(
    NATIVE_APP_NAME,
    { type: 'ping' }
  ).then(
    (response) => {
      isAppConnected = true;
      console.log('Desktop app connected via native messaging:', response);
      broadcastConnectionStatus();
    },
    (error) => {
      // Native messaging failed, but custom protocol might still work
      isAppConnected = true; // Optimistic
      console.log('Native messaging not available, using custom protocol');
      broadcastConnectionStatus();
    }
  );
}

// Broadcast connection status to all tabs
function broadcastConnectionStatus() {
  browser.runtime.sendMessage({
    type: 'connection_status',
    connected: isAppConnected
  }).catch(() => {});
}

// Update extension badge
function updateBadge() {
  const count = activeDownloads.size;
  if (count > 0) {
    browser.browserAction.setBadgeText({ text: count.toString() });
    browser.browserAction.setBadgeBackgroundColor({ color: '#4CAF50' });
  } else {
    browser.browserAction.setBadgeText({ text: '' });
  }
}

// Handle messages from popup or content scripts
browser.runtime.onMessage.addListener((request, sender, sendResponse) => {
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
    browser.tabs.create({ url: `${PROTOCOL_NAME}://open`, active: false }).then((tab) => {
      setTimeout(() => {
        if (tab && tab.id) {
          browser.tabs.remove(tab.id);
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
