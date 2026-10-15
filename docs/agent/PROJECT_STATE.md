# Project State

- **Updated:** 2026-10-15
- **Session:** 4
- **Phase / Milestone:** M3 Library + Format — Done | Next: M4 Overlay Daemon
- **Version released:** None | **Unreleased on main:** 0.1.0-dev

## Done (milestone-level)
- [x] Phase -1 Bootstrap (completed 2026-10-15)
- [x] Phase 0 Design (approved by user 2026-10-15)
- [x] M1 Foundations (completed 2026-10-15)
- [x] M2 Renderer + CLI (completed 2026-10-15)
- [x] M3 Library + Format (completed 2026-10-15)
- [ ] M4 Overlay Daemon
- [ ] M5 Editor UI
- [ ] M6 MCP Server
- [ ] M7 Effects & Polish
- [ ] M8 Calibration
- [ ] M9 Dynamics & Spray Lab
- [ ] M10 Packaging & Hardening

## In progress
- M4 Overlay Daemon: Implement Win32 layered click-through window, DPI awareness, foreground game tracking, and calibration test pattern.

## Next exact step
- Implement Win32 transparent click-through window with `UpdateLayeredWindow`, DPI handling, monitor enumeration, and foreground process following in `xhair-platform-win`.

## WIP notes
- `xhair-format`:
  - Canonical JSON serializer with float normalization (max 4 decimal places) and stable key order.
  - Brotli compressed share code format (`XH1-<payload>-<crc32>`) with 8 KB safety limit and CRC32 verification.
  - `.xhairpack` bundle ZIP archiver with `PackManifest` for multi-crosshair sharing.
  - Schema version migration framework (`migrate_document_json`) rejecting future versions cleanly.
  - Valorant in-game profile code parser importing inner/outer lines, center dot, outlines, colors, and gaps with unmappable warnings.
  - Fixtures in `tests/fixtures/formats/` validating format limits (assets, path points, group nesting <= 8, cycle detection, names, layers).
- `xhair-library`:
  - Atomic file writer with fsync, temporary file rename on same volume, and `.bak` preservation.
  - Portable mode resolution (`xhair.portable` marker beside executable) and `%APPDATA%/xhair` fallback.
  - `LibraryEngine` with full CRUD, folders, tagging, favorites, 1..=20 quick-select slots, 30-day trash lifecycle with purge, search, and bulk operations.
  - Bounded LRU thumbnail cache (8 MB budget) with LRU eviction.

## Known issues / blockers
- None.

## Measured numbers (latest)
- Workspace crates: 12 (all build & test green, 58 tests passing)
- Golden image tests: 8 tests passing
- Format & library tests: 18 tests passing (including property tests & limit fixtures)
- Clippy & rustfmt: Clean, 0 warnings with `-D warnings`

## Human decisions pending
- None.

