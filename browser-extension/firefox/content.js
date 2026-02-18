// AFK-Dunld Browser Extension - Content Script (Firefox)
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
    const isDownloadLink = downloadExtensions.some(ext => 
      href.includes(`.${ext}`) || href.includes(`download`)
    );
    
    if (isDownloadLink && !link.hasAttribute('data-afkdunld')) {
      link.setAttribute('data-afkdunld', 'detected');
      downloadLinkCount++;
    }
  });
  
  return downloadLinkCount;
}

// Add custom context menu hint for download links
document.addEventListener('contextmenu', (e) => {
  const target = e.target.closest('a[href], video, audio, img');
  if (target) {
    const url = target.href || target.src || target.currentSrc;
    if (url) {
      // Store the URL for context menu action
      sessionStorage.setItem('afkdunld_context_url', url);
    }
  }
});

// Listen for click events on download links
document.addEventListener('click', async (e) => {
  // Check if Ctrl/Cmd + Shift + Click (send to download manager)
  if ((e.ctrlKey || e.metaKey) && e.shiftKey) {
    const target = e.target.closest('a[href]');
    if (target && target.hasAttribute('data-afkdunld')) {
      e.preventDefault();
      
      // Send to extension background
      browser.runtime.sendMessage({
        type: 'send_download',
        url: target.href,
        referrer: document.location.href,
        filename: target.download || target.textContent.trim()
      });
      
      // Visual feedback
      const originalText = target.textContent;
      target.textContent = '✓ Sent to AFK-Dunld';
      target.style.color = '#4CAF50';
      
      setTimeout(() => {
        target.textContent = originalText;
        target.style.color = '';
      }, 2000);
    }
  }
}, true);

// Detect YouTube videos
function detectYouTubeVideo() {
  if (window.location.hostname.includes('youtube.com') || 
      window.location.hostname.includes('youtu.be')) {
    
    const videoId = new URLSearchParams(window.location.search).get('v') ||
                    window.location.pathname.split('/').pop();
    
    if (videoId) {
      // Add download button to YouTube player
      addYouTubeDownloadButton(videoId);
    }
  }
}

// Add download button to YouTube
function addYouTubeDownloadButton(videoId) {
  // Check if button already exists
  if (document.querySelector('.afkdunld-yt-button')) {
    return;
  }
  
  // Wait for YouTube UI to load
  const checkInterval = setInterval(() => {
    const controls = document.querySelector('.ytp-right-controls');
    if (controls) {
      clearInterval(checkInterval);
      
      const button = document.createElement('button');
      button.className = 'afkdunld-yt-button ytp-button';
      button.innerHTML = '↓ AFK-Dunld';
      button.title = 'Download with AFK-Dunld';
      button.style.cssText = `
        font-size: 14px;
        padding: 0 8px;
        margin: 0 4px;
        cursor: pointer;
        background: transparent;
        border: none;
        color: white;
      `;
      
      button.addEventListener('click', (e) => {
        e.stopPropagation();
        const videoUrl = window.location.href;
        browser.runtime.sendMessage({
          type: 'send_download',
          url: videoUrl,
          referrer: document.location.href,
          filename: document.title
        });
        
        button.innerHTML = '✓ Sent';
        button.style.color = '#4CAF50';
        setTimeout(() => {
          button.innerHTML = '↓ AFK-Dunld';
          button.style.color = 'white';
        }, 2000);
      });
      
      controls.insertBefore(button, controls.firstChild);
    }
  }, 500);
  
  // Stop trying after 10 seconds
  setTimeout(() => clearInterval(checkInterval), 10000);
}

// Initialize on page load
if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', init);
} else {
  init();
}

function init() {
  detectDownloadLinks();
  detectYouTubeVideo();
  
  // Re-detect on dynamic content changes
  const observer = new MutationObserver(() => {
    detectDownloadLinks();
  });
  
  observer.observe(document.body, {
    childList: true,
    subtree: true
  });
}

// Listen for messages from background script
browser.runtime.onMessage.addListener((request, sender, sendResponse) => {
  if (request.type === 'get_page_downloads') {
    const count = detectDownloadLinks();
    sendResponse({ count });
  }
  return true;
});
