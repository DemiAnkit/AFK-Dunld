// AFK-Dunld Browser Extension - Content Script
// Runs on all web pages to detect and handle download links

// Detect download links on page
function detectDownloadLinks() {
  const downloadExtensions = [
    // Archives
    'zip', 'rar', '7z', 'tar', 'gz', 'bz2', 'xz',
    // Executables
    'exe', 'msi', 'dmg', 'pkg', 'deb', 'rpm', 'apk',
    // Media
    'mp4', 'mkv', 'avi', 'mov', 'webm', 'mp3', 'flac', 'wav', 'aac',
    // Documents
    'pdf', 'doc', 'docx', 'xls', 'xlsx', 'ppt', 'pptx',
    // Images
    'jpg', 'jpeg', 'png', 'gif', 'webp', 'svg', 'bmp',
    // Disk images
    'iso', 'img', 'bin'
  ];
  
  const links = document.querySelectorAll('a[href]');
  let downloadLinkCount = 0;
  
  links.forEach(link => {
    const href = link.href.toLowerCase();
    // Check if URL ends with a downloadable extension OR contains 'download'
    const hasExtension = downloadExtensions.some(ext => 
      href.endsWith(`.${ext}`) || href.includes(`.${ext}?`) || href.includes(`.${ext}#`)
    );
    const isDownloadLink = hasExtension || href.includes('download');
    
    if (isDownloadLink && !link.hasAttribute('data-afkdunld')) {
      link.setAttribute('data-afkdunld', 'detected');
      downloadLinkCount++;
    }
  });
  
  return downloadLinkCount;
}

// Intercept ALL clicks on download links (not just Ctrl+Shift)
document.addEventListener('click', (e) => {
  const target = e.target.closest('a[href]');
  if (!target) return;
  
  const href = target.href;
  if (!href) return;
  
  // Skip if it's already a protocol URL
  if (href.startsWith('afk-dunld://') || href.startsWith('javascript:') || href.startsWith('#')) {
    return;
  }
  
  // Check if it looks like a download link
  const isDownloadLink = target.hasAttribute('data-afkdunld') ||
    target.hasAttribute('download') ||
    href.match(/\.(zip|rar|7z|tar|gz|exe|msi|dmg|deb|rpm|apk|mp4|mkv|avi|mov|webm|mp3|flac|wav|aac|pdf|doc|docx|xls|xlsx|ppt|pptx|jpg|jpeg|png|gif|webp|svg|bmp|iso|img|bin)(\?|#|$)/i) ||
    href.toLowerCase().includes('download');
  
  if (!isDownloadLink) return;
  
  // Only intercept if Ctrl/Cmd+Shift is held (user intent to use AFK-Dunld)
  if (e.ctrlKey || e.metaKey) {
    e.preventDefault();
    e.stopPropagation();
    
    chrome.runtime.sendMessage({
      type: 'send_download',
      url: href,
      referrer: document.location.href,
      filename: target.download || target.textContent.trim() || extractFilename(href)
    });
    
    // Visual feedback
    const originalText = target.textContent;
    const originalBg = target.style.background;
    target.textContent = '\u2713 Sent to AFK-Dunld';
    target.style.color = '#4CAF50';
    target.style.background = '#e8f5e9';
    
    setTimeout(() => {
      target.textContent = originalText;
      target.style.color = '';
      target.style.background = originalBg;
    }, 2000);
  }
}, true);

// Extract filename from URL
function extractFilename(url) {
  try {
    const urlObj = new URL(url);
    const pathname = urlObj.pathname;
    const filename = pathname.substring(pathname.lastIndexOf('/') + 1);
    return filename || 'unknown';
  } catch {
    return 'unknown';
  }
}

// Add AFK-Dunld indicator to download links
function addDownloadIndicators() {
  const links = document.querySelectorAll('a[data-afkdunld="detected"]');
  links.forEach(link => {
    if (!link.querySelector('.afkdunld-indicator')) {
      const indicator = document.createElement('span');
      indicator.className = 'afkdunld-indicator';
      indicator.textContent = '\u2193';
      indicator.title = 'Ctrl+Click to download with AFK-Dunld';
      indicator.style.cssText = `
        display: inline-block;
        margin-left: 4px;
        font-size: 10px;
        color: #667eea;
        cursor: pointer;
      `;
      link.appendChild(indicator);
    }
  });
}

// Initialize on page load
if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', init);
} else {
  init();
}

function init() {
  detectDownloadLinks();
  addDownloadIndicators();
  
  // Re-detect on dynamic content changes
  const observer = new MutationObserver(() => {
    detectDownloadLinks();
    addDownloadIndicators();
  });
  
  observer.observe(document.body, {
    childList: true,
    subtree: true
  });
}

// Listen for messages from background script
chrome.runtime.onMessage.addListener((request, sender, sendResponse) => {
  if (request.type === 'get_page_downloads') {
    const count = detectDownloadLinks();
    sendResponse({ count });
  }
  return true;
});
