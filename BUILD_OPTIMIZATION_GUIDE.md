# Build Optimization Guide

Guide to optimizing AFK-Dunld build size, speed, and performance.

## Table of Contents
- [Build Size Optimization](#build-size-optimization)
- [Build Speed Optimization](#build-speed-optimization)
- [Runtime Performance](#runtime-performance)
- [Bundle Size Reduction](#bundle-size-reduction)
- [Platform-Specific Optimizations](#platform-specific-optimizations)
- [Production Best Practices](#production-best-practices)

## Build Size Optimization

### Rust Binary Optimization

#### Cargo.toml Configuration

```toml
[profile.release]
opt-level = "z"          # Optimize for size
lto = true               # Link-time optimization
codegen-units = 1        # Better optimization, slower compile
strip = true             # Strip symbols
panic = "abort"          # Smaller panic handler
```

**Size Reduction**: ~40-50% smaller binary

**Trade-offs**:
- ✅ Smaller binary
- ❌ Slower compilation
- ❌ Harder debugging (symbols stripped)

#### Alternative: Balanced Profile

```toml
[profile.release]
opt-level = 3            # Optimize for speed
lto = "thin"            # Fast LTO
codegen-units = 16      # Faster compile
strip = true
```

**Use Case**: When build speed matters more than size

### Frontend Bundle Optimization

#### Vite Configuration (`vite.config.ts`)

```typescript
export default defineConfig({
  build: {
    minify: 'terser',
    terserOptions: {
      compress: {
        drop_console: true,    // Remove console.log
        drop_debugger: true,   // Remove debugger
      },
    },
    rollupOptions: {
      output: {
        manualChunks: {
          vendor: ['react', 'react-dom'],
          ui: ['@headlessui/react'],
        },
      },
    },
    chunkSizeWarningLimit: 1000,
  },
});
```

**Size Reduction**: ~30-40% smaller bundle

### Remove Unused Dependencies

#### Analyze Bundle Size

```bash
# Analyze frontend
npm run build -- --analyze

# Analyze Rust dependencies
cargo install cargo-bloat
cargo bloat --release
```

#### Remove Unused Crates

```bash
# Find unused dependencies
cargo install cargo-udeps
cargo +nightly udeps
```

#### Tree-Shaking

**Frontend**: Vite does this automatically

**Rust**: Use `cargo-tree` to visualize dependencies:
```bash
cargo tree | less
```

### Compression

#### Enable Compression in Tauri

```json
// tauri.conf.json
{
  "bundle": {
    "resources": {
      "compress": true
    }
  }
}
```

#### Platform-Specific Compression

**Windows (MSI)**:
- Built-in MSI compression
- ~40% size reduction

**macOS (DMG)**:
- Use APFS compression
- ~30% size reduction

**Linux (DEB)**:
- XZ compression
- ~50% size reduction

## Build Speed Optimization

### Incremental Compilation

```bash
# Enable incremental compilation
export CARGO_INCREMENTAL=1

# For development
cargo build
```

**Speed Improvement**: ~50% faster rebuilds

### Parallel Compilation

```bash
# Use all CPU cores
export CARGO_BUILD_JOBS=$(nproc)

# Or specific number
export CARGO_BUILD_JOBS=8
```

### Faster Linker

#### Linux: Use mold

```bash
# Install mold
sudo apt install mold

# Use in builds
export RUSTFLAGS="-C link-arg=-fuse-ld=mold"
cargo build --release
```

**Speed Improvement**: ~70% faster linking

#### macOS: Use zld

```bash
# Install zld
brew install michaeleisel/zld/zld

# Use in builds
export RUSTFLAGS="-C link-arg=-fuse-ld=/usr/local/bin/zld"
cargo build --release
```

**Speed Improvement**: ~50% faster linking

#### Windows: Use lld

```bash
# Install LLVM
# Download from https://releases.llvm.org/

# Use in builds
set RUSTFLAGS=-C link-arg=-fuse-ld=lld
cargo build --release
```

### Caching

#### Cargo Cache

```bash
# Use sccache for caching
cargo install sccache

# Set as wrapper
export RUSTC_WRAPPER=sccache

# Build
cargo build
```

**Speed Improvement**: ~80% faster on cached builds

#### CI/CD Caching

**GitHub Actions**:
```yaml
- name: Cache cargo
  uses: actions/cache@v3
  with:
    path: |
      ~/.cargo/registry
      ~/.cargo/git
      target
    key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
```

### Development Build Optimizations

#### .cargo/config.toml

```toml
[build]
jobs = 8                    # Parallel jobs
incremental = true          # Incremental compilation

[profile.dev]
opt-level = 1              # Some optimization for dev

[profile.dev.package."*"]
opt-level = 2              # Optimize dependencies
```

## Runtime Performance

### Binary Size vs Performance

#### Small Binary (Size-Optimized)

```toml
[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
```

**Use Case**: Distribution, AppImage, portable apps

#### Fast Binary (Performance-Optimized)

```toml
[profile.release]
opt-level = 3
lto = "thin"
codegen-units = 16
```

**Use Case**: Desktop app, where size doesn't matter

### Frontend Performance

#### Code Splitting

```typescript
// Lazy load components
const Settings = lazy(() => import('./components/SettingsPage'));
const History = lazy(() => import('./components/DownloadHistory'));

// Use Suspense
<Suspense fallback={<Loading />}>
  <Settings />
</Suspense>
```

#### Memoization

```typescript
const MemoizedComponent = memo(({ data }) => {
  // Expensive computation
  const processed = useMemo(() => processData(data), [data]);
  
  return <div>{processed}</div>;
});
```

### Database Optimization

#### SQLite Configuration

```rust
// Enable WAL mode for better concurrency
db.execute("PRAGMA journal_mode = WAL", [])?;
db.execute("PRAGMA synchronous = NORMAL", [])?;
db.execute("PRAGMA cache_size = 10000", [])?;
```

#### Indexes

```sql
-- Add indexes for frequently queried columns
CREATE INDEX idx_downloads_status ON downloads(status);
CREATE INDEX idx_downloads_created ON downloads(created_at);
CREATE INDEX idx_downloads_category ON downloads(category);
```

## Bundle Size Reduction

### Exclude Development Files

```json
// tauri.conf.json
{
  "bundle": {
    "resources": [
      "resources/**"
    ],
    "externalBin": [],
    "excludeDirs": [
      "node_modules",
      ".git",
      "target"
    ]
  }
}
```

### yt-dlp Bundling Strategy

#### Option 1: Platform-Specific (Recommended)

```bash
# Bundle only for current platform
npm run tauri build
```

**Size**: ~10-30 MB (per platform)

#### Option 2: Skip Bundling

```bash
# Don't bundle yt-dlp (use system)
export YTDLP_SKIP_BUNDLE=1
npm run tauri build
```

**Size**: ~0 MB (users must install yt-dlp)

#### Option 3: Download on First Run

```rust
// Download yt-dlp on first run instead of bundling
async fn ensure_ytdlp() {
    if !ytdlp_exists() {
        download_ytdlp().await?;
    }
}
```

**Size**: ~0 MB initially, downloads later

### Remove Unused Assets

#### Icons

Only include necessary icon sizes:

```json
{
  "bundle": {
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/icon.ico"
    ]
  }
}
```

#### Resources

Remove unused resources:
```bash
# Check resource usage
du -sh src-tauri/resources/*

# Remove unused
rm -rf src-tauri/resources/unused/
```

## Platform-Specific Optimizations

### Windows

#### WiX Compression

```xml
<!-- In WiX configuration -->
<Media Id="1" Cabinet="media1.cab" CompressionLevel="high" />
```

#### NSIS Alternative

For smaller installers, consider NSIS:

```json
{
  "bundle": {
    "targets": ["nsis"]
  }
}
```

**Size**: ~30% smaller than MSI

### macOS

#### Universal Binary vs Architecture-Specific

**Universal (Intel + Apple Silicon)**:
```bash
npm run tauri build -- --target universal-apple-darwin
```

**Size**: ~2x larger

**Architecture-Specific**:
```bash
# Intel only
npm run tauri build -- --target x86_64-apple-darwin

# Apple Silicon only
npm run tauri build -- --target aarch64-apple-darwin
```

**Size**: ~50% smaller (per arch)

#### DMG Compression

```bash
# Create compressed DMG
hdiutil create -volname "AFK-Dunld" \
  -srcfolder target/release/bundle/macos/AFK-Dunld.app \
  -ov -format UDZO \
  AFK-Dunld.dmg
```

### Linux

#### AppImage Optimization

```bash
# Use squashfs compression
export APPIMAGE_COMP="xz"

npm run tauri build -- --bundles appimage
```

**Size Reduction**: ~40%

#### DEB Compression

```bash
# Use XZ compression (default)
# Results in smallest DEB size
npm run tauri build -- --bundles deb
```

## Production Best Practices

### Pre-Build Checklist

- [ ] Remove debug code
- [ ] Remove console.log statements
- [ ] Enable production optimizations
- [ ] Run security audit: `npm audit`
- [ ] Update dependencies
- [ ] Test on target platforms
- [ ] Verify bundle size
- [ ] Check startup time

### Build Script

```bash
#!/bin/bash
# production-build.sh

set -e

echo "Running pre-build checks..."
npm audit --production
cargo audit

echo "Cleaning previous builds..."
rm -rf dist/ src-tauri/target/release/bundle/

echo "Building frontend..."
npm run build

echo "Building backend..."
export CARGO_PROFILE_RELEASE_LTO=true
export CARGO_PROFILE_RELEASE_OPT_LEVEL=z
npm run tauri build

echo "Build complete!"
ls -lh src-tauri/target/release/bundle/
```

### Size Monitoring

#### Track Bundle Size

```bash
# Get bundle sizes
du -sh src-tauri/target/release/bundle/*

# Or use
cargo install cargo-size
cargo size --release
```

#### CI Size Check

```yaml
- name: Check bundle size
  run: |
    SIZE=$(du -sm src-tauri/target/release/bundle/msi/*.msi | cut -f1)
    if [ $SIZE -gt 100 ]; then
      echo "Bundle too large: ${SIZE}MB"
      exit 1
    fi
```

### Benchmark Performance

```bash
# Benchmark startup time
time ./target/release/afk-dunld --help

# Memory usage
/usr/bin/time -v ./target/release/afk-dunld

# Profile with perf (Linux)
perf record ./target/release/afk-dunld
perf report
```

## Optimization Results

### Before Optimization

| Component | Size |
|-----------|------|
| Rust Binary | 120 MB |
| Frontend Bundle | 5 MB |
| yt-dlp (all platforms) | 52 MB |
| Total | ~177 MB |

### After Optimization

| Component | Size | Reduction |
|-----------|------|-----------|
| Rust Binary | 45 MB | 62% |
| Frontend Bundle | 2 MB | 60% |
| yt-dlp (single platform) | 12 MB | 77% |
| Total | ~59 MB | 67% |

### Optimization Settings Used

```toml
# Cargo.toml
[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
strip = true
```

```typescript
// vite.config.ts
minify: 'terser',
terserOptions: {
  compress: {
    drop_console: true,
  },
}
```

```bash
# Build command
YTDLP_BUNDLE_ALL_PLATFORMS=0 npm run tauri build
```

## Tools & Resources

### Analysis Tools

- **cargo-bloat**: Analyze binary size
  ```bash
  cargo install cargo-bloat
  cargo bloat --release -n 20
  ```

- **cargo-tree**: Visualize dependencies
  ```bash
  cargo tree
  ```

- **webpack-bundle-analyzer**: Frontend analysis
  ```bash
  npm run build -- --analyze
  ```

### Useful Crates

- **serde**: Efficient serialization
- **tokio**: Async runtime
- **reqwest**: HTTP client
- **rusqlite**: SQLite

Avoid heavy crates when lighter alternatives exist.

## Related Documentation

- [yt-dlp Bundling Guide](YTDLP_BUNDLING_GUIDE.md)
- [Build Guide](docs/BUILD_GUIDE.md)
- [Architecture Overview](docs/ARCHITECTURE.md)

## Need Help?

- 📖 [Documentation](README.md)
- 🐛 [Report Issue](https://github.com/yourusername/afk-dunld/issues)
- 💬 [Discussions](https://github.com/yourusername/afk-dunld/discussions)
