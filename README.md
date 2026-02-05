<p align="center">
  <h1 align="center">✨ Omni Text</h1>
  <p align="center">
    <strong>AI-powered text rewriting from anywhere on your Mac.</strong>
    <br />
    Select. Hotkey. Done.
  </p>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/macOS-13%2B-000000?style=flat-square&logo=apple&logoColor=white" />
  <img src="https://img.shields.io/badge/Tauri-v2-24C8D8?style=flat-square&logo=tauri&logoColor=white" />
  <img src="https://img.shields.io/badge/Svelte-5-FF3E00?style=flat-square&logo=svelte&logoColor=white" />
  <img src="https://img.shields.io/badge/Rust-🦀-DEA584?style=flat-square" />
  <img src="https://img.shields.io/badge/license-MIT-blue?style=flat-square" />
</p>

---

Omni Text lives in your menu bar and waits for a hotkey. When triggered, it grabs your selected text, sends it to an LLM, and replaces it with the result — all without leaving the app you're working in. Proofread an email, rephrase a Slack message, translate a paragraph, or run any custom prompt you can dream up.

## ⚡ How It Works

```
1. 📝 Select text in any application
2. ⌨️  Press your hotkey (e.g. ⌘⌥⇧P)
3. 🤖 Text is sent to your LLM provider
4. ✅ Rewritten text replaces your selection
5. ↩️  ⌘Z to undo if you don't like it
```

No context switching. No copy-paste. No browser tabs. Just better text.

## 🎯 Features

| | Feature | Description |
|---|---------|-------------|
| ⌨️ | **Global Hotkeys** | Trigger actions from any app — no window switching |
| 🎨 | **Custom Actions** | Unlimited rewrite prompts, each with its own hotkey |
| 🔌 | **Any LLM Provider** | OpenAI, Anthropic, or any OpenAI-compatible API |
| 🖥️ | **Menu Bar Native** | Lives in your menu bar — no Dock icon, zero distraction |
| 🔒 | **Privacy Mode** | Disable history logging for sensitive content |
| 📜 | **Local History** | Searchable SQLite log of every rewrite |
| 💎 | **Native macOS Feel** | Translucent HUD window with vibrancy effects |
| 🔐 | **Keychain Storage** | API keys stored in macOS Keychain, never on disk |

## 📋 Requirements

- **macOS 13+** (Ventura or later)
- **Accessibility permission** — required to read and replace selected text
- **API key** from OpenAI, Anthropic, or a compatible provider

## 🛠️ Development

### Prerequisites

- [Rust](https://rustup.rs/) 1.88+
- [Node.js](https://nodejs.org/) 18+
- [pnpm](https://pnpm.io/)

### Get Started

```bash
# Install dependencies
pnpm install

# Run in development mode
pnpm tauri dev
```

### Build for Production

```bash
pnpm tauri build
```

Find your `.app` bundle in `src-tauri/target/release/bundle/macos/`.

## 🏗️ Tech Stack

```
┌─────────────────────────────────────────┐
│  Frontend    Svelte 5 + Tailwind CSS v4 │
├─────────────────────────────────────────┤
│  Backend     Rust                       │
├─────────────────────────────────────────┤
│  Framework   Tauri v2                   │
├─────────────────────────────────────────┤
│  Storage     SQLite (tauri-plugin-sql)  │
│  Secrets     macOS Keychain (keyring)   │
│  Shortcuts   Global Shortcut Plugin     │
│  Text I/O    Accessibility API + rdev   │
│  UI          HUD Window + Vibrancy      │
└─────────────────────────────────────────┘
```

## 📄 License

MIT — do whatever you want with it.
