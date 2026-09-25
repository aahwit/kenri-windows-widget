# KENRI Desktop Companion — Specification

**Repository:** `kenri-windows-widget`  
**Product name:** KENRI Desktop Companion  
**Specification version:** v0.1  
**Initial target:** Windows 11  
**Architecture target:** Windows + macOS from one codebase

## 1. Vision

KENRI Desktop Companion is a persistent AI companion for the desktop. It is not simply the KENRI website placed inside a small window. It provides a desktop body for a KENRI identity.

> **One Identity — Multiple Bodies**

```text
Identity + Avatar + Personality + Voice + Senses + AI Provider + Body = Companion
```

KENRI remains the brain/platform while each device provides a body and available senses.

## 2. Cross-platform strategy

The product is **KENRI Desktop Companion**. Windows 11 ships first, while the architecture must support Windows and macOS from one codebase.

Preferred framework: **Tauri**.

```text
KENRI Desktop Companion
├── Shared Companion Core
│   ├── Identity
│   ├── Avatar
│   ├── Personality
│   ├── Chat / Voice
│   ├── Cloud Provider abstraction
│   ├── Device capability abstraction
│   └── Settings
├── Windows
│   ├── System Tray
│   ├── Start with Windows
│   ├── Windows secure credential storage
│   └── EXE / MSI installer
└── macOS
    ├── Menu Bar
    ├── Launch at Login
    ├── macOS Keychain
    └── APP / DMG distribution
```

## 3. Default Companion — Kelly

The first installation starts with **Kelly**.

- Name: Kelly
- Avatar: French Bulldog
- Presentation: male
- Personality: Kelly default personality
- Voice: male default voice
- Avatar background: transparent

Users can customize the Companion and must always have an easy **Reset to Kelly** option.

## 4. Desktop experience

The primary experience is an avatar floating directly on the desktop rather than a conventional application window.

```text
                    [Avatar]
                     Kelly

                +-------------+
                | Chat Mic ...|
                +-------------+
```

Requirements:

- Transparent, frameless presentation
- Always-on-top option
- Drag position and remember last position
- Multi-monitor operation
- Quick controls and Quick Chat
- Voice conversation
- Camera/Vision session
- Settings and hide/show behavior
- System Tray on Windows / Menu Bar on macOS
- Transparent areas should not unnecessarily block applications underneath
- Visible avatar/control regions remain interactive and draggable
- Windows DPI testing at 100%, 125%, and 150%

## 5. Presentation modes

### 5.1 Avatar Mode

Default mode. Shows the Companion avatar with minimal or auto-hidden controls.

### 5.2 Quick Controls

- Chat
- Voice / Microphone
- Camera / Vision
- Menu / Settings

### 5.3 Quick Chat

Compact conversation panel opened from the avatar. It supports text and can show voice/Vision status when those capabilities are active.

```text
+----------------------------+
| Kelly                  X   |
| Ready                      |
+----------------------------+
|                            |
| Hello. How can I help?     |
|                            |
+----------------------------+
| Type a message...      Send|
+----------------------------+
```

Closing Quick Chat returns to Avatar Mode rather than terminating the application.

### 5.4 Setup Menu

Setup is opened from the `...` quick control, Windows System Tray, or macOS Menu Bar.

```text
Setup
├── Companion
│   ├── Name
│   ├── Avatar
│   └── Reset to Kelly
├── Personality
│   ├── Personality description
│   ├── Speaking style
│   ├── How to address user
│   └── Custom instructions
├── Voice
│   ├── Voice selection
│   ├── Enable / Disable
│   └── Speaking speed
├── AI Provider
│   ├── Provider
│   ├── API Key / Credential
│   └── Test Connection
├── Devices
│   ├── Camera + Test Camera
│   ├── Microphone + Test Microphone
│   └── Speaker + Test Speaker
└── Desktop
    ├── Start with OS
    ├── Always on top
    ├── Global shortcut
    └── Remember position
```

The Desktop Companion is a focused standalone companion and does not need a Full KENRI presentation mode.

## 6. Companion Setup

### Identity

- Companion name
- Avatar image
- Upload custom image
- Reset to Kelly

### Personality

- Personality description
- Speaking style
- How the Companion addresses the user
- Custom instructions

Personality and Avatar are independent. Changing the avatar must not erase personality configuration.

### Voice

- Voice selection
- Voice enable/disable
- Speaking speed where supported
- Audio output selection where supported

## 7. Avatar system

