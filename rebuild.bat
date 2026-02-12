@echo off
chcp 65001 >nul
echo 🧹 Cleaning build cache...
if exist dist rmdir /s /q dist
if exist src-tauri\target\debug\build rmdir /s /q src-tauri\target\debug\build
if exist node_modules\.vite rmdir /s /q node_modules\.vite

echo 📦 Building frontend...
call npm run build
if errorlevel 1 (
    echo ❌ Frontend build failed!
    exit /b 1
)

echo 🔨 Building Tauri app...
cd src-tauri
cargo build --release
if errorlevel 1 (
    echo ❌ Tauri build failed!
    exit /b 1
)

echo ✅ Build complete!
echo.
echo To run the app:
echo   npm run tauri dev    # For development
echo   src-tauri\target\release\afk-dunld.exe  # For production
