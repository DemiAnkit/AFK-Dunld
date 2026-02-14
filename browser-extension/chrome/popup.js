// AFK-Dunld Browser Extension - Popup Script

// DOM elements
const statusDot = document.getElementById('statusDot');
const statusText = document.getElementById('statusText');
const downloadsList = document.getElementById('downloadsList');
const interceptToggle = document.getElementById('interceptToggle');
const openAppBtn = document.getElementById('openAppBtn');
const clearBtn = document.getElementById('clearBtn');

// Initialize popup
async function init() {
  // Load settings
  const settings = await chrome.storage.sync.get({
    interceptDownloads: true,
    sizeThreshold: 1
  });
  
  interceptToggle.checked = settings.interceptDownloads;
  
  // Check connection status
  checkConnection();
  
  // Load recent downloads
  loadDownloads();
  
  // Setup event listeners
  setupEventListeners();
}

// Check connection to desktop app
async function checkConnection() {
  chrome.runtime.sendMessage({ type: 'check_connection' }, (response) => {
    if (response && response.connected) {
      statusDot.classList.remove('disconnected');
      statusText.textContent = 'Connected to AFK-Dunld';
    } else {
      statusDot.classList.add('disconnected');
      statusText.textContent = 'Desktop app not running';
    }
  });
}

// Load recent downloads
async function loadDownloads() {
  chrome.runtime.sendMessage({ type: 'get_downloads' }, (response) => {
    if (response && response.downloads && response.downloads.length > 0) {
      displayDownloads(response.downloads);
    } else {
      downloadsList.innerHTML = '<div class="empty-state">No recent downloads</div>';
    }
  });
}

// Display downloads in the list
function displayDownloads(downloads) {
  // Sort by timestamp (newest first)
  downloads.sort((a, b) => b.timestamp - a.timestamp);
  
  // Take only last 5
  const recentDownloads = downloads.slice(0, 5);
  
  downloadsList.innerHTML = recentDownloads.map(download => {
    const url = download.url.length > 50 
      ? download.url.substring(0, 47) + '...' 
      : download.url;
    
    const time = formatTime(download.timestamp);
    
    return `
      <div class="download-item">
        <div class="download-url" title="${escapeHtml(download.url)}">${escapeHtml(url)}</div>
        <div class="download-time">${time}</div>
      </div>
    `;
  }).join('');
}

// Format timestamp to relative time
function formatTime(timestamp) {
  const now = Date.now();
  const diff = now - timestamp;
  
  const seconds = Math.floor(diff / 1000);
  const minutes = Math.floor(seconds / 60);
  const hours = Math.floor(minutes / 60);
  
  if (seconds < 60) {
    return 'Just now';
  } else if (minutes < 60) {
    return `${minutes} minute${minutes > 1 ? 's' : ''} ago`;
  } else if (hours < 24) {
    return `${hours} hour${hours > 1 ? 's' : ''} ago`;
  } else {
    return new Date(timestamp).toLocaleDateString();
  }
}

// Escape HTML to prevent XSS
function escapeHtml(text) {
  const div = document.createElement('div');
  div.textContent = text;
  return div.innerHTML;
}

// Setup event listeners
function setupEventListeners() {
  // Intercept toggle
  interceptToggle.addEventListener('change', async () => {
    await chrome.storage.sync.set({
      interceptDownloads: interceptToggle.checked
    });
  });
  
  // Open app button
  openAppBtn.addEventListener('click', () => {
    // Try to open the app via custom protocol
    chrome.tabs.create({ 
      url: 'afkdunld://open',
      active: false 
    }, (tab) => {
      // Close the tab after a short delay
      setTimeout(() => {
        if (tab && tab.id) {
          chrome.tabs.remove(tab.id);
        }
      }, 500);
    });
  });
  
  // Clear button
  clearBtn.addEventListener('click', () => {
    chrome.runtime.sendMessage({ type: 'clear_downloads' }, () => {
      downloadsList.innerHTML = '<div class="empty-state">No recent downloads</div>';
    });
  });
}

// Refresh data periodically
setInterval(() => {
  checkConnection();
  loadDownloads();
}, 5000);

// Initialize when popup opens
document.addEventListener('DOMContentLoaded', init);
