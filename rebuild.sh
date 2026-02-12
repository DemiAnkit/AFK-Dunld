#!/bin/bash

echo "🧹 Cleaning build cache..."
rm -rf dist
rm -rf src-tauri/target/debug/build
rm -rf node_modules/.vite

echo "📦 Building frontend..."
npm run build

echo "🔨 Building Tauri app..."
cd src-tauri
cargo build --release

echo "✅ Build complete!"
echo ""
echo "To run the app:"
echo "  npm run tauri dev    # For development"
echo "  ./src-tauri/target/release/afk-dunld.exe  # For production"
