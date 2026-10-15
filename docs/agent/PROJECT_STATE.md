# Project State

- **Updated:** 2026-10-16
- **Session:** 14
- **Phase / Milestone:** Native Desktop Shell (tao+wry), Profile Saving & Overlay Centering Verification
- **Version released:** 0.1.0 | **Release Artifacts:** Verified in `dist/`

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
- [x] Universal Feature Parity: All 15 shape primitives, visual effects (glow, shadow, blur), tactical preview backgrounds (5 presets), dynamic WCAG contrast scoring, universal import/export modal (.xhair, Brotli XH1, Valorant), resolution profiles manager

## In progress
- None. Entire specification (§1–§22) successfully audited, implemented, verified, packaged, and documented with zero missing features.

## Next exact step
- Human review and project release / deployment.

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
- Workspace crates: 12 (all build & test green, 84 tests passing)
- Golden image tests: 8 tests passing
- Geometry tests: 11 tests passing (symmetry detection, quarter-pixel quantization, multi-res verification)
- Render & effect tests: 12 unit tests passing (bloom, animation evaluation, static lock, heavy budget)
- Performance benchmarks: typical crosshair 2.91 µs (< 1000 µs budget), heavy crosshair 5470.72 µs (< 8000 µs budget)
- Platform tests: 12 tests passing (overlay styles, crash reports, startup registry, hotkeys, foreground detection)
- MCP tests & Parity: 7 tests passing (stdio, HTTP, auth, prompts, resources, parity)
- UI test suite: 22 tests passing in Vitest (100% green)
- TypeScript lint: 0 errors (`tsc --noEmit`)
- Vite production build: 201 kB JS, 52 kB CSS (built in ~600ms)
- Clippy & rustfmt: Clean, 0 warnings with `-D warnings`
- Agent continuity & memory graph: 100% validated (`cargo xtask check-agent-state`, `cargo xtask graph query --summary`)

## Human decisions pending
- None.
