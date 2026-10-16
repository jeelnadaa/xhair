# Project State

- **Updated:** 2026-10-16
- **Session:** 20
- **Phase / Milestone:** Production Readiness — Phase 1 Critical Fixes Complete
- **Version released:** 0.1.0 | **Release Artifacts:** Verified in `dist/` & `target/release/`

## Done (milestone-level)
- [x] Phase -1 Bootstrap (completed 2026-10-15)
- [x] Phase 0 Design (approved by user 2026-10-15)
- [x] M1 Foundations (completed 2026-10-15)
- [x] M2 Renderer + CLI (completed 2026-10-15)
- [x] M3 Library + Format (completed 2026-10-15)
- [x] M4 Overlay Daemon (completed 2026-10-15)
- [x] M5 Editor UI (completed 2026-10-16)
- [x] M6 MCP Server (completed 2026-10-16)
- [x] M7 Effects & Polish (completed 2026-10-16)
- [x] M8 Calibration (completed 2026-10-16)
- [x] M9 Dynamics & Spray Lab (completed 2026-10-16)
- [x] M10 Packaging & Hardening (completed 2026-10-16)
- [x] Design & Spec Update v2 (completed 2026-10-16)
- [x] Win32 Overlay Toggle Bug Resolution (completed 2026-10-16)
- [x] Complete Design & Spec Update v5 (completed 2026-10-16)
- [x] Production Readiness Documentation Suite (completed 2026-10-16):
  - Created `docs/production-readiness/` with comprehensive audits: `00_OVERVIEW_AND_EXECUTIVE_SUMMARY.md`, `01_WRONGLY_IMPLEMENTED_AND_BROKEN_FEATURES.md`, `02_MCP_ARCHITECTURE_AND_PARITY.md`, `03_UI_UX_PRODUCTION_REDESIGN_SPEC.md`, `04_CORE_APP_AND_STORAGE_ENGINE.md`, `05_STEP_BY_STEP_IMPLEMENTATION_ROADMAP.md`, and `CHECKLIST.md`.
- [x] Production Readiness Phase 1: Critical Engine & Interaction Fixes (completed 2026-10-16):
  - Win32 overlay calibration offset applied to physical window position.
  - Symmetrical zero-clipping buffer extents around `(0, 0)`.
  - Layer hit-testing separating canvas drag from marquee selection with 120fps redraw.
  - Tool palette mapping fixes (tool `R` inserts `Rect`).
  - Color picker undo coalescing with live preview and single history snapshot.

## In progress
- Phase 2: Unified Backend & Real-Time MCP Bridge (embedded MCP server in `xhair.exe`, `xhair-library` disk persistence, stdio proxying).

## Next exact step
- Implement Phase 2: Embedded loopback MCP server and physical overlay wiring.

## WIP notes
- Release artifacts generated in `dist/`:
  - `dist/installer.nsi`: Automated NSIS installer script with desktop/start menu shortcuts and uninstaller.
  - `dist/xhair-0.1.0-x64-portable.zip`: Standalone zero-install portable application bundle with `xhair.exe`, `README.md`, and license.
  - `dist/xhair-cli-0.1.0-x64.zip`: Standalone CLI bundle with `xhair-cli.exe` for offline rendering, format conversion, and MCP stdio.
  - `dist/sbom.json`: Software Bill of Materials (SBOM) listing package dependencies and licenses.
  - `dist/SHA256SUMS.txt`: Cryptographic SHA-256 integrity checksums for all release artifacts.
- Documentation deliverables (§15.2):
  - `README.md`, `CHANGELOG.md`, `docs/ARCHITECTURE.md`, `docs/GEOMETRY.md`, `docs/FORMAT.md`, `docs/MCP.md`, `docs/PROPERTIES.md`, `docs/COMMANDS.md`, `docs/COMPLIANCE.md`, `docs/DYNAMICS.md`, `docs/PERFORMANCE.md`, `docs/TESTING.md`, `docs/CONTRIBUTING.md`, `docs/ERRORS.md`.

## Known issues / blockers
- None.

## Measured numbers (latest)
- Workspace crates: 12 (all build & test green, 88 tests passing)
- Golden image tests: 8 tests passing
- Geometry tests: 11 tests passing (symmetry detection, quarter-pixel quantization, multi-res verification)
- Render & effect tests: 13 unit tests passing (four arms rendered, bloom, animation evaluation, static lock, heavy budget)
- Performance benchmarks: typical crosshair 2.91 µs (< 1000 µs budget), heavy crosshair 5470.72 µs (< 8000 µs budget)
- Platform tests: 15 tests passing (overlay styles, crash reports, startup registry, hotkeys, foreground detection, overlay toggle, calibration offset, zero-clipping bounds)
- MCP tests & Parity: 7 tests passing (stdio, HTTP, auth, prompts, resources, 100% parity across 35 tools and 25 commands)
- UI test suite: 23 tests passing in Vitest (100% green)
- TypeScript lint: 0 errors (`tsc --noEmit`)
- Vite production build: 239 kB JS, 63 kB CSS (built in ~800ms)
- Clippy & rustfmt: Clean, 0 warnings with `-D warnings`
- Agent continuity & memory graph: 100% validated (`cargo xtask check-agent-state`, `cargo xtask graph query --summary`)

## Human decisions pending
- None.
