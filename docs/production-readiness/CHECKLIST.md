# Production Readiness Master Checklist

**Project:** XHAIR — Commercial-Grade Crosshair Studio & MCP Daemon  
**Target Version:** 1.0.0 Production Release  
**Status Tracker:** Active Implementation

---

## Phase 1: Critical Engine & Interaction Fixes

- [x] **1.1 Overlay Calibration Offset Wiring**
  - [x] Add `config.calibration_offset.x` to `screen_center_x` in `render_and_update()` ([`crates/xhair-platform-win/src/overlay.rs`](file:///D:/cross-aim/crates/xhair-platform-win/src/overlay.rs)).
  - [x] Add `config.calibration_offset.y` to `screen_center_y` in `render_and_update()` ([`crates/xhair-platform-win/src/overlay.rs`](file:///D:/cross-aim/crates/xhair-platform-win/src/overlay.rs)).
  - [x] Verify hotkey `Ctrl+Shift+C` (recenter) resets calibration offset to `(0, 0)` and repositions overlay instantly.
  - [x] Add unit test verifying that `config.calibration_offset` shifts window position by exact physical pixels.

- [x] **1.2 Overlay Symmetrical Bounding Box & Zero-Clipping Engine**
  - [x] Update `calculate_crosshair_bounds` buffer allocation in [`crates/xhair-platform-win/src/overlay.rs`](file:///D:/cross-aim/crates/xhair-platform-win/src/overlay.rs) to compute symmetrical half-extents around `(0, 0)`:
    `max_half_w = max(abs(min_x), abs(max_x))` and `max_half_h = max(abs(min_y), abs(max_y))`.
  - [x] Ensure `win_w` and `win_h` fit all scaled layer and arm offsets with at least 16px conservative padding.
  - [x] Calculate `win_x` and `win_y` symmetrically around `screen_center_x` and `screen_center_y`.
  - [x] Add unit test verifying that an asymmetrical crosshair with large global offset (`offsetX = 50px`, `offsetY = -40px`) renders without clipping.

- [x] **1.3 Canvas Mouse Drag & Marquee State Separation**
  - [x] Implement hit-testing for layers and arms in [`ui/src/components/CanvasGrid.svelte`](file:///D:/cross-aim/ui/src/components/CanvasGrid.svelte).
  - [x] On `mousedown`: If click hits a layer, push undo state once and activate `isDraggingTarget = true; isMarquee = false`.
  - [x] On `mousedown`: If click is on empty canvas, activate `isMarquee = true; isDraggingTarget = false`.
  - [x] In `handleMouseMove`: Execute shape/arm drag when `isDraggingTarget` is active with smooth 120fps redraw.
  - [x] In `handleMouseUp`: Conclude drag, invoke `editorState.syncWithOverlay()`, and finalize marquee multi-selection.

- [x] **1.4 Tool Palette Mapping & Shape Creation Fixes**
  - [x] In [`ui/src/components/CanvasGrid.svelte`](file:///D:/cross-aim/ui/src/components/CanvasGrid.svelte), fix tool `R` to insert `Rect` (`RectProps: width: 8, height: 8`) instead of `TShape`.
  - [x] Add explicit tool definitions for `T` (`TShape`) and `+` (`Plus`).
  - [x] Add proper layer addition handler for `B` (Brush / Pixel bitmap).

- [x] **1.5 Color Picker Undo Coalescing**
  - [x] Extend `setLayerColor` in [`ui/src/lib/state.svelte.ts`](file:///D:/cross-aim/ui/src/lib/state.svelte.ts) with `pushUndoState?: boolean = true`.
  - [x] In [`ui/src/components/ColorPicker.svelte`](file:///D:/cross-aim/ui/src/components/ColorPicker.svelte), push undo snapshot on `mousedown`, stream color updates live with `applyColor(false)`, and sync overlay on `mouseup`.
  - [x] Verify dragging colors across SV box pushes exactly one undo entry.

---

## Phase 2: Unified Backend & Real-Time MCP Bridge

- [x] **2.1 Embedded Loopback MCP HTTP Server in Main Application**
  - [x] Mount and start `McpServer` inside [`crates/xhair-app/src/lib.rs`](file:///D:/cross-aim/crates/xhair-app/src/lib.rs) on loopback `127.0.0.1:0`.
  - [x] Generate secure 256-bit Bearer token on app startup.
  - [x] Write `%TEMP%/xhair-endpoint.json` with port, token, and PID on startup.
  - [x] Delete `%TEMP%/xhair-endpoint.json` on clean exit and panic hook.

- [x] **2.2 Automatic Stdio Proxying in `xhair-cli mcp`**
  - [x] Update `handle_mcp` in [`crates/xhair-cli/src/main.rs`](file:///D:/cross-aim/crates/xhair-cli/src/main.rs) to inspect `read_endpoint_info()`.
  - [x] If running instance is alive, proxy stdin/stdout via `run_stdio_proxy()`.
  - [x] If no instance is running, fallback to headless `ServerContext` with disk `LibraryEngine`.

- [x] **2.3 Physical Win32 Overlay Controls for MCP Tools**
  - [x] Wire `OverlaySender` into [`crates/xhair-mcp/src/context.rs`](file:///D:/cross-aim/crates/xhair-mcp/src/context.rs).
  - [x] Connect `overlay.set_enabled` to issue `OverlayCommand::SetEnabled(enabled)`.
  - [x] Connect `calibration.show` and `calibration.hide` to `OverlayCommand::ToggleCalibrationPattern`.
  - [x] Connect `calibration.set_offset` to `OverlayCommand::Nudge(Vec2::new(dx, dy))`.
  - [x] Connect `display.list` to Win32 `EnumDisplayMonitors` API.

- [x] **2.4 Disk Storage Integration (`xhair-library`)**
  - [x] Initialize `LibraryEngine::new(LibraryEngine::resolve_data_dir())` in [`crates/xhair-app/src/lib.rs`](file:///D:/cross-aim/crates/xhair-app/src/lib.rs).
  - [x] Expose local `/api/library` REST endpoints in [`crates/xhair-app/src/server.rs`](file:///D:/cross-aim/crates/xhair-app/src/server.rs).
  - [x] Update [`ui/src/lib/state.svelte.ts`](file:///D:/cross-aim/ui/src/lib/state.svelte.ts) to load and save crosshairs to disk library instead of `localStorage`.

- [x] **2.5 Real-Time UI Event Streaming for MCP Mutations**
  - [x] Emit custom DOM events into WebView2 webview when MCP tools mutate documents.
  - [x] Show pulsing neon "Agent is editing" indicator in `TopBar.svelte` during agent mutations.
  - [x] Populate `McpModal.svelte` with actual live tool invocation logs and timings.

---

## Phase 3: World-Class UI/UX Studio Redesign

- [x] **3.1 3-Column Studio Layout Architecture**
  - [x] Refactor [`ui/src/App.svelte`](file:///D:/cross-aim/ui/src/App.svelte) into:
    - Left Rail: Explorer & Layers (260px).
    - Center Stage: Viewport Canvas with interactive HUD.
    - Right Inspector: Contextual properties accordion (320px).
  - [x] Implement responsive layout supporting 1024×640 to 4K displays.

- [x] **3.2 Unified Contextual Inspector (Eliminating 11-Tab Dock)**
  - [x] Replace `DockSystem.svelte` with unified `Inspector.svelte`.
  - [x] Collapsible accordion cards: Transform, Shape Geometry, Fill & Stroke, Visual Effects.
  - [x] Context-sensitive controls that update immediately when selecting any layer.

- [x] **3.3 Figma-Style Drag-to-Scrub Numerical Inputs**
  - [x] Create `ScrubInput.svelte` component allowing horizontal drag on labels (`X`, `Y`, `Rot`, `Gap`, `Length`).
  - [x] Click to type numerical values; arrow keys to increment/decrement with Shift (10×) and Alt (0.1×).

- [x] **3.4 Clean In-Place Modal & Group Creation**
  - [x] Replace `window.prompt()` in [`ui/src/components/LibrarySidebar.svelte`](file:///D:/cross-aim/ui/src/components/LibrarySidebar.svelte) with inline form.
  - [x] Right-click context menus for profiles and groups.

- [x] **3.5 Collapsible Diagnostics & Labs Bottom Drawer**
  - [x] Create `BottomDock.svelte` with collapsible tabs for:
    - Animation Timeline.
    - Spray Lab recoil simulator.
    - Calibration Studio.
    - MCP Live Activity Log.

---

## Phase 4: Production Hardening, Verification & Release

- [x] **4.1 Full Workspace Test Suite Verification**
  - [x] Backend: `cargo test --workspace` 100% green (90 tests passing).
  - [x] Parity CI: `cargo test -p xhair-mcp --test parity` 100% verified.
  - [x] Frontend: `pnpm.cmd -C ui test` 100% green (24 tests passing).
  - [x] Typescript: `pnpm.cmd -C ui lint` 0 errors.

- [x] **4.2 End-to-End Integration Tests**
  - [x] Write integration test: MCP tool mutates crosshair -> disk library updates -> overlay updates (`test_e2e_mcp_mutates_crosshair_library_and_overlay`).
  - [x] Write integration test: UI saves profile -> file saved in `%APPDATA%/xhair/crosshairs/` (`test_e2e_ui_saves_profile_to_disk`).

- [x] **4.3 Automated Production Packaging**
  - [x] Execute `cargo xtask package --version 1.0.0`.
  - [x] Verify NSIS installer, portable zip bundle, CycloneDX SBOM, and SHA256 checksums in `dist/`.
