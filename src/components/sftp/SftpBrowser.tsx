// SFTP Browser Component
import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface SftpFileInfo {
  file_name: string;
  file_size: number | null;
  is_dir: boolean;
  modified: number | null;
  full_path: string;
}

interface SftpConnection {
  url: string;
  password?: string;
  keyPath?: string;
}

export function SftpBrowser() {
  const [connection, setConnection] = useState<SftpConnection>({
    url: '',
    password: '',
    keyPath: '',
  });
  const [connected, setConnected] = useState(false);
  const [currentPath, setCurrentPath] = useState('/');
  const [files, setFiles] = useState<SftpFileInfo[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [selectedFiles, setSelectedFiles] = useState<Set<string>>(new Set());

  const connect = async () => {
    if (!connection.url) {
      setError('Please enter a server URL');
      return;
    }

    setLoading(true);
    setError(null);

    try {
      await invoke('sftp_connect', {
        host: connection.url,
        password: connection.password || null,
        keyPath: connection.keyPath || null,
      });
      setConnected(true);
      await listFiles('/');
    } catch (err) {
      setError(`Connection failed: ${err}`);
    } finally {
      setLoading(false);
    }
  };

  const disconnect = async () => {
    try {
      await invoke('sftp_disconnect');
      setConnected(false);
      setFiles([]);
      setCurrentPath('/');
      setSelectedFiles(new Set());
    } catch (err) {
      console.error('Disconnect error:', err);
    }
  };

  const listFiles = async (path: string) => {
    setLoading(true);
    setError(null);

    try {
      const url = `sftp://${connection.url}${path}`;
      const fileList = await invoke<SftpFileInfo[]>('sftp_list_files', {
        url,
        password: connection.password || null,
        keyPath: connection.keyPath || null,
      });
      setFiles(fileList);
      setCurrentPath(path);
    } catch (err) {
      setError(`Failed to list files: ${err}`);
    } finally {
      setLoading(false);
    }
  };

  const navigateToFolder = (file: SftpFileInfo) => {
    if (file.is_dir) {
      listFiles(file.full_path);
    }
  };

  const navigateUp = () => {
    if (currentPath === '/') return;
    const parts = currentPath.split('/').filter(Boolean);
    parts.pop();
    const newPath = '/' + parts.join('/');
    listFiles(newPath || '/');
  };

  const downloadFile = async (file: SftpFileInfo) => {
    if (file.is_dir) return;

    try {
      const url = `sftp://${connection.url}${file.full_path}`;
      const localPath = `./downloads/${file.file_name}`;
      
      await invoke('sftp_download_file', {
        url,
        localPath,
        password: connection.password || null,
        keyPath: connection.keyPath || null,
        resume: true,
      });

      alert(`Download started: ${file.file_name}`);
    } catch (err) {
      setError(`Download failed: ${err}`);
    }
  };

  const downloadSelected = async () => {
    const selectedFileList = files.filter(f => 
      selectedFiles.has(f.full_path) && !f.is_dir
    );

    for (const file of selectedFileList) {
      await downloadFile(file);
    }

    setSelectedFiles(new Set());
  };

  const toggleFileSelection = (path: string) => {
    const newSelection = new Set(selectedFiles);
    if (newSelection.has(path)) {
      newSelection.delete(path);
    } else {
      newSelection.add(path);
    }
    setSelectedFiles(newSelection);
  };

  const formatSize = (bytes: number | null) => {
    if (bytes === null) return '-';
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return Math.round(bytes / Math.pow(k, i) * 100) / 100 + ' ' + sizes[i];
  };

  const formatDate = (timestamp: number | null) => {
    if (timestamp === null) return '-';
    return new Date(timestamp * 1000).toLocaleString();
  };

  return (
    <div className="sftp-browser p-6 max-w-6xl mx-auto">
      <h2 className="text-2xl font-bold mb-6">SFTP Browser</h2>

      {/* Connection Form */}
      {!connected && (
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow-md p-6 mb-6">
          <h3 className="text-lg font-semibold mb-4">Connect to Server</h3>
          
          <div className="space-y-4">
            <div>
              <label className="block text-sm font-medium mb-2">
                Server URL (user@host:port)
              </label>
              <input
                type="text"
                value={connection.url}
                onChange={(e) => setConnection({ ...connection, url: e.target.value })}
                placeholder="user@example.com:22"
                className="w-full px-3 py-2 border rounded-lg dark:bg-gray-700 dark:border-gray-600"
              />
            </div>

            <div>
              <label className="block text-sm font-medium mb-2">
                Password (optional if using SSH key)
              </label>
              <input
                type="password"
                value={connection.password}
                onChange={(e) => setConnection({ ...connection, password: e.target.value })}
                placeholder="Password"
                className="w-full px-3 py-2 border rounded-lg dark:bg-gray-700 dark:border-gray-600"
              />
            </div>

            <div>
              <label className="block text-sm font-medium mb-2">
                SSH Key Path (optional)
              </label>
              <input
                type="text"
                value={connection.keyPath}
                onChange={(e) => setConnection({ ...connection, keyPath: e.target.value })}
                placeholder="/home/user/.ssh/id_rsa"
                className="w-full px-3 py-2 border rounded-lg dark:bg-gray-700 dark:border-gray-600"
              />
            </div>

            <button
              onClick={connect}
              disabled={loading}
              className="w-full px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:opacity-50"
            >
              {loading ? 'Connecting...' : 'Connect'}
            </button>
          </div>
        </div>
      )}

      {/* File Browser */}
      {connected && (
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow-md">
          {/* Toolbar */}
          <div className="border-b dark:border-gray-700 p-4 flex items-center justify-between">
            <div className="flex items-center gap-3">
              <button
                onClick={navigateUp}
                disabled={currentPath === '/' || loading}
                className="px-3 py-1 bg-gray-200 dark:bg-gray-700 rounded hover:bg-gray-300 dark:hover:bg-gray-600 disabled:opacity-50"
              >
                ↑ Up
              </button>
              
              <button
                onClick={() => listFiles(currentPath)}
                disabled={loading}
                className="px-3 py-1 bg-gray-200 dark:bg-gray-700 rounded hover:bg-gray-300 dark:hover:bg-gray-600 disabled:opacity-50"
              >
                ↻ Refresh
              </button>

              <span className="text-sm font-mono bg-gray-100 dark:bg-gray-700 px-3 py-1 rounded">
                {currentPath}
              </span>
            </div>

            <div className="flex items-center gap-3">
              {selectedFiles.size > 0 && (
                <button
                  onClick={downloadSelected}
                  className="px-4 py-1 bg-green-600 text-white rounded hover:bg-green-700"
                >
                  Download {selectedFiles.size} file(s)
                </button>
              )}
              
              <button
                onClick={disconnect}
                className="px-4 py-1 bg-red-600 text-white rounded hover:bg-red-700"
              >
                Disconnect
              </button>
            </div>
          </div>

          {/* Error Message */}
          {error && (
            <div className="bg-red-100 dark:bg-red-900 text-red-800 dark:text-red-200 p-3 m-4 rounded">
              {error}
            </div>
          )}

          {/* File List */}
          <div className="overflow-x-auto">
            <table className="w-full">
              <thead className="bg-gray-50 dark:bg-gray-900">
                <tr>
                  <th className="w-10 px-4 py-3"></th>
                  <th className="text-left px-4 py-3">Name</th>
                  <th className="text-left px-4 py-3">Size</th>
                  <th className="text-left px-4 py-3">Modified</th>
                  <th className="text-left px-4 py-3">Actions</th>
                </tr>
              </thead>
              <tbody>
                {loading ? (
                  <tr>
                    <td colSpan={5} className="text-center py-8 text-gray-500">
                      Loading...
                    </td>
                  </tr>
                ) : files.length === 0 ? (
                  <tr>
                    <td colSpan={5} className="text-center py-8 text-gray-500">
                      Empty directory
                    </td>
                  </tr>
                ) : (
                  files.map((file) => (
                    <tr
                      key={file.full_path}
                      className={`border-t dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-700 ${
                        selectedFiles.has(file.full_path) ? 'bg-blue-50 dark:bg-blue-900' : ''
                      }`}
                    >
                      <td className="px-4 py-3">
                        {!file.is_dir && (
                          <input
                            type="checkbox"
                            checked={selectedFiles.has(file.full_path)}
                            onChange={() => toggleFileSelection(file.full_path)}
                            className="w-4 h-4"
                          />
                        )}
                      </td>
                      <td
                        className="px-4 py-3 cursor-pointer"
                        onClick={() => navigateToFolder(file)}
                      >
                        <div className="flex items-center gap-2">
                          <span className="text-xl">
                            {file.is_dir ? '📁' : '📄'}
                          </span>
                          <span className={file.is_dir ? 'font-semibold' : ''}>
                            {file.file_name}
                          </span>
                        </div>
                      </td>
                      <td className="px-4 py-3 text-sm text-gray-600 dark:text-gray-400">
                        {formatSize(file.file_size)}
                      </td>
                      <td className="px-4 py-3 text-sm text-gray-600 dark:text-gray-400">
                        {formatDate(file.modified)}
                      </td>
                      <td className="px-4 py-3">
                        {!file.is_dir && (
                          <button
                            onClick={() => downloadFile(file)}
                            className="px-3 py-1 text-sm bg-blue-600 text-white rounded hover:bg-blue-700"
                          >
                            Download
                          </button>
                        )}
                      </td>
                    </tr>
                  ))
                )}
              </tbody>
            </table>
          </div>
        </div>
      )}
    </div>
  );
}