### v0.1

- Default Kelly French Bulldog avatar
- Custom image upload
- Transparent-background avatar support
- Static avatar acceptable for MVP

### Future animated avatar

```text
avatar/
├── manifest.json
├── idle.webp
├── listening.webp
├── thinking.webp
├── searching.webp
├── working.webp
├── speaking.webp
├── success.webp
└── error.webp
```

Candidate states: Idle, Listening, Thinking, Searching, Working, Needs Input, Speaking, Success, Error.

## 8. AI Provider configuration

Customers use supported **cloud AI providers only**. Local AI / Ollama is intentionally not a customer-facing option.

```text
AI Provider
--------------------------------
Provider
[ OpenAI                    v ]

API Key
[ **********************  eye ]

             [ Test Connection ]

Status: Connected

                         [ Save ]
```

Initial provider abstraction should support:

- KENRI
- OpenAI
- Claude (Anthropic)
- Gemini
- Qwen (official supported cloud API)
- DeepSeek (official supported cloud API)

OpenRouter is not an initial customer-facing provider. The cloud provider list must remain extensible. Provider adapters keep endpoint, authentication, model naming, and request-format differences behind the provider abstraction.

## 9. Test Connection

A **Test Connection** button is required. It must use the selected provider and currently entered credential, perform a real connectivity/authentication check without requiring Save first, and return a human-readable result.

Examples: `Connection successful`, `Invalid API key`, `Server unreachable`, `Provider unavailable`, `Model unavailable`.

## 10. Credential security

API keys and secrets must never be stored in plaintext in source code, logs, localStorage, or ordinary configuration files.

Use OS-backed secure credential storage:

- Windows: secure Windows credential storage
- macOS: Keychain

Saved secrets must never be displayed in full when Settings is reopened.

## 11. Senses and device capabilities — v0.1

Camera, Microphone, and Speaker are **working v0.1 features**, not architecture-only placeholders.

### Eyes — Camera / Vision

v0.1 must support:

- Detect available cameras
- Select camera device
- Enable/disable camera
- Camera preview
- Test Camera action
- Explicit Vision session controlled by the user
- Capture/provide an image to a supported Vision-capable AI path
- Visible `Camera Active` state while the camera is active

v0.1 does **not** continuously watch the environment by default. Continuous Vision is a future explicit opt-in capability.

### Ears — Microphone / Speech Input

v0.1 must support:

- Detect available microphones
- Select microphone device
- Enable/disable microphone
- Test Microphone action with input-level feedback
- Capture speech for voice conversation
- Speech-to-text or provider-supported realtime voice input
- Visible `Listening` state while audio capture is active

### Voice — Speaker / Audio Output

v0.1 must support:

- Detect/select available audio output where the platform allows it
- Test Speaker action
- Text-to-speech or provider-supported realtime voice output
- Voice selection where supported
- Visible `Speaking` state while Kelly is producing audio

### Future perception sources

- Screen capture
- Selected screen region
- Clipboard
- Selected text
- Files
- External camera
- ESP32 sensors
- Other physical sensors

The KENRI brain should consume capabilities through abstractions so a future physical Kelly can use different hardware without changing identity/personality logic.

## 12. Permissions and onboarding

During first-run onboarding, explain that the Companion uses **Camera, Microphone, and Speaker** for Vision and voice conversation.

Actual access must follow operating-system permission/security mechanisms and must not bypass OS permissions. Camera and microphone permission should be requested through the OS when appropriate. Speaker availability is detected/configured even though it normally does not use the same privacy permission flow.

```text
Devices
-----------------------
Camera       Allowed
Microphone   Allowed
Speaker      Available
```

Users can disable Camera, Microphone, or Voice output from Settings.

## 13. Privacy indicators

Sensor use must always be visible to the user.

```text
Listening
Camera Active
Speaking
```

Camera and microphone must not silently activate without user-visible state/OS indication.

v0.1 camera use is session/user initiated. Always-listening, wake-word, and continuous Vision functionality require explicit opt-in and are outside the default v0.1 behavior.

## 14. Windows integration

- Windows 11
- Transparent frameless avatar window
- Always-on-top option
- Drag and remember position
- Multi-monitor support
- System Tray
- Start with Windows
- Minimize/hide to Tray
- Configurable global shortcut
- Secure credential storage
- Native Camera/Microphone access and permissions
- Speaker/audio output
- Installer packaging

Proposed default shortcut: `Ctrl + Alt + K`, user-configurable to avoid conflicts.

