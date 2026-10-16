# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0](https://github.com/jeelnadaa/xhair/compare/xhair-v0.1.0...xhair-v0.2.0) (2026-10-16)


### Features

* add attached and headless mcp modes with schema export ([94cba7e](https://github.com/jeelnadaa/xhair/commit/94cba7e91c8c8ee23c88815c206165296998b775))
* **app:** add save profile button, fix overlay centering, and launch in native desktop window ([66d7a59](https://github.com/jeelnadaa/xhair/commit/66d7a59f8f6a2d7ab3947531c4d66d996424db3e))
* **app:** keep xhair resident with tray daemon and auto-serve UI editor ([cd11d44](https://github.com/jeelnadaa/xhair/commit/cd11d44cdc83cba7664e5453d6cdcd63907f2b36))
* bootstrap repository skeleton, agent continuity, and release pipeline ([739d5c7](https://github.com/jeelnadaa/xhair/commit/739d5c75d7c5f8914efd801e5a153b69fbc51d2e))
* **design:** complete Phase 0 design tokens, components, screens, and route map ([d99e3a2](https://github.com/jeelnadaa/xhair/commit/d99e3a277bfe872fe7b5504c5ba59814ffa69f6b))
* **design:** complete Phase 0 v5 design deliverables with 24 screens and neutral tokens ([cc58019](https://github.com/jeelnadaa/xhair/commit/cc580190706063c13734a914fe1cedddc26ea944))
* **dynamics,ui:** implement M9 spray lab, dynamic state machine, gun profiles, and compliance panel ([644a2d0](https://github.com/jeelnadaa/xhair/commit/644a2d0567e09604878725525f1e8fefcfdfb561))
* fix desktop blank screen, add expandable resizable panels, and de-cramp left rail ([82e6ce5](https://github.com/jeelnadaa/xhair/commit/82e6ce55f77d738167558d4b1fea52b7d08208d0))
* **foundations:** implement M1 domain model, geometry proptests, command bus, and descriptors ([311494e](https://github.com/jeelnadaa/xhair/commit/311494e098c7a716f52e5659700ef17c490936fa))
* **geometry,ui:** implement M8 calibration wizard, offline screenshot align, alignment diagnostics, and multi-res tests ([2214fe7](https://github.com/jeelnadaa/xhair/commit/2214fe7ee7b2c114009216ab34fd6416b6f35559))
* implement Design & Spec Update v2 and fix overlay toggle bug ([b529760](https://github.com/jeelnadaa/xhair/commit/b52976070f7bffeeea047dddec56999f1c0bc791))
* implement phase 2 unified backend and real-time mcp bridge ([a7b8071](https://github.com/jeelnadaa/xhair/commit/a7b8071f669312be23e7ca599701c3f8289b27aa))
* implement phase 3 world-class ui/ux studio redesign ([9134934](https://github.com/jeelnadaa/xhair/commit/91349344e8919cd3df2b7519db63c0b8399f2005))
* implement phase 4 production hardening and release packaging ([97077d9](https://github.com/jeelnadaa/xhair/commit/97077d9f5446e1a50371509aa8fc7f146fa92e63))
* implement production readiness phase 1 critical fixes and architecture docs ([0e006c1](https://github.com/jeelnadaa/xhair/commit/0e006c1f6b5205d5be712c8b92fc9d1c79c8c30e))
* **library:** implement M3 disk storage, index, share codes, and valorant import ([0064ccb](https://github.com/jeelnadaa/xhair/commit/0064ccbc50d6636d9c0a18edf95cffb59a3f1680))
* **mcp:** implement M6 MCP server with stdio, HTTP, tools, resources, and parity test ([eae29c7](https://github.com/jeelnadaa/xhair/commit/eae29c70957fa352a08ef752829f46b8a4454de6))
* **platform:** implement M4 Win32 layered overlay daemon, monitors, hotkeys, and tray ([0fc8ea3](https://github.com/jeelnadaa/xhair/commit/0fc8ea36fab5af42701ac093f6000f8e300a086d))
* **release:** complete M10 packaging, startup settings, crash handler, and documentation deliverables ([812f826](https://github.com/jeelnadaa/xhair/commit/812f826b8edd8bab75befaa2d21af79a10893472))
* **render:** implement M2 vector rasterizer, visual effects, wasm target, and cli render ([bb93dd5](https://github.com/jeelnadaa/xhair/commit/bb93dd5605f5fcd5da8ce39160911a1f8d0f8c3e))
* **render:** implement M7 global bloom, animation tracks, timeline dock, and performance benchmarks ([b43cd47](https://github.com/jeelnadaa/xhair/commit/b43cd47eb4d23fc0873dcb0989d96ff205d13c62))
* **ui:** add atomic 2D element movement, arm selectors, custom shape builder, and canvas drag ([1b54227](https://github.com/jeelnadaa/xhair/commit/1b542273cf846b1e2c2d9a7f38088f29106100bb))
* **ui:** complete feature matrix for shapes, effects, backgrounds, import/export, and resolution manager ([270531a](https://github.com/jeelnadaa/xhair/commit/270531a08af8725bf1a6fd7f8418766d8a11ea55))
* **ui:** implement M5 Svelte 5 editor with canvas grid, inspector, layers, and nudge pad ([a77b26f](https://github.com/jeelnadaa/xhair/commit/a77b26fe44684f3ff545a978853290c158ca571b))
* **v5:** complete design and spec update v5 (dock system, neutral theming, parity tools, docs) ([08daab9](https://github.com/jeelnadaa/xhair/commit/08daab9ef1855c8d8868781220aee5d2ec22d351))


### Bug Fixes

* **app:** add dev script to ui and suppress console flash when opening browser ([f7895f3](https://github.com/jeelnadaa/xhair/commit/f7895f3036ce1ce663d8b741aebe8ddc2aeb81d1))
* **app:** embed UI assets via rust-embed and keep native webview alive in event loop ([649dfa3](https://github.com/jeelnadaa/xhair/commit/649dfa3c17aba99b332b9d80b56d531d8f984f1b))
* **ci:** remove missing tauri.conf.json from release-please extra-files ([112eb57](https://github.com/jeelnadaa/xhair/commit/112eb5779623e1c1b501dd44f02af6ec76ce92e9))
* **ci:** specify cache-dependency-path for pnpm and fix clippy --all-targets ([98d8393](https://github.com/jeelnadaa/xhair/commit/98d839352a717978f3a72a6dec546fca32bb40b5))
* **ci:** synchronize pnpm lockfile with ui dependencies and configure release workflow token ([41520f5](https://github.com/jeelnadaa/xhair/commit/41520f58514b71b5ed0d1af1ec4da6985981d68d))
* correct tiny-skia radial arm transforms and sync UI to overlay daemon ([8b45ae8](https://github.com/jeelnadaa/xhair/commit/8b45ae8e418f5db1d934170e3c7712ccebc52b44))
* harden state against deleted crosshairs and add multi-tier storage reset ([918e56f](https://github.com/jeelnadaa/xhair/commit/918e56f2e6c81d1a8e7929f357afbb629a30aba3))
* **ui:** import missing ResolutionModal in App.svelte ([8e92825](https://github.com/jeelnadaa/xhair/commit/8e92825958cbd74928be8a57faa5f4d3f818028c))
* **ui:** resolve ResolutionModal runtime error in webview and document build/run paths ([1ad2fd6](https://github.com/jeelnadaa/xhair/commit/1ad2fd68069b8739df45374c290ab1954b7eda75))

## [0.1.0] - 2026-10-16

### Added
- **Foundations (M1):** Canonical data models in `xhair-model` (`Crosshair`, `Layer`, `LayerKind`, `Effects`, `Symmetry`), subpixel coordinate math in `xhair-geometry`, and command bus with linear undo/redo stacks in `xhair-commands`.
- **Renderer (M2):** Native tiny-skia CPU rasterizer with WebAssembly bindings (`xhair-render`), dirty-rect tracking, bloom and drop-shadow effects, and golden test validation suite.
- **Format & Sharing (M3):** Canonical `.xhair` and `.xhairpack` JSON schemas, Brotli-compressed base64 share codes (`XH1-...`) with CRC32 integrity verification, Valorant crosshair string parser, and atomic disk writes.
- **Library (M4):** Catalog management, 30-day trash retention, search and tag filtering, and quick-switch slots (1–9).
- **Windows Platform Daemon (M5):** Win32 transparent click-through layered window (`WS_EX_LAYERED | WS_EX_TRANSPARENT | WS_EX_NOACTIVATE`), per-monitor V2 DPI awareness, global hotkeys, foreground window tracking, Windows startup registry toggle, and local zero-telemetry crash reporting.
- **Editor UI (M6):** Hardware-accelerated Svelte 5 editor with real-time canvas grid, multi-layer stack management, property inspector, 0.5 px fine nudge pad, and quick-export modals.
- **Model Context Protocol Server (M7):** Full MCP server supporting 32 tools, 5 resources, and 5 prompts across stdio and authenticated loopback HTTP transports, with continuous 1:1 human/agent parity enforcement.
- **Polish & Effects (M8):** Procedural animations, glow filters, heavy render benchmarking, and offline screenshot alignment tool (`screenshot_align.rs`) with 1-click calibration.
- **Reticle Dynamics & Spray Lab (M9):** Neutral weapon profiles, synthetic input state machines, dynamic spread driver bindings, and standalone offline Spray Lab simulator.
- **Packaging & Hardening (M10):** Automated release pipeline (`xtask package`), Windows NSIS installer script, portable zip, CLI zip, CycloneDX SBOM, SHA256 checksums, and exhaustive documentation deliverables.
