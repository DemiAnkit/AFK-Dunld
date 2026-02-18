import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import Button from '../common/Button';

export default function BrowserIntegrationSettings() {
  const [isInstalled, setIsInstalled] = useState(false);
  const [isChecking, setIsChecking] = useState(true);
  const [isProcessing, setIsProcessing] = useState(false);
  const [message, setMessage] = useState('');

  useEffect(() => {
    checkInstallation();
  }, []);

  const checkInstallation = async () => {
    try {
      setIsChecking(true);
      const installed = await invoke<boolean>('is_browser_extension_available');
      setIsInstalled(installed);
    } catch (error) {
      console.error('Failed to check browser extension status:', error);
    } finally {
      setIsChecking(false);
    }
  };

  const handleInstall = async () => {
    try {
      setIsProcessing(true);
      setMessage('');
      
      const result = await invoke<string>('install_browser_extension_support');
      setMessage(result);
      setIsInstalled(true);
      
      setTimeout(() => setMessage(''), 5000);
    } catch (error) {
      setMessage(`Error: ${error}`);
      console.error('Failed to install browser extension support:', error);
    } finally {
      setIsProcessing(false);
    }
  };

  const handleUninstall = async () => {
    try {
      setIsProcessing(true);
      setMessage('');
      
      const result = await invoke<string>('uninstall_browser_extension_support');
      setMessage(result);
      setIsInstalled(false);
      
      setTimeout(() => setMessage(''), 5000);
    } catch (error) {
      setMessage(`Error: ${error}`);
      console.error('Failed to uninstall browser extension support:', error);
    } finally {
      setIsProcessing(false);
    }
  };

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-xl font-semibold mb-2">Browser Integration</h2>
        <p className="text-sm text-gray-600 dark:text-gray-400">
          Connect your browser extensions to AFK-Dunld for seamless download management
        </p>
      </div>

      <div className="bg-white dark:bg-gray-800 rounded-lg p-6 shadow-sm">
        <div className="flex items-center justify-between mb-4">
          <div>
            <h3 className="font-medium">Native Messaging</h3>
            <p className="text-sm text-gray-600 dark:text-gray-400 mt-1">
              Allows browser extensions to communicate with AFK-Dunld
            </p>
          </div>
          <div className="flex items-center gap-2">
            {isChecking ? (
              <span className="text-sm text-gray-500">Checking...</span>
            ) : (
              <span className={`px-3 py-1 rounded-full text-sm font-medium ${
                isInstalled 
                  ? 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200'
                  : 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-200'
              }`}>
                {isInstalled ? 'Installed' : 'Not Installed'}
              </span>
            )}
          </div>
        </div>

        <div className="flex gap-3">
          {!isInstalled ? (
            <Button
              onClick={handleInstall}
              disabled={isProcessing || isChecking}
              variant="primary"
            >
              {isProcessing ? 'Installing...' : 'Install Browser Support'}
            </Button>
          ) : (
            <>
              <Button
                onClick={checkInstallation}
                disabled={isProcessing}
                variant="secondary"
              >
                Refresh Status
              </Button>
              <Button
                onClick={handleUninstall}
                disabled={isProcessing}
                variant="danger"
              >
                {isProcessing ? 'Uninstalling...' : 'Uninstall'}
              </Button>
            </>
          )}
        </div>

        {message && (
          <div className={`mt-4 p-3 rounded-md text-sm ${
            message.includes('Error') 
              ? 'bg-red-50 text-red-800 dark:bg-red-900 dark:text-red-200'
              : 'bg-blue-50 text-blue-800 dark:bg-blue-900 dark:text-blue-200'
          }`}>
            {message}
          </div>
        )}
      </div>

      <div className="bg-blue-50 dark:bg-blue-900/20 rounded-lg p-4">
        <h4 className="font-medium text-blue-900 dark:text-blue-200 mb-2">
          📦 Installing Browser Extensions
        </h4>
        <div className="text-sm text-blue-800 dark:text-blue-300 space-y-2">
          <p><strong>Chrome:</strong></p>
          <ol className="list-decimal list-inside ml-4 space-y-1">
            <li>Navigate to <code className="bg-blue-100 dark:bg-blue-800 px-1 rounded">chrome://extensions/</code></li>
            <li>Enable "Developer mode"</li>
            <li>Click "Load unpacked"</li>
            <li>Select the <code className="bg-blue-100 dark:bg-blue-800 px-1 rounded">browser-extension/chrome</code> folder</li>
          </ol>
          
          <p className="mt-3"><strong>Firefox:</strong></p>
          <ol className="list-decimal list-inside ml-4 space-y-1">
            <li>Navigate to <code className="bg-blue-100 dark:bg-blue-800 px-1 rounded">about:debugging#/runtime/this-firefox</code></li>
            <li>Click "Load Temporary Add-on"</li>
            <li>Select any file in <code className="bg-blue-100 dark:bg-blue-800 px-1 rounded">browser-extension/firefox</code> folder</li>
          </ol>

          <p className="mt-3 text-xs">
            💡 After installing, make sure to click "Install Browser Support" above to enable native messaging.
          </p>
        </div>
      </div>

      <div className="bg-gray-50 dark:bg-gray-800 rounded-lg p-4">
        <h4 className="font-medium mb-2">Features</h4>
        <ul className="text-sm text-gray-600 dark:text-gray-400 space-y-2">
          <li className="flex items-start gap-2">
            <span className="text-green-500 mt-0.5">✓</span>
            <span>Automatic download interception for large files</span>
          </li>
          <li className="flex items-start gap-2">
            <span className="text-green-500 mt-0.5">✓</span>
            <span>Right-click context menu for links, images, and videos</span>
          </li>
          <li className="flex items-start gap-2">
            <span className="text-green-500 mt-0.5">✓</span>
            <span>YouTube download button integration</span>
          </li>
          <li className="flex items-start gap-2">
            <span className="text-green-500 mt-0.5">✓</span>
            <span>Keyboard shortcut (Ctrl+Shift+Click) for quick downloads</span>
          </li>
        </ul>
      </div>
    </div>
  );
}
