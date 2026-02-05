# Omni Text

A macOS menu bar app that rewrites selected text using LLMs — triggered by global hotkeys.

Select text anywhere, press a shortcut, and get it rewritten in-place. Proofread, rephrase, translate, or run any custom prompt. Works across every app on your Mac.

## How It Works

1. Select text in any application
2. Press your configured hotkey (e.g. `Cmd+Alt+Shift+P`)
3. The text is sent to your chosen LLM provider
4. The rewritten text replaces your selection
5. `Cmd+Z` to undo if needed

## Features

- **Global hotkeys** — trigger actions from any app without switching windows
- **Custom actions** — define unlimited rewrite prompts with per-action hotkeys
- **Multiple providers** — OpenAI, Anthropic, or any OpenAI-compatible API
- **Menu bar only** — lives in your menu bar, no Dock icon, no distractions
- **Privacy mode** — disable history logging when working with sensitive text
- **Local history** — searchable SQLite log of all rewrites
- **Native feel** — translucent HUD window with macOS vibrancy
- **Secure** — API keys stored in macOS Keychain, never on disk

## Requirements

- macOS 13+ (Ventura or later)
- Accessibility permission (required to read/replace selected text)
- An API key from OpenAI, Anthropic, or a compatible provider

## Development

### Prerequisites

- [Rust](https://rustup.rs/) (1.88+)
- [Node.js](https://nodejs.org/) (18+)
- [pnpm](https://pnpm.io/)

### Setup

```bash
pnpm install
pnpm tauri dev
```

### Build

```bash
pnpm tauri build
```

The `.app` bundle will be in `src-tauri/target/release/bundle/macos/`.

## Stack

| Layer | Technology |
|-------|-----------|
| Framework | [Tauri v2](https://v2.tauri.app/) |
| Backend | Rust |
| Frontend | [Svelte 5](https://svelte.dev/) + [Tailwind CSS v4](https://tailwindcss.com/) |
| Storage | SQLite via `tauri-plugin-sql` |
| Keychain | `keyring` crate (apple-native) |
| Shortcuts | `tauri-plugin-global-shortcut` |
| Text capture | macOS Accessibility API |
| Input simulation | `rdev` |

## License

MIT
