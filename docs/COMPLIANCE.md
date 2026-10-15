# Anti-Cheat Compliance & OS API Transparency — XHAIR

This document provides a full audit and architectural proof of compliance with anti-cheat systems (Vanguard, Easy Anti-Cheat, BattlEye, VAC, Ricochet) (§2, §15.2).

---

## 1. Safety Rationale: What XHAIR Is and Is Not

### What XHAIR Is:
XHAIR is a standalone desktop application that renders a borderless, transparent, click-through 2D window on top of Windows Desktop Window Manager (DWM). It operates identically to hardware-based monitor crosshairs or legitimate overlays such as Discord, Steam, or NVIDIA GeForce Experience.

### What XHAIR Is NOT:
- **Not a game hook:** XHAIR never interacts with, injects into, or touches game memory.
- **Not a screen reader:** XHAIR never reads the screen buffer, capture card, or framebuffer.
- **Not a macro or bot:** XHAIR never sends synthetic mouse movement, aim adjustments, or recoil suppression inputs to Windows or any game.

---

## 2. Whitelisted Windows OS APIs

XHAIR uses exclusively standard, whitelisted Win32 APIs for windowing and hardware display query:

| Win32 API Function | Purpose & Bounds |
|---|---|
| `CreateWindowExW` | Creates top-level overlay with `WS_EX_LAYERED \| WS_EX_TRANSPARENT \| WS_EX_NOACTIVATE \| WS_EX_TOPMOST`. Never accepts mouse or keyboard focus. |
| `UpdateLayeredWindow` | Copies software-rasterized BGRA bitmap to DWM compositor. Zero GPU hooks. |
| `EnumDisplayMonitors` / `GetMonitorInfoW` | Queries physical desktop resolutions and coordinate bounds. |
| `GetDpiForMonitor` / `GetDpiForWindow` | Queries per-monitor DPI scaling to avoid blur. |
| `GetForegroundWindow` / `GetWindowThreadProcessId` | Identifies whether the active window matches a configured game profile to auto-hide overlay when alt-tabbing or minimized. Never opens process with read/write rights. |
| `RegisterHotKey` / `UnregisterHotKey` | Registers non-invasive global shortcut keys (e.g. `Ctrl+Shift+X`) to toggle overlay visibility. |
| `RegOpenKeyExW` / `RegSetValueExW` | Manages optional startup entry in `HKCU\Software\Microsoft\Windows\CurrentVersion\Run`. Never modifies `HKLM` or system keys. |
| `RegisterRawInputDevices` *(Opt-in Dynamics only)* | Passive, read-only mouse click counter for Spray Lab simulation. Does not install hooks and cannot block or manipulate inputs. |

---

## 3. Explicitly Prohibited APIs (Enforced by Invariants & Clippy)

The codebase strictly forbids the following APIs. Automated tests and static analysis reject any PR introducing them:

| Prohibited Category | Prohibited APIs | Rationale |
|---|---|---|
| **Process Inspection** | `OpenProcess` with `PROCESS_VM_READ` or `PROCESS_VM_WRITE`, `ReadProcessMemory`, `WriteProcessMemory` | Violates Invariant P1. Accessing game memory triggers instant anti-cheat detection. |
| **DLL Injection** | `CreateRemoteThread`, `VirtualAllocEx`, `LoadLibrary` into foreign process | Violates Invariant P1. Never executes code in another process address space. |
| **OS Hooks** | `SetWindowsHookExW` (`WH_KEYBOARD_LL`, `WH_MOUSE_LL`) | Global hooks intercept and delay user inputs system-wide; anti-cheats flag LL hooks. |
| **Screen Capture** | `BitBlt` against desktop DC, `GetPixel`, DXGI Desktop Duplication API | XHAIR never captures game visuals or scans for enemy health bars or colors. |
| **Input Synthesis** | `SendInput`, `mouse_event`, `keybd_event` | XHAIR never modifies player aim or moves the cursor. |
| **Network Exfiltration**| Sockets to external IPs, cloud telemetry | Zero network calls except local loopback (`127.0.0.1`) for local MCP agent automation. |

---

## 4. Compliance Audit Verification

Users and administrators can verify XHAIR's compliance at any time:
1. **Interactive Compliance Panel:** Accessible in the GUI editor under Settings $\to$ Anti-Cheat Compliance. Shows active APIs and static verification badges.
2. **Binary Import Table Audit:** Run `dumpbin /IMPORTS xhair.exe` or `llvm-readobj --coff-imports xhair.exe` to verify that `ReadProcessMemory`, `SetWindowsHookEx`, and `SendInput` are entirely absent from the import table.
3. **Open Source Build:** Build from source with `cargo build --release` and inspect reproducible hashes against official releases.
