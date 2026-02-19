// src-tauri/src/commands/system_commands.rs

use tauri::State;
use serde::{Deserialize, Serialize};

use crate::state::app_state::AppState;

/// System information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub os: String,
    pub os_version: String,
    pub available_disk_space: u64, // bytes
    pub total_disk_space: u64,     // bytes
    pub cpu_count: usize,
    pub total_memory: u64,         // bytes
}

/// Get system information
#[tauri::command]
pub async fn get_system_info(
    state: State<'_, AppState>,
) -> Result<SystemInfo, String> {
    let download_dir = &state.download_dir;
    
    // Get disk space info for download directory
    let (available_space, total_space) = get_disk_space(download_dir)
        .unwrap_or((0, 0));
    
    Ok(SystemInfo {
        os: std::env::consts::OS.to_string(),
        os_version: get_os_version(),
        available_disk_space: available_space,
        total_disk_space: total_space,
        cpu_count: num_cpus::get(),
        total_memory: get_total_memory(),
    })
}

/// Check if there's enough disk space for a download
#[tauri::command]
pub async fn check_disk_space(
    state: State<'_, AppState>,
    required_bytes: u64,
) -> Result<bool, String> {
    let download_dir = &state.download_dir;
    
    let (available_space, _) = get_disk_space(download_dir)
        .map_err(|e| e.to_string())?;
    
    // Add 10% buffer for safety
    let required_with_buffer = (required_bytes as f64 * 1.1) as u64;
    
    Ok(available_space >= required_with_buffer)
}

/// Get available and total disk space for a path
fn get_disk_space(path: &std::path::Path) -> Result<(u64, u64), String> {
    #[cfg(target_os = "windows")]
    {
        use std::ffi::OsStr;
        use std::os::windows::ffi::OsStrExt;
        use winapi::um::winnt::ULARGE_INTEGER;
        
        let path_wide: Vec<u16> = OsStr::new(path)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();
        
        unsafe {
            let mut free_bytes: ULARGE_INTEGER = std::mem::zeroed();
            let mut total_bytes: ULARGE_INTEGER = std::mem::zeroed();
            let mut total_free_bytes: ULARGE_INTEGER = std::mem::zeroed();
            
            if winapi::um::fileapi::GetDiskFreeSpaceExW(
                path_wide.as_ptr(),
                &mut free_bytes,
                &mut total_bytes,
                &mut total_free_bytes,
            ) != 0
            {
                Ok((*free_bytes.QuadPart() as u64, *total_bytes.QuadPart() as u64))
            } else {
                Err("Failed to get disk space".to_string())
            }
        }
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        use std::ffi::CString;
        use std::mem;
        
        let path_cstr = CString::new(path.to_str().unwrap_or("/"))
            .map_err(|e| e.to_string())?;
        
        unsafe {
            let mut stat: libc::statvfs = mem::zeroed();
            if libc::statvfs(path_cstr.as_ptr(), &mut stat) == 0 {
                let block_size = stat.f_frsize as u64;
                let available = stat.f_bavail as u64 * block_size;
                let total = stat.f_blocks as u64 * block_size;
                Ok((available, total))
            } else {
                Err("Failed to get disk space".to_string())
            }
        }
    }
}

