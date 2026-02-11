// src/utils/format.ts
import { clsx, type ClassValue } from 'clsx';
import { twMerge } from 'tailwind-merge';

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export function formatBytes(bytes: number): string {
    if (bytes === 0) return "0 B";
    const k = 1024;
    const sizes = ["B", "KB", "MB", "GB", "TB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return `${(bytes / Math.pow(k, i)).toFixed(2)} ${sizes[i]}`;
  }
  
  export function formatSpeed(bytesPerSecond: number): string {
    if (bytesPerSecond === 0) return "0 B/s";
    return `${formatBytes(bytesPerSecond)}/s`;
  }
  
  export function formatEta(seconds: number): string {
    if (seconds <= 0) return "∞";
    if (seconds < 60) return `${Math.round(seconds)}s`;
    if (seconds < 3600) {
      const m = Math.floor(seconds / 60);
      const s = Math.round(seconds % 60);
      return `${m}m ${s}s`;
    }
    const h = Math.floor(seconds / 3600);
    const m = Math.floor((seconds % 3600) / 60);
    return `${h}h ${m}m`;
  }
  
  export function formatDate(dateStr: string): string {
    return new Date(dateStr).toLocaleString();
  }
  
  export function getFileIcon(fileName: string): string {
    const ext = fileName.split(".").pop()?.toLowerCase() || "";
    const iconMap: Record<string, string> = {
      pdf: "📄",
      zip: "📦", rar: "📦", "7z": "📦", tar: "📦", gz: "📦",
      mp4: "🎬", mkv: "🎬", avi: "🎬", mov: "🎬", webm: "🎬",
      mp3: "🎵", flac: "🎵", wav: "🎵", aac: "🎵", ogg: "🎵",
      jpg: "🖼️", jpeg: "🖼️", png: "🖼️", gif: "🖼️", webp: "🖼️", svg: "🖼️",
      exe: "⚙️", msi: "⚙️", dmg: "⚙️", deb: "⚙️", rpm: "⚙️",
      iso: "💿",
      doc: "📝", docx: "📝", txt: "📝",
      xls: "📊", xlsx: "📊", csv: "📊",
    };
    return iconMap[ext] || "📁";
  }