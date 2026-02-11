// Settings types - matches backend AppSettings
export interface Settings {
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

export type ThemeMode = Settings['theme'];
