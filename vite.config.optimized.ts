import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// Optimized Vite configuration for production builds
export default defineConfig({
  plugins: [react()],
  
  // Optimize dependencies
  optimizeDeps: {
    include: [
      'react',
      'react-dom',
      'react-router-dom',
      'zustand',
      'date-fns',
      'lucide-react',
      '@tauri-apps/api',
      '@tauri-apps/plugin-shell',
      '@tauri-apps/plugin-dialog',
      '@tauri-apps/plugin-fs',
      '@tauri-apps/plugin-notification',
      '@tauri-apps/plugin-clipboard-manager',
    ],
  },

  build: {
    // Production optimizations
    target: 'esnext',
    minify: 'terser',
    terserOptions: {
      compress: {
        drop_console: true, // Remove console.logs in production
        drop_debugger: true,
        pure_funcs: ['console.log', 'console.debug'],
      },
    },
    
    // Code splitting for better caching
    rollupOptions: {
      output: {
        manualChunks: {
          // Vendor chunk for React and core libraries
          'vendor-react': ['react', 'react-dom', 'react-router-dom'],
          
          // UI components chunk
          'vendor-ui': ['lucide-react', 'framer-motion', 'react-hot-toast'],
          
          // State management
          'vendor-state': ['zustand', '@tanstack/react-query'],
          
          // Tauri APIs
          'vendor-tauri': [
            '@tauri-apps/api',
            '@tauri-apps/plugin-shell',
            '@tauri-apps/plugin-dialog',
            '@tauri-apps/plugin-fs',
            '@tauri-apps/plugin-notification',
            '@tauri-apps/plugin-clipboard-manager',
          ],
          
          // Date utilities
          'vendor-utils': ['date-fns', 'clsx', 'tailwind-merge'],
        },
      },
    },
    
    // Chunk size warnings
    chunkSizeWarningLimit: 1000,
    
    // Source maps for debugging (disable in production if not needed)
    sourcemap: false,
    
    // CSS optimization
    cssCodeSplit: true,
    cssMinify: true,
  },

  // Preview server config
  preview: {
    port: 1420,
    strictPort: true,
  },

  // Clear screen
  clearScreen: false,
  
  // Server config for development
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      ignored: ["**/src-tauri/**"],
    },
  },
});
