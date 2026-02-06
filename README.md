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

Ever typed a message, stared at it, and thought *"that sounds weird"*? Omni Text fixes that. It lives quietly in your menu bar, waiting for a hotkey. When you press it, your selected text gets sent to an LLM and replaced with the result — right where you typed it. No copy-pasting. No switching apps. No opening ChatGPT in a browser tab.

## ⚡ How It Works

```
1. 📝 Select text in any application
2. ⌨️  Press your hotkey
3. 🤖 Text is sent to your LLM provider
4. ✅ Rewritten text replaces your selection
5. ↩️  Cmd+Z to undo if you don't like it
```

That's it. The whole thing takes about a second.

## 💡 Real Use Cases

🔤 **Fixing embarrassing emails before you hit send** — Select your draft, press `Cmd+Shift+1`, and Omni Text fixes your spelling, grammar, and punctuation. Your coworkers will think you suddenly became meticulous.

💬 **Making Slack messages sound less unhinged** — You typed "pls fix the thing its broken again thx" at 2am. Select it, press `Cmd+Shift+2`, and it becomes "Could you take a look at the issue? It seems to have resurfaced. Thanks!" Crisis averted.

🌍 **Translating on the fly** — Create a custom action with a system prompt like "Translate the following text to Japanese. Output only the translation." Now you have a one-hotkey translator that works in any app.

📖 **Simplifying technical jargon** — Writing docs for non-technical stakeholders? Create an action that rewrites text for a general audience. Select the paragraph full of acronyms, hotkey, done.

📝 **Writing commit messages** — Yes, really. Select your messy notes in a text editor, run a custom "Write a concise git commit message from these notes" action, paste it into your terminal.

### 🎯 Default Actions

| Hotkey | Action | Description |
|--------|--------|-------------|
| `Cmd+Shift+1` | **Proofread** | Fix spelling, grammar, and punctuation |
| `Cmd+Shift+2` | **Rewrite** | Rewrite for clarity and readability |

These are just the defaults — you can create as many custom actions as you want, each with its own hotkey and system prompt.

## 🚀 Features

| | Feature | Description |
|---|---------|-------------|
| ⌨️ | **Global Hotkeys** | Trigger actions from any app — no window switching needed |
| 🎨 | **Custom Actions** | Create unlimited rewrite prompts, each with its own hotkey |
| 🔌 | **Multiple Providers** | Configure several LLM providers and switch between them with one click |
| 🌐 | **Any OpenAI-Compatible API** | Works with OpenAI, Ollama, LM Studio, Groq, Together AI, Azure OpenAI, or any server that speaks the OpenAI standard |
| 🧠 | **Smart URL Handling** | Just paste your base URL — Omni Text figures out the right endpoint automatically |
| 🖥️ | **Menu Bar Native** | Lives in your menu bar with zero Dock presence — there when you need it, invisible when you don't |
| 🐱 | **Animated Tray Icon** | A little pixelated cat blinks while processing and looks alarmed when something goes wrong |
| 🔒 | **Privacy Mode** | Toggle off history logging when working with sensitive content |
| 📜 | **Local History** | Searchable SQLite log of every rewrite — see what changed and when |
| 🔐 | **Keychain Storage** | API keys stored in macOS Keychain, never written to disk |
| 💾 | **Autosave** | All settings persist automatically as you type — no save buttons anywhere |
| 💎 | **Always-On Vibrancy** | Translucent glass UI stays beautiful whether the window is focused or not |

## 🔌 Supported Providers

Omni Text works with **any server that implements the OpenAI chat completions API standard** (`/v1/chat/completions`). This includes:

| Provider | Notes |
|----------|-------|
| 🟢 **OpenAI** | GPT-4o, GPT-4o mini, and all chat models |
| ⚡ **Groq** | Llama, Mixtral with ultra-fast inference |
| 🤝 **Together AI** | Open-source models hosted in the cloud |
| 🦙 **Ollama** | Run models locally on your Mac |
| 🖥️ **LM Studio** | Local model server with a nice UI |
| ☁️ **Azure OpenAI** | Enterprise OpenAI deployments |
| 🔧 **Any compatible server** | If it speaks the OpenAI API format, it works |

Configure multiple providers in the **Providers** tab and switch between them with a single click. Use GPT-4o for important emails and a local Llama model for quick proofreading — your call.

## 📦 Installation

Download the latest `.dmg` from the [Releases](https://github.com/addison-w/Omni-Text/releases) page.

1. 💿 Open the `.dmg` and drag **Omni Text** to Applications
2. 🐱 Launch it — a cat icon appears in your menu bar
3. ♿ Grant **Accessibility** permission when prompted
4. 🔑 Go to the **Providers** tab and add your API key
5. ✨ Select some text anywhere and press `Cmd+Shift+1` — welcome to the future

### 📋 Requirements

- **macOS 13+** (Ventura or later)
- **Accessibility permission** — macOS requires this for apps that read/replace text
- **An API key** from any OpenAI-compatible provider

## 🛠️ Development

### Prerequisites

- [Rust](https://rustup.rs/) 1.88+
- [Node.js](https://nodejs.org/) 18+
- [pnpm](https://pnpm.io/)

### Get Started

```bash
pnpm install
pnpm tauri dev
```

### Build for Production

```bash
pnpm tauri build
```

The `.app` and `.dmg` land in `src-tauri/target/release/bundle/`.

## 🏗️ Tech Stack

```
Frontend    Svelte 5 + Tailwind CSS v4
Backend     Rust
Framework   Tauri v2
Storage     SQLite (tauri-plugin-sql)
Secrets     macOS Keychain (keyring)
Shortcuts   Global Shortcut Plugin
Text I/O    Accessibility API + rdev
UI          HUD Window + Vibrancy
```

## 📄 License

MIT — do whatever you want with it.
