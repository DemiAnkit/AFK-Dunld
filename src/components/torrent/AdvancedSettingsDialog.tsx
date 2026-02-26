// src/components/torrent/AdvancedSettingsDialog.tsx
import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Modal } from '../common/Modal';

interface AdvancedSettingsDialogProps {
  isOpen: boolean;
  onClose: () => void;
  infoHash: string;
}

interface WebSeed {
  url: string;
  seed_type: 'GetRight' | 'WebSeed';
}

interface EncryptionConfig {
  enabled: boolean;
  mode: 'Disabled' | 'Enabled' | 'Required';
  prefer_encrypted: boolean;
}

interface IpFilter {
  blocked_ips: string[];
  blocked_ranges: string[];
  enabled: boolean;
}

export const AdvancedSettingsDialog: React.FC<AdvancedSettingsDialogProps> = ({
  isOpen,
  onClose,
  infoHash,
}) => {
  const [activeTab, setActiveTab] = useState<'webseeds' | 'encryption' | 'ipfilter' | 'limits'>('webseeds');
  const [webSeeds, setWebSeeds] = useState<WebSeed[]>([]);
  const [newWebSeedUrl, setNewWebSeedUrl] = useState('');
  const [webSeedType, setWebSeedType] = useState<'GetRight' | 'WebSeed'>('WebSeed');
  
  const [encryption, setEncryption] = useState<EncryptionConfig>({
    enabled: true,
    mode: 'Enabled',
    prefer_encrypted: true,
  });

  const [ipFilter, setIpFilter] = useState<IpFilter>({
    blocked_ips: [],
    blocked_ranges: [],
    enabled: false,
  });

  const [newBlockedIp, setNewBlockedIp] = useState('');
  const [seedRatioLimit, setSeedRatioLimit] = useState<string>('2.0');
  const [maxConnections, setMaxConnections] = useState<string>('200');
  
  const [loading, setLoading] = useState(false);
  const [saving, setSaving] = useState(false);

  useEffect(() => {
    if (isOpen) {
      loadSettings();
    }
  }, [isOpen, infoHash]);

  const loadSettings = async () => {
    try {
      setLoading(true);
      
      // Load web seeds
      const seeds = await invoke<WebSeed[]>('get_web_seeds', { infoHash });
      setWebSeeds(seeds);

      // Load encryption config
      const encConfig = await invoke<EncryptionConfig>('get_encryption_config', { infoHash });
      setEncryption(encConfig);

      // Load IP filter
      const filter = await invoke<IpFilter>('get_ip_filter', { infoHash });
      setIpFilter(filter);

    } catch (error) {
      console.error('Failed to load advanced settings:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleAddWebSeed = async () => {
    if (!newWebSeedUrl.trim()) return;

    try {
      await invoke('add_web_seed', {
        infoHash,
        url: newWebSeedUrl,
        seedType: webSeedType,
      });

      setWebSeeds([...webSeeds, { url: newWebSeedUrl, seed_type: webSeedType }]);
      setNewWebSeedUrl('');
    } catch (error) {
      console.error('Failed to add web seed:', error);
      alert('Failed to add web seed: ' + (error instanceof Error ? error.message : 'Unknown error'));
    }
  };

  const handleRemoveWebSeed = async (url: string) => {
    try {
      await invoke('remove_web_seed', { infoHash, url });
      setWebSeeds(webSeeds.filter(ws => ws.url !== url));
    } catch (error) {
      console.error('Failed to remove web seed:', error);
    }
  };

  const handleSaveEncryption = async () => {
    try {
      setSaving(true);
      await invoke('set_encryption_config', {
        infoHash,
        enabled: encryption.enabled,
        mode: encryption.mode,
        preferEncrypted: encryption.prefer_encrypted,
      });
    } catch (error) {
      console.error('Failed to save encryption config:', error);
      alert('Failed to save encryption settings');
    } finally {
      setSaving(false);
    }
  };

  const handleAddBlockedIp = async () => {
    if (!newBlockedIp.trim()) return;

    try {
      await invoke('add_blocked_ip', { infoHash, ip: newBlockedIp });
      setIpFilter({
        ...ipFilter,
        blocked_ips: [...ipFilter.blocked_ips, newBlockedIp],
        enabled: true,
      });
      setNewBlockedIp('');
    } catch (error) {
      console.error('Failed to add blocked IP:', error);
    }
  };

  const handleRemoveBlockedIp = async (ip: string) => {
    try {
      await invoke('remove_blocked_ip', { infoHash, ip });
      setIpFilter({
        ...ipFilter,
        blocked_ips: ipFilter.blocked_ips.filter(i => i !== ip),
      });
    } catch (error) {
      console.error('Failed to remove blocked IP:', error);
    }
  };

  const handleSaveLimits = async () => {
    try {
      setSaving(true);

      // Save seed ratio limit
      const ratio = seedRatioLimit ? parseFloat(seedRatioLimit) : null;
      await invoke('set_seed_ratio_limit', { infoHash, ratio });

      // Save max connections
      const connections = maxConnections ? parseInt(maxConnections) : null;
      await invoke('set_max_connections', { infoHash, maxConnections: connections });

      alert('Limits saved successfully!');
    } catch (error) {
      console.error('Failed to save limits:', error);
      alert('Failed to save limits');
    } finally {
      setSaving(false);
    }
  };

  return (
    <Modal isOpen={isOpen} onClose={onClose} title="Advanced Torrent Settings" size="large">
      {loading ? (
        <div className="flex items-center justify-center p-8">
          <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500"></div>
        </div>
      ) : (
        <div className="space-y-4">
          {/* Tabs */}
          <div className="flex border-b border-gray-200 dark:border-gray-700">
            <button
              className={`px-4 py-2 font-medium text-sm ${
                activeTab === 'webseeds'
                  ? 'border-b-2 border-blue-500 text-blue-600 dark:text-blue-400'
                  : 'text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-300'
              }`}
              onClick={() => setActiveTab('webseeds')}
            >
              Web Seeds
            </button>
            <button
              className={`px-4 py-2 font-medium text-sm ${
                activeTab === 'encryption'
                  ? 'border-b-2 border-blue-500 text-blue-600 dark:text-blue-400'
                  : 'text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-300'
              }`}
              onClick={() => setActiveTab('encryption')}
            >
              Encryption
            </button>
            <button
              className={`px-4 py-2 font-medium text-sm ${
                activeTab === 'ipfilter'
                  ? 'border-b-2 border-blue-500 text-blue-600 dark:text-blue-400'
                  : 'text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-300'
              }`}
              onClick={() => setActiveTab('ipfilter')}
            >
              IP Filter
            </button>
            <button
              className={`px-4 py-2 font-medium text-sm ${
                activeTab === 'limits'
                  ? 'border-b-2 border-blue-500 text-blue-600 dark:text-blue-400'
                  : 'text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-300'
              }`}
              onClick={() => setActiveTab('limits')}
            >
              Limits
            </button>
          </div>

          {/* Web Seeds Tab */}
          {activeTab === 'webseeds' && (
            <div className="space-y-4">
              <p className="text-sm text-gray-600 dark:text-gray-400">
                Web seeds provide HTTP/HTTPS fallback sources for faster downloads.
              </p>

              <div className="space-y-3">
                <div>
                  <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                    Add Web Seed URL
                  </label>
                  <div className="flex gap-2">
                    <input
                      type="url"
                      value={newWebSeedUrl}
                      onChange={(e) => setNewWebSeedUrl(e.target.value)}
                      placeholder="https://example.com/files/"
                      className="flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white"
                    />
                    <select
                      value={webSeedType}
                      onChange={(e) => setWebSeedType(e.target.value as 'GetRight' | 'WebSeed')}
                      className="px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white"
                    >
                      <option value="WebSeed">WebSeed</option>
                      <option value="GetRight">GetRight</option>
                    </select>
                    <button
                      onClick={handleAddWebSeed}
                      className="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg"
                    >
                      Add
                    </button>
                  </div>
                </div>

                <div className="space-y-2">
                  <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">
                    Current Web Seeds ({webSeeds.length})
                  </label>
                  {webSeeds.length === 0 ? (
                    <p className="text-sm text-gray-500 dark:text-gray-400">No web seeds configured</p>
                  ) : (
                    <div className="space-y-2">
                      {webSeeds.map((seed, index) => (
                        <div
                          key={index}
                          className="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-800 rounded-lg"
                        >
                          <div>
                            <div className="text-sm font-medium text-gray-900 dark:text-white">
                              {seed.url}
                            </div>
                            <div className="text-xs text-gray-500 dark:text-gray-400">
                              Type: {seed.seed_type}
                            </div>
                          </div>
                          <button
                            onClick={() => handleRemoveWebSeed(seed.url)}
                            className="p-2 text-red-600 hover:bg-red-50 dark:hover:bg-red-900/20 rounded"
                          >
                            <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
                            </svg>
                          </button>
                        </div>
                      ))}
                    </div>
                  )}
                </div>
              </div>
            </div>
          )}

          {/* Encryption Tab */}
          {activeTab === 'encryption' && (
            <div className="space-y-4">
              <p className="text-sm text-gray-600 dark:text-gray-400">
                Configure protocol encryption (MSE/PE) for enhanced privacy.
              </p>

              <div className="space-y-4">
                <label className="flex items-center">
                  <input
                    type="checkbox"
                    checked={encryption.enabled}
                    onChange={(e) => setEncryption({ ...encryption, enabled: e.target.checked })}
                    className="mr-2"
                  />
                  <span className="text-sm font-medium text-gray-700 dark:text-gray-300">
                    Enable Protocol Encryption
                  </span>
                </label>

                <div>
                  <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                    Encryption Mode
                  </label>
                  <select
                    value={encryption.mode}
                    onChange={(e) => setEncryption({ ...encryption, mode: e.target.value as any })}
                    disabled={!encryption.enabled}
                    className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white disabled:opacity-50"
                  >
                    <option value="Disabled">Disabled</option>
                    <option value="Enabled">Enabled (Optional)</option>
                    <option value="Required">Required (Forced)</option>
                  </select>
                </div>

                <label className="flex items-center">
                  <input
                    type="checkbox"
                    checked={encryption.prefer_encrypted}
                    onChange={(e) => setEncryption({ ...encryption, prefer_encrypted: e.target.checked })}
                    disabled={!encryption.enabled}
                    className="mr-2"
                  />
                  <span className="text-sm text-gray-700 dark:text-gray-300">
                    Prefer Encrypted Connections
                  </span>
                </label>

                <button
                  onClick={handleSaveEncryption}
                  disabled={saving}
                  className="px-4 py-2 bg-blue-600 hover:bg-blue-700 disabled:bg-gray-400 text-white rounded-lg"
                >
                  {saving ? 'Saving...' : 'Save Encryption Settings'}
                </button>
              </div>
            </div>
          )}

          {/* IP Filter Tab */}
          {activeTab === 'ipfilter' && (
            <div className="space-y-4">
              <p className="text-sm text-gray-600 dark:text-gray-400">
                Block specific IP addresses from connecting to this torrent.
              </p>

              <div className="space-y-3">
                <div>
                  <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                    Add Blocked IP
                  </label>
                  <div className="flex gap-2">
                    <input
                      type="text"
                      value={newBlockedIp}
                      onChange={(e) => setNewBlockedIp(e.target.value)}
                      placeholder="192.168.1.100"
                      className="flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white"
                    />
                    <button
                      onClick={handleAddBlockedIp}
                      className="px-4 py-2 bg-red-600 hover:bg-red-700 text-white rounded-lg"
                    >
                      Block
                    </button>
                  </div>
                </div>

                <div className="space-y-2">
                  <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">
                    Blocked IPs ({ipFilter.blocked_ips.length})
                  </label>
                  {ipFilter.blocked_ips.length === 0 ? (
                    <p className="text-sm text-gray-500 dark:text-gray-400">No IPs blocked</p>
                  ) : (
                    <div className="space-y-1">
                      {ipFilter.blocked_ips.map((ip, index) => (
                        <div
                          key={index}
                          className="flex items-center justify-between p-2 bg-gray-50 dark:bg-gray-800 rounded"
                        >
                          <span className="text-sm font-mono text-gray-900 dark:text-white">{ip}</span>
                          <button
                            onClick={() => handleRemoveBlockedIp(ip)}
                            className="text-red-600 hover:bg-red-50 dark:hover:bg-red-900/20 p-1 rounded"
                          >
                            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
                            </svg>
                          </button>
                        </div>
                      ))}
                    </div>
                  )}
                </div>
              </div>
            </div>
          )}

          {/* Limits Tab */}
          {activeTab === 'limits' && (
            <div className="space-y-4">
              <p className="text-sm text-gray-600 dark:text-gray-400">
                Configure seeding and connection limits.
              </p>

              <div className="space-y-4">
                <div>
                  <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                    Seed Ratio Limit
                  </label>
                  <input
                    type="number"
                    step="0.1"
                    value={seedRatioLimit}
                    onChange={(e) => setSeedRatioLimit(e.target.value)}
                    placeholder="2.0"
                    className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white"
                  />
                  <p className="mt-1 text-xs text-gray-500 dark:text-gray-400">
                    Stop seeding after this upload/download ratio (e.g., 2.0 = 200%)
                  </p>
                </div>

                <div>
                  <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                    Maximum Connections
                  </label>
                  <input
                    type="number"
                    value={maxConnections}
                    onChange={(e) => setMaxConnections(e.target.value)}
                    placeholder="200"
                    className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white"
                  />
                  <p className="mt-1 text-xs text-gray-500 dark:text-gray-400">
                    Maximum number of peer connections for this torrent
                  </p>
                </div>

                <button
                  onClick={handleSaveLimits}
                  disabled={saving}
                  className="px-4 py-2 bg-blue-600 hover:bg-blue-700 disabled:bg-gray-400 text-white rounded-lg"
                >
                  {saving ? 'Saving...' : 'Save Limits'}
                </button>
              </div>
            </div>
          )}

          {/* Close Button */}
          <div className="flex justify-end pt-4 border-t border-gray-200 dark:border-gray-700">
            <button
              onClick={onClose}
              className="px-4 py-2 border border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-300 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-800"
            >
              Close
            </button>
          </div>
        </div>
      )}
    </Modal>
  );
};
