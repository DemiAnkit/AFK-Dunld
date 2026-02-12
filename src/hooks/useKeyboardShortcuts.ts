import { useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { useDownloadStore } from '../stores/downloadStore';
import { useUIStore } from '../stores/uiStore';
import toast from 'react-hot-toast';

/**
 * Global keyboard shortcuts hook for download management
 * Supports:
 * - P: Pause selected/active downloads
 * - R: Resume selected/paused downloads
 * - Delete: Remove selected downloads
 * - Ctrl/Cmd + A: Select all
 * - Ctrl/Cmd + N: New download
 * - Ctrl/Cmd + S: Open settings
 * - Ctrl/Cmd + F: Focus search
 * - 1-8: Navigate between tabs
 * - Esc: Clear selection or close dialogs
 */
export function useKeyboardShortcuts() {
  const navigate = useNavigate();
  const { downloads, pauseDownload, resumeDownload, removeDownload } = useDownloadStore();
  const { selectedDownloads, selectAll, clearSelection, setAddDialogOpen } = useUIStore();

  useEffect(() => {
    const handleKeyPress = async (e: KeyboardEvent) => {
      // Ignore if typing in input fields
      const target = e.target as HTMLElement;
      if (
        target.tagName === 'INPUT' ||
        target.tagName === 'TEXTAREA' ||
        target.isContentEditable
      ) {
        return;
      }

      const selectedIds = Array.from(selectedDownloads);
      const selectedDownloadsList = downloads.filter(d => selectedIds.includes(d.id));

      // P key - Pause downloads
      if (e.key.toLowerCase() === 'p' && !e.ctrlKey && !e.metaKey) {
        e.preventDefault();
        
        let downloadsToPause = selectedDownloadsList.filter(d => 
          d.status === 'downloading' || d.status === 'connecting' || d.status === 'queued'
        );
        
        // If nothing selected, pause all active downloads
        if (downloadsToPause.length === 0) {
          downloadsToPause = downloads.filter(d => 
            d.status === 'downloading' || d.status === 'connecting' || d.status === 'queued'
          );
        }

        if (downloadsToPause.length > 0) {
          try {
            await Promise.all(downloadsToPause.map(d => pauseDownload(d.id)));
            toast.success(`Paused ${downloadsToPause.length} download${downloadsToPause.length > 1 ? 's' : ''}`);
          } catch (error) {
            console.error('Failed to pause downloads:', error);
          }
        } else {
          toast('No active downloads to pause', { icon: 'ℹ️' });
        }
      }

      // R key - Resume downloads
      if (e.key.toLowerCase() === 'r' && !e.ctrlKey && !e.metaKey) {
        e.preventDefault();
        
        let downloadsToResume = selectedDownloadsList.filter(d => d.status === 'paused');
        
        // If nothing selected, resume all paused downloads
        if (downloadsToResume.length === 0) {
          downloadsToResume = downloads.filter(d => d.status === 'paused');
        }

        if (downloadsToResume.length > 0) {
          try {
            await Promise.all(downloadsToResume.map(d => resumeDownload(d.id)));
            toast.success(`Resumed ${downloadsToResume.length} download${downloadsToResume.length > 1 ? 's' : ''}`);
          } catch (error) {
            console.error('Failed to resume downloads:', error);
          }
        } else {
          toast('No paused downloads to resume', { icon: 'ℹ️' });
        }
      }

      // Delete key - Remove selected downloads
      if (e.key === 'Delete' && selectedIds.length > 0) {
        e.preventDefault();
        
        if (window.confirm(`Remove ${selectedIds.length} download${selectedIds.length > 1 ? 's' : ''}?`)) {
          try {
            await Promise.all(selectedIds.map(id => removeDownload(id, false)));
            clearSelection();
            toast.success(`Removed ${selectedIds.length} download${selectedIds.length > 1 ? 's' : ''}`);
          } catch (error) {
            console.error('Failed to remove downloads:', error);
          }
        }
      }

      // Ctrl/Cmd + A - Select all
      if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'a') {
        e.preventDefault();
        selectAll(downloads.map(d => d.id));
        toast.success(`Selected ${downloads.length} download${downloads.length > 1 ? 's' : ''}`);
      }

      // Ctrl/Cmd + N - New Download
      if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'n') {
        e.preventDefault();
        setAddDialogOpen(true);
        toast.success('Opening new download dialog');
      }

      // Ctrl/Cmd + S - Settings (prevent browser save dialog)
      if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 's') {
        e.preventDefault();
        navigate('/settings');
        toast.success('Opening settings');
      }

      // Ctrl/Cmd + F - Focus search (if search input exists)
      if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'f') {
        e.preventDefault();
        const searchInput = document.querySelector('input[type="search"]') as HTMLInputElement;
        if (searchInput) {
          searchInput.focus();
          toast.success('Search focused');
        }
      }

      // Number keys 1-8 for tab navigation (without modifier keys)
      if (!e.ctrlKey && !e.metaKey && !e.shiftKey && !e.altKey) {
        const tabRoutes = [
          '/',           // 1 - All
          '/missing',    // 2 - Missing Files
          '/downloading',// 3 - Active
          '/completed',  // 4 - Completed
          '/youtube',    // 5 - YouTube
          '/torrent',    // 6 - Torrent
          '/video',      // 7 - Video
          '/music',      // 8 - Music
        ];

        const num = parseInt(e.key);
        if (num >= 1 && num <= 8) {
          e.preventDefault();
          navigate(tabRoutes[num - 1]);
        }
      }

      // Escape - Clear selection
      if (e.key === 'Escape' && selectedIds.length > 0) {
        e.preventDefault();
        clearSelection();
      }
    };

    window.addEventListener('keydown', handleKeyPress);
    return () => window.removeEventListener('keydown', handleKeyPress);
  }, [downloads, selectedDownloads, pauseDownload, resumeDownload, removeDownload, selectAll, clearSelection, setAddDialogOpen, navigate]);
}
