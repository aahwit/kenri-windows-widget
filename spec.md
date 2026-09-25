# KENRI Desktop Companion — Specification

**Repository:** `kenri-windows-widget`  
**Product name:** KENRI Desktop Companion  
**Specification version:** v0.1  
**Initial target:** Windows 11  
**Architecture target:** Windows + macOS from one codebase

## 1. Vision

KENRI Desktop Companion is a persistent AI companion for the desktop. It is not simply the KENRI website placed inside a small window. It provides a desktop body for a KENRI identity.

Core principle:

> **One Identity — Multiple Bodies**

The same identity may eventually appear as a Windows companion, macOS companion, web widget, mobile client, ESP32 device, or physical robot.

Conceptual model:

```text
Identity + Avatar + Personality + Voice + Senses + AI Provider + Body = Companion
```

KENRI remains the brain/platform while each device provides a body and available senses.

## 2. Cross-platform strategy

The application should be designed as **KENRI Desktop Companion**, even though Windows 11 is the first implementation target.

Preferred desktop framework: **Tauri**, with shared UI/business logic and platform-specific adapters where required.

```text
KENRI Desktop Companion
├── Shared Companion Core
│   ├── Identity
│   ├── Avatar
│   ├── Personality
│   ├── Chat
│   ├── Provider abstraction
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

Platform-specific implementation must not leak unnecessarily into Companion identity, personality, chat, or provider logic.

## 3. Default Companion — Kelly

The first installation starts with a default companion named **Kelly**.

Default profile:

- Name: Kelly
- Avatar: French Bulldog
- Presentation: male
- Personality: Kelly default personality
- Voice: male default voice
- Avatar background: transparent

Kelly is the default/example companion. The user can customize the companion and must always have an easy **Reset to Kelly** option.

## 4. Desktop experience

The primary experience is an avatar floating directly on the desktop rather than a conventional application window.

```text
                    [Avatar]
                     Kelly

                +-------------+
                | Chat Mic ...|
                +-------------+
```

The companion should support:

- Transparent, frameless presentation
- Always-on-top option
- Dragging to another screen position
- Remembering its last position
- Multi-monitor operation
- Quick controls
- Quick Chat
- Voice entry point
- Settings
- Hide/show behavior
- Background lifecycle through System Tray on Windows or Menu Bar on macOS

Transparent regions should not unnecessarily block interaction with applications underneath. Visible avatar/control regions must remain interactive and draggable.

DPI/scaling behavior must be tested on Windows at common scaling levels including 100%, 125%, and 150%.

## 5. Presentation modes

### 5.1 Avatar Mode

Default mode. Shows the companion avatar with minimal or auto-hidden controls.

### 5.2 Quick Controls

Initial controls:

- Chat
- Voice
- Menu / Settings

### 5.3 Quick Chat

A compact conversation panel opened from the avatar.

Example:

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

Closing Quick Chat should return to Avatar Mode rather than terminate the application.

### 5.4 Setup Menu

The fourth presentation surface is the Companion **Setup / Settings menu**, not a Full KENRI window.

The Desktop Companion should remain a focused standalone companion. It must not require a presentation mode whose purpose is simply to open or reproduce the full KENRI web application.

Setup is opened from the `...` quick control, System Tray on Windows, or Menu Bar on macOS.

Initial Setup sections:

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
│   ├── Model / Server URL where applicable
│   └── Test Connection
├── Devices
│   ├── Camera
│   ├── Microphone
│   └── Speaker
└── Desktop
    ├── Start with OS
    ├── Always on top
    ├── Global shortcut
    └── Remember position
```

## 6. Companion Setup

Setup must allow users to customize their companion without editing source code.

### Identity

- Companion name
- Avatar image
- Upload custom image
- Reset to Kelly

### Personality

- Personality description
- Speaking style
- How the companion addresses the user
- Custom instructions

Personality and Avatar are independent. Changing the avatar must not erase personality configuration.

