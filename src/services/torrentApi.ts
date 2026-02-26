// src/services/torrentApi.ts
// API functions for torrent operations

import { invoke } from '@tauri-apps/api/core';
import type {
  TorrentInfo,
  TorrentStats,
  TorrentState,
  TorrentMetadata,
  TorrentPriority,
  BandwidthLimit,
  TorrentSchedule,
} from '../types/torrent';

export const torrentApi = {
  // Basic operations
  async addTorrentFile(filePath: string): Promise<string> {
    return invoke('add_torrent_file', { filePath });
  },

  async addMagnetLink(magnetLink: string): Promise<string> {
    return invoke('add_magnet_link', { magnetLink });
  },

  async listTorrents(): Promise<TorrentInfo[]> {
    return invoke('list_torrents');
  },

  async getTorrentInfo(infoHash: string): Promise<TorrentInfo> {
    return invoke('get_torrent_info', { infoHash });
  },

  async getTorrentStats(infoHash: string): Promise<TorrentStats> {
    return invoke('get_torrent_stats', { infoHash });
  },

  async getTorrentState(infoHash: string): Promise<TorrentState> {
    return invoke('get_torrent_state', { infoHash });
  },

  async pauseTorrent(infoHash: string): Promise<void> {
    return invoke('pause_torrent', { infoHash });
  },

  async resumeTorrent(infoHash: string): Promise<void> {
    return invoke('resume_torrent', { infoHash });
  },

  async removeTorrent(infoHash: string, deleteFiles: boolean = false): Promise<void> {
    return invoke('remove_torrent', { infoHash, deleteFiles });
  },

  // Priority management
  async setTorrentPriority(infoHash: string, priority: TorrentPriority): Promise<void> {
    return invoke('set_torrent_priority', { infoHash, priority });
  },

  async getTorrentPriority(infoHash: string): Promise<number> {
    return invoke('get_torrent_priority', { infoHash });
  },

  // Bandwidth limiting
  async setTorrentBandwidthLimit(
    infoHash: string,
    downloadLimit: number | null,
    uploadLimit: number | null
  ): Promise<void> {
    return invoke('set_torrent_bandwidth_limit', {
      infoHash,
      downloadLimit,
      uploadLimit,
    });
  },

  async getTorrentBandwidthLimit(infoHash: string): Promise<BandwidthLimit> {
    return invoke('get_torrent_bandwidth_limit', { infoHash });
  },

  // Scheduling
  async setTorrentSchedule(
    infoHash: string,
    startTime: string | null,
    endTime: string | null,
    daysOfWeek: number[],
    enabled: boolean
  ): Promise<void> {
    return invoke('set_torrent_schedule', {
      infoHash,
      startTime,
      endTime,
      daysOfWeek,
      enabled,
    });
  },

  async getTorrentSchedule(infoHash: string): Promise<TorrentSchedule> {
    return invoke('get_torrent_schedule', { infoHash });
  },

  async isTorrentScheduledActive(infoHash: string): Promise<boolean> {
    return invoke('is_torrent_scheduled_active', { infoHash });
  },

  // Tags and categories
  async addTorrentTag(infoHash: string, tag: string): Promise<void> {
    return invoke('add_torrent_tag', { infoHash, tag });
  },

  async removeTorrentTag(infoHash: string, tag: string): Promise<void> {
    return invoke('remove_torrent_tag', { infoHash, tag });
  },

  async setTorrentCategory(infoHash: string, category: string | null): Promise<void> {
    return invoke('set_torrent_category', { infoHash, category });
  },

  async getTorrentMetadata(infoHash: string): Promise<TorrentMetadata> {
    return invoke('get_torrent_metadata', { infoHash });
  },
};
