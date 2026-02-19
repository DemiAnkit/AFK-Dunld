import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { 
  Calendar, 
  Download, 
  Trash2, 
  Search, 
  Filter,
  TrendingUp,
  Clock,
  FileText,
  X
} from 'lucide-react';
import { format } from 'date-fns';
import toast from 'react-hot-toast';

interface HistoryItem {
  id: string;
  url: string;
  file_name: string;
  total_size?: number;
  status: string;
  completed_at?: string;
  created_at: string;
  category?: string;
  download_speed_avg: number;
  download_time?: number;
}

interface HistoryStats {
  total_downloads: number;
  completed_downloads: number;
  failed_downloads: number;
  total_bytes_downloaded: number;
  average_speed: number;
  most_downloaded_category?: string;
}

export const DownloadHistory = () => {
  const [history, setHistory] = useState<HistoryItem[]>([]);
  const [stats, setStats] = useState<HistoryStats | null>(null);
  const [loading, setLoading] = useState(true);
  const [searchQuery, setSearchQuery] = useState('');
  const [filterStatus, setFilterStatus] = useState<string>('all');
  const [showFilters, setShowFilters] = useState(false);

  useEffect(() => {
    loadHistory();
    loadStats();
  }, []);

  const loadHistory = async () => {
    try {
      setLoading(true);
      const filter = {
        search_query: searchQuery || null,
        status: filterStatus !== 'all' ? filterStatus : null,
        limit: 100,
      };
      const data = await invoke<HistoryItem[]>('get_download_history', { filter });
      setHistory(data);
    } catch (error) {
      console.error('Failed to load history:', error);
      toast.error('Failed to load download history');
    } finally {
      setLoading(false);
    }
  };

  const loadStats = async () => {
    try {
      const data = await invoke<HistoryStats>('get_history_stats');
      setStats(data);
    } catch (error) {
      console.error('Failed to load stats:', error);
    }
  };

  const handleClearHistory = async (clearCompleted: boolean, clearFailed: boolean) => {
    if (!confirm('Are you sure you want to clear download history?')) return;

    try {
      const count = await invoke<number>('clear_download_history', {
        clearCompleted,
        clearFailed,
      });
      toast.success(`Cleared ${count} items from history`);
      loadHistory();
      loadStats();
    } catch (error) {
      console.error('Failed to clear history:', error);
      toast.error('Failed to clear history');
    }
  };

  const handleExportHistory = async () => {
    try {
      const timestamp = format(new Date(), 'yyyy-MM-dd-HHmmss');
      const fileName = `download-history-${timestamp}.json`;
      // In a real app, you'd use a file dialog here
      const result = await invoke<string>('export_history', {
        filePath: `./exports/${fileName}`,
      });
      toast.success(result);
    } catch (error) {
      console.error('Failed to export history:', error);
      toast.error('Failed to export history');
    }
  };

  const formatBytes = (bytes?: number) => {
    if (!bytes) return 'Unknown';
    const sizes = ['B', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(1024));
    return `${(bytes / Math.pow(1024, i)).toFixed(2)} ${sizes[i]}`;
  };

  const formatSpeed = (bytesPerSec: number) => {
    const sizes = ['B/s', 'KB/s', 'MB/s', 'GB/s'];
    const i = Math.floor(Math.log(bytesPerSec) / Math.log(1024));
    return `${(bytesPerSec / Math.pow(1024, i)).toFixed(2)} ${sizes[i]}`;
  };

  const formatDuration = (seconds?: number) => {
    if (!seconds) return 'Unknown';
    const hours = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    const secs = seconds % 60;
    
    if (hours > 0) return `${hours}h ${minutes}m ${secs}s`;
    if (minutes > 0) return `${minutes}m ${secs}s`;
    return `${secs}s`;
  };

  const getStatusColor = (status: string) => {
    if (status.includes('Completed')) return 'text-green-400';
    if (status.includes('Failed')) return 'text-red-400';
    if (status.includes('Downloading')) return 'text-blue-400';
    return 'text-gray-400';
  };

  const filteredHistory = history.filter(item => {
    if (searchQuery && !item.file_name.toLowerCase().includes(searchQuery.toLowerCase()) &&
        !item.url.toLowerCase().includes(searchQuery.toLowerCase())) {
      return false;
    }
    if (filterStatus !== 'all' && !item.status.toLowerCase().includes(filterStatus.toLowerCase())) {
      return false;
    }
    return true;
  });

  return (
    <div className="flex flex-col h-full bg-gray-950">
      {/* Stats Header */}
      {stats && (
        <div className="grid grid-cols-2 md:grid-cols-5 gap-4 p-4 bg-gray-900 border-b border-gray-800">
          <div className="flex flex-col">
            <span className="text-xs text-gray-400">Total Downloads</span>
            <span className="text-2xl font-bold text-white">{stats.total_downloads}</span>
          </div>
          <div className="flex flex-col">
            <span className="text-xs text-gray-400">Completed</span>
            <span className="text-2xl font-bold text-green-400">{stats.completed_downloads}</span>
          </div>
          <div className="flex flex-col">
            <span className="text-xs text-gray-400">Failed</span>
            <span className="text-2xl font-bold text-red-400">{stats.failed_downloads}</span>
          </div>
          <div className="flex flex-col">
            <span className="text-xs text-gray-400">Total Data</span>
            <span className="text-2xl font-bold text-blue-400">{formatBytes(stats.total_bytes_downloaded)}</span>
          </div>
          <div className="flex flex-col">
            <span className="text-xs text-gray-400">Avg Speed</span>
            <span className="text-2xl font-bold text-purple-400">{formatSpeed(stats.average_speed)}</span>
          </div>
        </div>
      )}

      {/* Toolbar */}
      <div className="flex items-center gap-4 p-4 bg-gray-900 border-b border-gray-800">
        <div className="flex-1 relative">
          <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 w-4 h-4 text-gray-400" />
          <input
            type="text"
            placeholder="Search downloads..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="w-full pl-10 pr-4 py-2 bg-gray-800 border border-gray-700 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
        </div>

        <button
          onClick={() => setShowFilters(!showFilters)}
          className="px-4 py-2 bg-gray-800 border border-gray-700 rounded-lg text-white hover:bg-gray-700 transition-colors flex items-center gap-2"
        >
          <Filter className="w-4 h-4" />
          Filters
        </button>

        <button
          onClick={handleExportHistory}
          className="px-4 py-2 bg-gray-800 border border-gray-700 rounded-lg text-white hover:bg-gray-700 transition-colors flex items-center gap-2"
        >
          <FileText className="w-4 h-4" />
          Export
        </button>

        <button
          onClick={() => handleClearHistory(true, true)}
          className="px-4 py-2 bg-red-600 hover:bg-red-700 rounded-lg text-white transition-colors flex items-center gap-2"
        >
          <Trash2 className="w-4 h-4" />
          Clear
        </button>
      </div>

      {/* Filters Panel */}
      {showFilters && (
        <div className="p-4 bg-gray-900 border-b border-gray-800">
          <div className="flex items-center gap-4">
            <label className="text-sm text-gray-400">Status:</label>
            <select
              value={filterStatus}
              onChange={(e) => setFilterStatus(e.target.value)}
              className="px-3 py-1.5 bg-gray-800 border border-gray-700 rounded text-white text-sm"
            >
              <option value="all">All</option>
              <option value="completed">Completed</option>
              <option value="failed">Failed</option>
              <option value="downloading">Downloading</option>
              <option value="paused">Paused</option>
            </select>
          </div>
        </div>
      )}

      {/* History List */}
      <div className="flex-1 overflow-auto">
        {loading ? (
          <div className="flex items-center justify-center h-full">
            <div className="text-gray-400">Loading history...</div>
          </div>
        ) : filteredHistory.length === 0 ? (
          <div className="flex flex-col items-center justify-center h-full text-gray-400">
            <Download className="w-16 h-16 mb-4 opacity-50" />
            <p className="text-lg">No download history</p>
            <p className="text-sm">Your download history will appear here</p>
          </div>
        ) : (
          <div>
            {/* Table Header */}
            <div className="grid grid-cols-[1fr_120px_100px_100px_180px_120px] gap-4 px-4 py-3 bg-gray-900/80 border-b border-gray-800 text-xs font-semibold text-gray-400 uppercase tracking-wider sticky top-0 z-10">
              <div className="flex items-center">File Name</div>
              <div className="flex items-center">Status</div>
              <div className="flex items-center">Size</div>
              <div className="flex items-center">Avg Speed</div>
              <div className="flex items-center">Date/Time</div>
              <div className="flex items-center">Duration</div>
            </div>
            
            {/* History Items */}
            <div className="divide-y divide-gray-800">
              {filteredHistory.map((item) => (
                <div key={item.id} className="grid grid-cols-[1fr_120px_100px_100px_180px_120px] gap-4 px-4 py-3 hover:bg-gray-900/50 transition-colors border-b border-gray-800/50">
                  {/* File Name */}
                  <div className="flex flex-col justify-center min-w-0">
                    <h3 className="font-medium text-white truncate" title={item.file_name}>
                      {item.file_name}
                    </h3>
                    <p className="text-xs text-gray-400 truncate mt-0.5" title={item.url}>
                      {item.url}
                    </p>
                    {item.category && (
                      <span className="inline-block px-2 py-0.5 bg-gray-800 rounded text-xs text-gray-300 mt-1 w-fit">
                        {item.category}
                      </span>
                    )}
                  </div>
                  
                  {/* Status */}
                  <div className="flex items-center">
                    <span className={`text-sm font-medium ${getStatusColor(item.status)}`}>
                      {item.status}
                    </span>
                  </div>
                  
                  {/* Size */}
                  <div className="flex items-center">
                    <span className="text-sm text-gray-300 font-medium tabular-nums">
                      {formatBytes(item.total_size)}
                    </span>
                  </div>
                  
                  {/* Avg Speed */}
                  <div className="flex items-center">
                    <span className="text-sm text-blue-400 font-medium tabular-nums">
                      {item.download_speed_avg > 0 ? formatSpeed(item.download_speed_avg) : '-'}
                    </span>
                  </div>
                  
                  {/* Date/Time */}
                  <div className="flex flex-col justify-center">
                    <span className="text-xs text-gray-300 font-medium">
                      {format(new Date(item.created_at), 'MMM dd, yyyy')}
                    </span>
                    <span className="text-xs text-gray-500 tabular-nums">
                      {format(new Date(item.created_at), 'HH:mm:ss')}
                    </span>
                  </div>
                  
                  {/* Duration */}
                  <div className="flex items-center">
                    <span className="text-sm text-gray-400 font-medium tabular-nums">
                      {item.download_time ? formatDuration(item.download_time) : '-'}
                    </span>
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}
      </div>
    </div>
  );
};
