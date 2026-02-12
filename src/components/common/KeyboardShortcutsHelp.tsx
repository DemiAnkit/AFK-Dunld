import { Keyboard } from 'lucide-react';
import { useState } from 'react';

export function KeyboardShortcutsHelp() {
  const [isOpen, setIsOpen] = useState(false);

  const shortcuts = [
    { key: 'Ctrl/Cmd + N', description: 'Open new download dialog', category: 'General' },
    { key: 'Ctrl/Cmd + S', description: 'Open settings', category: 'General' },
    { key: 'Ctrl/Cmd + F', description: 'Focus search box', category: 'General' },
    { key: '1-8', description: 'Navigate between tabs', category: 'Navigation' },
    { key: 'P', description: 'Pause selected/active downloads', category: 'Downloads' },
    { key: 'R', description: 'Resume selected/paused downloads', category: 'Downloads' },
    { key: 'Delete', description: 'Remove selected downloads', category: 'Downloads' },
    { key: 'Ctrl/Cmd + A', description: 'Select all downloads', category: 'Selection' },
    { key: 'Shift + Click', description: 'Select range of downloads', category: 'Selection' },
    { key: 'Ctrl/Cmd + Click', description: 'Toggle individual selection', category: 'Selection' },
    { key: 'Esc', description: 'Clear selection', category: 'Selection' },
  ];

  // Group shortcuts by category
  const groupedShortcuts = shortcuts.reduce((acc, shortcut) => {
    const category = shortcut.category || 'Other';
    if (!acc[category]) acc[category] = [];
    acc[category].push(shortcut);
    return acc;
  }, {} as Record<string, typeof shortcuts>);

  return (
    <>
      <button
        onClick={() => setIsOpen(true)}
        className="p-2 text-gray-400 hover:text-blue-400 hover:bg-blue-500/10 
                 rounded-xl transition-all duration-200 border border-transparent
                 hover:border-blue-500/30 hover:scale-110 active:scale-95"
        title="Keyboard Shortcuts"
      >
        <Keyboard size={18} />
      </button>

      {isOpen && (
        <div className="fixed inset-0 bg-black/50 backdrop-blur-sm flex items-center justify-center z-50"
             onClick={() => setIsOpen(false)}>
          <div className="bg-gray-900 border border-gray-700 rounded-2xl shadow-2xl max-w-md w-full mx-4"
               onClick={e => e.stopPropagation()}>
            <div className="p-6">
              <div className="flex items-center gap-3 mb-6">
                <div className="p-2 bg-blue-500/20 rounded-lg">
                  <Keyboard className="w-6 h-6 text-blue-400" />
                </div>
                <h2 className="text-xl font-bold text-white">Keyboard Shortcuts</h2>
              </div>
              
              <div className="space-y-4 max-h-96 overflow-y-auto scrollbar-thin scrollbar-thumb-gray-700 scrollbar-track-gray-900">
                {Object.entries(groupedShortcuts).map(([category, categoryShortcuts]) => (
                  <div key={category}>
                    <h3 className="text-xs font-semibold text-blue-400 uppercase tracking-wider mb-2 px-1">
                      {category}
                    </h3>
                    <div className="space-y-2">
                      {categoryShortcuts.map((shortcut, index) => (
                        <div key={index} className="flex items-center justify-between p-3 bg-gray-800/50 rounded-lg hover:bg-gray-800 transition-colors">
                          <span className="text-sm text-gray-300">{shortcut.description}</span>
                          <kbd className="px-3 py-1.5 text-xs font-mono bg-gray-950 text-blue-400 rounded border border-gray-700 shadow-sm whitespace-nowrap ml-3">
                            {shortcut.key}
                          </kbd>
                        </div>
                      ))}
                    </div>
                  </div>
                ))}
              </div>

              <button
                onClick={() => setIsOpen(false)}
                className="mt-6 w-full py-2.5 bg-blue-600 hover:bg-blue-500 text-white rounded-lg font-medium transition-colors"
              >
                Got it!
              </button>
            </div>
          </div>
        </div>
      )}
    </>
  );
}
