# KENRI Desktop Companion

Cross-platform desktop companion for KENRI. Windows 11 is the first target; macOS is part of the architecture from day one.

## v0.1 scaffold

Implemented UI scaffold:
- Kelly default French Bulldog avatar
- Transparent frameless Tauri window
- Avatar / Quick Chat / Setup surfaces
- Companion name and avatar setup
- Personality field
- Cloud provider selector: KENRI, OpenAI, Claude, Gemini, Qwen, DeepSeek
- API key field + Test Connection UI
- Camera / Microphone / Speaker test controls
- Always-on-top / remember-position / start-with-OS settings UI

The provider test and device buttons are currently UI scaffolding; real provider calls, OS secure credential storage, camera/mic streams, STT/TTS, tray, autostart and global shortcut are the next implementation step.

## Development

Prerequisites: Node.js, Rust, and Tauri platform prerequisites.

```bash
npm install
npm run tauri dev
```

Build installer:

```bash
npm run tauri build
```

See `spec.md` for the product specification.
