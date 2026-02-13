// src/services/phase2Api.ts - Phase 2 Service Commands
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

// ========== Clipboard Monitoring ==========

export const setClipboardMonitoring = async (enabled: boolean): Promise<void> => {
  return await invoke('set_clipboard_monitoring', { enabled });
};

// Listen for clipboard URL detection
export const onClipboardUrlDetected = (callback: (url: string) => void) => {
  return listen<string>('clipboard-url-detected', (event) => {
    callback(event.payload);
  });
};

// ========== Notifications ==========

export const setNotificationsEnabled = async (enabled: boolean): Promise<void> => {
  return await invoke('set_notifications_enabled', { enabled });
};

export const testNotification = async (): Promise<void> => {
  return await invoke('test_notification');
};

// ========== System Tray ==========

export const handleTrayMenuClick = async (menuId: string): Promise<void> => {
  return await invoke('handle_tray_menu_click', { menu_id: menuId });
};

// Listen for tray menu events
export const onTrayPauseAll = (callback: () => void) => {
  return listen('tray-pause-all', () => callback());
};

export const onTrayResumeAll = (callback: () => void) => {
  return listen('tray-resume-all', () => callback());
};

export const onTrayCancelAll = (callback: () => void) => {
  return listen('tray-cancel-all', () => callback());
};

export const onTrayShowSettings = (callback: () => void) => {
  return listen('tray-show-settings', () => callback());
};
