# Architecture — XHAIR

High-performance Windows crosshair overlay, reactive editor, and Model Context Protocol (MCP) server.

---

## 1. System Overview

XHAIR is built around a single foundational invariant: **the overlay is completely decoupled from any game process**. It never opens process handles with memory-read privileges, never injects DLLs, and never reads the screen buffer (§2).

```
 ┌──────────────────────┐         ┌──────────────────────┐
 │   Svelte 5 Web UI    │         │ AI Agent (via MCP)   │
 │ (Editor / Spray Lab) │         │ (Claude, Cursor, AGY)│
 └──────────┬───────────┘         └──────────┬───────────┘
            │                                │
            │ HTTP / IPC / JSON-RPC          │ stdio / Loopback HTTP (Bearer)
            ▼                                ▼
   ┌─────────────────────────────────────────────────────┐
   │                  Command Bus Core                   │
   │  (One Source of Truth, Undo/Redo, Diff Patches)     │
   └──────────┬──────────────────────────────┬───────────┘
              │                              │
              ▼                              ▼
    ┌──────────────────┐           ┌──────────────────┐
    │  tiny-skia CPU   │           │ Windows Platform │
    │    Rasterizer    │           │ (Layered Window, │
    │ (WASM & Native)  │           │  DPI, Hotkeys)   │
    └─────────┬────────┘           └──────────────────┘
              │                              ▲
              └───────── BitBlt / DWM ───────┘
```

---

## 2. Architectural Principles

1. **One Source of Truth (P3):** All mutations to crosshair documents, active slots, or overlay states flow through typed `Command` payloads handled by the `CommandBus`. UI, CLI, and MCP servers are purely thin clients.
2. **100% Human / Agent Parity (P4):** Every interactive action achievable in the editor UI maps to a corresponding command on the bus and an MCP tool. Continuous integration tests enforce 1:1 parity (`tests/parity.rs`).
3. **Zero Game Interference (P1):** Zero game memory reads, zero DLL injection, zero input blocking, and zero low-level keyboard/mouse hooks (`WH_KEYBOARD_LL`).
4. **Low Resource Footprint (P5):** Static overlay consumes **zero CPU** cycles; dirty-rect caching ensures re-renders only occur when state or dynamic bindings change. Idle private working set is budgeted under **20 MB**.
5. **Deterministic Rendering:** The rendering engine (`xhair-render`) produces bit-for-bit identical golden images across both native Rust (`tiny-skia`) and WebAssembly targets.

---

## 3. Crate Breakdown & Responsibilities

| Crate | Responsibility |
|---|---|
| `crates/xhair-model` | Domain data types (`Crosshair`, `Layer`, `LayerKind`, `Stroke`, `Paint`, `Effects`, `Symmetry`). Validation limits, Serde serialization, and JSON Schema generation. |
| `crates/xhair-geometry` | Mathematical foundations: Subpixel coordinate systems, center parity (even vs. odd resolutions), display aspect-ratio stretch (16:9, 4:3, 16:10, 5:4), DPI scaling (100%–500%), and offline screenshot calibration (`screenshot_align.rs`). |
| `crates/xhair-commands` | Command bus implementation, typed `Command` variants, linear undo/redo stacks, atomic command batches, patch diff calculation, and property descriptors. |
| `crates/xhair-render` | Tiny-skia CPU rendering pipeline, procedural animation, bloom / glow / drop-shadow effects, dirty-rect tracking, and WebAssembly bindings. |
| `crates/xhair-format` | Serialization formats: `.xhair` (single document JSON), `.xhairpack` (crosshair bundles), and compact base64 share codes (`XH1-...`) with Brotli compression and CRC32 verification. Also includes Valorant string parser. |
| `crates/xhair-library` | Catalog persistence, 30-day trash retention, active quick-switch slots (1–9), search indexing, and tag filtering. |
| `crates/xhair-platform-win` | Windows 10/11 Win32 integration: Click-through layered window (`WS_EX_LAYERED | WS_EX_TRANSPARENT | WS_EX_NOACTIVATE`), per-monitor DPI awareness, global hotkeys, foreground tracking, Windows startup registry, and zero-telemetry crash reporting. |
| `crates/xhair-dynamics` | Recoil & spread simulation models: neutral gun profile templates (Rifle, SMG, Pistol), synthetic input state machines, dynamic bindings (`gap = base + spread * scale`), and Spray Lab trajectory simulator. |
| `crates/xhair-mcp` | Model Context Protocol (MCP) server implementation with stdio and authenticated loopback HTTP transports. Exposes 32 tools, 5 resources, and 5 prompts. |
| `crates/xhair-app` | Tauri shell wiring, tray menus, window management, and application lifecycle. |
| `crates/xhair-cli` | Command-line interface for offline rendering, format conversion, startup toggle, and MCP stdio proxy. |
| `xtask` | Developer automation: schema checking, memory graph validation, release packaging (portable ZIP, CLI ZIP, NSIS script, SBOM, checksums). |
| `ui/` | Svelte 5 + Vite editor frontend: Canvas grid, layer panel, inspector, fine nudge pad, calibration modal, Spray Lab, and compliance monitor. |

---

## 4. Runtime Modes

1. **GUI Editor (`xhair.exe`):**
   - Launches the full interactive Svelte 5 editor.
   - Provides visual canvas editing, live preview, color pickers, nudge pad, and library manager.
2. **Overlay Daemon (`xhair.exe --minimized`):**
   - Headless background tray mode.
   - Hosts the click-through transparent Win32 layered overlay window.
   - Listens for global hotkeys to switch slots or toggle visibility.
3. **MCP Stdio Server (`xhair-cli mcp`):**
   - Autonomous agent interface over stdio.
   - Proxies JSON-RPC calls over authenticated loopback HTTP if an existing `xhair` process is running, or operates standalone.

---

## 5. Mutation & Rendering Data Flow

```
[UI / MCP Action]
       │
       ▼
Command::LayerUpdate { crosshair_id, layer_id, payload }
       │
       ▼
CommandBus::dispatch()
  ├─ 1. Records reverse command onto Undo stack
  ├─ 2. Applies payload mutations to Crosshair document
  ├─ 3. Calculates PatchDiff
  └─ 4. Emits DiffEvent to subscribers
       │
       ▼
Renderer / Overlay Window
  ├─ 1. Evaluates dynamic driver spread (if dynamics enabled)
  ├─ 2. Computes screen center with calibration offsets
  ├─ 3. tiny-skia rasterizes shapes into premultiplied 32-bit BGRA buffer
  └─ 4. Calls UpdateLayeredWindow to present frame to Desktop Window Manager (DWM)
```
