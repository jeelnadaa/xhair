# Project State

- **Updated:** 2026-10-16
- **Session:** 18
- **Phase / Milestone:** Design & Spec Update v2 & Overlay Toggle Resolution
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
- [x] Design & Spec Update v2 (completed 2026-10-16):
  - Phase 0 design tokens updated to monochromatic neutral gray ramp (`#111418`, `#181C22`, `#21262F`, `#282F3A`), Satoshi font from Fontshare, 12 screen designs exported and verified.
  - Command bus & registry: All v2 commands added (`LayerRename`, `LayerGroup`, `LayerUngroup`, `LayerSetPosition`, `LayerRecenter`, `CrosshairRecenter`, `PaintSet`, `SelectionSet`, `SelectionAdd`, `SelectionRemove`, `SelectionToggle`, `SelectionAll`, `SelectionClear`, `SelectionInvert`, plus all `Group*` and `*Pack` commands) with pure inverse mutations.
  - MCP Tools & 100% Parity: 30 new v2 tools implemented across `crates/xhair-mcp/src/tools/` (`edit.rs`, `library.rs`), parity test passing across 35 required tools and 25 commands.
  - UI Components: Hierarchical layer groups, multi-select bounding box & canvas marquee, 8-directional NudgePad with step chips & custom step, ColorPicker popover with SV picker / hue / alpha / custom swatches / colorblind palette, GroupSettingsModal with process binding, ShortcutsDialog cheat sheet, inline renaming with F2, and "Moved from center" indicator badge with 1-click recentering.
- [x] Win32 Overlay Toggle Bug Resolution:
  - Resolved overlay daemon toggle (`SetEnabled(bool)`) by explicitly issuing `ShowWindow(hwnd, SW_HIDE)` and `ShowWindow(hwnd, SW_SHOWNOACTIVATE)` on the layered transparent window.
  - Wired native `wry` IPC handlers for `set_overlay_enabled`, `toggle_overlay`, and `recenter`.
  - Registered Windows global hotkeys `Ctrl+Shift+X` (Toggle Overlay) and `Ctrl+Shift+C` (Recenter Crosshair).
- [x] Release Build Recompiled: `target/release/xhair.exe` and `target/release/xhair-cli.exe` freshly compiled with embedded frontend distribution.

## In progress
- None. All features fully implemented, tested, and verified.

## Next exact step
- Human review and verification of standalone executable.

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
- Workspace crates: 12 (all build & test green, 87 tests passing)
- Golden image tests: 8 tests passing
- Geometry tests: 11 tests passing (symmetry detection, quarter-pixel quantization, multi-res verification)
- Render & effect tests: 13 unit tests passing (four arms rendered, bloom, animation evaluation, static lock, heavy budget)
- Performance benchmarks: typical crosshair 2.91 µs (< 1000 µs budget), heavy crosshair 5470.72 µs (< 8000 µs budget)
- Platform tests: 13 tests passing (overlay styles, crash reports, startup registry, hotkeys, foreground detection, overlay toggle SW_HIDE)
- MCP tests & Parity: 7 tests passing (stdio, HTTP, auth, prompts, resources, 100% parity across 35 tools and 25 commands)
- UI test suite: 23 tests passing in Vitest (100% green)
- TypeScript lint: 0 errors (`tsc --noEmit`)
- Vite production build: 239 kB JS, 63 kB CSS (built in ~800ms)
- Clippy & rustfmt: Clean, 0 warnings with `-D warnings`
- Agent continuity & memory graph: 100% validated (`cargo xtask check-agent-state`, `cargo xtask graph query --summary`)

## Human decisions pending
- None.
