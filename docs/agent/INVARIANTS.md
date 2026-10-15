# INVARIANTS — Non-Negotiable Rules

These invariants must never be broken by any agent or commit under any circumstances.

## Core Principles (P1–P7)
| ID | Principle | Description |
|---|---|---|
| **P1** | **Overlay only** | Never inject into, attach to, read memory of, hook, or modify the game process. Never capture the game's screen. |
| **P2** | **Low RAM & CPU** | Idle overlay target $\le$ 20 MB private working set. Editor open $\le$ 150 MB. Zero CPU render loop when static. |
| **P3** | **One source of truth** | One core library owns all state and logic. UI, MCP, CLI, and hotkeys are thin clients of the same command bus. |
| **P4** | **Human/agent parity** | Anything a human can do in the UI exists as a command and therefore as an MCP tool. Enforced by CI test. |
| **P5** | **Schema-driven** | Properties are declared once (label, tooltip, type, range, unit, default). UI controls, tooltips, validation, and MCP schemas generate from descriptors. |
| **P6** | **Pixel accuracy over prettiness** | The center must be provably correct and user-verifiable via calibration across all resolutions and DPIs. |
| **P7** | **Clean, documented code** | Strictly typed, `#![forbid(unsafe_code)]` except safe wrappers in `xhair-platform-win`, no unwrap in production paths, full docs. |

## OS Compliance & Risk Guardrails (Section 2)
1. **Forbidden APIs**:
   - `OpenProcess`, `ReadProcessMemory`, `WriteProcessMemory`, `CreateRemoteThread`
   - `SetWindowsHookEx` targeting the game, `WH_KEYBOARD_LL`, `WH_MOUSE_LL`
   - `GetAsyncKeyState` polling of arbitrary keys
   - Screen/desktop duplication, `BitBlt` / `PrintWindow` on the game window
   - Keystroke logging or telemetry
2. **Allowed OS APIs**:
   - Layered, click-through, topmost window (`UpdateLayeredWindow`, `WS_EX_LAYERED | WS_EX_TRANSPARENT | WS_EX_TOPMOST | WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE`)
   - Display & Monitor: `EnumDisplayMonitors`, `GetMonitorInfo`, `GetDpiForMonitor`, `EnumDisplaySettings`
   - Foreground check: `GetForegroundWindow`, `GetWindowRect`, `DwmGetWindowAttribute` (rect only), process image name via limited-query rights
   - Foreground events: `SetWinEventHook(EVENT_SYSTEM_FOREGROUND, ...)` out-of-context
   - Hotkeys: `RegisterHotKey`
   - Raw Input: `RegisterRawInputDevices` ONLY if Dynamic Reticle is explicitly opted into by the user for user-bound keys only
   - Local filesystem in `%APPDATA%/xhair/` or user-selected paths
3. **Network**:
   - Zero outbound network calls.
   - Loopback only (`127.0.0.1`) for local MCP HTTP server with bearer token and origin validation.
4. **Dynamic Reticle Safety**:
   - Off by default.
   - Requires explicit opt-in confirmation dialog every session.
   - Ships with Spray Lab (safe standalone window).
   - Never reads game state or claims real game recoil numbers.
