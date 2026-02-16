// Torrent Manager Component
import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface TorrentStats {
  downloaded: number;
  uploaded: number;
  download_rate: number;
  upload_rate: number;
  peers: number;
  seeders: number;
  progress: number;
  eta: number | null;
}

interface TorrentState {
  state: 'Downloading' | 'Seeding' | 'Paused' | 'Checking' | { Error: string };
}

interface Torrent {
  info_hash: string;
  stats: TorrentStats;
  state: TorrentState;
}

export function TorrentManager() {
  const [torrents, setTorrents] = useState<Map<string, Torrent>>(new Map());
  const [magnetLink, setMagnetLink] = useState('');
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    const interval = setInterval(() => {
      updateAllStats();
    }, 2000);

    return () => clearInterval(interval);
  }, [torrents]);

  const updateAllStats = async () => {
    const updatedTorrents = new Map(torrents);
    
    for (const [hash, torrent] of torrents) {
      try {
        const stats = await invoke<TorrentStats>('get_torrent_stats', {
          infoHash: hash,
        });
        const state = await invoke<TorrentState>('get_torrent_state', {
          infoHash: hash,
        });
        
        updatedTorrents.set(hash, {
          info_hash: hash,
          stats,
          state,
        });
      } catch (error) {
        console.error(`Failed to update torrent ${hash}:`, error);
      }
    }
    
    setTorrents(updatedTorrents);
  };

  const addMagnet = async () => {
    if (!magnetLink.trim()) {
      alert('Please enter a magnet link');
      return;
    }

    setLoading(true);
    try {
      const infoHash = await invoke<string>('add_magnet_link', {
        magnetLink: magnetLink.trim(),
      });

      // Add to list and fetch initial stats
      const stats = await invoke<TorrentStats>('get_torrent_stats', { infoHash });
      const state = await invoke<TorrentState>('get_torrent_state', { infoHash });
      
      setTorrents(new Map(torrents.set(infoHash, {
        info_hash: infoHash,
        stats,
        state,
      })));

      setMagnetLink('');
      alert('Torrent added successfully!');
    } catch (error) {
      alert(`Failed to add torrent: ${error}`);
    } finally {
      setLoading(false);
    }
  };

  const pauseTorrent = async (infoHash: string) => {
    try {
      await invoke('pause_torrent', { infoHash });
      await updateAllStats();
    } catch (error) {
      alert(`Failed to pause: ${error}`);
    }
  };

  const resumeTorrent = async (infoHash: string) => {
    try {
      await invoke('resume_torrent', { infoHash });
      await updateAllStats();
    } catch (error) {
      alert(`Failed to resume: ${error}`);
    }
  };

  const removeTorrent = async (infoHash: string, deleteFiles: boolean) => {
    if (!confirm(`Remove torrent${deleteFiles ? ' and delete files' : ''}?`)) {
      return;
    }

    try {
      await invoke('remove_torrent', { infoHash, deleteFiles });
      const newTorrents = new Map(torrents);
      newTorrents.delete(infoHash);
      setTorrents(newTorrents);
    } catch (error) {
      alert(`Failed to remove: ${error}`);
    }
  };

  const formatSpeed = (bytesPerSec: number) => {
    if (bytesPerSec === 0) return '0 B/s';
    const k = 1024;
    const sizes = ['B/s', 'KB/s', 'MB/s', 'GB/s'];
    const i = Math.floor(Math.log(bytesPerSec) / Math.log(k));
    return Math.round(bytesPerSec / Math.pow(k, i) * 100) / 100 + ' ' + sizes[i];
  };

  const formatSize = (bytes: number) => {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return Math.round(bytes / Math.pow(k, i) * 100) / 100 + ' ' + sizes[i];
  };

  const formatETA = (seconds: number | null) => {
    if (seconds === null || seconds === 0) return '-';
    const hours = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    const secs = Math.floor(seconds % 60);
    return `${hours}h ${minutes}m ${secs}s`;
  };

  const getStateDisplay = (state: TorrentState) => {
    if (typeof state === 'string') return state;
    if ('Error' in state) return `Error: ${state.Error}`;
    return String(state);
  };

  const isPaused = (state: TorrentState) => {
    return typeof state === 'string' && state === 'Paused';
  };

  return (
    <div className="torrent-manager p-6">
      <h2 className="text-2xl font-bold mb-6">Torrent Manager</h2>

      {/* Add Torrent */}
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow-md p-6 mb-6">
        <h3 className="text-lg font-semibold mb-4">Add Magnet Link</h3>
        <div className="flex gap-3">
          <input
            type="text"
            value={magnetLink}
            onChange={(e) => setMagnetLink(e.target.value)}
            placeholder="magnet:?xt=urn:btih:..."
            className="flex-1 px-3 py-2 border rounded-lg dark:bg-gray-700 dark:border-gray-600"
            onKeyPress={(e) => e.key === 'Enter' && addMagnet()}
          />
          <button
            onClick={addMagnet}
            disabled={loading}
            className="px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:opacity-50"
          >
            {loading ? 'Adding...' : 'Add'}
          </button>
        </div>
      </div>

      {/* Torrent List */}
      <div className="space-y-4">
        {Array.from(torrents.values()).map((torrent) => (
          <div
            key={torrent.info_hash}
            className="bg-white dark:bg-gray-800 rounded-lg shadow-md p-6"
          >
            <div className="flex items-start justify-between mb-4">
              <div className="flex-1">
                <h3 className="font-semibold text-lg mb-1">
                  Torrent: {torrent.info_hash.substring(0, 16)}...
                </h3>
                <div className="flex items-center gap-4 text-sm text-gray-600 dark:text-gray-400">
                  <span className={`px-2 py-1 rounded ${
                    isPaused(torrent.state) ? 'bg-yellow-100 text-yellow-800' : 'bg-green-100 text-green-800'
                  }`}>
                    {getStateDisplay(torrent.state)}
                  </span>
                  <span>{torrent.stats.peers} peers</span>
                  <span>{torrent.stats.seeders} seeders</span>
                </div>
              </div>

              <div className="flex gap-2">
                {isPaused(torrent.state) ? (
                  <button
                    onClick={() => resumeTorrent(torrent.info_hash)}
                    className="px-3 py-1 bg-green-600 text-white rounded hover:bg-green-700"
                  >
                    Resume
                  </button>
                ) : (
                  <button
                    onClick={() => pauseTorrent(torrent.info_hash)}
                    className="px-3 py-1 bg-yellow-600 text-white rounded hover:bg-yellow-700"
                  >
                    Pause
                  </button>
                )}
                
                <button
                  onClick={() => removeTorrent(torrent.info_hash, false)}
                  className="px-3 py-1 bg-red-600 text-white rounded hover:bg-red-700"
                >
                  Remove
                </button>
              </div>
            </div>

            {/* Progress Bar */}
            <div className="mb-4">
              <div className="flex items-center justify-between text-sm mb-2">
                <span>Progress: {torrent.stats.progress.toFixed(2)}%</span>
                <span>ETA: {formatETA(torrent.stats.eta)}</span>
              </div>
              <div className="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2">
                <div
                  className="bg-blue-600 h-2 rounded-full transition-all"
                  style={{ width: `${torrent.stats.progress}%` }}
                ></div>
              </div>
            </div>

            {/* Stats Grid */}
            <div className="grid grid-cols-2 md:grid-cols-4 gap-4 text-sm">
              <div>
                <div className="text-gray-600 dark:text-gray-400">Downloaded</div>
                <div className="font-semibold">{formatSize(torrent.stats.downloaded)}</div>
              </div>
              
              <div>
                <div className="text-gray-600 dark:text-gray-400">Uploaded</div>
                <div className="font-semibold">{formatSize(torrent.stats.uploaded)}</div>
              </div>
              
              <div>
                <div className="text-gray-600 dark:text-gray-400">Download Speed</div>
                <div className="font-semibold text-green-600">↓ {formatSpeed(torrent.stats.download_rate)}</div>
              </div>
              
              <div>
                <div className="text-gray-600 dark:text-gray-400">Upload Speed</div>
                <div className="font-semibold text-blue-600">↑ {formatSpeed(torrent.stats.upload_rate)}</div>
              </div>
            </div>
          </div>
        ))}

        {torrents.size === 0 && (
          <div className="bg-gray-50 dark:bg-gray-800 rounded-lg p-12 text-center text-gray-500">
            No active torrents. Add a magnet link to get started!
          </div>
        )}
      </div>
    </div>
  );
}
