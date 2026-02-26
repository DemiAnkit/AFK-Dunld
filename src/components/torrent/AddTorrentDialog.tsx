// src/components/torrent/AddTorrentDialog.tsx
import React, { useState } from 'react';
import { open } from '@tauri-apps/plugin-dialog';
import { torrentApi } from '../../services/torrentApi';
import { Modal } from '../common/Modal';

interface AddTorrentDialogProps {
  isOpen: boolean;
  onClose: () => void;
  onAdded?: (infoHash: string) => void;
}

export const AddTorrentDialog: React.FC<AddTorrentDialogProps> = ({
  isOpen,
  onClose,
  onAdded,
}) => {
  const [activeTab, setActiveTab] = useState<'file' | 'magnet'>('file');
  const [magnetLink, setMagnetLink] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleFileSelect = async () => {
    try {
      const selected = await open({
        multiple: false,
        filters: [
          {
            name: 'Torrent Files',
            extensions: ['torrent'],
          },
        ],
      });

      if (selected && typeof selected === 'string') {
        setLoading(true);
        setError(null);
        
        const infoHash = await torrentApi.addTorrentFile(selected);
        onAdded?.(infoHash);
        onClose();
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to add torrent file');
    } finally {
      setLoading(false);
    }
  };

  const handleMagnetSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    
    if (!magnetLink.trim()) {
      setError('Please enter a magnet link');
      return;
    }

    if (!magnetLink.startsWith('magnet:?')) {
      setError('Invalid magnet link format');
      return;
    }

    try {
      setLoading(true);
      setError(null);
      
      const infoHash = await torrentApi.addMagnetLink(magnetLink);
      onAdded?.(infoHash);
      setMagnetLink('');
      onClose();
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to add magnet link');
    } finally {
      setLoading(false);
    }
  };

  return (
    <Modal isOpen={isOpen} onClose={onClose} title="Add Torrent">
      <div className="space-y-4">
        {/* Tabs */}
        <div className="flex border-b border-gray-200 dark:border-gray-700">
          <button
            className={`px-4 py-2 font-medium text-sm ${
              activeTab === 'file'
                ? 'border-b-2 border-blue-500 text-blue-600 dark:text-blue-400'
                : 'text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-300'
            }`}
            onClick={() => setActiveTab('file')}
          >
            Torrent File
          </button>
          <button
            className={`px-4 py-2 font-medium text-sm ${
              activeTab === 'magnet'
                ? 'border-b-2 border-blue-500 text-blue-600 dark:text-blue-400'
                : 'text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-300'
            }`}
            onClick={() => setActiveTab('magnet')}
          >
            Magnet Link
          </button>
        </div>

        {/* Error message */}
        {error && (
          <div className="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 text-red-800 dark:text-red-200 px-4 py-3 rounded">
            {error}
          </div>
        )}

        {/* File tab */}
        {activeTab === 'file' && (
          <div className="space-y-4">
            <p className="text-sm text-gray-600 dark:text-gray-400">
              Select a .torrent file to add to the download queue.
            </p>
            <button
              onClick={handleFileSelect}
              disabled={loading}
              className="w-full px-4 py-3 bg-blue-600 hover:bg-blue-700 disabled:bg-gray-400 text-white rounded-lg font-medium transition-colors flex items-center justify-center gap-2"
            >
              {loading ? (
                <>
                  <div className="animate-spin rounded-full h-5 w-5 border-b-2 border-white"></div>
                  Adding...
                </>
              ) : (
                <>
                  <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12" />
                  </svg>
                  Select Torrent File
                </>
              )}
            </button>
          </div>
        )}

        {/* Magnet tab */}
        {activeTab === 'magnet' && (
          <form onSubmit={handleMagnetSubmit} className="space-y-4">
            <div>
              <label htmlFor="magnet" className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                Magnet Link
              </label>
              <textarea
                id="magnet"
                value={magnetLink}
                onChange={(e) => setMagnetLink(e.target.value)}
                placeholder="magnet:?xt=urn:btih:..."
                rows={4}
                className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white focus:ring-2 focus:ring-blue-500 focus:border-transparent"
              />
              <p className="mt-1 text-xs text-gray-500 dark:text-gray-400">
                Paste a magnet link starting with "magnet:?"
              </p>
            </div>

            <button
              type="submit"
              disabled={loading || !magnetLink.trim()}
              className="w-full px-4 py-3 bg-blue-600 hover:bg-blue-700 disabled:bg-gray-400 text-white rounded-lg font-medium transition-colors flex items-center justify-center gap-2"
            >
              {loading ? (
                <>
                  <div className="animate-spin rounded-full h-5 w-5 border-b-2 border-white"></div>
                  Adding...
                </>
              ) : (
                <>
                  <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 4v16m8-8H4" />
                  </svg>
                  Add Magnet Link
                </>
              )}
            </button>
          </form>
        )}
      </div>
    </Modal>
  );
};
