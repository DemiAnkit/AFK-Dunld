import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import Modal from '../common/Modal';
import Button from '../common/Button';
import type { VideoInfo } from '../../types/youtube';
import { Clock, Eye, HardDrive, User, Film, Music } from 'lucide-react';

interface YouTubeDownloadDialogProps {
  isOpen: boolean;
  onClose: () => void;
  url: string;
  onDownloadStart?: () => void;
}

const YouTubeDownloadDialog: React.FC<YouTubeDownloadDialogProps> = ({
  isOpen,
  onClose,
  url,
  onDownloadStart,
}) => {
  const [videoInfo, setVideoInfo] = useState<VideoInfo | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  
  // Download options
  const [downloadType, setDownloadType] = useState<'video' | 'audio'>('video');
  const [selectedQuality, setSelectedQuality] = useState('1080p');
  const [videoFormat, setVideoFormat] = useState('mp4');
  const [audioFormat, setAudioFormat] = useState('mp3');
  const [downloadPlaylist, setDownloadPlaylist] = useState(false);
  const [savePath, setSavePath] = useState('');
  const [fileName, setFileName] = useState('');

  useEffect(() => {
    if (isOpen && url) {
      fetchVideoInfo();
    }
  }, [isOpen, url]);

  const fetchVideoInfo = async () => {
    setLoading(true);
    setError(null);
    
    try {
      const isInstalled = await invoke<boolean>('check_ytdlp_installed');
      if (!isInstalled) {
        setError('yt-dlp is not installed. Please install it first.');
        setLoading(false);
        return;
      }

      const info = await invoke<VideoInfo>('get_video_info', { url });
      setVideoInfo(info);
      setFileName(info.title);
      setDownloadPlaylist(info.is_playlist);
    } catch (err) {
      setError(err as string);
    } finally {
      setLoading(false);
    }
  };

  const handleDownload = async () => {
    try {
      const request = {
        url,
        youtube_format: downloadType,
        youtube_quality: selectedQuality,
        youtube_video_format: videoFormat,
        youtube_audio_format: audioFormat,
        save_path: savePath || null,
        file_name: fileName || null,
        segments: null,
        max_retries: null,
        expected_checksum: null,
        checksum_type: null,
        category: 'youtube',
        priority: null,
      };

      await invoke('add_download', { request });
      
      if (onDownloadStart) {
        onDownloadStart();
      }
      
      onClose();
    } catch (err) {
      setError(err as string);
    }
  };

  const formatDuration = (seconds: number): string => {
    const h = Math.floor(seconds / 3600);
    const m = Math.floor((seconds % 3600) / 60);
    const s = seconds % 60;
    
    if (h > 0) {
      return `${h}:${m.toString().padStart(2, '0')}:${s.toString().padStart(2, '0')}`;
    }
    return `${m}:${s.toString().padStart(2, '0')}`;
  };

  const formatFileSize = (bytes: number | null): string => {
    if (!bytes) return 'Unknown';
    const mb = bytes / (1024 * 1024);
    const gb = mb / 1024;
    if (gb >= 1) return `${gb.toFixed(2)} GB`;
    return `${mb.toFixed(2)} MB`;
  };

  return (
    <Modal isOpen={isOpen} onClose={onClose} title="YouTube Download" size="md">
      <div className="flex flex-col h-full">
        {loading && (
          <div className="flex flex-col items-center justify-center py-8">
            <div className="animate-spin rounded-full h-10 w-10 border-b-2 border-blue-500"></div>
            <p className="mt-3 text-gray-400 text-sm">Loading video information...</p>
          </div>
        )}

        {error && (
          <div className="bg-red-500/10 border border-red-500/30 text-red-400 px-4 py-3 rounded-xl mb-4">
            <p className="font-semibold text-sm">Error</p>
            <p className="text-xs mt-1">{error}</p>
          </div>
        )}

        {!loading && !error && videoInfo && (
          <div className="flex flex-col min-h-0">
            {/* Compact Video Info */}
            <div className="flex gap-3 mb-4">
              {videoInfo.thumbnail && (
                <div className="flex-shrink-0">
                  <img 
                    src={videoInfo.thumbnail} 
                    alt={videoInfo.title}
                    className="w-32 h-20 object-cover rounded-lg border border-gray-700"
                  />
                </div>
              )}
              <div className="flex-1 min-w-0">
                <h3 className="font-semibold text-white text-sm leading-tight line-clamp-2 mb-2">
                  {videoInfo.title}
                </h3>
                <div className="flex flex-wrap gap-x-3 gap-y-1 text-xs text-gray-400">
                  {videoInfo.uploader && (
                    <span className="flex items-center gap-1">
                      <User size={12} />
                      {videoInfo.uploader}
                    </span>
                  )}
                  <span className="flex items-center gap-1">
                    <Clock size={12} />
                    {formatDuration(videoInfo.duration)}
                  </span>
                  {videoInfo.view_count && (
                    <span className="flex items-center gap-1">
                      <Eye size={12} />
                      {videoInfo.view_count.toLocaleString()}
                    </span>
                  )}
                  {videoInfo.filesize && (
                    <span className="flex items-center gap-1">
                      <HardDrive size={12} />
                      {formatFileSize(videoInfo.filesize)}
                    </span>
                  )}
                </div>
              </div>
            </div>

            {/* Playlist Badge */}
            {videoInfo.is_playlist && videoInfo.playlist_count && (
              <div className="bg-blue-500/10 border border-blue-500/30 text-blue-400 px-3 py-2 rounded-lg mb-4 text-xs">
                <span className="font-medium">
                  <Film size={14} className="inline mr-1" />
                  Playlist: {videoInfo.playlist_count} videos
                </span>
              </div>
            )}

            {/* Scrollable Options */}
            <div className="flex-1 overflow-y-auto scrollbar-thin pr-1 space-y-3">
              {/* Download Type Toggle */}
              <div>
                <label className="block text-xs font-medium text-gray-300 mb-2">
                  Download Type
                </label>
                <div className="flex gap-2">
                  <button
                    onClick={() => setDownloadType('video')}
                    className={`flex-1 py-2 px-3 rounded-lg text-sm font-medium transition-all duration-200 flex items-center justify-center gap-2 ${
                      downloadType === 'video'
                        ? 'bg-blue-600 text-white shadow-lg shadow-blue-500/25'
                        : 'bg-gray-800 text-gray-400 hover:bg-gray-700 hover:text-gray-200'
                    }`}
                  >
                    <Film size={16} />
                    Video
                  </button>
                  <button
                    onClick={() => setDownloadType('audio')}
                    className={`flex-1 py-2 px-3 rounded-lg text-sm font-medium transition-all duration-200 flex items-center justify-center gap-2 ${
                      downloadType === 'audio'
                        ? 'bg-blue-600 text-white shadow-lg shadow-blue-500/25'
                        : 'bg-gray-800 text-gray-400 hover:bg-gray-700 hover:text-gray-200'
                    }`}
                  >
                    <Music size={16} />
                    Audio
                  </button>
                </div>
              </div>

              {/* Video Options */}
              {downloadType === 'video' && (
                <div className="grid grid-cols-2 gap-3">
                  <div>
                    <label className="block text-xs font-medium text-gray-300 mb-1">
                      Quality
                    </label>
                    <select
                      value={selectedQuality}
                      onChange={(e) => setSelectedQuality(e.target.value)}
                      className="w-full px-3 py-2 bg-gray-800 border border-gray-700 rounded-lg text-sm text-white focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500 transition-all"
                    >
                      <option value="best">Best</option>
                      <option value="2160p">4K</option>
                      <option value="1440p">2K</option>
                      <option value="1080p">1080p</option>
                      <option value="720p">720p</option>
                      <option value="480p">480p</option>
                      <option value="360p">360p</option>
                    </select>
                  </div>

                  <div>
                    <label className="block text-xs font-medium text-gray-300 mb-1">
                      Format
                    </label>
                    <select
                      value={videoFormat}
                      onChange={(e) => setVideoFormat(e.target.value)}
                      className="w-full px-3 py-2 bg-gray-800 border border-gray-700 rounded-lg text-sm text-white focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500 transition-all"
                    >
                      <option value="mp4">MP4</option>
                      <option value="mkv">MKV</option>
                      <option value="webm">WebM</option>
                    </select>
                  </div>
                </div>
              )}

              {/* Audio Options */}
              {downloadType === 'audio' && (
                <div>
                  <label className="block text-xs font-medium text-gray-300 mb-1">
                    Audio Format
                  </label>
                  <select
                    value={audioFormat}
                    onChange={(e) => setAudioFormat(e.target.value)}
                    className="w-full px-3 py-2 bg-gray-800 border border-gray-700 rounded-lg text-sm text-white focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500 transition-all"
                  >
                    <option value="mp3">MP3</option>
                    <option value="aac">AAC</option>
                    <option value="flac">FLAC</option>
                    <option value="opus">Opus</option>
                    <option value="m4a">M4A</option>
                  </select>
                </div>
              )}

              {/* Playlist Option */}
              {videoInfo.is_playlist && (
                <div className="flex items-center bg-gray-800/50 p-3 rounded-lg">
                  <input
                    type="checkbox"
                    id="downloadPlaylist"
                    checked={downloadPlaylist}
                    onChange={(e) => setDownloadPlaylist(e.target.checked)}
                    className="h-4 w-4 text-blue-600 bg-gray-800 border-gray-600 rounded focus:ring-blue-500 focus:ring-offset-gray-900"
                  />
                  <label htmlFor="downloadPlaylist" className="ml-2 text-sm text-gray-300">
                    Download entire playlist ({videoInfo.playlist_count} videos)
                  </label>
                </div>
              )}

              {/* File Name */}
              <div>
                <label className="block text-xs font-medium text-gray-300 mb-1">
                  File Name
                </label>
                <input
                  type="text"
                  value={fileName}
                  onChange={(e) => setFileName(e.target.value)}
                  className="w-full px-3 py-2 bg-gray-800 border border-gray-700 rounded-lg text-sm text-white placeholder-gray-500 focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500 transition-all"
                  placeholder="Enter file name"
                />
              </div>

              {/* Save Path */}
              <div>
                <label className="block text-xs font-medium text-gray-300 mb-1">
                  Save Location (Optional)
                </label>
                <input
                  type="text"
                  value={savePath}
                  onChange={(e) => setSavePath(e.target.value)}
                  className="w-full px-3 py-2 bg-gray-800 border border-gray-700 rounded-lg text-sm text-white placeholder-gray-500 focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500 transition-all"
                  placeholder="Default download folder"
                />
              </div>
            </div>

            {/* Action Buttons - Always Visible */}
            <div className="flex justify-end gap-3 pt-4 mt-4 border-t border-gray-800 flex-shrink-0">
              <Button variant="secondary" onClick={onClose}>
                Cancel
              </Button>
              <Button variant="primary" onClick={handleDownload}>
                Download
              </Button>
            </div>
          </div>
        )}
      </div>
    </Modal>
  );
};

export default YouTubeDownloadDialog;