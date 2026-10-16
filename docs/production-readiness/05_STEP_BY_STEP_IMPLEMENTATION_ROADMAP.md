# Step-by-Step Production Implementation Roadmap

**Project:** XHAIR — Production Hardening & Release Roadmap  
**Target:** Version 1.0.0 Commercial Grade Release  
**Phases:** 4 Testable Iterations  
**Master Checklist:** See [`docs/production-readiness/CHECKLIST.md`](file:///D:/cross-aim/docs/production-readiness/CHECKLIST.md) for live task progress.

---

## Phase 1: Critical Engine & Interaction Fixes

**Goal:** Resolve immediate rendering bugs, clipping errors, and canvas mouse interaction deadlocks.

| Step | Component | Action | Verification |
|---|---|---|---|
| **1.1** | `xhair-platform-win` | Fix `render_and_update()` to apply `config.calibration_offset` to `screen_center_x` and `screen_center_y`. | Nudging in Calibration Wizard moves the overlay on screen. |
| **1.2** | `xhair-platform-win` | Fix overlay bounding box calculation to size symmetrically around `(0, 0)` so offset crosshairs never clip. | Reticle with `offsetX = 50px` renders completely on the overlay. |
| **1.3** | `ui/CanvasGrid.svelte` | Fix mouse drag deadlock by separating marquee drag from shape drag. | Left-click drag on canvas smoothly moves the selected shape. |
| **1.4** | `ui/CanvasGrid.svelte` | Fix tool selection so `R` tool creates `RectProps` instead of `TShape`. | Clicking `R` adds a Rectangle layer. |
| **1.5** | `ui/ColorPicker.svelte` | Coalesce undo history during color dragging; push undo snapshot only on `mouseup`. | Scrubbing color for 5 seconds pushes only 1 undo step. |

---

## Phase 2: Unified Backend & Real-Time MCP Bridge

**Goal:** Connect the MCP server to the running application, wire `OverlaySender`, and enable real-time agent editing.

| Step | Component | Action | Verification |
|---|---|---|---|
| **2.1** | `xhair-app` | Embed `McpHttpServer` in `xhair.exe` on loopback `127.0.0.1` with dynamic port and Bearer auth token. | Running `xhair.exe` generates `%TEMP%/xhair-endpoint.json`. |
| **2.2** | `xhair-cli` | Update `handle_mcp()` to inspect `read_endpoint_info()` and proxy stdio to the running app via `run_stdio_proxy()`. | `xhair-cli mcp` connects to running app; fallback works offline. |
| **2.3** | `xhair-mcp` | Wire `OverlaySender` into `ServerContext`. Connect `overlay.set_enabled`, `calibration.*`, and `display.list` to actual Win32 overlay controls. | Calling `overlay.set_enabled(false)` via MCP immediately hides overlay. |
| **2.4** | `xhair-app` | Mount `LibraryEngine::new(resolve_data_dir())` in `xhair.exe` and expose `/api/library` endpoints. | Profiles are saved to disk in `%APPDATA%/xhair/crosshairs/`. |
| **2.5** | `ui` & `xhair-mcp` | Stream tool execution events to webview via Wry IPC / custom event; activate pulsing "Agent is editing" banner and populate MCP modal log. | MCP edits live-update editor canvas and log entries. |

---

## Phase 3: World-Class UI/UX Studio Redesign

**Goal:** Transform the UI into a Figma/Linear-grade 3-column studio with a unified contextual inspector.

| Step | Component | Action | Verification |
|---|---|---|---|
| **3.1** | `ui/src/App.svelte` | Restructure layout into Left Explorer (260px), Center Stage (Canvas), and Right Inspector (320px). | Layout resizes cleanly down to 1024×640 without horizontal scroll. |
| **3.2** | `ui/Inspector.svelte` | Replace 11-tab dock with Unified Contextual Inspector (Transform, Geometry, Fill & Stroke, Effects cards). | Selecting a shape shows all its properties at once without tab switching. |
| **3.3** | `ui/CanvasGrid.svelte` | Implement Figma-style scrubbable labels (`X`, `Y`, `Rotation`, `Opacity`). | Dragging label scrubs values smoothly; clicking allows typing. |
| **3.4** | `ui/LibrarySidebar.svelte`| Replace `window.prompt()` for group creation with an inline input box. | Groups can be created and renamed with keyboard without modal freezes. |
| **3.5** | `ui/BottomDock.svelte` | Implement collapsible bottom drawer housing Timeline, Spray Lab, and Calibration Studio. | Bottom dock expands smoothly and collapses cleanly. |

---

## Phase 4: Production Hardening, Verification & Release

**Goal:** Full verification, memory budget enforcement, automated release packaging.

| Step | Component | Action | Verification |
|---|---|---|---|
| **4.1** | `tests/` | Add end-to-end integration test verifying that an MCP edit updates both the disk library and the overlay. | `cargo test --workspace` 100% green. |
| **4.2** | `ui/tests/` | Update Vitest test suite for new 3-column layout and contextual inspector. | `pnpm.cmd -C ui test` 100% green. |
| **4.3** | `xtask` | Package final v1.0.0 installer and portable zip with `cargo xtask package`. | Verified artifacts in `dist/` with SHA256 sums and SBOM. |
