// src/components/dialogs/AddCategoryDialog.tsx
import { useState } from "react";
import { X, FolderPlus } from "lucide-react";
import toast from "react-hot-toast";

interface AddCategoryDialogProps {
  onClose: () => void;
  onAdd: (categoryName: string) => void;
}

export function AddCategoryDialog({ onClose, onAdd }: AddCategoryDialogProps) {
  const [categoryName, setCategoryName] = useState("");

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    
    if (!categoryName.trim()) {
      toast.error("Please enter a category name");
      return;
    }

    onAdd(categoryName.trim());
    toast.success(`Category "${categoryName}" created!`);
    onClose();
  };

  return (
    <div className="fixed inset-0 bg-black/60 backdrop-blur-sm flex items-center justify-center z-50">
      <div className="bg-gray-900 rounded-xl shadow-2xl w-full max-w-md border border-gray-800 animate-slide-in-up">
        {/* Header */}
        <div className="flex items-center justify-between p-6 border-b border-gray-800">
          <div className="flex items-center gap-3">
            <div className="p-2 bg-blue-600/20 rounded-lg">
              <FolderPlus className="w-6 h-6 text-blue-500" />
            </div>
            <div>
              <h2 className="text-xl font-semibold text-white">Add Category</h2>
              <p className="text-sm text-gray-400">Create a new download category</p>
            </div>
          </div>
          <button
            onClick={onClose}
            className="p-2 hover:bg-gray-800 rounded-lg transition-colors group"
          >
            <X size={20} className="text-gray-400 group-hover:text-white" />
          </button>
        </div>

        {/* Body */}
        <form onSubmit={handleSubmit} className="p-6">
          <div className="space-y-2">
            <label className="block text-sm font-medium text-gray-300">
              Category Name
            </label>
            <input
              type="text"
              value={categoryName}
              onChange={(e) => setCategoryName(e.target.value)}
              placeholder="e.g., Documents, Software, Games..."
              className="w-full px-4 py-3 bg-gray-800 border border-gray-700 rounded-lg text-white placeholder-gray-500 focus:outline-none focus:border-blue-500 focus:ring-2 focus:ring-blue-500/20 transition-all"
              autoFocus
            />
          </div>

          {/* Footer */}
          <div className="flex justify-end gap-3 mt-6">
            <button
              type="button"
              onClick={onClose}
              className="px-5 py-2.5 text-gray-300 hover:bg-gray-800 rounded-lg transition-colors font-medium"
            >
              Cancel
            </button>
            <button
              type="submit"
              disabled={!categoryName.trim()}
              className="px-5 py-2.5 bg-blue-600 hover:bg-blue-700 disabled:bg-gray-700 disabled:cursor-not-allowed text-white rounded-lg transition-all flex items-center gap-2 font-medium shadow-lg shadow-blue-500/20 disabled:shadow-none"
            >
              <FolderPlus size={18} />
              Create Category
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
