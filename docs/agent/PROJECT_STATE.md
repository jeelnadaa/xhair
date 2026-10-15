# Project State

- **Updated:** 2026-10-15
- **Session:** 1
- **Phase / Milestone:** M1 Foundations — Done | Next: M2 Renderer + CLI
- **Version released:** None | **Unreleased on main:** 0.1.0-dev

## Done (milestone-level)
- [x] Phase -1 Bootstrap (completed 2026-10-15)
- [x] Phase 0 Design (approved by user 2026-10-15)
- [x] M1 Foundations (completed 2026-10-15)
- [ ] M2 Renderer + CLI
- [ ] M3 Library + Format
- [ ] M4 Overlay Daemon
- [ ] M5 Editor UI
- [ ] M6 MCP Server
- [ ] M7 Effects & Polish
- [ ] M8 Calibration
- [ ] M9 Dynamics & Spray Lab
- [ ] M10 Packaging & Hardening

## In progress
- M2 Renderer + CLI: Implement `xhair-render` tiny-skia rasterizers, paints/gradients/strokes, effects, and WASM/native golden tests.

## Next exact step
- Implement shape rasterizers in `xhair-render` (`lines.rs`, `dot.rs`, `circle.rs`, `cross.rs`, `t_shape.rs`, `polygon.rs`, `pixels.rs`, `text.rs`).

## WIP notes
- Domain model implemented in `xhair-model` with validation, error catalog, settings, and templates.
- Alignment and coordinate math with even/odd rules, non-uniform stretch, and proptests implemented in `xhair-geometry`.
- Command bus with pure state transitions, atomic batches, coalescing nudges, undo/redo, and dry-run implemented in `xhair-commands`.
- Property descriptors with tooltips and validation registry implemented with `cargo xtask schema` export (`ui/src/generated/properties.json` and `docs/PROPERTIES.md`).

## Known issues / blockers
- None.

## Measured numbers (latest)
- Workspace crates: 12 (all build & test green, 25 tests passing)
- Proptests: 3 suites in `xhair-geometry` (center coordinates, DPI scaling, coordinate mapping round trip)
- Clippy & rustfmt: Clean, 0 warnings with `-D warnings`

## Human decisions pending
- None.