/// Open the download folder in the system file manager
#[tauri::command]
pub async fn open_download_folder(
    state: State<'_, AppState>,
) -> Result<(), String> {
    use std::process::Command;
    
    // Get the download path from settings or use the default from state
    let settings_map = state.db.get_all_settings()
        .await
        .map_err(|e| format!("Failed to get settings: {}", e))?;
    
    let download_path = settings_map
        .get("download_path")
        .filter(|p| !p.is_empty())
        .map(|p| p.to_string())
        .unwrap_or_else(|| state.download_dir.to_string_lossy().to_string());
    
    tracing::info!("Opening download folder: {}", download_path);
    
    // Check if the folder exists
    let path = std::path::Path::new(&download_path);
    if !path.exists() {
        tracing::warn!("Download folder does not exist, creating: {}", download_path);
        tokio::fs::create_dir_all(&path)
            .await
            .map_err(|e| format!("Failed to create download folder: {}", e))?;
    }
    
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        
        let path_str = download_path.replace("/", "\\");
        tracing::info!("Windows: Opening with 'explorer {}'", path_str);
        
        let result = Command::new("explorer")
            .creation_flags(CREATE_NO_WINDOW)
            .arg(&path_str)
            .spawn();
        
        match result {
            Ok(_) => {
                tracing::info!("Successfully opened Explorer");
                Ok(())
            }
            Err(e) => {
                tracing::error!("Failed to open Explorer: {}", e);
                Err(format!("Failed to open Explorer: {}", e))
            }
        }
    }
    
    #[cfg(target_os = "macos")]
    {
        tracing::info!("macOS: Opening with 'open {}'", download_path);
        
        let result = Command::new("open")
            .arg(&download_path)
            .spawn();
        
        match result {
            Ok(_) => {
                tracing::info!("Successfully opened Finder");
                Ok(())
            }
            Err(e) => {
                tracing::error!("Failed to open Finder: {}", e);
                Err(format!("Failed to open Finder: {}", e))
            }
        }
    }
    
    #[cfg(target_os = "linux")]
    {
        tracing::info!("Linux: Opening with 'xdg-open {}'", download_path);
        
        let result = Command::new("xdg-open")
            .arg(&download_path)
            .spawn();
        
        match result {
            Ok(_) => {
                tracing::info!("Successfully opened file manager");
                Ok(())
            }
            Err(e) => {
                tracing::error!("Failed to open file manager: {}", e);
                Err(format!("Failed to open file manager: {}", e))
            }
        }
    }
    
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        Err("Unsupported operating system".to_string())
    }
}

/// Get OS version string
fn get_os_version() -> String {
    #[cfg(target_os = "windows")]
    {
        // GetVersionEx is deprecated, use a simpler approach
        // We can use the registry or just return a generic version
        // For now, return Windows with generic identifier
        "Windows".to_string()
    }
    
    #[cfg(target_os = "macos")]
    {
        "macOS".to_string() // Could use sysctl for more details
    }
    
    #[cfg(target_os = "linux")]
    {
        std::fs::read_to_string("/etc/os-release")
            .ok()
            .and_then(|s| {
                s.lines()
                    .find(|line| line.starts_with("PRETTY_NAME="))
                    .map(|line| line.trim_start_matches("PRETTY_NAME=").trim_matches('"').to_string())
            })
            .unwrap_or_else(|| "Linux".to_string())
    }
    
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        "Unknown".to_string()
    }
}

/// Get total system memory
fn get_total_memory() -> u64 {
    #[cfg(target_os = "windows")]
    {
        use winapi::um::sysinfoapi::{GlobalMemoryStatusEx, MEMORYSTATUSEX};
        use std::mem;
        
        unsafe {
            let mut mem_info: MEMORYSTATUSEX = mem::zeroed();
            mem_info.dwLength = mem::size_of::<MEMORYSTATUSEX>() as u32;
            
            if GlobalMemoryStatusEx(&mut mem_info) != 0 {
                mem_info.ullTotalPhys as u64
            } else {
                0
            }
        }
    }
    
    #[cfg(target_os = "macos")]
    {
        use std::mem;
        unsafe {
            let mut size: u64 = 0;
            let mut len = mem::size_of::<u64>();
            let name = b"hw.memsize\0";
            if libc::sysctlbyname(
                name.as_ptr() as *const i8,
                &mut size as *mut _ as *mut libc::c_void,
                &mut len,
                std::ptr::null_mut(),
                0,
            ) == 0
            {
                size
            } else {
                0
            }
        }
    }
    
    #[cfg(target_os = "linux")]
    {
        std::fs::read_to_string("/proc/meminfo")
            .ok()
            .and_then(|s| {
                s.lines()
                    .find(|line| line.starts_with("MemTotal:"))
                    .and_then(|line| {
                        line.split_whitespace()
                            .nth(1)
                            .and_then(|n| n.parse::<u64>().ok())
                            .map(|kb| kb * 1024)
                    })
            })
            .unwrap_or(0)
    }
    
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        0
    }
}
