// YouTube download types

export interface VideoInfo {
  title: string;
  duration: number;
  filesize: number | null;
  thumbnail: string | null;
  uploader: string | null;
  upload_date: string | null;
  view_count: number | null;
  is_playlist: boolean;
  playlist_count: number | null;
}

export interface QualityOption {
  format_id: string;
  resolution: string;
  ext: string;
  filesize: number | null;
  fps: number | null;
  has_audio: boolean;
}

export interface YouTubeProgress {
  percentage: number;
  downloaded_bytes: number;
  total_bytes: number;
  speed: number;
  eta: number;
  status: 'downloading' | 'processing' | 'finished';
}

export interface YouTubeDownloadOptions {
  url: string;
  youtube_format: 'video' | 'audio';
  youtube_quality: string;
  youtube_video_format: string;
  youtube_audio_format: string;
  save_path?: string | null;
  file_name?: string | null;
  segments?: number | null;
  max_retries?: number | null;
  expected_checksum?: string | null;
  checksum_type?: string | null;
  category?: string | null;
  priority?: number | null;
}

export type VideoQuality = 'best' | '2160p' | '1440p' | '1080p' | '720p' | '480p' | '360p';
export type VideoFormat = 'mp4' | 'mkv' | 'webm';
export type AudioFormat = 'mp3' | 'aac' | 'flac' | 'opus' | 'm4a';
