// src/components/settings/GeneralSettings.tsx
import { useState, useEffect } from 'react';
import { Bell, Clipboard, TestTube, Folder } from 'lucide-react';
import { getSettings, updateSettings, type AppSettings } from '../../services/phase1Api';
import { setClipboardMonitoring, setNotificationsEnabled, testNotification } from '../../services/phase2Api';
import { toast } from 'react-hot-toast';

export function GeneralSettings() {
  const [settings, setSettings] = useState<AppSettings | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadSettings();
  }, []);

  const loadSettings = async () => {
    try {
      const appSettings = await getSettings();
      setSettings(appSettings);
    } catch (error) {
      toast.error('Failed to load settings');
    } finally {
      setLoading(false);
    }
  };

  const handleToggle = async (key: keyof AppSettings, value: boolean) => {
    if (!settings) return;

    try {
      const updated = { ...settings, [key]: value };
      await updateSettings(updated);
      setSettings(updated);

      // Apply service-specific settings
      if (key === 'monitor_clipboard') {
        await setClipboardMonitoring(value);
      } else if (key === 'show_notifications') {
        await setNotificationsEnabled(value);
      }

      toast.success('Setting updated');
    } catch (error) {
      toast.error('Failed to update setting');
    }
  };

  const handleTestNotification = async () => {
    try {
      await testNotification();
      toast.success('Test notification sent!');
    } catch (error) {
      toast.error('Failed to send notification');
    }
  };

  if (loading) {
    return (
      <div className="flex justify-center py-12">
        <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-500"></div>
      </div>
    );
  }

  if (!settings) return null;

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-2xl font-bold text-white mb-2">General Settings</h2>
        <p className="text-gray-400">Configure application behavior and notifications</p>
      </div>

      {/* Clipboard Monitoring */}
      <div className="bg-gray-900/50 rounded-xl p-6 border border-gray-700">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-3">
            <Clipboard className="w-5 h-5 text-blue-400" />
            <div>
              <h3 className="text-lg font-semibold text-white">Monitor Clipboard</h3>
              <p className="text-sm text-gray-400">Automatically detect download URLs copied to clipboard</p>
            </div>
          </div>
          <label className="relative inline-flex items-center cursor-pointer">
            <input
              type="checkbox"
              checked={settings.monitor_clipboard}
              onChange={(e) => handleToggle('monitor_clipboard', e.target.checked)}
              className="sr-only peer"
            />
            <div className="w-11 h-6 bg-gray-700 peer-focus:outline-none peer-focus:ring-4 
                          peer-focus:ring-blue-800 rounded-full peer 
                          peer-checked:after:translate-x-full peer-checked:after:border-white 
                          after:content-[''] after:absolute after:top-[2px] after:left-[2px] 
                          after:bg-white after:border-gray-300 after:border after:rounded-full 
                          after:h-5 after:w-5 after:transition-all peer-checked:bg-blue-600"></div>
          </label>
        </div>
      </div>

      {/* Notifications */}
      <div className="bg-gray-900/50 rounded-xl p-6 border border-gray-700">
        <div className="flex items-center justify-between mb-4">
          <div className="flex items-center gap-3">
            <Bell className="w-5 h-5 text-green-400" />
            <div>
              <h3 className="text-lg font-semibold text-white">Desktop Notifications</h3>
              <p className="text-sm text-gray-400">Show notifications for download events</p>
            </div>
          </div>
          <label className="relative inline-flex items-center cursor-pointer">
            <input
              type="checkbox"
              checked={settings.show_notifications}
              onChange={(e) => handleToggle('show_notifications', e.target.checked)}
              className="sr-only peer"
            />
            <div className="w-11 h-6 bg-gray-700 peer-focus:outline-none peer-focus:ring-4 
                          peer-focus:ring-blue-800 rounded-full peer 
                          peer-checked:after:translate-x-full peer-checked:after:border-white 
                          after:content-[''] after:absolute after:top-[2px] after:left-[2px] 
                          after:bg-white after:border-gray-300 after:border after:rounded-full 
                          after:h-5 after:w-5 after:transition-all peer-checked:bg-blue-600"></div>
          </label>
        </div>
        
        {settings.show_notifications && (
          <button
            onClick={handleTestNotification}
            className="flex items-center gap-2 px-4 py-2 bg-green-600 hover:bg-green-700 
                     text-white rounded-lg transition-colors text-sm"
          >
            <TestTube className="w-4 h-4" />
            Test Notification
          </button>
        )}
      </div>

      {/* Auto Start Downloads */}
      <div className="bg-gray-900/50 rounded-xl p-6 border border-gray-700">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-3">
            <Folder className="w-5 h-5 text-purple-400" />
            <div>
              <h3 className="text-lg font-semibold text-white">Auto-Start Downloads</h3>
              <p className="text-sm text-gray-400">Automatically start downloads when added</p>
            </div>
          </div>
          <label className="relative inline-flex items-center cursor-pointer">
            <input
              type="checkbox"
              checked={settings.auto_start_downloads}
              onChange={(e) => handleToggle('auto_start_downloads', e.target.checked)}
              className="sr-only peer"
            />
            <div className="w-11 h-6 bg-gray-700 peer-focus:outline-none peer-focus:ring-4 
                          peer-focus:ring-blue-800 rounded-full peer 
                          peer-checked:after:translate-x-full peer-checked:after:border-white 
                          after:content-[''] after:absolute after:top-[2px] after:left-[2px] 
                          after:bg-white after:border-gray-300 after:border after:rounded-full 
                          after:h-5 after:w-5 after:transition-all peer-checked:bg-blue-600"></div>
          </label>
        </div>
      </div>
    </div>
  );
}
