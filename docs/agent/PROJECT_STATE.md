# Project State

- **Updated:** 2026-10-16
- **Session:** 7
- **Phase / Milestone:** M6 MCP Server — Done | Next: M7 Effects & Polish
- **Version released:** None | **Unreleased on main:** 0.1.0-dev

## Done (milestone-level)
- [x] Phase -1 Bootstrap (completed 2026-10-15)
- [x] Phase 0 Design (approved by user 2026-10-15)
- [x] M1 Foundations (completed 2026-10-15)
- [x] M2 Renderer + CLI (completed 2026-10-15)
- [x] M3 Library + Format (completed 2026-10-15)
- [x] M4 Overlay Daemon (completed 2026-10-15)
- [x] M5 Editor UI (completed 2026-10-16)
- [x] M6 MCP Server (completed 2026-10-16)
- [ ] M7 Effects & Polish
- [ ] M8 Calibration
- [ ] M9 Dynamics & Spray Lab
- [ ] M10 Packaging & Hardening

## In progress
- M7 Effects & Polish: Global bloom, advanced glow compositing, animation tracks, timeline dock, and heavy crosshair performance optimization.

## Next exact step
- Implement Milestone 7: Global bloom rasterizer, keyframe animation engine, timeline dock, and static-only lock (§4.6, §4.7, §7.4).

## WIP notes
- `ui`:
  - Complete Svelte 5 + Vite application conforming to `design/tokens.json` and `design/XHAIR — App Design.html`.
  - High-performance canvas grid view with DPR scaling (`window.devicePixelRatio`), 1px / 5px / 10px major lines, distance rings, and center indicators (`#FF4FD8`).
  - Descriptor-driven inspector controls generated from `ui/src/generated/properties.json` with horizontal scrub handles and sliders.
  - Interactive 8-way Nudge Pad with step chips (`0.1`, `0.25`, `0.5`, `1`, `5`, `10`), target selector (`Arm` | `Layer` | `All`), and keyboard shortcuts.
  - Layers panel with solo/hide/lock, alignment diagnostics (`ALIGNED` vs `OFF-GRID`), and duplication/deletion.
  - Color picker with hex input and preset swatches.
  - Command palette modal (`Ctrl+K`) executing registered shortcuts and command bus operations.
  - Accessible `Tooltip` component on every interactive control with title, description, and shortcut badge.

## Known issues / blockers
- None.

## Measured numbers (latest)
- Workspace crates: 12 (all build & test green, 71 tests passing)
- Golden image tests: 8 tests passing
- Platform tests: 9 tests passing (5 platform integration tests + 4 unit tests)
- MCP tests & Parity: 7 tests passing (stdio, HTTP, auth, prompts, resources, parity)
- UI test suite: 11 tests passing in Vitest (100% green)
- TypeScript lint: 0 errors (`tsc --noEmit`)
- Vite production build: 93 kB JS, 18 kB CSS (built in ~370ms)
- Clippy & rustfmt: Clean, 0 warnings with `-D warnings`

## Human decisions pending
- None.

