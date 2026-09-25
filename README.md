# KENRI Desktop Companion

Tauri-based desktop body for a KENRI companion. Windows 11 is the first build target; shared web/device abstractions and a macOS icon keep the project macOS-ready.

## Working in v0.1

- Transparent, frameless, always-on-top Avatar Mode with the real Kelly French Bulldog asset
- Draggable avatar/window region, Quick Controls, Quick Chat, and Setup
- Tauri Windows/macOS icon set and Windows system tray with Show Kelly / Settings / Exit KENRI
- Camera enumeration, selection, explicit permission request, live preview, stop control, and visible `Camera Active` indicator
- Microphone enumeration, selection, explicit permission request, live input meter, stop control, and visible `Listening` indicator
- Speaker test through system speech output and visible `Speaking` indicator; output-device listing where the runtime exposes it
- Custom runtime avatar upload and Reset to Kelly
- Provider list: KENRI, OpenAI, Claude / Anthropic, Gemini, Qwen, DeepSeek
- API credential stays in React runtime memory only (no localStorage, file persistence, or logging)

Camera and microphone only activate from an explicit user action and always show an in-app state indicator in addition to any OS indicator.

## Explicit stubs / roadmap

- Provider `Test Connection` is a labelled stub until a safe native/backend provider contract exists
- OS secure credential storage (Windows Credential Manager / macOS Keychain)
- AI chat responses, provider Vision image upload, STT/realtime voice, and provider TTS
- Autostart, global shortcut, saved window position, and native output-device routing
- Code signing/notarization and production installer release work

No credential is hard-coded or persisted. Do not replace the runtime-only behavior with plaintext configuration or browser storage.

## Development

Prerequisites: Node.js, Rust, and the Tauri platform prerequisites.

```bash
npm install
npm run build
npm run tauri dev
```

Build installers with `npm run tauri build`. See `spec.md` for the complete product specification.
