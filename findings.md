# Findings & Decisions — Omni Text (macOS) — Tauri + Rust

## Requirements

### Core
- macOS menu bar (system tray) app — text rewriting via global hotkeys
- Select text anywhere → hotkey → LLM rewrites → replaces selection
- OpenAI-compatible API support (cloud + local endpoints)
- Glassmorphism / Liquid Glass UI via native vibrancy + CSS
- **Tauri v2 + Rust backend + Svelte 5 frontend + Tailwind CSS v4**

### MVP Features
- System tray with 3 icon states: Ready, Processing, Not Ready
- 2 default actions: Paraphrase (⌘⌥⌃P), Grammar Fix (⌘⌥⌃G)
- Add/edit/delete custom actions with hotkey capture
- 1 provider profile + connection test
- Accessibility-based text replace + clipboard fallback
- History with search + copy (SQLite)
- Settings window with 5 tabs
- Glassmorphism UI with native macOS vibrancy

## Research Findings

### Tauri v2 System Tray
- `TrayIconBuilder` struct (new in v2, replaces v1's tray_handle)
- Supports dynamic icon changes, custom menus, event handling
- Use `tauri-plugin-positioner` with `Position::TrayCenter` for popover below tray
- `app.set_activation_policy(tauri::ActivationPolicy::Accessory)` to hide from Dock
- Window config: `decorations: false`, `transparent: true`, `visible: false`, `alwaysOnTop: true`

### Global Hotkeys
- `tauri-plugin-global-shortcut` (official Tauri v2 plugin)
- `app.global_shortcut().on_shortcut("cmd+ctrl+alt+p", handler)`
- If another app uses the shortcut, handler won't trigger — need alternatives

### Text Capture (Accessibility API from Rust)
- **`get-selected-text`** crate — specialized for getting selected text
- **`macos-accessibility-client`** — high-level wrapper, permission handling
- **`accessibility-sys`** — low-level AXUIElement bindings
- Requires Accessibility permission in System Settings
- Use `AXUIElementCreateSystemWide()` → focused element → `kAXSelectedTextAttribute`

### Clipboard & Keystroke Simulation
- **`tauri-plugin-clipboard`** — official Tauri clipboard access
- **`arboard`** crate — cross-platform clipboard (1Password maintained)
- **`enigo`** crate — keystroke simulation (Cmd+C/V) — CAUTION: crashes reported on macOS
- **`rdev`** crate — alternative for keystroke simulation
- Must save/restore original clipboard contents

### Keychain Storage
- **`keyring`** crate with `apple-native` feature — macOS Keychain access
- `Entry::new(service, username)` → `set_password()` / `get_password()`
- Alternative: `security-framework` crate for direct API

### Persistent Storage
- **Settings/Preferences**: `tauri-plugin-store` (JSON key-value)
- **History/Logs**: SQLite via `tauri-plugin-sql` or `rusqlite`
- **Secrets**: macOS Keychain via `keyring`

### Frontend
- **Svelte 5** recommended: smallest bundle, best DX for small apps, compile-time optimization
- **Tailwind CSS v4**: one-line setup with Vite plugin, 182x faster incremental builds
- Font stack: `-apple-system, BlinkMacSystemFont, "SF Pro Display", sans-serif` (avoid `system-ui` bug)

### Native Vibrancy / Glassmorphism
- `window-vibrancy` crate integrated into Tauri v2
- `apply_vibrancy(window, NSVisualEffectMaterial::HudWindow, None, None)`
- Requires `macOSPrivateApi: true` + `transparent: true` in tauri.conf.json
- **CSS `backdrop-filter` does NOT work** with transparent Tauri windows for behind-window blur
- Use native vibrancy for window-level blur, CSS glassmorphism for internal card effects
- `tauri-plugin-liquid-glass` for macOS 26+ native glass effect

### Dark Mode
- `appWindow.theme()` and `onThemeChanged()` API in Tauri v2
- Tailwind `dark:` variant works automatically
- Listen for theme changes and update `data-theme` attribute

## Technical Decisions

| Decision | Rationale |
|----------|-----------|
| Tauri v2 + Rust | User preference; cross-platform potential; small binary |
| Svelte 5 frontend | Smallest bundle, compile-time optimization, great DX |
| Tailwind CSS v4 | Fast builds, utility-first, dark mode built-in |
| tauri-plugin-global-shortcut | Official plugin, simple API |
| get-selected-text + accessibility-sys | Text capture from any app |
| arboard + rdev for clipboard fallback | Clipboard access + keystroke simulation |
| keyring (apple-native) | macOS Keychain for API key storage |
| tauri-plugin-store + tauri-plugin-sql | Settings (JSON) + History (SQLite) |
| window-vibrancy (HudWindow) | Native macOS frosted glass effect |
| ActivationPolicy::Accessory | Hide from Dock, menu bar only |
| tauri-plugin-positioner | Position popover below tray icon |

## Resources
- Tauri v2 System Tray: https://v2.tauri.app/learn/system-tray/
- Tauri Global Shortcut: https://v2.tauri.app/plugin/global-shortcut/
- Tauri Positioner: https://v2.tauri.app/plugin/positioner/
- Tauri Store: https://v2.tauri.app/plugin/store/
- Tauri SQL: https://v2.tauri.app/plugin/sql/
- window-vibrancy: https://github.com/tauri-apps/window-vibrancy
- keyring crate: https://crates.io/crates/keyring
- get-selected-text: https://crates.io/crates/get-selected-text
- macos-accessibility-client: https://crates.io/crates/macos-accessibility-client
- arboard: https://github.com/1Password/arboard
- rdev: https://github.com/Narsil/rdev
- tauri-plugin-liquid-glass: https://github.com/hkandala/tauri-plugin-liquid-glass
- Tauri + SvelteKit: https://v2.tauri.app/start/frontend/sveltekit/
