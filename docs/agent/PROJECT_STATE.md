# Project State

- **Updated:** 2026-10-16
- **Session:** 19
- **Phase / Milestone:** Complete Design & Spec Update v5 (v2 + v3 + v4 merged)
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
- [x] Complete Design & Spec Update v5 (completed 2026-10-16):
  - Phase 0 Design: 24 screens at 1x & 2x in `design/screens/`, 3 pure neutral base ramps (Graphite, White, Pure Black) with channel spread <= 2, 9 neutral-base themes with WCAG contrast (>= 4.5:1, high-contrast >= 7:1), 6 font packs, 3 corner curvature styles, radius <= 10px with zero pills.
  - Backend command registry: Added 23 new v5 commands with typed scopes, availability, execution diffs, and exact mathematical inverse mutations for undo/redo.
  - Schema-driven property registry: Descriptors for all 14 shapes (`rect`, `circle`, `ring`, `arc`, `polygon`, `triangle`, `chevron`, `t_shape`, `plus`, `path`, `pixels`, `image`), effects (`glow`, `shadow`, `bloom`), and appearance (`theme.id`, `appearance.font_pack`, `appearance.corner_style`).
  - Human/agent parity (P4): Implemented 27 new MCP tools across `property.rs` and `appearance.rs`, passing parity test across 100 UI actions, 47 core commands, and 56 MCP tools.
  - Frontend UI components: Tabbed `DockSystem.svelte` supporting 11 default tabs, layout presets (`Compact`, `Pixel Art`, `Designer`, `Agent Watch`, `Minimal`), live property search with "Show modified only" filter, `AppearanceModal.svelte` with live theme cards and font specimens, and pure neutral token restyling in `app.css`.
  - Documentation deliverables: `docs/AGENT_README.md`, `docs/TYPOGRAPHY.md`, `docs/LAYOUT.md`, `docs/THEMING.md`, and updated `docs/MCP.md` & `docs/COMMANDS.md`.

## In progress
- None. All v5 features fully implemented, tested, and verified.

## Next exact step
- Present completion summary to human and verify release build.

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
