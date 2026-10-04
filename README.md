# Extinction Point Overlay (Windows)

A tiny transparent window that shows https://extinctionpoint.app/overlay on top of The Isle.

## Player controls
- Run The Isle in **Borderless Windowed** mode.
- **` (tilde / Backquote)** (or **Ctrl+Alt+M** as a backup) – switch between "click-through" (playing) and "interactive" (use F6 setup / F3 console, drag the health hexagon).
- **Tray icon** (skull, bottom-right of the taskbar) – left-click toggles mouse control; right-click for Toggle / Quit. Works even if the game blocks hotkeys.
- **Ctrl+Shift+Q** – close the overlay.
- Sign in once in interactive mode (Steam login) so your dino data shows.

## Building the .exe (easiest: GitHub, no installs)
1. Create a new GitHub repository and upload everything in this folder.
2. Open the **Actions** tab → "Build Windows overlay" → **Run workflow**.
3. After ~10 minutes, download the `ExtinctionPointOverlay` artifact.
   - `extinction-overlay.exe` = portable, just double-click.
   - `bundle/nsis/...setup.exe` = optional installer.

## Building on your own Windows PC
1. Install Node.js 20+ and Rust (https://rustup.rs).
2. In this folder: `npm install` then `npm run build`.

## Updates
All HUD changes come from the website automatically. Only rebuild if you change hotkeys or window behavior here.
