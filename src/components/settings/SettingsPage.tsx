// src/components/settings/SettingsPage.tsx
import React from "react";
import { useSettingsStore } from "../../stores/settingsStore";
import { useTheme } from "../../hooks/useTheme";
import { 
  Palette, 
  Download, 
  Sliders,
  Bell,
  Globe,
  Sun,
  Moon
} from "lucide-react";
import { YouTubeSettings } from "./YouTubeSettings";
import BrowserIntegrationSettings from "./BrowserIntegrationSettings";

export function SettingsPage() {
  const { settings, updateSettings, loadSettings } = useSettingsStore();
  const { theme, setTheme } = useTheme();

  React.useEffect(() => {
    loadSettings();
  }, [loadSettings]);

  if (!settings) {
    return (
      <div className="max-w-4xl mx-auto p-6">
        <div className="flex items-center justify-center h-64">
          <p className="text-gray-500 dark:text-gray-400">Loading settings...</p>
        </div>
      </div>
    );
  }

  return (
    <div className="max-w-4xl mx-auto p-6">
      <h2 className="text-2xl font-bold text-gray-900 dark:text-white mb-6">Settings</h2>
      
      <div className="space-y-6">
        {/* General Settings */}
        <section className="bg-white dark:bg-gray-900 rounded-lg p-6 border border-gray-200 dark:border-gray-800">
          <div className="flex items-center gap-3 mb-4">
            <Sliders className="w-5 h-5 text-blue-500" />
            <h3 className="text-lg font-semibold text-gray-900 dark:text-white">General</h3>
          </div>
          
          <div className="space-y-4">
            <div>
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                Download Location
              </label>
              <div className="flex gap-2">
                <input
                  type="text"
                  value={settings?.downloadPath || ''}
                  readOnly
                  className="flex-1 px-3 py-2 bg-gray-100 dark:bg-gray-800 border border-gray-300 dark:border-gray-700 
                           rounded-lg text-gray-900 dark:text-white text-sm"
                  placeholder="Default download folder"
                />
                <button
                  className="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white 
                           rounded-lg text-sm transition-colors"
                >
                  Browse
                </button>
              </div>
            </div>

            <div className="flex items-center justify-between">
              <div>
                <label className="text-sm font-medium text-gray-700 dark:text-gray-300">
                  Auto-start Downloads
                </label>
                <p className="text-xs text-gray-500">
                  Automatically start downloading when URL is added
                </p>
              </div>
              <button
                onClick={() => updateSettings({ autoStartDownloads: !settings?.autoStartDownloads })}
                className={`w-12 h-6 rounded-full transition-colors ${
                  settings?.autoStartDownloads ? "bg-blue-600" : "bg-gray-300 dark:bg-gray-700"
                }`}
              >
                <div
                  className={`w-5 h-5 bg-white rounded-full transition-transform ${
                    settings?.autoStartDownloads ? "translate-x-6" : "translate-x-1"
                  }`}
                />
              </button>
            </div>

            <div className="flex items-center justify-between">
              <div>
                <label className="text-sm font-medium text-gray-700 dark:text-gray-300">
                  Minimize to Tray
                </label>
                <p className="text-xs text-gray-500">
                  Keep running in system tray when closed
                </p>
              </div>
              <button
                onClick={() => updateSettings({ minimizeToTray: !settings?.minimizeToTray })}
                className={`w-12 h-6 rounded-full transition-colors ${
                  settings?.minimizeToTray ? "bg-blue-600" : "bg-gray-300 dark:bg-gray-700"
                }`}
              >
                <div
                  className={`w-5 h-5 bg-white rounded-full transition-transform ${
                    settings?.minimizeToTray ? "translate-x-6" : "translate-x-1"
                  }`}
                />
              </button>
            </div>
          </div>
        </section>

        {/* Download Settings */}
        <section className="bg-white dark:bg-gray-900 rounded-lg p-6 border border-gray-200 dark:border-gray-800">
          <div className="flex items-center gap-3 mb-4">
            <Download className="w-5 h-5 text-green-500" />
            <h3 className="text-lg font-semibold text-gray-900 dark:text-white">Downloads</h3>
          </div>
          
          <div className="space-y-4">
            <div>
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                Max Concurrent Downloads: {settings?.maxConcurrentDownloads || 3}
              </label>
              <input
                type="range"
                min="1"
                max="10"
                value={settings?.maxConcurrentDownloads || 3}
                onChange={(e) => updateSettings({ maxConcurrentDownloads: parseInt(e.target.value) })}
                className="w-full h-2 bg-gray-200 dark:bg-gray-700 rounded-lg appearance-none 
                         cursor-pointer accent-blue-500"
              />
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                Speed Limit (KB/s): {(settings?.maxDownloadSpeed || 0) === 0 ? "Unlimited" : settings?.maxDownloadSpeed}
              </label>
              <input
                type="range"
                min="0"
                max="10240"
                step="100"
                value={settings?.maxDownloadSpeed || 0}
                onChange={(e) => updateSettings({ maxDownloadSpeed: parseInt(e.target.value) })}
                className="w-full h-2 bg-gray-200 dark:bg-gray-700 rounded-lg appearance-none 
                         cursor-pointer accent-blue-500"
              />
              <p className="text-xs text-gray-500 mt-1">0 = Unlimited</p>
            </div>
          </div>
        </section>

        {/* Theme Settings */}
        <section className="bg-white dark:bg-gray-900 rounded-lg p-6 border border-gray-200 dark:border-gray-800">
          <div className="flex items-center gap-3 mb-4">
            <Palette className="w-5 h-5 text-purple-500" />
            <h3 className="text-lg font-semibold text-gray-900 dark:text-white">Appearance</h3>
          </div>
          
          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-3">
              Theme
            </label>
            <div className="grid grid-cols-2 gap-3">
              <button
                onClick={() => setTheme('dark')}
                className={`flex items-center gap-3 px-4 py-3 rounded-lg transition-colors ${
                  theme === 'dark'
                    ? "bg-blue-600 text-white"
                    : "bg-gray-200 dark:bg-gray-800 text-gray-700 dark:text-gray-300 hover:bg-gray-300 dark:hover:bg-gray-700"
                }`}
              >
                <Moon className="w-5 h-5" />
                <span className="font-medium">Dark</span>
              </button>
              <button
                onClick={() => setTheme('light')}
                className={`flex items-center gap-3 px-4 py-3 rounded-lg transition-colors ${
                  theme === 'light'
                    ? "bg-blue-600 text-white"
                    : "bg-gray-200 dark:bg-gray-800 text-gray-700 dark:text-gray-300 hover:bg-gray-300 dark:hover:bg-gray-700"
                }`}
              >
                <Sun className="w-5 h-5" />
                <span className="font-medium">Light</span>
              </button>
            </div>
          </div>
        </section>

        {/* Notifications */}
        <section className="bg-white dark:bg-gray-900 rounded-lg p-6 border border-gray-200 dark:border-gray-800">
          <div className="flex items-center gap-3 mb-4">
            <Bell className="w-5 h-5 text-yellow-500" />
            <h3 className="text-lg font-semibold text-gray-900 dark:text-white">Notifications</h3>
          </div>
          
          <div className="flex items-center justify-between">
            <div>
              <label className="text-sm font-medium text-gray-700 dark:text-gray-300">
                Show Notifications
              </label>
              <p className="text-xs text-gray-500">
                Display notifications when downloads complete
              </p>
            </div>
            <button
              onClick={() => updateSettings({ showNotifications: !settings?.showNotifications })}
              className={`w-12 h-6 rounded-full transition-colors ${
                settings?.showNotifications ? "bg-blue-600" : "bg-gray-300 dark:bg-gray-700"
              }`}
            >
              <div
                className={`w-5 h-5 bg-white rounded-full transition-transform ${
                  settings?.showNotifications ? "translate-x-6" : "translate-x-1"
                }`}
              />
            </button>
          </div>
        </section>

        {/* YouTube Settings */}
        <section className="bg-white dark:bg-gray-900 rounded-lg p-6 border border-gray-200 dark:border-gray-800">
          <YouTubeSettings />
        </section>

        {/* Browser Integration */}
        <section className="bg-white dark:bg-gray-900 rounded-lg p-6 border border-gray-200 dark:border-gray-800">
          <div className="flex items-center gap-3 mb-4">
            <Globe className="w-5 h-5 text-cyan-500" />
            <h3 className="text-lg font-semibold text-gray-900 dark:text-white">Browser Integration</h3>
          </div>
          <BrowserIntegrationSettings />
        </section>
      </div>
    </div>
  );
}