Closing the visible Companion normally hides it while the application remains available from System Tray. Explicit termination is available from the Tray menu.

```text
KENRI
├── Show Kelly
├── Settings
└── Exit KENRI
```

## 15. macOS integration

macOS must be considered from v0.1 even if Windows ships first.

- Transparent frameless companion window
- Floating/always-on-top behavior
- Drag and remember position
- Multiple-display support
- Menu Bar background control
- Launch at Login
- macOS Keychain
- Native Camera permission handling
- Native Microphone permission handling
- Camera preview/Vision support
- Microphone voice input
- Speaker/audio output
- Configurable global shortcut
- APP/DMG packaging
- Code signing and notarization for production distribution

Camera and Microphone permission descriptions must be declared correctly for the macOS application bundle.

## 16. Platform capability abstraction

```text
Companion Core
      |
      +-- CredentialStore
      |     +-- WindowsCredentialStore
      |     +-- MacKeychainStore
      +-- AutoStart
      +-- BackgroundMenu
      +-- CameraProvider
      +-- MicrophoneProvider
      +-- SpeechInputProvider
      +-- AudioOutputProvider
      +-- TextToSpeechProvider
      +-- GlobalShortcutProvider
      +-- WindowPositionStore
```

This separation is a design requirement.

## 17. KENRI architecture relationship

The Desktop Companion should not duplicate the full KENRI backend and must not expose a customer-facing local AI execution path.

```text
Camera ─────┐
Microphone ─┼──> Desktop Companion ──> KENRI / Selected Cloud AI Provider
Text ───────┘                              |
                                             +-- LLM / Vision / Voice
                                             +-- RAG / Knowledge
                                             +-- Personality
                                             +-- Agent
                                             +-- Skill / Tools
                                             |
Speaker <──────────── Voice response <───────+
```

Direct cloud-provider mode may provide only the capabilities supported by that provider. KENRI-specific services such as Knowledge, RAG, Agents, Skills, Bot configuration, or account data may still require a KENRI service connection.

## 18. Future physical embodiment

The desktop is not assumed to be the final body. Future bodies may include ESP32 companion hardware, camera-equipped companion devices, mobile devices, and a physical robot.

The long-term objective is that Kelly on Windows/macOS and Kelly in physical hardware can represent the same identity rather than separate AI personalities.

```text
                  Kelly Identity
                       |
             +---------+---------+
             |         |         |
          Windows    macOS     Physical
             |         |         |
         Cam/Mic/    Cam/Mic/   Camera/Mic/
         Speaker     Speaker    Sensors
             +---------+---------+
                       |
                   KENRI Brain
```

## 19. MVP scope — v0.1

- Windows 11 application
- Tauri-based cross-platform-ready architecture
- Kelly default French Bulldog avatar
- Transparent floating Avatar Mode
- Drag and remember position
- Quick Controls
- Text Quick Chat
- **Voice conversation**
- **Microphone input**
- **Speech-to-text or realtime voice input**
- **Text-to-speech / voice response through speaker**
- **Camera selection and preview**
- **User-initiated Camera/Vision session**
- **Test Camera / Test Microphone / Test Speaker**
- **Visible Camera Active / Listening / Speaking indicators**
- Setup / Settings menu
- Custom name
- Custom avatar image
- Personality configuration
- Cloud provider selection
- API key/credential entry
- Test Connection
- Secure credential storage
- System Tray
- Start with Windows
- Configurable global shortcut
- Device permission/status UI
- macOS-ready platform abstractions

macOS does not have to ship simultaneously with the first Windows MVP, but no core architectural decision should unnecessarily prevent it.

## 20. Post-MVP roadmap

### v0.2

- Animated avatar states
- Drag/drop files and images
- Improved streaming/realtime interaction
- Additional voice/provider optimizations
- Screen perception
- Selected-text actions
- Clipboard integration

### v0.3

- Wake word / explicit always-listening mode
- Continuous Vision as explicit opt-in
- Context menu integration
- Agent notifications
- Deeper desktop awareness

### Future

- User-generated animated companion from uploaded image
- Multiple companions
- Companion marketplace/profile packages
- ESP32 body
- Physical Kelly robot
- Shared identity/state across multiple bodies

## 21. Product principle

KENRI Desktop Companion should feel like a persistent character with an identity, personality, **eyes, ears, voice**, and abilities — not merely a chat window.

The desktop implementation is the first body, not the final destination.
