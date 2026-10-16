# Production Readiness Audit & Executive Summary

**Project:** XHAIR — High-Performance Windows Crosshair Overlay, Editor & MCP Server  
**Date:** 2026-10-16  
**Auditor:** Antigravity Senior Engineering & Systems Architecture  
**Target:** Production-Ready Commercial Release (v1.0.0)

---

## 1. Executive Summary

A comprehensive architectural and codebase audit was conducted across all 12 workspace crates (`crates/*`), the Svelte 5 frontend (`ui/*`), the Win32 platform layer (`xhair-platform-win`), and the Model Context Protocol (MCP) server (`xhair-mcp`).

### The Verdict
The project has **extraordinary mathematical rigor, high-speed rasterization algorithms, clean domain types, and passing unit tests**. However, it is currently **architecturally fragmented into three isolated islands**:

```
┌─────────────────────────────────┐      ┌───────────────────────────────┐      ┌──────────────────────────────┐
│           Svelte 5 UI           │      │        Running Overlay        │      │          xhair-cli           │
│  - In-memory Svelte $state      │      │  - Separate thread            │      │  - Isolated MCP server       │
│  - LocalStorage persistence     │ ──X─ │  - Hardcoded to primary mon.  │ ──X─ │  - Empty in-memory context   │
│  - Directly mutates JS objects  │      │  - Calibration offset ignored │      │  - Changes disappear on exit │
│  - Never calls CommandBus       │      │  - Clipping on offset shapes  │      │  - Cannot control overlay/UI │
└─────────────────────────────────┘      └───────────────────────────────┘      └──────────────────────────────┘
                                                       ▲
                                                       │
                                   Three Disconnected Siloes
```

1. **The Desktop App Island:** `xhair.exe` runs a Win32 layered overlay and opens a webview hosting the Svelte 5 UI. But it **never starts the MCP server**, never writes `xhair-endpoint.json`, never connects to the backend `CommandBus`, and never initializes the crash-safe `LibraryEngine` (saving only to browser `localStorage`).
2. **The MCP Server Island:** `xhair-cli mcp` launches a standalone, isolated, in-memory dummy context. It does not proxy to the running `xhair.exe` application. Tools like `overlay.set_enabled`, `calibration.show`, and `layer.update` mutate an in-memory struct in a temporary CLI process and vanish upon process exit. They **never reach the running overlay or the editor UI**.
3. **The Overlay Island:** The Win32 overlay window ignores the user's calibration offset, clips offset or asymmetric crosshairs due to a bounding box origin mismatch, and is hardcoded to the primary monitor without multi-monitor selection.
4. **The UI/UX Polish Gap:** The editor UI relies on an 11-tab dock system that forces horizontal tab scrolling in a 320px sidebar for basic properties (Transform, Shape, Color, Layers), contains canvas mouse-drag deadlocks due to marquee collisions, uses raw browser `window.prompt()` for folder creation, and floods undo history with uncoalesced mouse moves.

---

## 2. Production Readiness Scorecard

| Subsystem | Score | Status | Key Blockers |
|---|:---:|:---:|---|
| **Mathematical Domain & Rasterization** (`xhair-model`, `xhair-render`, `xhair-geometry`) | **9.5 / 10** | **Production Ready** | Deterministic tiny-skia pipelines, symmetry engine, and golden image tests pass. Minor bounding box origin calculation fix needed for overlay windows. |
| **Command Bus & Inverses** (`xhair-commands`) | **9.0 / 10** | **Ready, Disconnected** | Pure mathematical inverse mutations and 100-entry history stack are fully implemented, but completely bypassed by the running application and UI. |
| **Storage & File Formats** (`xhair-library`, `xhair-format`) | **9.0 / 10** | **Ready, Disconnected** | Atomic crash-safe file writes, Brotli share codes, `.xhairpack` zips, and migrations are implemented, but the UI bypasses them to write to browser `localStorage`. |
| **Win32 Overlay Daemon** (`xhair-platform-win`) | **6.5 / 10** | **Needs Fixes** | Click-through transparency and zero-CPU static loop work well, but **calibration offset is completely ignored** in rendering, offset crosshairs clip, and multi-monitor picker is absent. |
| **Model Context Protocol (MCP)** (`xhair-mcp`) | **4.0 / 10** | **Broken Runtime IPC** | Schema & static tool definitions pass parity tests, but runtime execution is disconnected from the running app; tools are no-ops on actual overlay and UI state. |
| **Desktop Application Wiring** (`xhair-app`, `xhair-cli`) | **5.0 / 10** | **Prototype Wiring** | `xhair.exe` lacks MCP server initialization, lacks endpoint registration, lacks disk library mounting, and lacks IPC event broadcasting to webview. |
| **Frontend UI/UX** (`ui/`) | **6.0 / 10** | **Needs Polishing** | Svelte 5 runes state works, but 11-tab dock causes severe friction, canvas drag has a marquee deadlock bug, `R` tool adds `TShape`, and browser `window.prompt()` is used. |

