import { useState } from 'react';
import { Youtube, Save, Trash2, Plus } from 'lucide-react';
import toast from 'react-hot-toast';

interface YouTubePreset {
  id: string;
  name: string;
  format_type: 'video' | 'audio';
  quality: string;
  video_format: string;
  audio_format: string;
}

const DEFAULT_PRESETS: YouTubePreset[] = [
  {
    id: 'best-video',
    name: 'Best Quality Video',
    format_type: 'video',
    quality: 'best',
    video_format: 'mp4',
    audio_format: 'mp3',
  },
  {
    id: '1080p-video',
    name: '1080p Video (MP4)',
    format_type: 'video',
    quality: '1080p',
    video_format: 'mp4',
    audio_format: 'mp3',
  },
  {
    id: '720p-video',
    name: '720p Video (MP4)',
    format_type: 'video',
    quality: '720p',
    video_format: 'mp4',
    audio_format: 'mp3',
  },
  {
    id: 'audio-mp3',
    name: 'Audio Only (MP3)',
    format_type: 'audio',
    quality: 'best',
    video_format: 'mp4',
    audio_format: 'mp3',
  },
  {
    id: 'audio-flac',
    name: 'Audio Only (FLAC Lossless)',
    format_type: 'audio',
    quality: 'best',
    video_format: 'mp4',
    audio_format: 'flac',
  },
];

export function YouTubeSettings() {
  const [presets, setPresets] = useState<YouTubePreset[]>(() => {
    const saved = localStorage.getItem('youtube-presets');
    return saved ? JSON.parse(saved) : DEFAULT_PRESETS;
  });

  const [editingPreset, setEditingPreset] = useState<YouTubePreset | null>(null);
  const [isAddingNew, setIsAddingNew] = useState(false);

  const savePresets = (newPresets: YouTubePreset[]) => {
    localStorage.setItem('youtube-presets', JSON.stringify(newPresets));
    setPresets(newPresets);
    toast.success('Presets saved');
  };

  const handleSavePreset = (preset: YouTubePreset) => {
    if (editingPreset) {
      const updated = presets.map(p => p.id === preset.id ? preset : p);
      savePresets(updated);
    } else {
      savePresets([...presets, { ...preset, id: Date.now().toString() }]);
    }
    setEditingPreset(null);
    setIsAddingNew(false);
  };

  const handleDeletePreset = (id: string) => {
    if (confirm('Are you sure you want to delete this preset?')) {
      savePresets(presets.filter(p => p.id !== id));
      toast.success('Preset deleted');
    }
  };

  const handleResetToDefaults = () => {
    if (confirm('Reset to default presets? This will remove all custom presets.')) {
      savePresets(DEFAULT_PRESETS);
      toast.success('Reset to defaults');
    }
  };

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-3">
          <Youtube className="w-6 h-6 text-red-400" />
          <div>
            <h2 className="text-xl font-semibold text-white">YouTube Download Settings</h2>
            <p className="text-sm text-gray-400">Configure presets for quick downloads</p>
          </div>
        </div>
        <div className="flex gap-2">
          <button
            onClick={handleResetToDefaults}
            className="px-4 py-2 bg-gray-700 hover:bg-gray-600 text-white rounded-lg transition-colors text-sm"
          >
            Reset to Defaults
          </button>
          <button
            onClick={() => setIsAddingNew(true)}
            className="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg transition-colors flex items-center gap-2 text-sm"
          >
            <Plus className="w-4 h-4" />
            Add Preset
          </button>
        </div>
      </div>

      {/* Presets List */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        {presets.map(preset => (
          <div
            key={preset.id}
            className="bg-gray-800 border border-gray-700 rounded-lg p-4 hover:border-gray-600 transition-colors"
          >
            <div className="flex items-start justify-between mb-3">
              <div>
                <h3 className="font-semibold text-white">{preset.name}</h3>
                <div className="flex items-center gap-2 mt-1">
                  <span className={`text-xs px-2 py-0.5 rounded ${
                    preset.format_type === 'video' 
                      ? 'bg-blue-500/20 text-blue-300' 
                      : 'bg-purple-500/20 text-purple-300'
                  }`}>
                    {preset.format_type === 'video' ? '📹 Video' : '🎵 Audio'}
                  </span>
                  {preset.format_type === 'video' && (
                    <span className="text-xs text-gray-400">{preset.quality}</span>
                  )}
                </div>
              </div>
              <div className="flex gap-1">
                <button
                  onClick={() => setEditingPreset(preset)}
                  className="p-2 hover:bg-gray-700 rounded transition-colors"
                  title="Edit preset"
                >
                  <Save className="w-4 h-4 text-gray-400" />
                </button>
                <button
                  onClick={() => handleDeletePreset(preset.id)}
                  className="p-2 hover:bg-red-900/30 rounded transition-colors"
                  title="Delete preset"
                >
                  <Trash2 className="w-4 h-4 text-red-400" />
                </button>
              </div>
            </div>
            
            <div className="text-sm text-gray-400 space-y-1">
              {preset.format_type === 'video' ? (
                <>
                  <p>Video Format: <span className="text-white">{preset.video_format.toUpperCase()}</span></p>
                  <p>Quality: <span className="text-white">{preset.quality}</span></p>
                </>
              ) : (
                <p>Audio Format: <span className="text-white">{preset.audio_format.toUpperCase()}</span></p>
              )}
            </div>
          </div>
        ))}
      </div>

      {/* Add/Edit Preset Modal */}
      {(isAddingNew || editingPreset) && (
        <PresetEditor
          preset={editingPreset}
          onSave={handleSavePreset}
          onCancel={() => {
            setIsAddingNew(false);
            setEditingPreset(null);
          }}
        />
      )}
    </div>
  );
}

