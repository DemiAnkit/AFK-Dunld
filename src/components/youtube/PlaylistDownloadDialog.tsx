import { useState, useEffect } from 'react';
import { X, Download, List, Video, Music, AlertCircle, Loader2 } from 'lucide-react';
import { invoke } from '@tauri-apps/api/core';
import toast from 'react-hot-toast';

interface PlaylistDownloadDialogProps {
  url: string;
  onClose: () => void;
}

interface VideoInfo {
  title: string;
  duration: number;
  filesize?: number;
  thumbnail?: string;
  uploader?: string;
  upload_date?: string;
  view_count?: number;
  is_playlist: boolean;
  playlist_count?: number;
}

export const PlaylistDownloadDialog = ({ url, onClose }: PlaylistDownloadDialogProps) => {
  const [loading, setLoading] = useState(true);
  const [playlistInfo, setPlaylistInfo] = useState<VideoInfo | null>(null);
  const [downloadType, setDownloadType] = useState<'video' | 'audio'>('video');
  const [videoQuality, setVideoQuality] = useState('best');
  const [audioFormat, setAudioFormat] = useState('mp3');
  const [videoFormat, setVideoFormat] = useState('mp4');
  const [savePath, setSavePath] = useState('');
  const [downloading, setDownloading] = useState(false);

  useEffect(() => {
    fetchPlaylistInfo();
  }, [url]);

  const fetchPlaylistInfo = async () => {
    try {
      setLoading(true);
      const info = await invoke<VideoInfo>('get_video_info', { url });
      setPlaylistInfo(info);
      
      if (!info.is_playlist) {
        toast.error('This URL is not a playlist');
        onClose();
        return;
      }
    } catch (error) {
      console.error('Failed to get playlist info:', error);
      toast.error('Failed to load playlist information');
    } finally {
      setLoading(false);
    }
  };

  const handleDownload = async () => {
    if (!playlistInfo) return;

    try {
      setDownloading(true);

      // Add download for the entire playlist
      await invoke('add_download', {
        request: {
          url,
          file_name: playlistInfo.title,
          save_path: savePath || null,
          headers: null,
          priority: 0,
          youtube_format: downloadType,
          youtube_quality: downloadType === 'video' ? videoQuality : null,
          youtube_video_format: downloadType === 'video' ? videoFormat : null,
          youtube_audio_format: downloadType === 'audio' ? audioFormat : null,
          youtube_playlist: true,
        },
      });

      toast.success(`Started downloading ${playlistInfo.playlist_count || 0} videos from playlist`);
      onClose();
    } catch (error) {
      console.error('Failed to start playlist download:', error);
      toast.error('Failed to start playlist download');
    } finally {
      setDownloading(false);
    }
  };

  const selectSavePath = async () => {
    try {
      const selected = await invoke<string>('select_directory');
      if (selected) {
        setSavePath(selected);
      }
    } catch (error) {
      console.error('Failed to select directory:', error);
    }
  };

  if (loading) {
    return (
      <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4">
        <div className="bg-gray-900 rounded-lg p-8 max-w-md w-full border border-gray-800">
          <div className="flex flex-col items-center gap-4">
            <Loader2 className="w-12 h-12 text-blue-500 animate-spin" />
            <p className="text-white">Loading playlist information...</p>
          </div>
        </div>
      </div>
    );
  }

  if (!playlistInfo) return null;

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4">
      <div className="bg-gray-900 rounded-lg max-w-2xl w-full border border-gray-800 max-h-[90vh] overflow-auto">
        {/* Header */}
        <div className="flex items-center justify-between p-6 border-b border-gray-800">
          <div className="flex items-center gap-3">
            <List className="w-6 h-6 text-blue-500" />
            <h2 className="text-xl font-bold text-white">Download Playlist</h2>
          </div>
          <button
            onClick={onClose}
            className="p-2 hover:bg-gray-800 rounded-lg transition-colors"
          >
            <X className="w-5 h-5 text-gray-400" />
          </button>
        </div>

        {/* Content */}
        <div className="p-6 space-y-6">
          {/* Playlist Info */}
          <div className="bg-gray-800/50 rounded-lg p-4 border border-gray-700">
            <div className="flex items-start gap-4">
              {playlistInfo.thumbnail && (
                <img
                  src={playlistInfo.thumbnail}
                  alt={playlistInfo.title}
                  className="w-32 h-20 object-cover rounded"
                />
              )}
              <div className="flex-1">
                <h3 className="font-medium text-white mb-2">{playlistInfo.title}</h3>
                <div className="grid grid-cols-2 gap-2 text-sm text-gray-400">
                  <div>Videos: {playlistInfo.playlist_count || 0}</div>
                  {playlistInfo.uploader && <div>By: {playlistInfo.uploader}</div>}
                </div>
              </div>
            </div>
          </div>

          {/* Warning */}
          <div className="bg-yellow-900/20 border border-yellow-700/50 rounded-lg p-4 flex items-start gap-3">
            <AlertCircle className="w-5 h-5 text-yellow-500 flex-shrink-0 mt-0.5" />
            <div className="text-sm text-yellow-200">
              <p className="font-medium mb-1">Large Playlist Download</p>
              <p className="text-yellow-300/80">
                This will download all {playlistInfo.playlist_count || 0} videos from the playlist. 
                This may take a while and consume significant storage space.
              </p>
            </div>
          </div>

          {/* Download Type */}
          <div>
            <label className="block text-sm font-medium text-gray-300 mb-2">
              Download Type
            </label>
            <div className="grid grid-cols-2 gap-3">
              <button
                onClick={() => setDownloadType('video')}
                className={`p-4 rounded-lg border-2 transition-all ${
                  downloadType === 'video'
                    ? 'border-blue-500 bg-blue-500/10'
                    : 'border-gray-700 bg-gray-800 hover:border-gray-600'
                }`}
              >
                <Video className="w-6 h-6 mx-auto mb-2 text-blue-400" />
                <div className="text-white font-medium">Video</div>
                <div className="text-xs text-gray-400">Download with video</div>
              </button>
              <button
                onClick={() => setDownloadType('audio')}
                className={`p-4 rounded-lg border-2 transition-all ${
                  downloadType === 'audio'
                    ? 'border-blue-500 bg-blue-500/10'
                    : 'border-gray-700 bg-gray-800 hover:border-gray-600'
                }`}
              >
                <Music className="w-6 h-6 mx-auto mb-2 text-purple-400" />
                <div className="text-white font-medium">Audio Only</div>
                <div className="text-xs text-gray-400">Extract audio</div>
              </button>
            </div>
          </div>

          {/* Format Settings */}
          {downloadType === 'video' ? (
            <div className="grid grid-cols-2 gap-4">
              <div>
                <label className="block text-sm font-medium text-gray-300 mb-2">
                  Quality
                </label>
                <select
                  value={videoQuality}
                  onChange={(e) => setVideoQuality(e.target.value)}
                  className="w-full px-3 py-2 bg-gray-800 border border-gray-700 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                >
                  <option value="best">Best Quality</option>
                  <option value="2160p">4K (2160p)</option>
                  <option value="1440p">2K (1440p)</option>
                  <option value="1080p">Full HD (1080p)</option>
                  <option value="720p">HD (720p)</option>
                  <option value="480p">SD (480p)</option>
                  <option value="360p">360p</option>
                </select>
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-300 mb-2">
                  Format
                </label>
                <select
                  value={videoFormat}
                  onChange={(e) => setVideoFormat(e.target.value)}
                  className="w-full px-3 py-2 bg-gray-800 border border-gray-700 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                >
                  <option value="mp4">MP4</option>
                  <option value="mkv">MKV</option>
                  <option value="webm">WebM</option>
                </select>
              </div>
            </div>
          ) : (
            <div>
              <label className="block text-sm font-medium text-gray-300 mb-2">
                Audio Format
              </label>
              <select
                value={audioFormat}
                onChange={(e) => setAudioFormat(e.target.value)}
                className="w-full px-3 py-2 bg-gray-800 border border-gray-700 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
              >
                <option value="mp3">MP3</option>
                <option value="aac">AAC</option>
                <option value="flac">FLAC</option>
                <option value="opus">Opus</option>
                <option value="m4a">M4A</option>
              </select>
            </div>
          )}

          {/* Save Path */}
          <div>
            <label className="block text-sm font-medium text-gray-300 mb-2">
              Save Location
            </label>
            <div className="flex gap-2">
              <input
                type="text"
                value={savePath}
                onChange={(e) => setSavePath(e.target.value)}
                placeholder="Default download folder"
                className="flex-1 px-3 py-2 bg-gray-800 border border-gray-700 rounded-lg text-white placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-blue-500"
              />
              <button
                onClick={selectSavePath}
                className="px-4 py-2 bg-gray-800 border border-gray-700 rounded-lg text-white hover:bg-gray-700 transition-colors"
              >
                Browse
              </button>
            </div>
          </div>
        </div>

        {/* Footer */}
        <div className="flex items-center justify-end gap-3 p-6 border-t border-gray-800">
          <button
            onClick={onClose}
            className="px-4 py-2 text-gray-400 hover:text-white transition-colors"
            disabled={downloading}
          >
            Cancel
          </button>
          <button
            onClick={handleDownload}
            disabled={downloading}
            className="px-6 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg font-medium transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
          >
            {downloading ? (
              <>
                <Loader2 className="w-4 h-4 animate-spin" />
                Starting...
              </>
            ) : (
              <>
                <Download className="w-4 h-4" />
                Download Playlist ({playlistInfo.playlist_count || 0} videos)
              </>
            )}
          </button>
        </div>
      </div>
    </div>
  );
};
