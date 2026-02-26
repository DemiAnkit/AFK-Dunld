// src/components/torrent/TorrentDetailsPanel.tsx
import React, { useState, useEffect } from 'react';
import { torrentApi } from '../../services/torrentApi';
import type { TorrentInfo, TorrentStats, TorrentMetadata } from '../../types/torrent';
import { formatBytes } from '../../utils/format';

interface TorrentDetailsPanelProps {
  infoHash: string | null;
}

export const TorrentDetailsPanel: React.FC<TorrentDetailsPanelProps> = ({ infoHash }) => {
  const [info, setInfo] = useState<TorrentInfo | null>(null);
  const [stats, setStats] = useState<TorrentStats | null>(null);
  const [metadata, setMetadata] = useState<TorrentMetadata | null>(null);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    if (infoHash) {
      loadDetails();
      const interval = setInterval(loadDetails, 2000);
      return () => clearInterval(interval);
    }
  }, [infoHash]);

  const loadDetails = async () => {
    if (!infoHash) return;

    try {
      setLoading(true);
      const [infoData, statsData, metadataData] = await Promise.all([
        torrentApi.getTorrentInfo(infoHash),
        torrentApi.getTorrentStats(infoHash),
        torrentApi.getTorrentMetadata(infoHash),
      ]);

      setInfo(infoData);
      setStats(statsData);
      setMetadata(metadataData);
    } catch (error) {
      console.error('Failed to load torrent details:', error);
    } finally {
      setLoading(false);
    }
  };

  if (!infoHash) {
    return (
      <div className="flex items-center justify-center h-full text-gray-500 dark:text-gray-400">
        Select a torrent to view details
      </div>
    );
  }

  if (loading && !info) {
    return (
      <div className="flex items-center justify-center h-full">
        <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500"></div>
      </div>
    );
  }

  if (!info || !stats || !metadata) {
    return (
      <div className="flex items-center justify-center h-full text-red-500">
        Failed to load torrent details
      </div>
    );
  }

  return (
    <div className="h-full overflow-y-auto p-4 space-y-6">
      {/* General Info */}
      <section>
        <h3 className="text-lg font-semibold text-gray-900 dark:text-white mb-3">
          General Information
        </h3>
        <div className="space-y-2 text-sm">
          <div className="flex justify-between">
            <span className="text-gray-600 dark:text-gray-400">Name:</span>
            <span className="text-gray-900 dark:text-white font-medium">{info.name}</span>
          </div>
          <div className="flex justify-between">
            <span className="text-gray-600 dark:text-gray-400">Size:</span>
            <span className="text-gray-900 dark:text-white">{formatBytes(info.total_size)}</span>
          </div>
          <div className="flex justify-between">
            <span className="text-gray-600 dark:text-gray-400">Info Hash:</span>
            <span className="text-gray-900 dark:text-white font-mono text-xs">{info.info_hash}</span>
          </div>
          <div className="flex justify-between">
            <span className="text-gray-600 dark:text-gray-400">Pieces:</span>
            <span className="text-gray-900 dark:text-white">{info.num_pieces}</span>
          </div>
          <div className="flex justify-between">
            <span className="text-gray-600 dark:text-gray-400">Piece Size:</span>
            <span className="text-gray-900 dark:text-white">{formatBytes(info.piece_length)}</span>
          </div>
        </div>
      </section>

      {/* Transfer Stats */}
      <section>
        <h3 className="text-lg font-semibold text-gray-900 dark:text-white mb-3">
          Transfer Statistics
        </h3>
        <div className="space-y-2 text-sm">
          <div className="flex justify-between">
            <span className="text-gray-600 dark:text-gray-400">Downloaded:</span>
            <span className="text-gray-900 dark:text-white">{formatBytes(stats.downloaded)}</span>
          </div>
          <div className="flex justify-between">
            <span className="text-gray-600 dark:text-gray-400">Uploaded:</span>
            <span className="text-gray-900 dark:text-white">{formatBytes(stats.uploaded)}</span>
          </div>
          <div className="flex justify-between">
            <span className="text-gray-600 dark:text-gray-400">Ratio:</span>
            <span className="text-gray-900 dark:text-white">
              {stats.downloaded > 0 ? (stats.uploaded / stats.downloaded).toFixed(2) : '0.00'}
            </span>
          </div>
          <div className="flex justify-between">
            <span className="text-gray-600 dark:text-gray-400">Peers:</span>
            <span className="text-gray-900 dark:text-white">{stats.peers}</span>
          </div>
          <div className="flex justify-between">
            <span className="text-gray-600 dark:text-gray-400">Seeders:</span>
            <span className="text-gray-900 dark:text-white">{stats.seeders}</span>
          </div>
        </div>
      </section>

      {/* Files */}
      <section>
        <h3 className="text-lg font-semibold text-gray-900 dark:text-white mb-3">
          Files ({info.files.length})
        </h3>
        <div className="space-y-1">
          {info.files.map((file, index) => (
            <div key={index} className="flex justify-between text-sm p-2 hover:bg-gray-50 dark:hover:bg-gray-800 rounded">
              <span className="text-gray-900 dark:text-white truncate">{file.path}</span>
              <span className="text-gray-600 dark:text-gray-400 ml-2 flex-shrink-0">
                {formatBytes(file.size)}
              </span>
            </div>
          ))}
        </div>
      </section>

      {/* Metadata */}
      <section>
        <h3 className="text-lg font-semibold text-gray-900 dark:text-white mb-3">
          Metadata
        </h3>
        <div className="space-y-2 text-sm">
          <div className="flex justify-between">
            <span className="text-gray-600 dark:text-gray-400">Added:</span>
            <span className="text-gray-900 dark:text-white">
              {new Date(metadata.added_time).toLocaleString()}
            </span>
          </div>
          {metadata.completed_time && (
            <div className="flex justify-between">
              <span className="text-gray-600 dark:text-gray-400">Completed:</span>
              <span className="text-gray-900 dark:text-white">
                {new Date(metadata.completed_time).toLocaleString()}
              </span>
            </div>
          )}
          {metadata.category && (
            <div className="flex justify-between">
              <span className="text-gray-600 dark:text-gray-400">Category:</span>
              <span className="text-gray-900 dark:text-white">{metadata.category}</span>
            </div>
          )}
          <div className="flex justify-between">
            <span className="text-gray-600 dark:text-gray-400">Save Path:</span>
            <span className="text-gray-900 dark:text-white text-xs truncate">{metadata.save_path}</span>
          </div>
        </div>
      </section>

      {/* Tags */}
      {metadata.tags.length > 0 && (
        <section>
          <h3 className="text-lg font-semibold text-gray-900 dark:text-white mb-3">
            Tags
          </h3>
          <div className="flex flex-wrap gap-2">
            {metadata.tags.map((tag) => (
              <span
                key={tag}
                className="px-3 py-1 bg-gray-200 dark:bg-gray-700 text-gray-800 dark:text-gray-200 rounded-full text-sm"
              >
                {tag}
              </span>
            ))}
          </div>
        </section>
      )}
    </div>
  );
};