interface PresetEditorProps {
  preset: YouTubePreset | null;
  onSave: (preset: YouTubePreset) => void;
  onCancel: () => void;
}

function PresetEditor({ preset, onSave, onCancel }: PresetEditorProps) {
  const [name, setName] = useState(preset?.name || '');
  const [formatType, setFormatType] = useState<'video' | 'audio'>(preset?.format_type || 'video');
  const [quality, setQuality] = useState(preset?.quality || 'best');
  const [videoFormat, setVideoFormat] = useState(preset?.video_format || 'mp4');
  const [audioFormat, setAudioFormat] = useState(preset?.audio_format || 'mp3');

  const handleSubmit = () => {
    if (!name.trim()) {
      toast.error('Please enter a preset name');
      return;
    }

    onSave({
      id: preset?.id || '',
      name: name.trim(),
      format_type: formatType,
      quality,
      video_format: videoFormat,
      audio_format: audioFormat,
    });
  };

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4">
      <div className="bg-gray-800 rounded-lg border border-gray-700 p-6 max-w-md w-full">
        <h3 className="text-lg font-semibold text-white mb-4">
          {preset ? 'Edit Preset' : 'Add New Preset'}
        </h3>

        <div className="space-y-4">
          <div>
            <label className="block text-sm font-medium text-gray-300 mb-2">
              Preset Name
            </label>
            <input
              type="text"
              value={name}
              onChange={(e) => setName(e.target.value)}
              placeholder="e.g., Best Quality Video"
              className="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white focus:outline-none focus:border-blue-500"
            />
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-300 mb-2">
              Download Type
            </label>
            <div className="grid grid-cols-2 gap-2">
              <button
                onClick={() => setFormatType('video')}
                className={`px-4 py-2 rounded-lg text-sm font-medium transition-all ${
                  formatType === 'video'
                    ? 'bg-blue-600 text-white'
                    : 'bg-gray-700 text-gray-300 hover:bg-gray-600'
                }`}
              >
                📹 Video
              </button>
              <button
                onClick={() => setFormatType('audio')}
                className={`px-4 py-2 rounded-lg text-sm font-medium transition-all ${
                  formatType === 'audio'
                    ? 'bg-purple-600 text-white'
                    : 'bg-gray-700 text-gray-300 hover:bg-gray-600'
                }`}
              >
                🎵 Audio
              </button>
            </div>
          </div>

          {formatType === 'video' ? (
            <>
              <div>
                <label className="block text-sm font-medium text-gray-300 mb-2">
                  Video Quality
                </label>
                <select
                  value={quality}
                  onChange={(e) => setQuality(e.target.value)}
                  className="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white focus:outline-none focus:border-blue-500"
                >
                  <option value="best">Best Available</option>
                  <option value="2160p">4K (2160p)</option>
                  <option value="1440p">2K (1440p)</option>
                  <option value="1080p">Full HD (1080p)</option>
                  <option value="720p">HD (720p)</option>
                  <option value="480p">SD (480p)</option>
                  <option value="360p">Low (360p)</option>
                </select>
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-300 mb-2">
                  Video Format
                </label>
                <select
                  value={videoFormat}
                  onChange={(e) => setVideoFormat(e.target.value)}
                  className="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white focus:outline-none focus:border-blue-500"
                >
                  <option value="mp4">MP4 (Recommended)</option>
                  <option value="mkv">MKV</option>
                  <option value="webm">WebM</option>
                </select>
              </div>
            </>
          ) : (
            <div>
              <label className="block text-sm font-medium text-gray-300 mb-2">
                Audio Format
              </label>
              <select
                value={audioFormat}
                onChange={(e) => setAudioFormat(e.target.value)}
                className="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white focus:outline-none focus:border-blue-500"
              >
                <option value="mp3">MP3 (Recommended)</option>
                <option value="aac">AAC</option>
                <option value="flac">FLAC (Lossless)</option>
                <option value="opus">Opus</option>
                <option value="m4a">M4A</option>
              </select>
            </div>
          )}
        </div>

        <div className="flex justify-end gap-3 mt-6">
          <button
            onClick={onCancel}
            className="px-4 py-2 text-gray-300 hover:bg-gray-700 rounded-lg transition-colors"
          >
            Cancel
          </button>
          <button
            onClick={handleSubmit}
            className="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg transition-colors"
          >
            Save Preset
          </button>
        </div>
      </div>
    </div>
  );
}
