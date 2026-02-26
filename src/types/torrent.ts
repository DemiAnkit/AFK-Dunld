// src/types/torrent.ts
// TypeScript types for torrent functionality

export interface TorrentFile {
  path: string;
  size: number;
}

export interface TorrentInfo {
  info_hash: string;
  name: string;
  total_size: number;
  piece_length: number;
  num_pieces: number;
  files: TorrentFile[];
}

export interface TorrentStats {
  downloaded: number;
  uploaded: number;
  download_rate: number;
  upload_rate: number;
  peers: number;
  seeders: number;
  progress: number;
  eta: number | null;
}

export type TorrentState = 
  | { Downloading: null }
  | { Seeding: null }
  | { Paused: null }
  | { Checking: null }
  | { Error: string };

export enum TorrentPriority {
  Low = 0,
  Normal = 1,
  High = 2,
  Critical = 3,
}

export interface BandwidthLimit {
  download_limit: number | null;
  upload_limit: number | null;
  enabled: boolean;
}

export interface TorrentSchedule {
  start_time: string | null;
  end_time: string | null;
  days_of_week: number[];
  enabled: boolean;
}

export interface TorrentMetadata {
  info_hash: string;
  priority: TorrentPriority;
  bandwidth_limit: BandwidthLimit;
  schedule: TorrentSchedule;
  category: string | null;
  tags: string[];
  added_time: string;
  completed_time: string | null;
  save_path: string;
}

export interface TorrentWithMetadata {
  info: TorrentInfo;
  stats: TorrentStats;
  state: TorrentState;
  metadata: TorrentMetadata;
}
