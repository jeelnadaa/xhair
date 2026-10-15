# Project State

- **Updated:** 2026-10-15
- **Session:** 5
- **Phase / Milestone:** M4 Overlay Daemon — Done | Next: M5 Editor UI
- **Version released:** None | **Unreleased on main:** 0.1.0-dev

## Done (milestone-level)
- [x] Phase -1 Bootstrap (completed 2026-10-15)
- [x] Phase 0 Design (approved by user 2026-10-15)
- [x] M1 Foundations (completed 2026-10-15)
- [x] M2 Renderer + CLI (completed 2026-10-15)
- [x] M3 Library + Format (completed 2026-10-15)
- [x] M4 Overlay Daemon (completed 2026-10-15)
- [ ] M5 Editor UI
- [ ] M6 MCP Server
- [ ] M7 Effects & Polish
- [ ] M8 Calibration
- [ ] M9 Dynamics & Spray Lab
- [ ] M10 Packaging & Hardening

## In progress
- M5 Editor UI: Svelte 5 + Vite + Tauri 2 editor UI, high-performance canvas grid with DPR scaling, descriptor-driven inspector, layers panel, and 8-way nudge pad.

## Next exact step
- Scaffold Tauri 2 + Svelte 5 application shell in `ui/` integrating tokens from `design/tokens.json` and WASM renderer.

## WIP notes
- `xhair-platform-win`:
  - Dedicated background message-loop thread with `WS_EX_LAYERED | WS_EX_TRANSPARENT | WS_EX_TOPMOST | WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE` popup window.
  - 32-bit premultiplied BGRA DIB surface rasterized via `tiny-skia` and committed via `UpdateLayeredWindow`.
  - Zero CPU idle render loop: renders strictly on update commands or geometry change.
  - Per-Monitor-V2 DPI awareness and physical pixel monitor bounds enumeration.
  - Safe limited-query foreground process tracking (`VALORANT-Win64-Shipping.exe`) and game window client rect following (`follow_game_window`).
  - System tray icon with status flags tooltip (`TrayIcon`).
  - Global hotkeys manager (`RegisterHotKey`) with conflict detection.
  - Calibration test pattern generator (`create_calibration_crosshair`).

## Known issues / blockers
- None.

## Measured numbers (latest)
- Workspace crates: 12 (all build & test green, 67 tests passing)
- Golden image tests: 8 tests passing
- Platform tests: 9 tests passing (5 platform integration tests + 4 unit tests)
- Clippy & rustfmt: Clean, 0 warnings with `-D warnings`

## Human decisions pending
- None.

