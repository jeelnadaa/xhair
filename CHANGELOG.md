# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
