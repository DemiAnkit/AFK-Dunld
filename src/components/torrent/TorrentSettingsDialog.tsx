// src/components/torrent/TorrentSettingsDialog.tsx
import React, { useState, useEffect } from 'react';
import { torrentApi } from '../../services/torrentApi';
import { Modal } from '../common/Modal';
import type { TorrentPriority, BandwidthLimit, TorrentSchedule } from '../../types/torrent';

interface TorrentSettingsDialogProps {
  isOpen: boolean;
  onClose: () => void;
  infoHash: string;
}

export const TorrentSettingsDialog: React.FC<TorrentSettingsDialogProps> = ({
  isOpen,
  onClose,
  infoHash,
}) => {
  const [priority, setPriority] = useState<TorrentPriority>(1);
  const [downloadLimit, setDownloadLimit] = useState<string>('');
  const [uploadLimit, setUploadLimit] = useState<string>('');
  const [bandwidthEnabled, setBandwidthEnabled] = useState(false);
  const [scheduleEnabled, setScheduleEnabled] = useState(false);
  const [startTime, setStartTime] = useState('');
  const [endTime, setEndTime] = useState('');
  const [selectedDays, setSelectedDays] = useState<number[]>([]);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);

  const days = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];

  useEffect(() => {
    if (isOpen) {
      loadSettings();
    }
  }, [isOpen, infoHash]);

  const loadSettings = async () => {
    try {
      setLoading(true);
      const [priorityValue, bandwidthLimit, schedule] = await Promise.all([
        torrentApi.getTorrentPriority(infoHash),
        torrentApi.getTorrentBandwidthLimit(infoHash),
        torrentApi.getTorrentSchedule(infoHash),
      ]);

      setPriority(priorityValue as TorrentPriority);
      
      setBandwidthEnabled(bandwidthLimit.enabled);
      setDownloadLimit(bandwidthLimit.download_limit ? (bandwidthLimit.download_limit / 1024).toString() : '');
      setUploadLimit(bandwidthLimit.upload_limit ? (bandwidthLimit.upload_limit / 1024).toString() : '');

      setScheduleEnabled(schedule.enabled);
      setStartTime(schedule.start_time || '');
      setEndTime(schedule.end_time || '');
      setSelectedDays(schedule.days_of_week);
    } catch (error) {
      console.error('Failed to load settings:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleSave = async () => {
    try {
      setSaving(true);

      // Save priority
      await torrentApi.setTorrentPriority(infoHash, priority);

      // Save bandwidth limits
      const dlLimit = downloadLimit ? parseFloat(downloadLimit) * 1024 : null;
      const ulLimit = uploadLimit ? parseFloat(uploadLimit) * 1024 : null;
      await torrentApi.setTorrentBandwidthLimit(infoHash, dlLimit, ulLimit);

      // Save schedule
      await torrentApi.setTorrentSchedule(
        infoHash,
        scheduleEnabled ? startTime || null : null,
        scheduleEnabled ? endTime || null : null,
        scheduleEnabled ? selectedDays : [],
        scheduleEnabled
      );

      onClose();
    } catch (error) {
      console.error('Failed to save settings:', error);
      alert('Failed to save settings: ' + (error instanceof Error ? error.message : 'Unknown error'));
    } finally {
      setSaving(false);
    }
  };

  const toggleDay = (day: number) => {
    setSelectedDays((prev) =>
      prev.includes(day) ? prev.filter((d) => d !== day) : [...prev, day]
    );
  };

  return (
    <Modal isOpen={isOpen} onClose={onClose} title="Torrent Settings">
      {loading ? (
        <div className="flex items-center justify-center p-8">
          <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500"></div>
        </div>
      ) : (
        <div className="space-y-6">
          {/* Priority */}
          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
              Priority
            </label>
            <select
              value={priority}
              onChange={(e) => setPriority(parseInt(e.target.value) as TorrentPriority)}
              className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white"
            >
              <option value={0}>Low</option>
              <option value={1}>Normal</option>
              <option value={2}>High</option>
              <option value={3}>Critical</option>
            </select>
          </div>

          {/* Bandwidth Limits */}
          <div>
            <div className="flex items-center justify-between mb-3">
              <label className="text-sm font-medium text-gray-700 dark:text-gray-300">
                Bandwidth Limits
              </label>
              <label className="flex items-center">
                <input
                  type="checkbox"
                  checked={bandwidthEnabled}
                  onChange={(e) => setBandwidthEnabled(e.target.checked)}
                  className="mr-2"
                />
                <span className="text-sm text-gray-600 dark:text-gray-400">Enable</span>
              </label>
            </div>
            
            <div className="space-y-3">
              <div>
                <label className="block text-xs text-gray-600 dark:text-gray-400 mb-1">
                  Download Limit (KB/s)
                </label>
                <input
                  type="number"
                  value={downloadLimit}
                  onChange={(e) => setDownloadLimit(e.target.value)}
                  disabled={!bandwidthEnabled}
                  placeholder="Unlimited"
                  min="0"
                  className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white disabled:opacity-50"
                />
              </div>
              <div>
                <label className="block text-xs text-gray-600 dark:text-gray-400 mb-1">
                  Upload Limit (KB/s)
                </label>
                <input
                  type="number"
                  value={uploadLimit}
                  onChange={(e) => setUploadLimit(e.target.value)}
                  disabled={!bandwidthEnabled}
                  placeholder="Unlimited"
                  min="0"
                  className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white disabled:opacity-50"
                />
              </div>
            </div>
          </div>

          {/* Schedule */}
          <div>
            <div className="flex items-center justify-between mb-3">
              <label className="text-sm font-medium text-gray-700 dark:text-gray-300">
                Schedule
              </label>
              <label className="flex items-center">
                <input
                  type="checkbox"
                  checked={scheduleEnabled}
                  onChange={(e) => setScheduleEnabled(e.target.checked)}
                  className="mr-2"
                />
                <span className="text-sm text-gray-600 dark:text-gray-400">Enable</span>
              </label>
            </div>

            <div className="space-y-3">
              <div className="grid grid-cols-2 gap-3">
                <div>
                  <label className="block text-xs text-gray-600 dark:text-gray-400 mb-1">
                    Start Time
                  </label>
                  <input
                    type="time"
                    value={startTime}
                    onChange={(e) => setStartTime(e.target.value)}
                    disabled={!scheduleEnabled}
                    className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white disabled:opacity-50"
                  />
                </div>
                <div>
                  <label className="block text-xs text-gray-600 dark:text-gray-400 mb-1">
                    End Time
                  </label>
                  <input
                    type="time"
                    value={endTime}
                    onChange={(e) => setEndTime(e.target.value)}
                    disabled={!scheduleEnabled}
                    className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white disabled:opacity-50"
                  />
                </div>
              </div>

              <div>
                <label className="block text-xs text-gray-600 dark:text-gray-400 mb-2">
                  Days of Week
                </label>
                <div className="flex gap-2">
                  {days.map((day, index) => (
                    <button
                      key={index}
                      type="button"
                      onClick={() => toggleDay(index)}
                      disabled={!scheduleEnabled}
                      className={`flex-1 px-2 py-2 text-sm rounded-lg transition-colors ${
                        selectedDays.includes(index)
                          ? 'bg-blue-600 text-white'
                          : 'bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300'
                      } ${!scheduleEnabled ? 'opacity-50 cursor-not-allowed' : 'hover:opacity-80'}`}
                    >
                      {day}
                    </button>
                  ))}
                </div>
              </div>
            </div>
          </div>

          {/* Actions */}
          <div className="flex gap-3 pt-4 border-t border-gray-200 dark:border-gray-700">
            <button
              onClick={onClose}
              disabled={saving}
              className="flex-1 px-4 py-2 border border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-300 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-800 disabled:opacity-50"
            >
              Cancel
            </button>
            <button
              onClick={handleSave}
              disabled={saving}
              className="flex-1 px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg disabled:opacity-50 flex items-center justify-center gap-2"
            >
              {saving ? (
                <>
                  <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-white"></div>
                  Saving...
                </>
              ) : (
                'Save Settings'
              )}
            </button>
          </div>
        </div>
      )}
    </Modal>
  );
};
