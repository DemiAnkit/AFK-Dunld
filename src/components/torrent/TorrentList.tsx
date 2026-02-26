// src/components/torrent/TorrentList.tsx
import React, { useState, useEffect } from 'react';
import { torrentApi } from '../../services/torrentApi';
import type { TorrentInfo, TorrentStats, TorrentMetadata, TorrentPriority } from '../../types/torrent';
import { formatBytes, formatSpeed } from '../../utils/format';

interface TorrentListProps {
  onTorrentSelect?: (infoHash: string) => void;
}

interface TorrentItem {
  info: TorrentInfo;
  stats: TorrentStats;
  metadata: TorrentMetadata;
}

export const TorrentList: React.FC<TorrentListProps> = ({ onTorrentSelect }) => {
  const [torrents, setTorrents] = useState<TorrentItem[]>([]);
  const [loading, setLoading] = useState(true);
  const [selectedHash, setSelectedHash] = useState<string | null>(null);

  useEffect(() => {
    loadTorrents();
    const interval = setInterval(loadTorrents, 2000); // Refresh every 2 seconds
    return () => clearInterval(interval);
  }, []);

  const loadTorrents = async () => {
    try {
      const torrentList = await torrentApi.listTorrents();
      const items: TorrentItem[] = [];

      for (const info of torrentList) {
        const [stats, metadata] = await Promise.all([
          torrentApi.getTorrentStats(info.info_hash),
          torrentApi.getTorrentMetadata(info.info_hash),
        ]);
        items.push({ info, stats, metadata });
      }

      setTorrents(items);
    } catch (error) {
      console.error('Failed to load torrents:', error);
    } finally {
      setLoading(false);
    }
  };

  const handlePause = async (infoHash: string, e: React.MouseEvent) => {
    e.stopPropagation();
    try {
      await torrentApi.pauseTorrent(infoHash);
      await loadTorrents();
    } catch (error) {
      console.error('Failed to pause torrent:', error);
    }
  };

  const handleResume = async (infoHash: string, e: React.MouseEvent) => {
    e.stopPropagation();
    try {
      await torrentApi.resumeTorrent(infoHash);
      await loadTorrents();
    } catch (error) {
      console.error('Failed to resume torrent:', error);
    }
  };

  const handleRemove = async (infoHash: string, e: React.MouseEvent) => {
    e.stopPropagation();
    if (confirm('Are you sure you want to remove this torrent?')) {
      try {
        await torrentApi.removeTorrent(infoHash, false);
        await loadTorrents();
      } catch (error) {
        console.error('Failed to remove torrent:', error);
      }
    }
  };

  const handleSelect = (infoHash: string) => {
    setSelectedHash(infoHash);
    onTorrentSelect?.(infoHash);
  };

  const getPriorityLabel = (priority: TorrentPriority): string => {
    switch (priority) {
      case 0: return 'Low';
      case 1: return 'Normal';
      case 2: return 'High';
      case 3: return 'Critical';
      default: return 'Normal';
    }
  };

  const getPriorityColor = (priority: TorrentPriority): string => {
    switch (priority) {
      case 0: return 'text-gray-500';
      case 1: return 'text-blue-500';
      case 2: return 'text-orange-500';
      case 3: return 'text-red-500';
      default: return 'text-blue-500';
    }
  };

  if (loading) {
    return (
      <div className="flex items-center justify-center p-8">
        <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500"></div>
      </div>
    );
  }

  if (torrents.length === 0) {
    return (
      <div className="text-center p-8 text-gray-500">
        <p>No torrents added yet.</p>
        <p className="text-sm mt-2">Add a torrent file or magnet link to get started.</p>
      </div>
    );
  }

  return (
    <div className="divide-y divide-gray-200 dark:divide-gray-700">
      {torrents.map((torrent) => (
        <div
          key={torrent.info.info_hash}
          className={`p-4 hover:bg-gray-50 dark:hover:bg-gray-800 cursor-pointer transition-colors ${
            selectedHash === torrent.info.info_hash ? 'bg-blue-50 dark:bg-blue-900/20' : ''
          }`}
          onClick={() => handleSelect(torrent.info.info_hash)}
        >
          <div className="flex items-start justify-between">
            <div className="flex-1 min-w-0">
              <div className="flex items-center gap-2 mb-1">
                <h3 className="font-medium text-gray-900 dark:text-white truncate">
                  {torrent.info.name}
                </h3>
                <span className={`text-xs px-2 py-0.5 rounded-full ${getPriorityColor(torrent.metadata.priority)}`}>
                  {getPriorityLabel(torrent.metadata.priority)}
                </span>
                {torrent.metadata.category && (
                  <span className="text-xs px-2 py-0.5 rounded-full bg-purple-100 dark:bg-purple-900 text-purple-800 dark:text-purple-200">
                    {torrent.metadata.category}
                  </span>
                )}
              </div>

              <div className="flex items-center gap-4 text-sm text-gray-600 dark:text-gray-400 mb-2">
                <span>{formatBytes(torrent.info.total_size)}</span>
                <span>{torrent.stats.peers} peers</span>
                <span>{torrent.stats.seeders} seeders</span>
              </div>

              {/* Progress bar */}
              <div className="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2 mb-2">
                <div
                  className="bg-blue-600 h-2 rounded-full transition-all duration-300"
                  style={{ width: `${torrent.stats.progress * 100}%` }}
                ></div>
              </div>

              <div className="flex items-center gap-4 text-xs text-gray-500 dark:text-gray-400">
                <span>{(torrent.stats.progress * 100).toFixed(1)}%</span>
                <span>↓ {formatSpeed(torrent.stats.download_rate)}</span>
                <span>↑ {formatSpeed(torrent.stats.upload_rate)}</span>
                {torrent.stats.eta && (
                  <span>ETA: {Math.floor(torrent.stats.eta / 60)}m</span>
                )}
              </div>

              {/* Tags */}
              {torrent.metadata.tags.length > 0 && (
                <div className="flex gap-1 mt-2">
                  {torrent.metadata.tags.map((tag) => (
                    <span
                      key={tag}
                      className="text-xs px-2 py-0.5 rounded bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300"
                    >
                      {tag}
                    </span>
                  ))}
                </div>
              )}
            </div>

            {/* Action buttons */}
            <div className="flex gap-2 ml-4">
              <button
                onClick={(e) => handlePause(torrent.info.info_hash, e)}
                className="p-2 hover:bg-gray-200 dark:hover:bg-gray-700 rounded"
                title="Pause"
              >
                <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 9v6m4-6v6" />
                </svg>
              </button>
              <button
                onClick={(e) => handleResume(torrent.info.info_hash, e)}
                className="p-2 hover:bg-gray-200 dark:hover:bg-gray-700 rounded"
                title="Resume"
              >
                <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M14.752 11.168l-3.197-2.132A1 1 0 0010 9.87v4.263a1 1 0 001.555.832l3.197-2.132a1 1 0 000-1.664z" />
                </svg>
              </button>
              <button
                onClick={(e) => handleRemove(torrent.info.info_hash, e)}
                className="p-2 hover:bg-red-100 dark:hover:bg-red-900/30 text-red-600 rounded"
                title="Remove"
              >
                <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                </svg>
              </button>
            </div>
          </div>
        </div>
      ))}
    </div>
  );
};
