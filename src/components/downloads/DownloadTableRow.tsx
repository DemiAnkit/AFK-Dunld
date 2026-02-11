// src/components/downloads/DownloadTableRow.tsx
import { Download, useDownloadStore } from "../../stores/downloadStore";
import { formatBytes, formatSpeed } from "../../utils/format";
import { format } from "date-fns";
import { downloadApi } from "../../services/tauriApi";
import toast from "react-hot-toast";
import { useState } from "react";
import { FileDetailsDialog } from "../dialogs/FileDetailsDialog";
import { 
  CheckCircle, 
  Loader2, 
  AlertCircle, 
  Clock,
  XCircle,
  Pause,
  FolderOpen,
  Trash2,
  Info
} from "lucide-react";

interface DownloadTableRowProps {
  download: Download;
}

export function DownloadTableRow({ download }: DownloadTableRowProps) {
  const [isSelected, setIsSelected] = useState(false);
  const [showDetails, setShowDetails] = useState(false);
  const { removeDownload } = useDownloadStore();
  const getFileTypeIcon = (fileName?: string) => {
    if (!fileName) return '📄';
    
    const ext = fileName.split('.').pop()?.toLowerCase();
    if (!ext) return '📄';
    
    if (['mp4', 'avi', 'mkv', 'mov', 'wmv', 'flv', 'webm'].includes(ext)) return '🎥';
    if (['mp3', 'wav', 'flac', 'aac', 'ogg', 'm4a'].includes(ext)) return '🎵';
    if (['jpg', 'jpeg', 'png', 'gif', 'webp', 'svg', 'bmp', 'ico'].includes(ext)) return '🖼️';
    if (['pdf', 'doc', 'docx', 'txt', 'rtf', 'odt'].includes(ext)) return '📄';
    if (['zip', 'rar', '7z', 'tar', 'gz', 'bz2', 'xz'].includes(ext)) return '📦';
    if (['exe', 'msi', 'dmg', 'deb', 'rpm', 'apk'].includes(ext)) return '⚙️';
    if (['iso', 'img'].includes(ext)) return '💿';
    return '📄';
  };

  const handleOpenLocation = async () => {
    try {
      // First try the backend API
      await downloadApi.openFileLocation(download.id);
      toast.success("Opening file location...");
    } catch (error) {
      console.error("Backend command not available, trying fallback:", error);
      
      // Fallback: Try to open using file path directly if available
      if (download.savePath) {
        try {
          // Use Tauri shell plugin as fallback
          const { open } = await import('@tauri-apps/plugin-shell');
          
          // Detect platform and use appropriate command
          const platform = window.navigator.platform.toLowerCase();
          
          if (platform.includes('win')) {
            // Windows
            await open(`explorer /select,"${download.savePath}"`);
          } else if (platform.includes('mac')) {
            // macOS
            await open(`open -R "${download.savePath}"`);
          } else {
            // Linux - open containing folder
            const folderPath = download.savePath.substring(0, download.savePath.lastIndexOf('/'));
            await open(`xdg-open "${folderPath}"`);
          }
          
          toast.success("Opening file location...");
        } catch (fallbackError) {
          console.error("Fallback failed:", fallbackError);
          toast.error("Failed to open file location. Backend command not implemented yet.");
        }
      } else {
        toast.error("File path not available");
      }
    }
  };

  const handleFileClick = () => {
    if (download.status === 'completed') {
      handleOpenLocation();
    }
  };

  const getStatusDisplay = () => {
    switch (download.status) {
      case 'completed':
        return { icon: <CheckCircle className="w-4 h-4 text-green-400" />, text: 'Completed', color: 'text-green-400' };
      case 'downloading':
        return { icon: <Loader2 className="w-4 h-4 text-blue-400 animate-spin" />, text: 'Downloading', color: 'text-blue-400' };
      case 'paused':
        return { icon: <Pause className="w-4 h-4 text-orange-400" />, text: 'Paused', color: 'text-orange-400' };
      case 'failed':
        return { icon: <AlertCircle className="w-4 h-4 text-red-400" />, text: 'Failed', color: 'text-red-400' };
      case 'cancelled':
        return { icon: <XCircle className="w-4 h-4 text-gray-400" />, text: 'Cancelled', color: 'text-gray-400' };
      case 'queued':
        return { icon: <Clock className="w-4 h-4 text-yellow-400" />, text: 'Queued', color: 'text-yellow-400' };
      case 'connecting':
        return { icon: <Loader2 className="w-4 h-4 text-blue-300 animate-spin" />, text: 'Connecting', color: 'text-blue-300' };
      default:
        return { icon: <Clock className="w-4 h-4 text-gray-400" />, text: download.status, color: 'text-gray-400' };
    }
  };

  const formatDateTime = (dateString?: string) => {
    if (!dateString) return { date: '-', time: '-' };
    
    try {
      const date = new Date(dateString);
      if (isNaN(date.getTime())) {
        return { date: '-', time: '-' };
      }
      return {
        date: format(date, 'MMM dd, yyyy'),
        time: format(date, 'HH:mm:ss')
      };
    } catch {
      return { date: '-', time: '-' };
    }
  };

  const status = getStatusDisplay();
  const progress = download.totalSize
    ? (download.downloadedSize / download.totalSize) * 100
    : 0;

  const handleDelete = async () => {
    if (window.confirm(`Delete "${download.fileName || 'this download'}"?`)) {
      try {
        await removeDownload(download.id, false);
      } catch (error) {
        console.error("Failed to delete:", error);
      }
    }
  };

  return (
    <>
      {showDetails && <FileDetailsDialog download={download} onClose={() => setShowDetails(false)} />}
      <div className={`grid grid-cols-12 gap-4 px-4 py-3 border-b border-gray-800/50 hover:bg-gray-900/30 transition-colors group ${isSelected ? 'bg-blue-900/20' : ''}`}>
      {/* Checkbox */}
      <div className="col-span-1 flex items-center">
        <input
          type="checkbox"
          checked={isSelected}
          onChange={(e) => setIsSelected(e.target.checked)}
          className="w-3.5 h-3.5 bg-gray-700 border-gray-600 rounded text-blue-600 focus:ring-blue-500"
        />
      </div>

      {/* File Name */}
      <div className="col-span-4 flex items-center gap-3 min-w-0">
        <button
          onClick={handleOpenLocation}
          className="text-xl flex-shrink-0 hover:scale-110 transition-transform cursor-pointer"
          title="Open file location"
        >
          {getFileTypeIcon(download.fileName)}
        </button>
        <div className="min-w-0 flex-1">
          <p 
            className="text-sm text-white truncate font-medium cursor-pointer hover:text-blue-400 transition-colors"
            onClick={handleFileClick}
            title={download.fileName || download.url || 'Unknown'}
          >
            {download.fileName || download.url?.split('/').pop() || 'Downloading...'}
          </p>
          {download.status === 'downloading' && (
            <div className="flex items-center gap-2 mt-1">
              <div className="flex-1 h-1 bg-gray-800 rounded-full overflow-hidden">
                <div
                  className="h-full bg-blue-500 rounded-full transition-all"
                  style={{ width: `${Math.min(progress, 100)}%` }}
                />
              </div>
              <span className="text-xs text-gray-500">{progress.toFixed(1)}%</span>
            </div>
          )}
        </div>
        {download.status === 'completed' && (
          <button
            onClick={handleOpenLocation}
            className="opacity-0 group-hover:opacity-100 transition-opacity p-1 hover:bg-gray-800 rounded"
            title="Open folder"
          >
            <FolderOpen className="w-4 h-4 text-blue-400" />
          </button>
        )}
      </div>

      {/* Status */}
      <div className="col-span-1 flex items-center gap-2">
        {status.icon}
        <span className={`text-xs ${status.color}`}>{status.text}</span>
      </div>

      {/* Size */}
      <div className="col-span-1 flex items-center">
        <span className="text-sm text-white font-semibold">
          {download.totalSize && download.totalSize > 0 
            ? formatBytes(download.totalSize) 
            : download.downloadedSize && download.downloadedSize > 0 
            ? formatBytes(download.downloadedSize) 
            : '-'}
        </span>
      </div>

      {/* Speed */}
      <div className="col-span-1 flex items-center">
        <span className="text-sm text-blue-400">
          {download.status === 'downloading' && download.speed ? formatSpeed(download.speed) : '-'}
        </span>
      </div>

      {/* Added Date/Time */}
      <div className="col-span-2 flex flex-col">
        <span className="text-xs text-gray-300 font-medium">
          {formatDateTime(download.createdAt).date}
        </span>
        <span className="text-xs text-gray-500">
          {formatDateTime(download.createdAt).time}
        </span>
      </div>

      {/* Actions */}
      <div className="col-span-2 flex items-center gap-2">
        <button
          onClick={handleOpenLocation}
          className="p-1.5 hover:bg-gray-800 rounded transition-colors opacity-0 group-hover:opacity-100"
          title="Open folder"
        >
          <FolderOpen className="w-4 h-4 text-blue-400" />
        </button>
        
        <button
          onClick={() => setShowDetails(true)}
          className="p-1.5 hover:bg-gray-800 rounded transition-colors opacity-0 group-hover:opacity-100"
          title="View details"
        >
          <Info className="w-4 h-4 text-gray-400" />
        </button>
        
        <button
          onClick={handleDelete}
          className="p-1.5 hover:bg-gray-800 rounded transition-colors opacity-0 group-hover:opacity-100"
          title="Delete"
        >
          <Trash2 className="w-4 h-4 text-red-400" />
        </button>
      </div>
    </div>
    </>
  );
}
