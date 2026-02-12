// src/components/downloads/DownloadTableRow.tsx
import { Download, useDownloadStore } from "../../stores/downloadStore";
import { useUIStore } from "../../stores/uiStore";
import { formatBytes, formatSpeed } from "../../utils/format";
import { format, isValid, parseISO } from "date-fns";
import { downloadApi } from "../../services/tauriApi";
import toast from "react-hot-toast";
import { useState, useEffect } from "react";
import { FileDetailsDialog } from "../dialogs/FileDetailsDialog";
import { 
  CheckCircle, 
  Loader2, 
  AlertCircle, 
  Clock,
  XCircle,
  Pause,
  Play,
  FolderOpen,
  Trash2,
  Info,
  FileWarning,
  RotateCcw
} from "lucide-react";

interface DownloadTableRowProps {
  download: Download;
}

export function DownloadTableRow({ download }: DownloadTableRowProps) {
  const [showDetails, setShowDetails] = useState(false);
  const [fileExists, setFileExists] = useState<boolean | null>(null);
  const [actualFileSize, setActualFileSize] = useState<number | null>(null);
  const { removeDownload, pauseDownload, resumeDownload, retryDownload } = useDownloadStore();
  const { isSelected, toggleSelection } = useUIStore();
  const isRowSelected = isSelected(download.id);

  const handlePause = async (e: React.MouseEvent) => {
    e.stopPropagation();
    try {
      await pauseDownload(download.id);
      toast.success("Download paused");
    } catch (error) {
      console.error("Failed to pause:", error);
      toast.error("Failed to pause download");
    }
  };

  const handleResume = async (e: React.MouseEvent) => {
    e.stopPropagation();
    try {
      await resumeDownload(download.id);
      toast.success("Download resumed");
    } catch (error) {
      console.error("Failed to resume:", error);
      toast.error("Failed to resume download");
    }
  };

  const handleRetry = async (e: React.MouseEvent) => {
    e.stopPropagation();
    try {
      await retryDownload(download.id);
      toast.success("Retrying download...");
    } catch (error) {
      console.error("Failed to retry:", error);
      toast.error("Failed to retry download");
    }
  };

  // Check if file exists on disk (for completed downloads)
  useEffect(() => {
    if (download.status === 'completed' && download.savePath) {
      checkFileExists();
      fetchActualFileSize();
    }
  }, [download.status, download.savePath]);

  const checkFileExists = async () => {
    try {
      // We'll check file existence periodically
      const exists = await downloadApi.checkFileExists(download.id);
      setFileExists(exists);
    } catch (error) {
      // If check fails, assume file exists
      setFileExists(true);
    }
  };

  const fetchActualFileSize = async () => {
    try {
      const size = await downloadApi.getFileSize(download.id);
      setActualFileSize(size);
    } catch (error) {
      console.error("Failed to get file size:", error);
      setActualFileSize(null);
    }
  };

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
      await downloadApi.openFileLocation(download.id);
      toast.success("Opening file location...");
    } catch (error) {
      console.error("Failed to open location:", error);
      
      // Fallback: Try to open using file path directly if available
      if (download.savePath) {
        try {
          const { open } = await import('@tauri-apps/plugin-shell');
          const platform = window.navigator.platform.toLowerCase();
          
          if (platform.includes('win')) {
            await open(`explorer /select,"${download.savePath}"`);
          } else if (platform.includes('mac')) {
            await open(`open -R "${download.savePath}"`);
          } else {
            const folderPath = download.savePath.substring(0, download.savePath.lastIndexOf('/'));
            await open(`xdg-open "${folderPath}"`);
          }
          
          toast.success("Opening file location...");
        } catch (fallbackError) {
          console.error("Fallback failed:", fallbackError);
          toast.error("Failed to open file location");
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
        return { 
          icon: <CheckCircle className="w-4 h-4 text-green-400" />, 
          text: 'Completed', 
          color: 'text-green-400',
          bgColor: 'bg-green-400/10'
        };
      case 'downloading':
        return { 
          icon: <Loader2 className="w-4 h-4 text-blue-400 animate-spin" />, 
          text: 'Downloading', 
          color: 'text-blue-400',
          bgColor: 'bg-blue-400/10'
        };
      case 'paused':
        return { 
          icon: <Pause className="w-4 h-4 text-orange-400" />, 
          text: 'Paused', 
          color: 'text-orange-400',
          bgColor: 'bg-orange-400/10'
        };
      case 'failed':
        return { 
          icon: <AlertCircle className="w-4 h-4 text-red-400" />, 
          text: 'Failed', 
          color: 'text-red-400',
          bgColor: 'bg-red-400/10'
        };
      case 'cancelled':
        return { 
          icon: <XCircle className="w-4 h-4 text-gray-400" />, 
          text: 'Cancelled', 
          color: 'text-gray-400',
          bgColor: 'bg-gray-400/10'
        };
      case 'queued':
        return { 
          icon: <Clock className="w-4 h-4 text-yellow-400" />, 
          text: 'Queued', 
          color: 'text-yellow-400',
          bgColor: 'bg-yellow-400/10'
        };
      case 'connecting':
        return { 
          icon: <Loader2 className="w-4 h-4 text-blue-300 animate-spin" />, 
          text: 'Connecting', 
          color: 'text-blue-300',
          bgColor: 'bg-blue-300/10'
        };
      default:
        return { 
          icon: <Clock className="w-4 h-4 text-gray-400" />, 
          text: download.status, 
          color: 'text-gray-400',
          bgColor: 'bg-gray-400/10'
        };
    }
  };

  // Improved date/time formatting with better error handling
  const formatDateTime = (dateString?: string | Date | null): { date: string; time: string; full: string } => {
    if (!dateString) {
      return { date: '-', time: '-', full: '-' };
    }
    
    try {
      let date: Date;
      
      // Handle different date formats
      if (dateString instanceof Date) {
        date = dateString;
      } else if (typeof dateString === 'string') {
        // Try parsing ISO string first
        date = parseISO(dateString);
        
        // If not valid, try standard Date parsing
        if (!isValid(date)) {
          date = new Date(dateString);
        }
      } else {
        return { date: '-', time: '-', full: '-' };
      }
      
      // Validate the date
      if (!isValid(date) || isNaN(date.getTime())) {
        return { date: '-', time: '-', full: '-' };
      }
      
      return {
        date: format(date, 'MMM dd, yyyy'),
        time: format(date, 'HH:mm:ss'),
        full: format(date, 'MMM dd, yyyy HH:mm:ss')
      };
    } catch (error) {
      console.error('Date formatting error:', error, 'Input:', dateString);
      return { date: '-', time: '-', full: '-' };
    }
  };

  const status = getStatusDisplay();
  const progress = download.totalSize && download.totalSize > 0
    ? (download.downloadedSize / download.totalSize) * 100
    : 0;

  const handleDelete = async () => {
    const fileName = download.fileName || download.url?.split('/').pop() || 'this download';
    if (window.confirm(`Delete "${fileName}"?\n\nThis will remove the download from the list.`)) {
      try {
        await removeDownload(download.id, false);
        toast.success("Download removed from list");
      } catch (error) {
        console.error("Failed to delete:", error);
        toast.error("Failed to remove download");
      }
    }
  };

  const dateTime = formatDateTime(download.createdAt);

  const handleRowClick = (e: React.MouseEvent) => {
    // Don't select if clicking on buttons or interactive elements
    const target = e.target as HTMLElement;
    if (target.closest('button') || target.closest('input[type="checkbox"]')) {
      return;
    }
    
    // Get all download IDs for shift selection (need to pass from parent)
    toggleSelection(download.id, e.shiftKey, e.ctrlKey || e.metaKey);
  };

  const handleCheckboxChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    e.stopPropagation();
    const nativeEvent = e.nativeEvent as MouseEvent;
    toggleSelection(download.id, nativeEvent.shiftKey, nativeEvent.ctrlKey || nativeEvent.metaKey);
  };

  return (
    <>
      {showDetails && <FileDetailsDialog download={download} onClose={() => setShowDetails(false)} />}
      <div 
        onClick={handleRowClick}
        className={`grid grid-cols-12 gap-4 px-4 py-3 border-b border-gray-800/50 
                   hover:bg-gray-800/40 transition-all duration-200 group cursor-pointer
                   ${isRowSelected ? 'bg-blue-900/20 border-l-4 border-l-blue-500' : 'border-l-4 border-l-transparent'}
                   ${download.status === 'downloading' ? 'bg-blue-900/5' : ''}`}
      >
        {/* Checkbox */}
        <div className="col-span-1 flex items-center">
          <input
            type="checkbox"
            checked={isRowSelected}
            onChange={handleCheckboxChange}
            onClick={(e) => e.stopPropagation()}
            className="w-3.5 h-3.5 bg-gray-800 border-gray-600 rounded text-blue-600 
                     focus:ring-blue-500/20 focus:ring-offset-0 cursor-pointer"
          />
        </div>

        {/* File Name - Shows exact filename with extension */}
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
              title={`${download.fileName || 'Unknown'}\nSize: ${
                download.status === 'completed' && actualFileSize !== null
                  ? formatBytes(actualFileSize)
                  : download.totalSize 
                  ? formatBytes(download.totalSize) 
                  : (download.downloadedSize > 0 ? formatBytes(download.downloadedSize) : 'Unknown')
              }\nPath: ${download.savePath || 'Not saved yet'}`}
            >
              {download.fileName || download.url?.split('/').pop() || 'Downloading...'}
            </p>
            {download.fileName && (
              <p className="text-xs text-gray-500 truncate">
                {(() => {
                  const ext = download.fileName.split('.').pop()?.toLowerCase();
                  return ext ? `.${ext.toUpperCase()} file` : 'Unknown type';
                })()}
              </p>
            )}
            
            {/* Progress bar for downloading files */}
            {download.status === 'downloading' && (
              <div className="flex items-center gap-2 mt-1.5">
                <div className="flex-1 h-1.5 bg-gray-800 rounded-full overflow-hidden">
                  <div
                    className="h-full bg-gradient-to-r from-blue-500 to-blue-400 rounded-full transition-all duration-300"
                    style={{ width: `${Math.min(progress, 100)}%` }}
                  />
                </div>
                <span className="text-xs text-gray-500 font-medium min-w-[40px] text-right">
                  {progress.toFixed(1)}%
                </span>
              </div>
            )}
            
            {/* Progress bar with pause indicator for paused files */}
            {download.status === 'paused' && (
              <div className="flex items-center gap-2 mt-1.5">
                <div className="flex-1 h-1.5 bg-gray-800 rounded-full overflow-hidden relative">
                  <div
                    className="h-full bg-gradient-to-r from-orange-500 to-orange-400 rounded-full transition-all duration-300"
                    style={{ width: `${Math.min(progress, 100)}%` }}
                  />
                  {/* Pause marker - shows where download was paused */}
                  <div
                    className="absolute top-0 bottom-0 w-0.5 bg-orange-300 shadow-lg shadow-orange-500/50"
                    style={{ left: `${Math.min(progress, 100)}%` }}
                    title={`Paused at ${progress.toFixed(1)}%`}
                  >
                    <div className="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 w-2 h-2 bg-orange-400 rounded-full border-2 border-gray-950 shadow-lg shadow-orange-500/50" />
                  </div>
                </div>
                <span className="text-xs text-orange-400 font-medium min-w-[40px] text-right" title={`Paused at ${progress.toFixed(1)}%`}>
                  {progress.toFixed(1)}%
                </span>
              </div>
            )}
            
            {/* Warning for missing files */}
            {download.status === 'completed' && fileExists === false && (
              <div className="flex items-center gap-1.5 mt-1 text-xs text-orange-400">
                <FileWarning className="w-3.5 h-3.5" />
                <span>File not found on disk</span>
              </div>
            )}
          </div>
          
          {/* Quick action for completed downloads */}
          {download.status === 'completed' && (
            <button
              onClick={handleOpenLocation}
              className="opacity-0 group-hover:opacity-100 transition-all duration-200 
                       p-1.5 hover:bg-gray-700 rounded-lg"
              title="Open folder"
            >
              <FolderOpen className="w-4 h-4 text-blue-400" />
            </button>
          )}
        </div>

        {/* Status */}
        <div className="col-span-1 flex items-center">
          <span className={`inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full text-xs font-medium
                         ${status.bgColor} ${status.color} border border-opacity-20`}>
            {status.icon}
            <span className="hidden sm:inline">{status.text}</span>
          </span>
        </div>

        {/* Size - Shows exact file size */}
        <div className="col-span-1 flex items-center">
          <span 
            className="text-sm text-gray-300 font-medium tabular-nums"
            title={
              download.status === 'completed' && actualFileSize !== null
                ? `${actualFileSize.toLocaleString()} bytes (actual file size)`
                : download.totalSize && download.totalSize > 0 
                ? `${download.totalSize.toLocaleString()} bytes` 
                : download.downloadedSize > 0 
                ? `${download.downloadedSize.toLocaleString()} bytes (downloaded)` 
                : 'Size unknown'
            }
          >
            {download.status === 'completed' && actualFileSize !== null
              ? formatBytes(actualFileSize)
              : download.totalSize && download.totalSize > 0 
              ? formatBytes(download.totalSize) 
              : download.downloadedSize > 0 
              ? formatBytes(download.downloadedSize) 
              : '-'}
          </span>
        </div>

        {/* Speed */}
        <div className="col-span-1 flex items-center">
          <span className="text-sm text-blue-400 font-medium tabular-nums">
            {download.status === 'downloading' && download.speed && download.speed > 0 
              ? formatSpeed(download.speed) 
              : '-'}
          </span>
        </div>

        {/* Date/Time - Show completion date for completed, created date for others */}
        <div className="col-span-2 flex flex-col justify-center">
          {download.status === 'completed' && download.completedAt ? (
            <>
              <span className="text-xs text-gray-300 font-medium">
                {formatDateTime(download.completedAt).date}
              </span>
              <span className="text-xs text-green-400 tabular-nums">
                Completed: {formatDateTime(download.completedAt).time}
              </span>
            </>
          ) : (
            <>
              <span className="text-xs text-gray-300 font-medium">
                {dateTime.date}
              </span>
              <span className="text-xs text-gray-500 tabular-nums">
                {dateTime.time}
              </span>
            </>
          )}
        </div>

        {/* Actions */}
        <div className="col-span-2 flex items-center justify-end gap-1">
          {/* Pause button - only for downloading */}
          {download.status === 'downloading' && (
            <button
              onClick={handlePause}
              className="p-2 hover:bg-orange-500/20 hover:text-orange-400 text-gray-400 
                       rounded-xl transition-all duration-200 opacity-0 group-hover:opacity-100
                       border border-transparent hover:border-orange-500/30
                       hover:scale-110 active:scale-95"
              title="Pause download"
            >
              <Pause className="w-4 h-4" />
            </button>
          )}
          
          {/* Resume button - only for paused */}
          {download.status === 'paused' && (
            <button
              onClick={handleResume}
              className="p-2 hover:bg-green-500/20 hover:text-green-400 text-gray-400 
                       rounded-xl transition-all duration-200 opacity-0 group-hover:opacity-100
                       border border-transparent hover:border-green-500/30
                       hover:scale-110 active:scale-95"
              title="Resume download"
            >
              <Play className="w-4 h-4" />
            </button>
          )}
          
          {/* Retry button - only for failed */}
          {(download.status === 'failed' || download.status === 'cancelled') && (
            <button
              onClick={handleRetry}
              className="p-2 hover:bg-blue-500/20 hover:text-blue-400 text-gray-400 
                       rounded-xl transition-all duration-200 opacity-0 group-hover:opacity-100
                       border border-transparent hover:border-blue-500/30
                       hover:scale-110 active:scale-95"
              title="Retry download"
            >
              <RotateCcw className="w-4 h-4" />
            </button>
          )}
          
          {/* Open folder - for completed downloads */}
          {download.status === 'completed' && (
            <button
              onClick={handleOpenLocation}
              className="p-2 hover:bg-blue-500/20 hover:text-blue-400 text-gray-400 
                       rounded-xl transition-all duration-200 opacity-0 group-hover:opacity-100
                       border border-transparent hover:border-blue-500/30
                       hover:scale-110 active:scale-95"
              title="Open folder"
            >
              <FolderOpen className="w-4 h-4" />
            </button>
          )}
          
          <button
            onClick={() => setShowDetails(true)}
            className="p-2 hover:bg-gray-700/80 hover:text-white text-gray-400 
                     rounded-xl transition-all duration-200 opacity-0 group-hover:opacity-100
                     border border-transparent hover:border-gray-500/30
                     hover:scale-110 active:scale-95"
            title="View details"
          >
            <Info className="w-4 h-4" />
          </button>
          
          <button
            onClick={handleDelete}
            className="p-2 hover:bg-red-500/20 hover:text-red-400 text-gray-400 
                     rounded-xl transition-all duration-200 opacity-0 group-hover:opacity-100
                     border border-transparent hover:border-red-500/30
                     hover:scale-110 active:scale-95"
            title="Remove from list"
          >
            <Trash2 className="w-4 h-4" />
          </button>
        </div>
      </div>
    </>
  );
}
