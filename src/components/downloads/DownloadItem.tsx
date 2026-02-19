// src/components/downloads/DownloadItem.tsx
import React from "react";
  import { 
    Pause, Play, X, Trash2, RotateCcw, 
    FolderOpen, CheckCircle2,
    AlertCircle, Clock, Loader2
  } from "lucide-react";
  import { useDownloadStore, Download } from "../../stores/downloadStore";
  import { formatBytes, formatSpeed, formatEta } from "../../utils/format";
  import { downloadApi } from "../../services/tauriApi";
  import { format } from "date-fns";
  import toast from "react-hot-toast";
  
  interface DownloadItemProps {
    download: Download;
  }
  
  export function DownloadItem({ download }: DownloadItemProps) {
    const { 
      pauseDownload, resumeDownload, cancelDownload, 
      removeDownload, retryDownload 
    } = useDownloadStore();
    const [actualFileSize, setActualFileSize] = React.useState<number | null>(null);

    // Fetch actual file size for completed downloads
    React.useEffect(() => {
      if (download.status === 'completed' && download.savePath) {
        fetchActualFileSize();
      }
    }, [download.status, download.savePath]);

    const fetchActualFileSize = async () => {
      try {
        const size = await downloadApi.getFileSize(download.id);
        setActualFileSize(size);
      } catch (error) {
        console.error("Failed to get file size:", error);
        setActualFileSize(null);
      }
    };
  
    const progress = download.totalSize
      ? (download.downloadedSize / download.totalSize) * 100
      : 0;
  
    const statusIcon: Record<string, JSX.Element> = {
      queued: <Clock className="w-4 h-4 text-yellow-400" />,
      connecting: <Loader2 className="w-4 h-4 text-blue-300 animate-spin" />,
      downloading: <Loader2 className="w-4 h-4 text-blue-400 animate-spin" />,
      paused: <Pause className="w-4 h-4 text-orange-400" />,
      completed: <CheckCircle2 className="w-4 h-4 text-green-400" />,
      failed: <AlertCircle className="w-4 h-4 text-red-400" />,
      cancelled: <X className="w-4 h-4 text-gray-400" />,
      merging: <Loader2 className="w-4 h-4 text-purple-400 animate-spin" />,
      verifying: <Loader2 className="w-4 h-4 text-cyan-400 animate-spin" />,
    };
  
    const statusColor: Record<string, string> = {
      queued: "bg-yellow-500",
      connecting: "bg-blue-400",
      downloading: "bg-blue-500",
      paused: "bg-orange-500",
      completed: "bg-green-500",
      failed: "bg-red-500",
      cancelled: "bg-gray-500",
      merging: "bg-purple-500",
      verifying: "bg-cyan-500",
    };

    const getFileTypeIcon = (fileName?: string) => {
      if (!fileName) return '📁';
      
      const ext = fileName.split('.').pop()?.toLowerCase();
      if (!ext) return '📁';
      
      if (['mp4', 'avi', 'mkv', 'mov', 'wmv'].includes(ext)) {
        return '🎥';
      }
      if (['mp3', 'wav', 'flac', 'aac'].includes(ext)) {
        return '🎵';
      }
      if (['jpg', 'jpeg', 'png', 'gif', 'webp'].includes(ext)) {
        return '🖼️';
      }
      if (['pdf', 'doc', 'docx', 'txt'].includes(ext)) {
        return '📄';
      }
      if (['zip', 'rar', '7z', 'tar', 'gz'].includes(ext)) {
        return '📦';
      }
      return '📁';
    };

    const handleOpenFileLocation = async () => {
      try {
        await downloadApi.openFileLocation(download.id);
        toast.success("Opening file location...");
      } catch (error) {
        console.error("Failed to open file location:", error);
        toast.error("Failed to open file location");
      }
    };

    // Format date and time
    const formatDateTime = (dateString: string) => {
      try {
        const date = new Date(dateString);
        return {
          date: format(date, 'MMM dd, yyyy'),
          time: format(date, 'HH:mm:ss')
        };
      } catch {
        return { date: '-', time: '-' };
      }
    };

    const createdDateTime = formatDateTime(download.createdAt);
    const completedDateTime = download.completedAt ? formatDateTime(download.completedAt) : null;
  
    return (
      <div className="grid grid-cols-12 gap-4 px-4 py-3 items-center 
                      border-b border-gray-800/50 hover:bg-gray-900/50 
                      transition-colors group">
        {/* Checkbox + File Name */}
        <div className="col-span-4 flex items-center gap-3 min-w-0">
          <input
            type="checkbox"
            className="w-3.5 h-3.5 bg-gray-700 border-gray-600 rounded text-blue-600 focus:ring-blue-500 focus:ring-2 flex-shrink-0"
          />
          <button
            onClick={handleOpenFileLocation}
            className="flex-shrink-0 text-lg hover:scale-110 transition-transform cursor-pointer"
            title="Open file location"
          >
            {getFileTypeIcon(download.fileName)}
          </button>
          <div className="flex-shrink-0">{statusIcon[download.status] || statusIcon.queued}</div>
          <div className="min-w-0 flex-1">
            <p className="text-sm font-medium text-white truncate">
              {download.fileName || 'Unknown'}
            </p>
            <p className="text-xs text-gray-500 truncate">
              {download.url || ''}
            </p>
          </div>
        </div>
  
        {/* Size & Status */}
        <div className="col-span-2 text-sm">
          <div className="flex flex-col">
            <span className="text-white font-medium">
              {download.status === 'completed' && actualFileSize !== null
                ? formatBytes(actualFileSize)
                : download.totalSize 
                ? formatBytes(download.totalSize) 
                : formatBytes(download.downloadedSize)}
            </span>
            <span className={`text-xs ${
              download.status === 'completed' ? 'text-green-400' :
              download.status === 'downloading' ? 'text-blue-400' :
              download.status === 'paused' ? 'text-orange-400' :
              download.status === 'failed' ? 'text-red-400' :
              'text-gray-500'
            }`}>
              {download.status === 'completed' ? '✓ Downloaded' :
               download.status === 'downloading' ? '↓ Downloading' :
               download.status === 'paused' ? '⏸ Paused' :
               download.status === 'failed' ? '✗ Failed' :
               download.status === 'queued' ? '⏳ Queued' :
               download.status}
            </span>
          </div>
        </div>
  
        {/* Progress Bar */}
        <div className="col-span-2">
          <div className="flex items-center gap-2">
            <div className="flex-1 h-2 bg-gray-800 rounded-full overflow-hidden">
              <div
                className={`h-full ${statusColor[download.status] || statusColor.queued} rounded-full transition-all 
                           duration-300 ease-out`}
                style={{ width: `${Math.min(progress, 100)}%` }}
              />
            </div>
            <span className="text-xs text-gray-400 w-12 text-right">
              {progress.toFixed(1)}%
            </span>
          </div>
        </div>
  
        {/* Speed */}
        <div className="col-span-1 text-sm text-gray-300">
          <div className="flex flex-col">
            {download.status === "downloading" ? (
              <>
                <span>{formatSpeed(download.speed)}</span>
                <span className="text-xs text-gray-500">current</span>
              </>
            ) : (
              "-"
            )}
          </div>
        </div>
  
        {/* Date & Time */}
        <div className="col-span-2 text-sm">
          <div className="flex flex-col">
            {completedDateTime ? (
              <>
                <span className="text-gray-300">{completedDateTime.date}</span>
                <span className="text-xs text-gray-500">{completedDateTime.time}</span>
              </>
            ) : download.status === "downloading" && download.eta ? (
              <>
                <span className="text-gray-300">ETA: {formatEta(download.eta)}</span>
                <span className="text-xs text-gray-500">{createdDateTime.time}</span>
              </>
            ) : (
              <>
                <span className="text-gray-300">{createdDateTime.date}</span>
                <span className="text-xs text-gray-500">{createdDateTime.time}</span>
              </>
            )}
          </div>
        </div>
  
        {/* Actions */}
        <div className="col-span-1 flex items-center justify-end gap-1">
          {download.status === "downloading" && (
            <button
              onClick={() => pauseDownload(download.id)}
              className="p-1.5 hover:bg-gray-700 rounded transition-colors"
              title="Pause"
            >
              <Pause className="w-3.5 h-3.5 text-orange-400" />
            </button>
          )}
  
          {download.status === "paused" && (
            <button
              onClick={() => resumeDownload(download.id)}
              className="p-1.5 hover:bg-gray-700 rounded transition-colors"
              title="Resume"
            >
              <Play className="w-3.5 h-3.5 text-green-400" />
            </button>
          )}
  
          {["downloading", "paused", "queued", "connecting"].includes(download.status) && (
            <button
              onClick={() => cancelDownload(download.id)}
              className="p-1.5 hover:bg-gray-700 rounded transition-colors"
              title="Cancel"
            >
              <X className="w-3.5 h-3.5 text-red-400" />
            </button>
          )}
  
          {["failed", "cancelled"].includes(download.status) && (
            <button
              onClick={() => retryDownload(download.id)}
              className="p-1.5 hover:bg-gray-700 rounded transition-colors"
              title="Retry"
            >
              <RotateCcw className="w-3.5 h-3.5 text-blue-400" />
            </button>
          )}
  
          {/* Open Folder - Always visible for easy access */}
          <button
            onClick={handleOpenFileLocation}
            className="p-1.5 hover:bg-gray-700 rounded transition-colors"
            title={`Open ${download.savePath ? 'file location' : 'download folder'}`}
          >
            <FolderOpen className="w-3.5 h-3.5 text-blue-400" />
          </button>
  
          <button
            onClick={() => removeDownload(download.id)}
            className="p-1.5 hover:bg-gray-700 rounded transition-colors 
                       opacity-0 group-hover:opacity-100"
            title="Remove"
          >
            <Trash2 className="w-3.5 h-3.5 text-gray-400" />
          </button>
        </div>
      </div>
    );
  }
