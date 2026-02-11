import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import Modal from '../common/Modal';
import Button from '../common/Button';
import type { VideoInfo, QualityOption } from '../../types/youtube';

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
  const [qualities, setQualities] = useState<QualityOption[]>([]);
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
      // Check if yt-dlp is installed
      const isInstalled = await invoke<boolean>('check_ytdlp_installed');
      if (!isInstalled) {
        setError('yt-dlp is not installed. Please install it first.');
        setLoading(false);
        return;
      }

      // Get video info
      const info = await invoke<VideoInfo>('get_video_info', { url });
      setVideoInfo(info);
      setFileName(info.title);
      setDownloadPlaylist(info.is_playlist);

      // Get available qualities (optional - can be slow)
      // const quals = await invoke<QualityOption[]>('get_video_qualities', { url });
      // setQualities(quals);

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
    <Modal isOpen={isOpen} onClose={onClose} title="YouTube Download">
      <div className="space-y-4">
        {loading && (
          <div className="text-center py-8">
            <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-500 mx-auto"></div>
            <p className="mt-4 text-gray-600">Loading video information...</p>
          </div>
        )}

        {error && (
          <div className="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded">
            <p className="font-semibold">Error</p>
            <p className="text-sm">{error}</p>
          </div>
        )}

        {!loading && !error && videoInfo && (
          <>
            {/* Video Information */}
            <div className="bg-gray-50 p-4 rounded-lg">
              {videoInfo.thumbnail && (
                <img 
                  src={videoInfo.thumbnail} 
                  alt={videoInfo.title}
                  className="w-full h-48 object-cover rounded mb-3"
                />
              )}
              
              <h3 className="font-semibold text-lg mb-2">{videoInfo.title}</h3>
              
              <div className="grid grid-cols-2 gap-2 text-sm text-gray-600">
                {videoInfo.uploader && (
                  <div>
                    <span className="font-medium">Uploader:</span> {videoInfo.uploader}
                  </div>
                )}
                
                <div>
                  <span className="font-medium">Duration:</span> {formatDuration(videoInfo.duration)}
                </div>
                
                {videoInfo.view_count && (
                  <div>
                    <span className="font-medium">Views:</span> {videoInfo.view_count.toLocaleString()}
                  </div>
                )}
                
                {videoInfo.filesize && (
                  <div>
                    <span className="font-medium">Size:</span> {formatFileSize(videoInfo.filesize)}
                  </div>
                )}

                {videoInfo.is_playlist && videoInfo.playlist_count && (
                  <div className="col-span-2 bg-blue-50 border border-blue-200 px-3 py-2 rounded">
                    <span className="font-medium text-blue-700">
                      Playlist: {videoInfo.playlist_count} videos
                    </span>
                  </div>
                )}
              </div>
            </div>

            {/* Download Options */}
            <div className="space-y-3">
              {/* Download Type */}
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  Download Type
                </label>
                <div className="flex gap-2">
                  <button
                    onClick={() => setDownloadType('video')}
                    className={`flex-1 py-2 px-4 rounded ${
                      downloadType === 'video'
                        ? 'bg-blue-500 text-white'
                        : 'bg-gray-200 text-gray-700 hover:bg-gray-300'
                    }`}
                  >
                    Video
                  </button>
                  <button
                    onClick={() => setDownloadType('audio')}
                    className={`flex-1 py-2 px-4 rounded ${
                      downloadType === 'audio'
                        ? 'bg-blue-500 text-white'
                        : 'bg-gray-200 text-gray-700 hover:bg-gray-300'
                    }`}
                  >
                    Audio Only
                  </button>
                </div>
              </div>

              {/* Quality Selection for Video */}
              {downloadType === 'video' && (
                <>
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-2">
                      Video Quality
                    </label>
                    <select
                      value={selectedQuality}
                      onChange={(e) => setSelectedQuality(e.target.value)}
                      className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
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
                    <label className="block text-sm font-medium text-gray-700 mb-2">
                      Video Format
                    </label>
                    <select
                      value={videoFormat}
                      onChange={(e) => setVideoFormat(e.target.value)}
                      className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                    >
                      <option value="mp4">MP4 (Recommended)</option>
                      <option value="mkv">MKV</option>
                      <option value="webm">WebM</option>
                    </select>
                  </div>
                </>
              )}

              {/* Audio Format Selection */}
              {downloadType === 'audio' && (
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    Audio Format
                  </label>
                  <select
                    value={audioFormat}
                    onChange={(e) => setAudioFormat(e.target.value)}
                    className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                  >
                    <option value="mp3">MP3 (Recommended)</option>
                    <option value="aac">AAC</option>
                    <option value="flac">FLAC (Lossless)</option>
                    <option value="opus">Opus</option>
                    <option value="m4a">M4A</option>
                  </select>
                </div>
              )}

              {/* Playlist Option */}
              {videoInfo.is_playlist && (
                <div className="flex items-center">
                  <input
                    type="checkbox"
                    id="downloadPlaylist"
                    checked={downloadPlaylist}
                    onChange={(e) => setDownloadPlaylist(e.target.checked)}
                    className="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
                  />
                  <label htmlFor="downloadPlaylist" className="ml-2 block text-sm text-gray-700">
                    Download entire playlist ({videoInfo.playlist_count} videos)
                  </label>
                </div>
              )}

              {/* File Name */}
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  File Name
                </label>
                <input
                  type="text"
                  value={fileName}
                  onChange={(e) => setFileName(e.target.value)}
                  className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                  placeholder="Enter file name"
                />
              </div>

              {/* Save Path */}
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  Save Location (Optional)
                </label>
                <input
                  type="text"
                  value={savePath}
                  onChange={(e) => setSavePath(e.target.value)}
                  className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                  placeholder="Default download folder"
                />
              </div>
            </div>

            {/* Action Buttons */}
            <div className="flex justify-end gap-3 pt-4 border-t">
              <Button variant="secondary" onClick={onClose}>
                Cancel
              </Button>
              <Button variant="primary" onClick={handleDownload}>
                Download
              </Button>
            </div>
          </>
        )}
      </div>
    </Modal>
  );
};

export default YouTubeDownloadDialog;