---

## 3. Top 5 Critical Architectural Gaps to Resolve

### Gap 1: Connect MCP to Running Application (Principle P3 & P4)
- **Problem:** When an AI agent connects to `xhair-cli mcp`, it talks to a phantom server with blank state. Edits do not show in the UI or on the screen.
- **Solution:** `xhair.exe` must start a local loopback HTTP server with Bearer auth, write `xhair-endpoint.json` on startup, and `xhair-cli mcp` must proxy requests to this endpoint using the existing `proxy.rs`. When edits occur, push events to the webview and overlay.

### Gap 2: Replace LocalStorage with Backend `LibraryEngine`
- **Problem:** Profiles are currently saved in browser `localStorage`. Clearing cache or switching machines loses profiles, while `%APPDATA%/xhair/crosshairs/` remains empty.
- **Solution:** Expose `/api/library` endpoints in `server.rs` or Wry IPC handlers that delegate to `LibraryEngine`. The UI becomes a thin client of the disk-backed library.

### Gap 3: Fix Win32 Overlay Calibration and Clipping Bugs
- **Problem:** Calibration offsets are stored in config but never applied in `render_and_update()`. Any crosshair with an offset is clipped or rendered off-screen.
- **Solution:** Fix `render_and_update()` to apply `config.calibration_offset`, calculate bounding boxes around the true origin, and position the overlay window with proper physical coordinates.

### Gap 4: Transform UI into a Figma/Linear-Grade Studio Experience
- **Problem:** The right panel's 11 horizontal tabs force tedious tab-switching just to change a line thickness and a color. Canvas mouse dragging is blocked by an unhandled marquee state.
- **Solution:** Replace the 11-tab dock with a **Unified 3-Column Studio Layout**: Left Rail (Library & Layers), Center Stage (Canvas with interactive drag handles), and Right Inspector (Contextual accordion: Transform, Geometry, Fill/Stroke, Effects).

### Gap 5: Wire the Command Bus as Single Source of Truth
- **Problem:** The UI mutates internal JavaScript objects directly, bypassing the Rust `CommandBus` and undo coalescing.
- **Solution:** Dispatch UI actions through the backend command bus, enabling unified undo/redo across both human UI clicks and autonomous AI agent MCP calls.

---

## 4. Documentation Index in this Folder

The detailed analysis and roadmap are documented across the following files:

1. [01_WRONGLY_IMPLEMENTED_AND_BROKEN_FEATURES.md](file:///D:/cross-aim/docs/production-readiness/01_WRONGLY_IMPLEMENTED_AND_BROKEN_FEATURES.md): Comprehensive breakdown of all 12 identified bugs and architectural disconnects with exact file locations and code snippets.
2. [02_MCP_ARCHITECTURE_AND_PARITY.md](file:///D:/cross-aim/docs/production-readiness/02_MCP_ARCHITECTURE_AND_PARITY.md): Complete audit of the MCP server, proxy mechanism, tool dispatchers, and live event synchronization.
3. [03_UI_UX_PRODUCTION_REDESIGN_SPEC.md](file:///D:/cross-aim/docs/production-readiness/03_UI_UX_PRODUCTION_REDESIGN_SPEC.md): Full blueprint for the redesigned 3-column studio UI, contextual inspector, canvas viewport interactions, and typography.
4. [04_CORE_APP_AND_STORAGE_ENGINE.md](file:///D:/cross-aim/docs/production-readiness/04_CORE_APP_AND_STORAGE_ENGINE.md): Architecture for disk library synchronization, Win32 overlay precision, and multi-monitor handling.
5. [05_STEP_BY_STEP_IMPLEMENTATION_ROADMAP.md](file:///D:/cross-aim/docs/production-readiness/05_STEP_BY_STEP_IMPLEMENTATION_ROADMAP.md): Actionable 4-phase roadmap with testable milestones, verification steps, and release checklist.
