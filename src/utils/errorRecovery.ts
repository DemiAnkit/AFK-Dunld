import { invoke } from '@tauri-apps/api/core';
import toast from 'react-hot-toast';

interface RetryConfig {
  maxRetries: number;
  delay: number;
  backoff: boolean;
}

const DEFAULT_RETRY_CONFIG: RetryConfig = {
  maxRetries: 3,
  delay: 1000,
  backoff: true,
};

/**
 * Retry a function with exponential backoff
 */
export async function retryWithBackoff<T>(
  fn: () => Promise<T>,
  config: Partial<RetryConfig> = {}
): Promise<T> {
  const { maxRetries, delay, backoff } = { ...DEFAULT_RETRY_CONFIG, ...config };
  
  let lastError: Error | unknown;
  
  for (let attempt = 0; attempt < maxRetries; attempt++) {
    try {
      return await fn();
    } catch (error) {
      lastError = error;
      
      if (attempt < maxRetries - 1) {
        const waitTime = backoff ? delay * Math.pow(2, attempt) : delay;
        console.log(`Retry attempt ${attempt + 1}/${maxRetries} after ${waitTime}ms`);
        await new Promise(resolve => setTimeout(resolve, waitTime));
      }
    }
  }
  
  throw lastError;
}

/**
 * Retry a download with automatic error recovery
 */
export async function retryDownload(downloadId: string): Promise<void> {
  try {
    await retryWithBackoff(
      async () => {
        // First, check if download still exists
        const exists = await invoke<boolean>('check_file_exists', { downloadId });
        if (!exists) {
          throw new Error('Download no longer exists');
        }
        
        // Try to resume
        await invoke('resume_download', { downloadId });
      },
      { maxRetries: 3, delay: 2000, backoff: true }
    );
    
    toast.success('Download resumed successfully');
  } catch (error) {
    console.error('Failed to retry download:', error);
    toast.error('Failed to resume download. Please try again manually.');
    throw error;
  }
}

/**
 * Recover from network errors
 */
export async function recoverFromNetworkError(error: unknown): Promise<void> {
  const errorMessage = error instanceof Error ? error.message : String(error);
  
  // Check if it's a network error
  if (
    errorMessage.includes('network') ||
    errorMessage.includes('timeout') ||
    errorMessage.includes('connection')
  ) {
    toast.error('Network error detected. Retrying...', { duration: 3000 });
    
    // Wait a bit before retrying
    await new Promise(resolve => setTimeout(resolve, 2000));
    
    return;
  }
  
  throw error;
}

/**
 * Handle API call errors with retry
 */
export async function apiCallWithRetry<T>(
  command: string,
  args?: Record<string, unknown>
): Promise<T> {
  return retryWithBackoff(
    async () => {
      try {
        return await invoke<T>(command, args);
      } catch (error) {
        await recoverFromNetworkError(error);
        throw error;
      }
    },
    { maxRetries: 2, delay: 1000, backoff: true }
  );
}

/**
 * Batch retry failed downloads
 */
export async function retryAllFailed(): Promise<number> {
  try {
    const downloads = await invoke<any[]>('get_all_downloads');
    const failed = downloads.filter((d: any) => d.status === 'failed' || d.status === 'error');
    
    if (failed.length === 0) {
      toast.success('No failed downloads to retry');
      return 0;
    }
    
    let successCount = 0;
    
    for (const download of failed) {
      try {
        await invoke('retry_download', { downloadId: download.id });
        successCount++;
      } catch (error) {
        console.error(`Failed to retry ${download.id}:`, error);
      }
    }
    
    toast.success(`Retried ${successCount}/${failed.length} downloads`);
    return successCount;
  } catch (error) {
    console.error('Failed to retry failed downloads:', error);
    toast.error('Failed to retry downloads');
    return 0;
  }
}

/**
 * Auto-recovery for crashed downloads
 */
export function enableAutoRecovery(): () => void {
  let recoveryInterval: any;
  
  const checkAndRecover = async () => {
    try {
      const downloads = await invoke<any[]>('get_all_downloads');
      const stalled = downloads.filter((d: any) => {
        // Check if download has been "downloading" for too long without progress
        if (d.status !== 'downloading') return false;
        
        // If no progress in last 5 minutes, consider it stalled
        const lastUpdate = new Date(d.updated_at || d.created_at);
        const now = new Date();
        const minutesSinceUpdate = (now.getTime() - lastUpdate.getTime()) / 1000 / 60;
        
        return minutesSinceUpdate > 5;
      });
      
      for (const download of stalled) {
        console.log(`Auto-recovering stalled download: ${download.id}`);
        try {
          await invoke('pause_download', { downloadId: download.id });
          await new Promise(resolve => setTimeout(resolve, 1000));
          await invoke('resume_download', { downloadId: download.id });
        } catch (error) {
          console.error(`Failed to recover ${download.id}:`, error);
        }
      }
    } catch (error) {
      console.error('Auto-recovery check failed:', error);
    }
  };
  
  // Check every 2 minutes
  recoveryInterval = setInterval(checkAndRecover, 2 * 60 * 1000);
  
  // Initial check after 1 minute
  setTimeout(checkAndRecover, 60 * 1000);
  
  // Return cleanup function
  return () => {
    if (recoveryInterval) {
      clearInterval(recoveryInterval);
    }
  };
}

/**
 * Error boundary fallback
 */
export function handleGlobalError(error: Error, errorInfo: any): void {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  console.error('Global error caught:', error, errorInfo);
  
  // Log to backend if possible
  try {
    invoke('log_error', {
      error: error.message,
      stack: error.stack,
      info: JSON.stringify(errorInfo),
    }).catch(console.error);
  } catch (e) {
    console.error('Failed to log error to backend:', e);
  }
  
  // Show user-friendly message
  toast.error(
    'An unexpected error occurred. The app will try to recover.',
    { duration: 5000 }
  );
}
