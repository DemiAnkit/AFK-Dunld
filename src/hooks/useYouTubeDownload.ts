import { useState, useCallback } from 'react';
import { youtubeApi } from '../services/tauriApi';
import type { VideoInfo, QualityOption } from '../types/youtube';

export const useYouTubeDownload = () => {
  const [isInstalled, setIsInstalled] = useState<boolean | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const checkInstallation = useCallback(async () => {
    try {
      setLoading(true);
      const installed = await youtubeApi.checkYtDlpInstalled();
      setIsInstalled(installed);
      return installed;
    } catch (err) {
      setError(err as string);
      return false;
    } finally {
      setLoading(false);
    }
  }, []);

  const getVideoInfo = useCallback(async (url: string): Promise<VideoInfo | null> => {
    try {
      setLoading(true);
      setError(null);
      const info = await youtubeApi.getVideoInfo(url);
      return info;
    } catch (err) {
      setError(err as string);
      return null;
    } finally {
      setLoading(false);
    }
  }, []);

  const getVideoQualities = useCallback(async (url: string): Promise<QualityOption[]> => {
    try {
      setLoading(true);
      setError(null);
      const qualities = await youtubeApi.getVideoQualities(url);
      return qualities;
    } catch (err) {
      setError(err as string);
      return [];
    } finally {
      setLoading(false);
    }
  }, []);

  const checkIsPlaylist = useCallback(async (url: string): Promise<boolean> => {
    try {
      const isPlaylist = await youtubeApi.checkIsPlaylist(url);
      return isPlaylist;
    } catch (err) {
      setError(err as string);
      return false;
    }
  }, []);

  const isYouTubeUrl = useCallback((url: string): boolean => {
    return url.includes('youtube.com') || 
           url.includes('youtu.be') ||
           url.includes('youtube-nocookie.com');
  }, []);

  const isSupportedUrl = useCallback((url: string): boolean => {
    const supportedDomains = [
      'youtube.com',
      'youtu.be',
      'vimeo.com',
      'dailymotion.com',
      'twitch.tv',
      'twitter.com',
      'x.com',
      'facebook.com',
      'instagram.com',
      'tiktok.com',
      'reddit.com',
      'soundcloud.com',
    ];

    return supportedDomains.some(domain => url.includes(domain));
  }, []);

  return {
    isInstalled,
    loading,
    error,
    checkInstallation,
    getVideoInfo,
    getVideoQualities,
    checkIsPlaylist,
    isYouTubeUrl,
    isSupportedUrl,
  };
};
