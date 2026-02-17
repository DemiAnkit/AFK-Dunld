import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { List, Play, Pause, SkipForward, Settings, ChevronUp, ChevronDown } from 'lucide-react';
import toast from 'react-hot-toast';

interface QueueInfo {
  active_count: number;
  queued_count: number;
  max_concurrent: number;
  total_speed: number;
}

interface QueuedDownload {
  id: string;
  url: string;
  file_name: string;
  priority: number;
  status: string;
  total_size?: number;
}

export const QueueManager = () => {
  const [queueInfo, setQueueInfo] = useState<QueueInfo | null>(null);
  const [queuedDownloads, setQueuedDownloads] = useState<QueuedDownload[]>([]);
  const [maxConcurrent, setMaxConcurrent] = useState(5);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadQueueInfo();
    loadQueuedDownloads();
    
    // Refresh every 2 seconds
    const interval = setInterval(() => {
      loadQueueInfo();
      loadQueuedDownloads();
    }, 2000);

    return () => clearInterval(interval);
  }, []);

  const loadQueueInfo = async () => {
    try {
      const info = await invoke<QueueInfo>('get_queue_info');
      setQueueInfo(info);
      setMaxConcurrent(info.max_concurrent);
    } catch (error) {
      console.error('Failed to load queue info:', error);
    }
  };

  const loadQueuedDownloads = async () => {
    try {
      setLoading(true);
      const downloads = await invoke<QueuedDownload[]>('get_all_downloads');
      // Filter for queued/downloading items
      const queued = downloads.filter(d => 
        d.status === 'Queued' || d.status === 'Downloading'
      ).sort((a, b) => b.priority - a.priority);
      setQueuedDownloads(queued);
    } catch (error) {
      console.error('Failed to load queued downloads:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleUpdateMaxConcurrent = async () => {
    try {
      await invoke('set_max_concurrent', { max: maxConcurrent });
      toast.success(`Max concurrent downloads set to ${maxConcurrent}`);
      loadQueueInfo();
    } catch (error) {
      console.error('Failed to update max concurrent:', error);
      toast.error('Failed to update settings');
    }
  };

  const handlePauseAll = async () => {
    try {
      await invoke('pause_all');
      toast.success('All downloads paused');
      loadQueuedDownloads();
    } catch (error) {
      console.error('Failed to pause all:', error);
      toast.error('Failed to pause downloads');
    }
  };

  const handleResumeAll = async () => {
    try {
      await invoke('resume_all');
      toast.success('All downloads resumed');
      loadQueuedDownloads();
    } catch (error) {
      console.error('Failed to resume all:', error);
      toast.error('Failed to resume downloads');
    }
  };

  const handleChangePriority = async (id: string, direction: 'up' | 'down') => {
    const download = queuedDownloads.find(d => d.id === id);
    if (!download) return;

    const newPriority = direction === 'up' ? download.priority + 1 : download.priority - 1;
    
    try {
      // Note: You'll need to add this command to backend
      // await invoke('update_download_priority', { downloadId: id, priority: newPriority });
      toast.success('Priority updated');
      loadQueuedDownloads();
    } catch (error) {
      console.error('Failed to update priority:', error);
      toast.error('Failed to update priority');
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

  return (
    <div className="flex flex-col h-full bg-gray-950 p-6">
      <div className="mb-6">
        <h2 className="text-2xl font-bold text-white mb-2">Download Queue</h2>
        <p className="text-gray-400">Manage concurrent downloads and priorities</p>
      </div>

      {/* Queue Stats */}
      {queueInfo && (
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4 mb-6">
          <div className="bg-gray-900 rounded-lg p-4 border border-gray-800">
            <div className="text-sm text-gray-400 mb-1">Active</div>
            <div className="text-2xl font-bold text-blue-400">{queueInfo.active_count}</div>
          </div>
          <div className="bg-gray-900 rounded-lg p-4 border border-gray-800">
            <div className="text-sm text-gray-400 mb-1">Queued</div>
            <div className="text-2xl font-bold text-purple-400">{queueInfo.queued_count}</div>
          </div>
          <div className="bg-gray-900 rounded-lg p-4 border border-gray-800">
            <div className="text-sm text-gray-400 mb-1">Max Concurrent</div>
            <div className="text-2xl font-bold text-green-400">{queueInfo.max_concurrent}</div>
          </div>
          <div className="bg-gray-900 rounded-lg p-4 border border-gray-800">
            <div className="text-sm text-gray-400 mb-1">Total Speed</div>
            <div className="text-2xl font-bold text-orange-400">{formatSpeed(queueInfo.total_speed)}</div>
          </div>
        </div>
      )}

      {/* Controls */}
      <div className="flex items-center gap-4 mb-6">
        <div className="flex items-center gap-2">
          <label className="text-sm text-gray-400">Max Concurrent:</label>
          <input
            type="number"
            min="1"
            max="20"
            value={maxConcurrent}
            onChange={(e) => setMaxConcurrent(parseInt(e.target.value) || 1)}
            className="w-20 px-3 py-2 bg-gray-800 border border-gray-700 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
          <button
            onClick={handleUpdateMaxConcurrent}
            className="px-3 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg transition-colors"
          >
            <Settings className="w-4 h-4" />
          </button>
        </div>

        <button
          onClick={handlePauseAll}
          className="px-4 py-2 bg-gray-800 hover:bg-gray-700 text-white rounded-lg transition-colors flex items-center gap-2"
        >
          <Pause className="w-4 h-4" />
          Pause All
        </button>

        <button
          onClick={handleResumeAll}
          className="px-4 py-2 bg-green-600 hover:bg-green-700 text-white rounded-lg transition-colors flex items-center gap-2"
        >
          <Play className="w-4 h-4" />
          Resume All
        </button>
      </div>

      {/* Queue List */}
      <div className="flex-1 overflow-auto">
        {loading ? (
          <div className="flex items-center justify-center h-64">
            <div className="text-gray-400">Loading queue...</div>
          </div>
        ) : queuedDownloads.length === 0 ? (
          <div className="flex flex-col items-center justify-center h-64 text-gray-400">
            <List className="w-16 h-16 mb-4 opacity-50" />
            <p className="text-lg">Queue is empty</p>
            <p className="text-sm">Add downloads to see them here</p>
          </div>
        ) : (
          <div className="space-y-3">
            {queuedDownloads.map((download, index) => (
              <div
                key={download.id}
                className="bg-gray-900 rounded-lg p-4 border border-gray-800 hover:border-gray-700 transition-colors"
              >
                <div className="flex items-center justify-between">
                  <div className="flex items-center gap-4 flex-1 min-w-0">
                    <div className="flex flex-col gap-1">
                      <button
                        onClick={() => handleChangePriority(download.id, 'up')}
                        disabled={index === 0}
                        className="p-1 hover:bg-gray-800 rounded disabled:opacity-30 disabled:cursor-not-allowed"
                      >
                        <ChevronUp className="w-4 h-4 text-gray-400" />
                      </button>
                      <button
                        onClick={() => handleChangePriority(download.id, 'down')}
                        disabled={index === queuedDownloads.length - 1}
                        className="p-1 hover:bg-gray-800 rounded disabled:opacity-30 disabled:cursor-not-allowed"
                      >
                        <ChevronDown className="w-4 h-4 text-gray-400" />
                      </button>
                    </div>

                    <div className="flex-1 min-w-0">
                      <h3 className="font-medium text-white truncate">{download.file_name}</h3>
                      <p className="text-sm text-gray-400 truncate">{download.url}</p>
                    </div>
                  </div>

                  <div className="flex items-center gap-3">
                    <div className={`px-3 py-1 rounded-lg text-sm ${
                      download.status === 'Downloading'
                        ? 'bg-blue-900/30 text-blue-300'
                        : 'bg-purple-900/30 text-purple-300'
                    }`}>
                      {download.status}
                    </div>
                    
                    <div className="px-3 py-1 bg-gray-800 rounded-lg text-sm text-gray-300">
                      Priority: {download.priority}
                    </div>

                    {download.total_size && (
                      <div className="text-sm text-gray-400">
                        {formatBytes(download.total_size)}
                      </div>
                    )}
                  </div>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
};

export default QueueManager;
