# Audit: Wrongly Implemented & Broken Features

**Project:** XHAIR — High-Performance Crosshair Overlay  
**Scope:** `crates/*` (Rust Backend), `ui/*` (Svelte 5 Frontend), `xhair-mcp` (MCP Server)

This document provides a line-by-line technical audit of all features that are currently non-functional, partially implemented, broken by logical collisions, or operating under simulated or disconnected behavior.

---

## 1. Disconnected MCP Server in Main Application

- **Files:**
  - [crates/xhair-app/src/lib.rs](file:///D:/cross-aim/crates/xhair-app/src/lib.rs#L1-L121)
  - [crates/xhair-app/src/server.rs](file:///D:/cross-aim/crates/xhair-app/src/server.rs#L64-L85)
  - [crates/xhair-mcp/src/proxy.rs](file:///D:/cross-aim/crates/xhair-mcp/src/proxy.rs#L10-L49)
- **Problem:**
  `xhair-app` lists `xhair-mcp` in its `Cargo.toml` dependencies, but **never imports or initializes `McpServer` or `HttpServer`**. When `xhair.exe` runs:
  - It does not start the MCP loopback HTTP server.
  - It does not write the endpoint discovery file `xhair-endpoint.json` via `write_endpoint_info()`.
  - Secondary processes (like IDE extensions, Claude Desktop, Cursor, or `xhair-cli mcp`) have no way to discover or communicate with the running application.
- **Impact:**
  Violates Invariant **P3** ("One source of truth") and Invariant **P4** ("Human/agent parity"). The running overlay app is completely blind to any AI agent actions.

---

## 2. Standalone Phantom MCP CLI Server (`xhair-cli mcp`)

- **Files:**
  - [crates/xhair-cli/src/main.rs](file:///D:/cross-aim/crates/xhair-cli/src/main.rs#L158-L161)
  - [crates/xhair-mcp/src/context.rs](file:///D:/cross-aim/crates/xhair-mcp/src/context.rs#L60-L91)
- **Problem:**
  In `crates/xhair-cli/src/main.rs`:
  ```rust
  fn handle_mcp(_args: &[String]) -> Result<()> {
      let server = xhair_mcp::McpServer::default();
      server.run_stdio().context("MCP stdio loop exited")
  }
  ```
  `handle_mcp` calls `McpServer::default()`, which invokes `ServerContext::new(None)`. It creates a temporary, empty `CommandBus` in its own isolated memory space.
  Even though `proxy.rs` implements `run_stdio_proxy(info)` to forward stdio JSON-RPC requests to a running instance, `xhair-cli` **never checks `read_endpoint_info()`**.
- **Impact:**
  When an AI agent runs `xhair-cli mcp` to modify the crosshair, the mutations occur inside a dead CLI process and are immediately lost on exit. The running overlay and editor UI never receive or display the changes.

---

## 3. MCP Overlay & Calibration Tools are In-Memory No-Ops

- **Files:**
  - [crates/xhair-mcp/src/tools/overlay.rs](file:///D:/cross-aim/crates/xhair-mcp/src/tools/overlay.rs#L22-L36)
  - [crates/xhair-mcp/src/tools/overlay.rs](file:///D:/cross-aim/crates/xhair-mcp/src/tools/overlay.rs#L128-L163)
- **Problem:**
  In `overlay.rs`:
  ```rust
  pub fn overlay_set_enabled(ctx: &ServerContext, args: Value) -> CallToolResult {
      let enabled = ...;
      ctx.with_state(|state| {
          state.overlay_enabled = enabled; // <-- Only mutates local struct!
          CallToolResult::json(...)
      })
  }
  ```
  `ServerContext` does not hold an `OverlaySender` channel or handle to the Win32 overlay thread.
  - Calling `overlay.set_enabled` only flips `state.overlay_enabled` in memory. It never sends `OverlayCommand::SetEnabled` to the overlay window.
  - Calling `calibration.show` only flips `state.calibration_active`. It never shows the calibration pattern on screen.
  - Calling `calibration.set_offset` only updates `state.calibration_offset`. It never nudges the overlay window.
  - Calling `display.list` returns a hardcoded mock array containing `\\\\.\\DISPLAY1` without querying Win32 display APIs.
- **Impact:**
  Agents believe tool calls succeeded because the server returns `{"status": "overlay_updated"}`, but zero physical changes occur on the user's screen.

---

## 4. Win32 Overlay Calibration Offset is Completely Ignored in Rendering

- **File:**
  - [crates/xhair-platform-win/src/overlay.rs](file:///D:/cross-aim/crates/xhair-platform-win/src/overlay.rs#L404-L510)
- **Problem:**
  `OverlayConfig` declares:
  ```rust
  pub calibration_offset: Vec2,
  ```
  And `run_overlay_thread` updates `config.calibration_offset` when hotkeys or commands arrive.
  However, in `render_and_update(hwnd, crosshair, profile, config)`:
  ```rust
  let screen_center_x = mapping.out_center_x as f32;
  let screen_center_y = mapping.out_center_y as f32;
  let win_x = (screen_center_x - (win_w as f32 / 2.0)).round() as i32;
  let win_y = (screen_center_y - (win_h as f32 / 2.0)).round() as i32;
  ```
  `config.calibration_offset` is **NEVER added to `screen_center_x` or `screen_center_y`**, nor passed into `RenderContext`.
- **Impact:**
  The calibration wizard, the hotkey `Ctrl+Shift+C` (recenter), and fine sub-pixel nudging do nothing to the actual overlay position.

---

## 5. Overlay Bounding Box Origin Mismatch & Offset Clipping Bug

- **Files:**
  - [crates/xhair-platform-win/src/overlay.rs](file:///D:/cross-aim/crates/xhair-platform-win/src/overlay.rs#L483-L509)
  - [crates/xhair-render/src/bounds.rs](file:///D:/cross-aim/crates/xhair-render/src/bounds.rs#L127-L158)
  - [crates/xhair-render/src/renderer.rs](file:///D:/cross-aim/crates/xhair-render/src/renderer.rs#L72-L89)
- **Problem:**
  In `overlay.rs`:
  ```rust
  let bbox = calculate_crosshair_bounds(&active_xhair);
  let win_w = (bbox.width().ceil() as u32).clamp(16, 2048);
  let win_h = (bbox.height().ceil() as u32).clamp(16, 2048);
  let mut pixmap = Pixmap::new(win_w, win_h)?;
  render(&active_xhair, &render_ctx, &mut pixmap)?;
  ```
  - `calculate_crosshair_bounds()` returns an AABB in local coordinates, shifting `min_x` and `max_x` by `global.offset.x`. If a crosshair has offset `x = 50px` and width `20px`, `bbox = [40, 60]`. `bbox.width()` is `20px`.
  - `renderer.rs::render()` places the reticle center at `center_x = win_w * 0.5 = 10px`, and then adds `ch.global.offset.x` (`50px`), rasterizing at coordinate `10 + 50 = 60px`!
  - But the allocated pixmap is only `20px` wide! Drawing at `60px` falls outside the buffer bounds and is clipped to complete transparency.
  - Furthermore, `win_x` is placed at `screen_center_x - win_w / 2`, ignoring `bbox.min_x`.
- **Impact:**
  Any crosshair moved from center, or any asymmetrical reticle, is partially clipped or completely invisible on the overlay.

---

## 6. Disconnected Disk Library Engine vs Browser LocalStorage

- **Files:**
  - [crates/xhair-library/src/engine.rs](file:///D:/cross-aim/crates/xhair-library/src/engine.rs#L25-L98)
  - [crates/xhair-app/src/server.rs](file:///D:/cross-aim/crates/xhair-app/src/server.rs#L174-L227)
  - [ui/src/lib/state.svelte.ts](file:///D:/cross-aim/ui/src/lib/state.svelte.ts#L803-L822)
- **Problem:**
  `crates/xhair-library` provides a complete, battle-tested library engine with atomic file writes, crash recovery, folders, slots 1..=20, and 30-day trash lifecycle.
  However, `xhair-app` **never initializes `LibraryEngine`**. In `ui/src/lib/state.svelte.ts`:
  ```typescript
  saveCurrentProfile() {
    ...
    if (typeof localStorage !== 'undefined') {
      localStorage.setItem('xhair_library', JSON.stringify(this.library));
      localStorage.setItem('xhair_active_crosshair', JSON.stringify(snapshot));
    }
  }
  ```
  The UI stores crosshairs exclusively in the WebView2 browser instance's `localStorage`.
- **Impact:**
  - Profiles are not saved to `%APPDATA%/xhair/crosshairs/`.
  - If a user runs the portable zip or clears cache, all profiles vanish.
  - Profiles created in the UI cannot be read by `xhair-cli render` or `xhair-mcp`.

---

## 7. Bypassed Command Bus in UI

- **Files:**
  - [crates/xhair-commands/src/bus.rs](file:///D:/cross-aim/crates/xhair-commands/src/bus.rs#L46-L140)
  - [ui/src/lib/state.svelte.ts](file:///D:/cross-aim/ui/src/lib/state.svelte.ts#L858-L1050)
- **Problem:**
  `xhair-commands` implements a central `CommandBus` with typed commands, inverse mutations, coalesced rapid nudges, and subscriber events.
  However, the UI executes all mutations by directly modifying JavaScript `$state` variables and pushing raw JSON snapshots to a local TypeScript array.
  There are no API endpoints or IPC handlers dispatching commands to the Rust `CommandBus`.
- **Impact:**
  Two separate, conflicting undo systems exist. Action parity is merely a static schema assertion in `parity.rs` rather than runtime parity.

---

## 8. Canvas Mouse Drag Deadlock Due to Marquee Collision

- **File:**
  - [ui/src/components/CanvasGrid.svelte](file:///D:/cross-aim/ui/src/components/CanvasGrid.svelte#L521-L589)
- **Problem:**
  In `handleMouseDown`:
  ```typescript
  // Start marquee selection drag
  isMarquee = true;
  marqueeStartX = mouseX;
  ...
  isDraggingTarget = true;
  ```
  Both `isMarquee` and `isDraggingTarget` are set to `true` on left click.
  Then in `handleMouseMove`:
  ```typescript
  if (isMarquee) {
    marqueeEndX = mouseX;
    marqueeEndY = mouseY;
    renderCanvas();
  }

  if (isDraggingTarget && !isMarquee) { // <-- ALWAYS FALSE!
    // Shape drag code never executes
  }
  ```
  Because `isMarquee` is `true`, `!isMarquee` evaluates to `false`.
- **Impact:**
  Direct mouse dragging on the canvas does not work. Left-click drag always draws a marquee box instead of moving shapes or arms.

---

## 9. Tool Mapping Inversion in Tool Palette

- **File:**
  - [ui/src/components/CanvasGrid.svelte](file:///D:/cross-aim/ui/src/components/CanvasGrid.svelte#L641-L658)
- **Problem:**
  In `handleToolSelect`:
  ```typescript
  else if (toolId === 'rect') {
    editorState.addLayer('TShape', 'T-Shape');
  }
  ```
  - Selecting the `R` tool (labeled "Rectangle / Box element") adds a `TShape` layer instead of a `Rect` shape.
  - Selecting the `B` tool (Brush) has no implementation.
  - Clicking any creation tool immediately inserts a new layer into the document instead of activating a creation mode on the canvas.
- **Impact:**
  Confusing user experience where tool buttons perform unexpected document mutations.

---

## 10. Undo History Flooding in ColorPicker

- **File:**
  - [ui/src/components/ColorPicker.svelte](file:///D:/cross-aim/ui/src/components/ColorPicker.svelte#L74-L97)
  - [ui/src/lib/state.svelte.ts](file:///D:/cross-aim/ui/src/lib/state.svelte.ts#L1402-L1420)
- **Problem:**
  In `ColorPicker.svelte`:
  When dragging the SV saturation/value box or the Hue slider:
  `handleSvMouseMove` calls `applyColor()` on every mousemove event.
  `applyColor()` calls `editorState.setLayerColor(color)`.
  `setLayerColor()` calls `this.pushUndo()` unconditionally on every invocation.
- **Impact:**
  Dragging a color for 1 second pushes 60+ undo states to the history, filling the undo stack with micro-color increments and making `Ctrl+Z` unusable.

---

## 11. Mocked MCP Activity Log in UI

- **Files:**
  - [ui/src/components/TopBar.svelte](file:///D:/cross-aim/ui/src/components/TopBar.svelte#L155-L163)
  - [ui/src/components/McpModal.svelte](file:///D:/cross-aim/ui/src/components/McpModal.svelte#L42-L68)
  - [ui/src/lib/state.svelte.ts](file:///D:/cross-aim/ui/src/lib/state.svelte.ts#L670-L680)
- **Problem:**
  - `TopBar.svelte` hardcodes `Status: Running on 127.0.0.1` and `pill active` for MCP status, even when no MCP server is running.
  - `McpModal.svelte` displays a static banner with hardcoded strings.
  - `editorState.notifyAgentActivity()` is defined in `state.svelte.ts` but never called by any backend handler or WebSocket/SSE event.
- **Impact:**
  The MCP activity log is perpetually empty, and status indicators show false positives.

---

## 12. Blocking Browser `window.prompt()` in Desktop App

- **File:**
  - [ui/src/components/LibrarySidebar.svelte](file:///D:/cross-aim/ui/src/components/LibrarySidebar.svelte#L25-L30)
- **Problem:**
  In `LibrarySidebar.svelte`:
  ```typescript
  function promptNewGroup() {
    const name = prompt('Enter new crosshair group name (e.g. Valorant, Apex, CS2):');
    if (name && name.trim()) {
      editorState.createCrosshairGroup(name.trim());
    }
  }
  ```
- **Impact:**
  Native desktop apps running WebView2 should never trigger blocking browser dialogs. On certain Windows configurations or minimized states, `prompt()` can lock the webview thread or appear behind other windows.