### Voice

The architecture should support:

- Voice selection
- Voice enable/disable
- Speaking speed where supported
- Audio output selection where supported

## 7. Avatar system

### v0.1

- Default Kelly French Bulldog avatar
- Custom image upload
- Transparent-background avatar support
- Static avatar is acceptable for MVP

### Future animated avatar

The avatar architecture should allow state-based animation without redesigning the Companion core.

Example package:

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

Candidate states:

- Idle
- Listening
- Thinking
- Searching
- Working
- Needs Input
- Speaking
- Success
- Error

## 8. AI Provider configuration

Provider setup should remain simple in v0.1.

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

Initial provider abstraction should be capable of supporting:

- KENRI
- OpenAI
- OpenRouter
- Gemini
- Ollama / local provider

The provider list must be extensible.

For providers that do not use an API key, the credential form should adapt appropriately. For example, Ollama may use a Server URL and model selection.

## 9. Test Connection

A **Test Connection** button is required.

The test must:

1. Use the currently selected provider.
2. Use the credential currently entered in the form.
3. Perform a real provider connectivity/authentication check.
4. Not require Save before testing.
5. Return a human-readable result.

Examples:

```text
Connection successful
Invalid API key
Server unreachable
Provider unavailable
Model unavailable
```

## 10. Credential security

API keys and secrets must never be stored in plaintext in source code, logs, localStorage, or ordinary configuration files.

Use OS-backed secure credential storage:

- Windows: secure Windows credential storage
- macOS: Keychain

Settings may store non-secret metadata such as selected provider and model.

When reopening Settings, a saved secret must never be displayed in full.

## 11. Local AI

The provider abstraction must support local AI.

Example:

```text
Provider: Ollama
Server URL: http://localhost:11434
Model: [ Select Model ]

[Test Connection]
```

Local AI support should not require a cloud API key.

## 12. Senses and device capabilities

The architecture must include a capability/perception layer from the beginning, even when a capability is not fully used in v0.1.

### Eyes

Camera / Vision provider

### Ears

Microphone / audio input

### Voice

Speaker / audio output

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

## 13. Permissions and onboarding

During onboarding, explain that the Companion can use Camera, Microphone, and Speaker capabilities.

Actual access must follow operating-system permission/security mechanisms. The application must not bypass OS permissions.

Where the OS requires permission at first use, request it at the appropriate point and explain why it is needed.

Example device status:

```text
Devices
-----------------------
Camera       Allowed
Microphone   Allowed
Speaker      Available
```

Users must be able to disable available capabilities from Settings.

Note: speaker/audio output normally does not require the same privacy permission prompt as camera or microphone; it should still be detected/configurable as a capability.

## 14. Privacy indicators

Sensor use must be visible to the user.

Examples:

```text
Listening
Camera Active
Speaking
```

Camera and microphone must not silently activate without user-visible state/OS indication.

Future always-listening or wake-word functionality must have an explicit opt-in setting.

## 15. Windows integration

Initial Windows requirements:

- Windows 11
- Transparent window
- Frameless avatar window
- Always-on-top option
- Drag position
- Remember position
- Multi-monitor support
- System Tray
- Start with Windows
- Minimize/hide to Tray
- Configurable global shortcut
- Secure credential storage
- Installer packaging

Proposed default shortcut:

```text
Ctrl + Alt + K
```

The shortcut must be user-configurable to avoid conflicts.

### Close behavior

Closing the visible companion should normally hide it while keeping the application available from System Tray.

Explicit application termination should be available from the Tray menu:

```text
KENRI
├── Show Kelly
├── Settings
└── Exit KENRI
```

## 16. macOS integration

macOS must be considered in architecture from v0.1 even if Windows ships first.

macOS requirements/planned equivalents:

