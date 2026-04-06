// AFK-Dunld Browser Extension - Popup Script (Firefox)

// Initialize popup
async function init() {
  // DOM elements - must be accessed after DOMContentLoaded
  const statusDot = document.getElementById('statusDot');
  const statusText = document.getElementById('statusText');
  const downloadsList = document.getElementById('downloadsList');
  const interceptToggle = document.getElementById('interceptToggle');
  const openAppBtn = document.getElementById('openAppBtn');
  const clearBtn = document.getElementById('clearBtn');

  // Load settings
  const settings = await browser.storage.sync.get({
    interceptDownloads: true,
    sizeThreshold: 1
  });
  
  if (interceptToggle) {
    interceptToggle.checked = settings.interceptDownloads;
  }
  
  // Check connection status
  checkConnection(statusDot, statusText);
  
  // Load recent downloads
  loadDownloads(downloadsList);
  
  // Setup event listeners
  setupEventListeners(interceptToggle, openAppBtn, clearBtn, downloadsList);
}

// Check connection to desktop app
function checkConnection(statusDot, statusText) {
  browser.runtime.sendMessage({ type: 'check_connection' }, (response) => {
    if (response && response.connected) {
      if (statusDot) statusDot.classList.remove('disconnected');
      if (statusText) statusText.textContent = 'Connected to AFK-Dunld';
    } else {
      if (statusDot) statusDot.classList.add('disconnected');
      if (statusText) statusText.textContent = 'Desktop app not running';
    }
  });
}

// Load recent downloads
function loadDownloads(downloadsList) {
  browser.runtime.sendMessage({ type: 'get_downloads' }, (response) => {
    if (response && response.downloads && response.downloads.length > 0) {
      displayDownloads(response.downloads, downloadsList);
    } else {
      if (downloadsList) {
        downloadsList.innerHTML = '<div class="empty-state">No recent downloads</div>';
      }
    }
  });
}

// Display downloads in the list
function displayDownloads(downloads, downloadsList) {
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
function setupEventListeners(interceptToggle, openAppBtn, clearBtn, downloadsList) {
  // Intercept toggle
  if (interceptToggle) {
    interceptToggle.addEventListener('change', async () => {
      await browser.storage.sync.set({
        interceptDownloads: interceptToggle.checked
      });
    });
  }
  
  // Open app button
  if (openAppBtn) {
    openAppBtn.addEventListener('click', () => {
      browser.runtime.sendMessage({ type: 'open_app' });
    });
  }
  
  // Clear button
  if (clearBtn) {
    clearBtn.addEventListener('click', () => {
      browser.runtime.sendMessage({ type: 'clear_downloads' }, () => {
        if (downloadsList) {
          downloadsList.innerHTML = '<div class="empty-state">No recent downloads</div>';
        }
      });
    });
  }
}

// Refresh data periodically
setInterval(() => {
  const statusDot = document.getElementById('statusDot');
  const statusText = document.getElementById('statusText');
  const downloadsList = document.getElementById('downloadsList');
  checkConnection(statusDot, statusText);
  loadDownloads(downloadsList);
}, 5000);

// Initialize when DOM is ready
document.addEventListener('DOMContentLoaded', init);
