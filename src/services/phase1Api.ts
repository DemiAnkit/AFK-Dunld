// src/services/phase1Api.ts - New Phase 1 Commands
import { invoke } from '@tauri-apps/api/core';

// ========== Type Definitions ==========

export interface GlobalStats {
  total_downloads: number;
  active_downloads: number;
  queued_downloads: number;
  completed_downloads: number;
  failed_downloads: number;
  paused_downloads: number;
  total_downloaded_bytes: number;
  total_size_bytes: number;
  current_speed: number;
  estimated_time_remaining: number | null;
}

export interface QueueInfo {
  max_concurrent: number;
  active_count: number;
  queued_count: number;
  total_count: number;
}

export interface AppSettings {
  download_path: string;
  max_concurrent_downloads: number;
  default_segments: number;
  speed_limit: number;
  theme: string;
  start_with_system: boolean;
  show_notifications: boolean;
  monitor_clipboard: boolean;
  auto_start_downloads: boolean;
  default_category: string;
}

export interface SystemInfo {
  os: string;
  os_version: string;
  available_disk_space: number;
  total_disk_space: number;
  cpu_count: number;
  total_memory: number;
}

// ========== Bulk Operations ==========

export const pauseAll = async (): Promise<string[]> => {
  return await invoke('pause_all');
};

export const resumeAll = async (): Promise<string[]> => {
  return await invoke('resume_all');
};

export const cancelAll = async (): Promise<string[]> => {
  return await invoke('cancel_all');
};

// ========== Statistics ==========

export const getGlobalStats = async (): Promise<GlobalStats> => {
  return await invoke('get_global_stats');
};

// ========== Queue Management ==========

export const getQueueInfo = async (): Promise<QueueInfo> => {
  return await invoke('get_queue_info');
};

export const setMaxConcurrent = async (max: number): Promise<void> => {
  return await invoke('set_max_concurrent', { max });
};

// ========== Speed Control ==========

export const setSpeedLimit = async (limit: number | null): Promise<void> => {
  return await invoke('set_speed_limit', { limit });
};

// ========== Settings Management ==========

export const getSettings = async (): Promise<AppSettings> => {
  return await invoke('get_settings');
};

export const getSetting = async (key: string): Promise<string | null> => {
  return await invoke('get_setting', { key });
};

export const updateSettings = async (settings: AppSettings): Promise<void> => {
  return await invoke('update_settings', { settings });
};

export const resetSettings = async (): Promise<void> => {
  return await invoke('reset_settings');
};

// ========== System Information ==========

export const getSystemInfo = async (): Promise<SystemInfo> => {
  return await invoke('get_system_info');
};

export const checkDiskSpace = async (requiredBytes: number): Promise<boolean> => {
  return await invoke('check_disk_space', { required_bytes: requiredBytes });
};
