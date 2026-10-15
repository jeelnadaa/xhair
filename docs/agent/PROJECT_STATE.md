# Project State

- **Updated:** 2026-10-15
- **Session:** 1
- **Phase / Milestone:** M2 Renderer + CLI — Done | Next: M3 Library + Format
- **Version released:** None | **Unreleased on main:** 0.1.0-dev

## Done (milestone-level)
- [x] Phase -1 Bootstrap (completed 2026-10-15)
- [x] Phase 0 Design (approved by user 2026-10-15)
- [x] M1 Foundations (completed 2026-10-15)
- [x] M2 Renderer + CLI (completed 2026-10-15)
- [ ] M3 Library + Format
- [ ] M4 Overlay Daemon
- [ ] M5 Editor UI
- [ ] M6 MCP Server
- [ ] M7 Effects & Polish
- [ ] M8 Calibration
- [ ] M9 Dynamics & Spray Lab
- [ ] M10 Packaging & Hardening

## In progress
- M3 Library + Format: Implement disk storage, atomic writes, index.json, `.xhair` / `.xhairpack` serialization, and share codes.

## Next exact step
- Implement storage and index management in `xhair-library` and format serialization in `xhair-format`.

## WIP notes
- `xhair-render`: Complete deterministic vector rasterizer for all shapes (Lines, Dot, Rect, Circle, Ring, Arc, Polygon, Triangle, Chevron, T-Shape, Plus, Pixels, Path).
- Glow, bloom, drop shadow, and separable multi-pass gaussian blur compositing.
- Native and `wasm32-unknown-unknown` compilation verified.
- Deterministic golden image test suite (8 test suites) passing.
- `xhair-cli render` command implemented with support for presets (classic, dot, t_shape), input files, scale factors, and arbitrary output dimensions.

## Known issues / blockers
- None.

## Measured numbers (latest)
- Workspace crates: 12 (all build & test green, 36 tests passing)
- Golden image tests: 8 tests passing
- Typical crosshair render time: < 1 ms (< 1000 µs)
- Clippy & rustfmt: Clean, 0 warnings with `-D warnings`

## Human decisions pending
- None.

