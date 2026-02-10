# 1. Install prerequisites
# Install Rust: https://rustup.rs
# Install Node.js: https://nodejs.org

# 2. Create project
npm create tauri-app@latest AFK-Dunld -- \
  --template react-ts

cd AFK-Dunld

# 3. Install frontend dependencies
npm install zustand @tanstack/react-query framer-motion \
  lucide-react react-router-dom react-hot-toast recharts \
  clsx tailwind-merge date-fns

npm install -D tailwindcss postcss autoprefixer
npx tailwindcss init -p

# 4. Add Tauri plugins
cd src-tauri
cargo add tauri-plugin-dialog tauri-plugin-fs \
  tauri-plugin-notification tauri-plugin-clipboard-manager \
  tauri-plugin-shell tauri-plugin-autostart \
  tauri-plugin-single-instance

cargo add tokio --features full
cargo add reqwest --features "stream json gzip socks"
cargo add sqlx --features "runtime-tokio sqlite chrono"
cargo add serde --features derive
cargo add serde_json uuid --features "uuid/v4 uuid/serde"
cargo add chrono --features serde
cargo add thiserror anyhow url sha2 md-5
cargo add tracing tracing-subscriber
cargo add flume arboard notify-rust dirs humansize
cargo add tokio-util --features "io sync"
cargo add futures-util governor regex

cd ..

# 5. Run development
npm run tauri dev

# 6. Build for production
npm run tauri build