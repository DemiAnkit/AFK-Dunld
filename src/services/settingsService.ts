// src/services/settingsService.ts
import { invoke } from '@tauri-apps/api/core';

export interface AppSettings {
  theme: 'light' | 'dark' | 'system';
  downloadPath: string;
  maxConcurrentDownloads: number;
  maxDownloadSpeed: number; // bytes per second, 0 = unlimited
  autoStartDownloads: boolean;
  showNotifications: boolean;
  minimizeToTray: boolean;
  closeToTray: boolean;
  autoStart: boolean;
  monitorClipboard: boolean;
  defaultSegments: number;
  maxRetries: number;
}

export class SettingsService {
  async getSettings(): Promise<AppSettings> {
    return await invoke<AppSettings>('get_settings');
  }

  async updateSettings(settings: Partial<AppSettings>): Promise<void> {
    return await invoke('update_settings', { settings });
  }

  async resetSettings(): Promise<AppSettings> {
    return await invoke<AppSettings>('reset_settings');
  }

  async getDownloadPath(): Promise<string> {
    return await invoke<string>('get_download_path');
  }

  async setDownloadPath(path: string): Promise<void> {
    return await invoke('set_download_path', { path });
  }

  async selectDownloadPath(): Promise<string | null> {
    return await invoke<string | null>('select_download_path');
  }
}

export const settingsService = new SettingsService();
