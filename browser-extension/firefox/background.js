// AFK-Dunld Browser Extension - Background Script (Firefox)
// Handles download interception and communication with desktop app

const NATIVE_APP_NAME = 'com.ankit.afkdunld';
const DOWNLOAD_THRESHOLD_MB = 1; // Downloads larger than this will be intercepted

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
    // Try to extract URL from selected text
    const selectedText = info.selectionText.trim();
    if (selectedText.match(/^https?:\/\//)) {
      sendToDesktopApp(selectedText, info.pageUrl, tab.title);
    }
  }
});

// Intercept downloads
browser.downloads.onCreated.addListener(async (downloadItem) => {
  // Get user preferences
  const settings = await browser.storage.sync.get({
    interceptDownloads: true,
    sizeThreshold: DOWNLOAD_THRESHOLD_MB
  });
  
  if (!settings.interceptDownloads) {
    return;
  }
  
  // Check if file size exceeds threshold
  const sizeMB = downloadItem.fileSize / (1024 * 1024);
  if (downloadItem.fileSize > 0 && sizeMB < settings.sizeThreshold) {
    return; // Let browser handle small files
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
  // Try native messaging first
  if (isAppConnected) {
    sendViaNativeMessaging(url, referrer, filename);
  } else {
    // Fallback: Try to open custom protocol
    sendViaCustomProtocol(url, referrer, filename);
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

// Send via native messaging
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
      console.log('Download sent via native messaging:', response);
      isAppConnected = true;
    },
    (error) => {
      console.error('Native messaging error:', error);
      isAppConnected = false;
      // Fallback to custom protocol
      sendViaCustomProtocol(url, referrer, filename);
    }
  );
}

// Send via custom protocol (fallback)
function sendViaCustomProtocol(url, referrer, filename) {
  // Encode download info in URL
  const params = new URLSearchParams({
    url: url,
    referrer: referrer || '',
    filename: filename || ''
  });
  
  const protocolUrl = `afkdunld://download?${params.toString()}`;
  
  // Try to open the protocol URL
  browser.tabs.create({ url: protocolUrl, active: false }).then((tab) => {
    // Close the tab after a short delay
    setTimeout(() => {
      if (tab && tab.id) {
        browser.tabs.remove(tab.id);
      }
    }, 500);
  });
}

// Check if desktop app is connected
function checkAppConnection() {
  browser.runtime.sendNativeMessage(
    NATIVE_APP_NAME,
    { type: 'ping' }
  ).then(
    (response) => {
      isAppConnected = true;
      console.log('Desktop app connected:', response);
    },
    (error) => {
      isAppConnected = false;
      console.log('Desktop app not connected via native messaging');
    }
  );
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
    }, 100);
    return true; // Keep channel open for async response
  } else if (request.type === 'send_download') {
    sendToDesktopApp(request.url, request.referrer, request.filename);
    sendResponse({ success: true });
  }
  
  return true;
});

// Periodic connection check
setInterval(() => {
  checkAppConnection();
}, 30000); // Check every 30 seconds
