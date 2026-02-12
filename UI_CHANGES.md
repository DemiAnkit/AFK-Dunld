# UI Changes Not Showing? Here's How to Fix It

## The Problem
Tauri caches the frontend build, so you need to **fully rebuild** to see CSS and UI changes.

## Quick Fix

### Option 1: Use the Rebuild Script (Recommended)

**On Windows:**
```bash
rebuild.bat
```

**On Mac/Linux:**
```bash
bash rebuild.sh
```

### Option 2: Manual Steps

1. **Clean the cache:**
   ```bash
   # Delete old builds
   rm -rf dist
   rm -rf src-tauri/target/debug/build
   rm -rf node_modules/.vite
   ```

2. **Rebuild frontend:**
   ```bash
   npm run build
   ```

3. **Rebuild Tauri:**
   ```bash
   cd src-tauri
   cargo build --release
   ```

4. **Run the app:**
   ```bash
   npm run tauri dev
   ```

## What Was Fixed

### 1. **Tailwind CSS v4 Configuration**
   - Updated `globals.css` to use proper v4 syntax (`@import "tailwindcss"`)
   - Added custom theme variables for colors
   - Included custom utility classes (glow effects, animations, etc.)

### 2. **CSS is Now Building Correctly**
   - Before: 18KB CSS file (missing custom styles)
   - After: 66KB CSS file (all styles included)

### 3. **New UI Features**
   - **Gradient buttons** with hover animations
   - **Glow effects** on interactive elements
   - **Smooth animations** (scale, rotate, fade)
   - **Glass morphism** effects
   - **Custom scrollbars**
   - **Button shine effects**

## Testing the Changes

After rebuilding, you should see:

1. **Header buttons** have gradient backgrounds and glow on hover
2. **Add Download button** has a shine effect animation
3. **Modal dialogs** have smooth fade/scale animations
4. **Action buttons** in download rows scale up and glow on hover
5. **View toggle buttons** have blue highlight when active

## Development Mode

For real-time changes during development:

```bash
npm run tauri dev
```

**Note:** CSS changes in `globals.css` still require a page refresh to see. For instant updates, modify component files directly.

## Troubleshooting

### Still not seeing changes?

1. **Hard refresh:** Press `Ctrl+Shift+R` in the app
2. **Clear app data:** Delete `%APPDATA%/afk-dunld` folder
3. **Check console:** Open DevTools with `Ctrl+Shift+I` and look for errors
4. **Verify build:** Check that `dist/assets/index-*.css` is ~66KB

### Build errors?

```bash
# Clean everything and reinstall
rm -rf node_modules
rm -rf dist
rm -rf src-tauri/target
npm install
npm run build
```

## UI Components Enhanced

### Button Component
- ✅ Gradient backgrounds (blue, green, red, orange)
- ✅ Scale animations on hover/click
- ✅ Box shadows with glow effects
- ✅ Loading state with spinner
- ✅ Icon support (left/right)

### Modal Component  
- ✅ Smooth entrance/exit animations
- ✅ Backdrop blur effect
- ✅ Scale and fade transitions
- ✅ Escape key to close
- ✅ Click outside to close

### Header
- ✅ "Add Download" button with shine effect
- ✅ Icon buttons with color-coded hover states
- ✅ View mode toggle with blue highlight
- ✅ Gradient text for title

### Download Table Row
- ✅ Action buttons with hover glow
- ✅ Scale effects on hover
- ✅ Color-coded icons (blue folder, red delete, etc.)

### Toolbar
- ✅ Gradient buttons
- ✅ Rotating settings icon
- ✅ Hover scale effects

## Color Scheme

- **Primary (Blue):** `from-blue-600 to-blue-700`
- **Success (Green):** `from-green-600 to-green-700`
- **Danger (Red):** `from-red-600 to-red-700`
- **Warning (Orange):** `from-orange-500 to-orange-600`
- **Secondary:** `from-gray-700 to-gray-800`

All buttons have:
- Border radius: `rounded-xl` (12px)
- Shadow: `shadow-lg` with colored glow
- Hover: `hover:scale-105` (5% scale up)
- Active: `active:scale-95` (5% scale down)
- Transition: `duration-200` (200ms)
