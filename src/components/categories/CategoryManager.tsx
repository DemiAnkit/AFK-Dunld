// Category Manager Component
import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface Category {
  id: string;
  name: string;
  color: string | null;
  icon: string | null;
  save_path: string | null;
  created_at: number;
  updated_at: number;
}

interface CategoryStats {
  category_id: string;
  total_downloads: number;
  completed_downloads: number;
  total_size: number;
  downloaded_size: number;
}

export function CategoryManager() {
  const [categories, setCategories] = useState<Category[]>([]);
  const [selectedCategory, setSelectedCategory] = useState<Category | null>(null);
  const [stats, setStats] = useState<CategoryStats | null>(null);
  const [showCreateModal, setShowCreateModal] = useState(false);
  const [newCategory, setNewCategory] = useState({
    name: '',
    color: '#6B7280',
    icon: 'folder',
    save_path: '',
  });

  useEffect(() => {
    loadCategories();
  }, []);

  useEffect(() => {
    if (selectedCategory) {
      loadStats(selectedCategory.id);
    }
  }, [selectedCategory]);

  const loadCategories = async () => {
    try {
      const cats = await invoke<Category[]>('get_categories');
      setCategories(cats);
    } catch (error) {
      console.error('Failed to load categories:', error);
    }
  };

  const loadStats = async (categoryId: string) => {
    try {
      const categoryStats = await invoke<CategoryStats>('get_category_stats', {
        categoryId,
      });
      setStats(categoryStats);
    } catch (error) {
      console.error('Failed to load stats:', error);
    }
  };

  const createCategory = async () => {
    if (!newCategory.name) {
      alert('Please enter a category name');
      return;
    }

    try {
      await invoke('create_category', {
        name: newCategory.name,
        color: newCategory.color || null,
        icon: newCategory.icon || null,
        savePath: newCategory.save_path || null,
      });

      setShowCreateModal(false);
      setNewCategory({ name: '', color: '#6B7280', icon: 'folder', save_path: '' });
      await loadCategories();
    } catch (error) {
      alert(`Failed to create category: ${error}`);
    }
  };

  const deleteCategory = async (categoryId: string) => {
    if (categoryId === 'default') {
      alert('Cannot delete the default category');
      return;
    }

    if (!confirm('Are you sure? Downloads will be moved to Default category.')) {
      return;
    }

    try {
      await invoke('delete_category', { categoryId });
      await loadCategories();
      setSelectedCategory(null);
      setStats(null);
    } catch (error) {
      alert(`Failed to delete category: ${error}`);
    }
  };

  const formatSize = (bytes: number) => {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return Math.round(bytes / Math.pow(k, i) * 100) / 100 + ' ' + sizes[i];
  };

  const getIconEmoji = (icon: string | null) => {
    const iconMap: Record<string, string> = {
      'folder': '📁',
      'file-text': '📄',
      'video': '🎥',
      'music': '🎵',
      'image': '🖼️',
      'package': '📦',
      'archive': '🗜️',
      'star': '⭐',
      'briefcase': '💼',
    };
    return iconMap[icon || 'folder'] || '📁';
  };

  return (
    <div className="category-manager p-6">
      <div className="flex items-center justify-between mb-6">
        <h2 className="text-2xl font-bold">Download Categories</h2>
        <button
          onClick={() => setShowCreateModal(true)}
          className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700"
        >
          + Create Category
        </button>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        {/* Category List */}
        <div className="lg:col-span-1 space-y-2">
          {categories.map((category) => (
            <div
              key={category.id}
              onClick={() => setSelectedCategory(category)}
              className={`
                p-4 rounded-lg border-2 cursor-pointer transition-all
                ${selectedCategory?.id === category.id
                  ? 'border-blue-500 bg-blue-50 dark:bg-blue-900'
                  : 'border-gray-200 dark:border-gray-700 hover:border-gray-300 dark:hover:border-gray-600'
                }
              `}
            >
              <div className="flex items-center gap-3">
                <span className="text-2xl">{getIconEmoji(category.icon)}</span>
                <div className="flex-1">
                  <div className="font-semibold">{category.name}</div>
                  {category.color && (
                    <div className="flex items-center gap-2 mt-1">
                      <div
                        className="w-4 h-4 rounded"
                        style={{ backgroundColor: category.color }}
                      ></div>
                      <span className="text-xs text-gray-500">{category.color}</span>
                    </div>
                  )}
                </div>
              </div>
            </div>
          ))}
        </div>

        {/* Category Details */}
        <div className="lg:col-span-2">
          {selectedCategory ? (
            <div className="bg-white dark:bg-gray-800 rounded-lg shadow-md p-6">
              <div className="flex items-center justify-between mb-6">
                <div className="flex items-center gap-3">
                  <span className="text-4xl">{getIconEmoji(selectedCategory.icon)}</span>
                  <div>
                    <h3 className="text-2xl font-bold">{selectedCategory.name}</h3>
                    <p className="text-sm text-gray-500">ID: {selectedCategory.id}</p>
                  </div>
                </div>
                {selectedCategory.id !== 'default' && (
                  <button
                    onClick={() => deleteCategory(selectedCategory.id)}
                    className="px-4 py-2 bg-red-600 text-white rounded hover:bg-red-700"
                  >
                    Delete
                  </button>
                )}
              </div>

              {/* Statistics */}
              {stats && (
                <div className="grid grid-cols-2 gap-4 mb-6">
                  <div className="bg-gray-50 dark:bg-gray-700 rounded-lg p-4">
                    <div className="text-sm text-gray-600 dark:text-gray-400">Total Downloads</div>
                    <div className="text-2xl font-bold">{stats.total_downloads}</div>
                  </div>
                  
                  <div className="bg-gray-50 dark:bg-gray-700 rounded-lg p-4">
                    <div className="text-sm text-gray-600 dark:text-gray-400">Completed</div>
                    <div className="text-2xl font-bold text-green-600">{stats.completed_downloads}</div>
                  </div>
                  
                  <div className="bg-gray-50 dark:bg-gray-700 rounded-lg p-4">
                    <div className="text-sm text-gray-600 dark:text-gray-400">Total Size</div>
                    <div className="text-2xl font-bold">{formatSize(stats.total_size)}</div>
                  </div>
                  
                  <div className="bg-gray-50 dark:bg-gray-700 rounded-lg p-4">
                    <div className="text-sm text-gray-600 dark:text-gray-400">Downloaded</div>
                    <div className="text-2xl font-bold">{formatSize(stats.downloaded_size)}</div>
                  </div>
                </div>
              )}

              {/* Details */}
              <div className="space-y-3">
                <div>
                  <label className="text-sm font-semibold text-gray-600 dark:text-gray-400">Color</label>
                  <div className="flex items-center gap-2 mt-1">
                    <div
                      className="w-8 h-8 rounded border"
                      style={{ backgroundColor: selectedCategory.color || '#6B7280' }}
                    ></div>
                    <span className="font-mono">{selectedCategory.color || 'Not set'}</span>
                  </div>
                </div>

                <div>
                  <label className="text-sm font-semibold text-gray-600 dark:text-gray-400">Icon</label>
                  <p>{selectedCategory.icon || 'Not set'}</p>
                </div>

                <div>
                  <label className="text-sm font-semibold text-gray-600 dark:text-gray-400">Save Path</label>
                  <p className="font-mono text-sm">{selectedCategory.save_path || 'Default'}</p>
                </div>

                <div>
                  <label className="text-sm font-semibold text-gray-600 dark:text-gray-400">Created</label>
                  <p>{new Date(selectedCategory.created_at * 1000).toLocaleString()}</p>
                </div>
              </div>
            </div>
          ) : (
            <div className="bg-gray-50 dark:bg-gray-800 rounded-lg p-12 text-center text-gray-500">
              Select a category to view details
            </div>
          )}
        </div>
      </div>

      {/* Create Category Modal */}
      {showCreateModal && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white dark:bg-gray-800 rounded-lg p-6 max-w-md w-full mx-4">
            <h3 className="text-xl font-bold mb-4">Create New Category</h3>
            
            <div className="space-y-4 mb-6">
              <div>
                <label className="block text-sm font-medium mb-2">Name</label>
                <input
                  type="text"
                  value={newCategory.name}
                  onChange={(e) => setNewCategory({ ...newCategory, name: e.target.value })}
                  placeholder="Category name"
                  className="w-full px-3 py-2 border rounded-lg dark:bg-gray-700 dark:border-gray-600"
                />
              </div>

              <div>
                <label className="block text-sm font-medium mb-2">Color</label>
                <div className="flex items-center gap-2">
                  <input
                    type="color"
                    value={newCategory.color}
                    onChange={(e) => setNewCategory({ ...newCategory, color: e.target.value })}
                    className="w-16 h-10 rounded cursor-pointer"
                  />
                  <input
                    type="text"
                    value={newCategory.color}
                    onChange={(e) => setNewCategory({ ...newCategory, color: e.target.value })}
                    placeholder="#6B7280"
                    className="flex-1 px-3 py-2 border rounded-lg dark:bg-gray-700 dark:border-gray-600 font-mono"
                  />
                </div>
              </div>

              <div>
                <label className="block text-sm font-medium mb-2">Icon</label>
                <select
                  value={newCategory.icon}
                  onChange={(e) => setNewCategory({ ...newCategory, icon: e.target.value })}
                  className="w-full px-3 py-2 border rounded-lg dark:bg-gray-700 dark:border-gray-600"
                >
                  <option value="folder">📁 Folder</option>
                  <option value="file-text">📄 Document</option>
                  <option value="video">🎥 Video</option>
                  <option value="music">🎵 Music</option>
                  <option value="image">🖼️ Image</option>
                  <option value="package">📦 Package</option>
                  <option value="archive">🗜️ Archive</option>
                  <option value="star">⭐ Star</option>
                  <option value="briefcase">💼 Briefcase</option>
                </select>
              </div>

              <div>
                <label className="block text-sm font-medium mb-2">Save Path (Optional)</label>
                <input
                  type="text"
                  value={newCategory.save_path}
                  onChange={(e) => setNewCategory({ ...newCategory, save_path: e.target.value })}
                  placeholder="/path/to/save"
                  className="w-full px-3 py-2 border rounded-lg dark:bg-gray-700 dark:border-gray-600"
                />
              </div>
            </div>
            
            <div className="flex gap-3">
              <button
                onClick={createCategory}
                className="flex-1 px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700"
              >
                Create
              </button>
              
              <button
                onClick={() => setShowCreateModal(false)}
                className="flex-1 px-4 py-2 bg-gray-300 dark:bg-gray-600 text-gray-800 dark:text-gray-200 rounded hover:bg-gray-400 dark:hover:bg-gray-500"
              >
                Cancel
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
