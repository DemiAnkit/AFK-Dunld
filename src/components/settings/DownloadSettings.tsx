// src/components/settings/DownloadSettings.tsx
import { useEffect, useState } from 'react';
import { getSettings, updateSettings, setMaxConcurrent, setSpeedLimit, type AppSettings } from '../../services/phase1Api';
import { toast } from 'react-hot-toast';
import { Download, Gauge, Layers, Save } from 'lucide-react';

export function DownloadSettings() {
  const [settings, setSettings] = useState<AppSettings | null>(null);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);

  // Local state for inputs
  const [maxConcurrent, setMaxConcurrentLocal] = useState(3);
  const [speedLimit, setSpeedLimitLocal] = useState(0);
  const [defaultSegments, setDefaultSegments] = useState(4);

  useEffect(() => {
    loadSettings();
  }, []);

  const loadSettings = async () => {
    try {
      const appSettings = await getSettings();
      setSettings(appSettings);
      setMaxConcurrentLocal(appSettings.max_concurrent_downloads);
      setSpeedLimitLocal(appSettings.speed_limit);
      setDefaultSegments(appSettings.default_segments);
    } catch (error) {
      toast.error('Failed to load settings');
      console.error(error);
    } finally {
      setLoading(false);
    }
  };

  const handleSave = async () => {
    if (!settings) return;

    try {
      setSaving(true);

      const updatedSettings: AppSettings = {
        ...settings,
        max_concurrent_downloads: maxConcurrent,
        speed_limit: speedLimit,
        default_segments: defaultSegments,
      };

      await updateSettings(updatedSettings);
      
      // Apply settings immediately
      await setMaxConcurrent(maxConcurrent);
      await setSpeedLimit(speedLimit > 0 ? speedLimit : null);

      setSettings(updatedSettings);
      toast.success('Settings saved successfully!');
    } catch (error) {
      toast.error('Failed to save settings');
      console.error(error);
    } finally {
      setSaving(false);
    }
  };

  if (loading) {
    return (
      <div className="flex justify-center py-12">
        <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-500"></div>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-2xl font-bold text-white mb-2">Download Settings</h2>
        <p className="text-gray-400">Configure download behavior and performance</p>
      </div>

      {/* Max Concurrent Downloads */}
      <div className="bg-gray-900/50 rounded-xl p-6 border border-gray-700">
        <div className="flex items-center gap-3 mb-4">
          <Layers className="w-5 h-5 text-blue-400" />
          <h3 className="text-lg font-semibold text-white">Concurrent Downloads</h3>
        </div>
        <div className="space-y-4">
          <div>
            <label className="block text-sm text-gray-400 mb-2">
              Maximum concurrent downloads: <span className="text-white font-bold">{maxConcurrent}</span>
            </label>
            <input
              type="range"
              min="1"
              max="10"
              value={maxConcurrent}
              onChange={(e) => setMaxConcurrentLocal(parseInt(e.target.value))}
              className="w-full h-2 bg-gray-700 rounded-lg appearance-none cursor-pointer 
                       slider-thumb:appearance-none slider-thumb:w-4 slider-thumb:h-4 
                       slider-thumb:bg-blue-500 slider-thumb:rounded-full"
            />
            <div className="flex justify-between text-xs text-gray-500 mt-1">
              <span>1</span>
              <span>5</span>
              <span>10</span>
            </div>
          </div>
          <p className="text-sm text-gray-500">
            Number of downloads that can run simultaneously. Higher values may increase overall speed but use more resources.
          </p>
        </div>
      </div>

      {/* Speed Limit */}
      <div className="bg-gray-900/50 rounded-xl p-6 border border-gray-700">
        <div className="flex items-center gap-3 mb-4">
          <Gauge className="w-5 h-5 text-green-400" />
          <h3 className="text-lg font-semibold text-white">Speed Limit</h3>
        </div>
        <div className="space-y-4">
          <div>
            <label className="block text-sm text-gray-400 mb-2">
              {speedLimit === 0 ? (
                'No limit (Unlimited)'
              ) : (
                <>Limit: <span className="text-white font-bold">{(speedLimit / 1024 / 1024).toFixed(1)} MB/s</span></>
              )}
            </label>
            <input
              type="range"
              min="0"
              max="104857600"
              step="1048576"
              value={speedLimit}
              onChange={(e) => setSpeedLimitLocal(parseInt(e.target.value))}
              className="w-full h-2 bg-gray-700 rounded-lg appearance-none cursor-pointer"
            />
            <div className="flex justify-between text-xs text-gray-500 mt-1">
              <span>Unlimited</span>
              <span>50 MB/s</span>
              <span>100 MB/s</span>
            </div>
          </div>
          <p className="text-sm text-gray-500">
            Limit the maximum download speed. Set to 0 for unlimited speed. Useful for managing bandwidth usage.
          </p>
        </div>
      </div>

      {/* Default Segments */}
      <div className="bg-gray-900/50 rounded-xl p-6 border border-gray-700">
        <div className="flex items-center gap-3 mb-4">
          <Download className="w-5 h-5 text-purple-400" />
          <h3 className="text-lg font-semibold text-white">Download Segments</h3>
        </div>
        <div className="space-y-4">
          <div>
            <label className="block text-sm text-gray-400 mb-2">
              Default number of segments: <span className="text-white font-bold">{defaultSegments}</span>
            </label>
            <input
              type="range"
              min="1"
              max="16"
              value={defaultSegments}
              onChange={(e) => setDefaultSegments(parseInt(e.target.value))}
              className="w-full h-2 bg-gray-700 rounded-lg appearance-none cursor-pointer"
            />
            <div className="flex justify-between text-xs text-gray-500 mt-1">
              <span>1</span>
              <span>8</span>
              <span>16</span>
            </div>
          </div>
          <p className="text-sm text-gray-500">
            Number of parallel connections per download. More segments can increase speed for large files but may stress the server.
          </p>
        </div>
      </div>

      {/* Save Button */}
      <div className="flex justify-end pt-4">
        <button
          onClick={handleSave}
          disabled={saving}
          className="flex items-center gap-2 px-6 py-3 bg-blue-600 hover:bg-blue-700 
                   disabled:bg-gray-600 disabled:cursor-not-allowed text-white rounded-xl 
                   transition-all duration-200 font-semibold shadow-lg hover:shadow-xl 
                   hover:scale-105 active:scale-95"
        >
          <Save className="w-5 h-5" />
          {saving ? 'Saving...' : 'Save Settings'}
        </button>
      </div>
    </div>
  );
}
