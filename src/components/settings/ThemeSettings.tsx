import { useTheme } from '../../hooks/useTheme';
import { Sun, Moon, Monitor } from 'lucide-react';

export function ThemeSettings() {
  const { theme, setTheme } = useTheme();

  const themes = [
    { value: 'dark', label: 'Dark', icon: Moon, description: 'Easy on the eyes in low light' },
    { value: 'light', label: 'Light', icon: Sun, description: 'Best for bright environments' },
  ] as const;

  return (
    <div className="space-y-6">
      <div>
        <h3 className="text-lg font-semibold text-gray-900 dark:text-white">Appearance</h3>
        <p className="text-sm text-gray-500 dark:text-gray-400">
          Customize how the application looks
        </p>
      </div>

      <div className="grid grid-cols-2 gap-4">
        {themes.map((t) => (
          <button
            key={t.value}
            onClick={() => setTheme(t.value)}
            className={`flex flex-col items-center gap-3 p-4 rounded-xl border-2 transition-all duration-200 ${
              theme === t.value
                ? 'border-blue-500 bg-blue-500/10'
                : 'border-gray-200 dark:border-gray-700 hover:border-gray-300 dark:hover:border-gray-600'
            }`}
          >
            <t.icon className={`w-8 h-8 ${theme === t.value ? 'text-blue-400' : 'text-gray-400'}`} />
            <div className="text-center">
              <p className={`font-medium ${theme === t.value ? 'text-blue-400' : 'text-gray-700 dark:text-gray-300'}`}>
                {t.label}
              </p>
              <p className="text-xs text-gray-500 dark:text-gray-400 mt-1">
                {t.description}
              </p>
            </div>
          </button>
        ))}
      </div>
    </div>
  );
}

export default ThemeSettings;