- Transparent frameless companion window
- Floating/always-on-top behavior using supported macOS window levels
- Drag position
- Remember position
- Multiple-display support
- Menu Bar background control
- Launch at Login
- macOS Keychain for API keys/secrets
- Native Camera permission handling
- Native Microphone permission handling
- Audio output support
- Configurable global shortcut
- APP/DMG packaging
- Code signing and notarization for production distribution

Camera and Microphone permission descriptions must be declared correctly for the macOS application bundle before distribution.

The user experience and Companion identity should remain consistent between Windows and macOS even when OS integration differs.

## 17. Platform capability abstraction

Platform-dependent functionality should sit behind interfaces/adapters.

Conceptually:

```text
Companion Core
      |
      +-- CredentialStore
      |     +-- WindowsCredentialStore
      |     +-- MacKeychainStore
      |
      +-- AutoStart
      |     +-- WindowsAutoStart
      |     +-- MacLaunchAtLogin
      |
      +-- BackgroundMenu
      |     +-- WindowsTray
      |     +-- MacMenuBar
      |
      +-- CameraProvider
      +-- MicrophoneProvider
      +-- AudioOutputProvider
      +-- GlobalShortcutProvider
      +-- WindowPositionStore
```

This separation is a design requirement, not an optional cleanup task.

## 18. KENRI architecture relationship

The Desktop Companion should not duplicate the full KENRI backend.

Conceptually:

```text
Desktop Companion
       |
       +-- Identity / Avatar / local UI
       +-- Device capabilities
       +-- Secure provider credentials
       |
       v
KENRI / Selected AI Provider
       |
       +-- LLM
       +-- RAG / Knowledge
       +-- Personality
       +-- Agent
       +-- Skill / Tools
```

Some providers may provide direct LLM chat only. KENRI-specific services such as Knowledge, RAG, Agents, Skills, Bot configuration, or account data may still require a KENRI service connection. Provider selection and KENRI service connection must therefore remain conceptually separable.

## 19. Future physical embodiment

The architecture must not assume the desktop is the final body.

Future bodies may include:

- ESP32 companion hardware
- Camera-equipped companion
- Speaker/microphone device
- Mobile device
- Physical robot

The long-term objective is that Kelly on Windows/macOS and Kelly in physical hardware can represent the same identity rather than separate AI personalities.

```text
                  Kelly Identity
                       |
             +---------+---------+
             |         |         |
          Windows    macOS     Physical
             |         |         |
           Screen    Screen    Camera/Mic
             +---------+---------+
                       |
                   KENRI Brain
```

## 20. MVP scope — v0.1

Required for first usable release:

- Windows 11 application
- Tauri-based cross-platform-ready architecture
- Kelly default French Bulldog avatar
- Transparent floating Avatar Mode
- Drag and remember position
- Quick Controls
- Text Quick Chat
- Setup / Settings menu
- Custom name
- Custom avatar image
- Personality configuration
- Provider selection
- API key/credential entry
- Test Connection
- Secure credential storage
- System Tray
- Start with Windows
- Configurable global shortcut
- Camera/Microphone/Speaker capability architecture
- Device permission/status UI
- macOS-ready platform abstractions

macOS does not have to ship simultaneously with the first Windows MVP, but no core architectural decision should unnecessarily prevent it.

## 21. Post-MVP roadmap

### v0.2

- Voice conversation
- Microphone input
- Text-to-speech
- Animated avatar states
- Drag/drop files and images
- Streaming responses
- Improved local AI support

### v0.3

- Camera/Vision
- Screen perception
- Selected-text actions
- Clipboard integration
- Context menu integration
- Wake word
- Agent notifications

### Future

- User-generated animated companion from uploaded image
- Multiple companions
- Companion marketplace/profile packages
- ESP32 body
- Physical Kelly robot
- Shared identity/state across multiple bodies

## 22. Product principle

KENRI Desktop Companion should feel like a persistent character with an identity, personality, senses, and abilities — not merely a chat window.

The desktop implementation is the first body, not the final destination.
