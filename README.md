# XHAIR — High-Precision Crosshair Overlay & Editor

<div align="center">

**A low-resource Windows crosshair overlay with subpixel-accurate centering, a reactive Svelte 5 editor, and full Model Context Protocol (MCP) server automation.**

[![CI](https://github.com/jeelnadaa/xhair/actions/workflows/ci.yml/badge.svg)](https://github.com/jeelnadaa/xhair/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![RAM](https://img.shields.io/badge/idle%20RAM-%E2%89%A4%2020%20MB-brightgreen.svg)](#performance)
[![CPU](https://img.shields.io/badge/static%20CPU-0.00%25-brightgreen.svg)](#performance)

</div>

---

## Key Features

- **100% Anti-Cheat Compliant Overlay:** Operates purely as a transparent Windows layered window (`WS_EX_LAYERED | WS_EX_TRANSPARENT | WS_EX_NOACTIVATE`). **Zero** game process memory access, **zero** DLL injection, and **zero** screen capture.
- **Microscopic Resource Consumption:** Consumes **$0.00\%$ CPU** when static (event-driven rendering via Win32 message loop). Idle private working set is budgeted under **20 MB**.
- **Pixel-Accurate Coordinate Math:** Mathematical centering accounting for even/odd monitor dimensions, subpixel anti-aliasing snapping, custom stretch modes (4:3, 16:10, 5:4), and per-monitor DPI scaling (100%–500%).
- **Reactive Svelte 5 Editor:** Hardware-accelerated canvas editor with real-time preview, multi-layer stacking, nudge pad (0.5 px fine positioning), symmetry generators, and color adjustments.
- **Model Context Protocol (MCP) Server:** 32 JSON-RPC tools and 5 prompt templates allowing AI coding assistants (Claude Desktop, Cursor, Antigravity IDE) to design, tune, calibrate, and manage crosshairs with complete human/agent parity.
- **Offline Spray Lab & Dynamics:** Safe recoil trajectory simulator with synthetic input sources, neutral gun templates, and dynamic spread driver bindings.
- **Crosshair Sharing & Valorant Import:** Compact Brotli-compressed share codes (`XH1-...`) with CRC32 integrity verification, plus 1-click import of official Valorant profile strings.

---

## Quick Start

### 1. Download & Installation

Grab the latest release from the [Releases](https://github.com/jeelnadaa/xhair/releases) page:
- **`xhair-setup-<version>.exe`**: Windows NSIS setup wizard with start menu and desktop shortcuts.
- **`xhair-<version>-x64-portable.zip`**: Standalone zero-install portable folder. Unpack and double-click `xhair.exe`.
- **`xhair-cli-<version>-x64.zip`**: Standalone command-line binary for offline rendering, format conversion, and MCP stdio.

### 2. Running the Editor
Launch `xhair.exe` to open the GUI editor. Design your crosshair, adjust shapes and strokes, and assign it to an active quick-switch slot (1–9).

### 3. Global Hotkeys
- `Ctrl + Shift + X`: Toggle overlay visibility on/off.
- `Ctrl + Shift + 1` .. `9`: Switch active crosshair profile instantly mid-game.
- `Ctrl + Shift + C`: Recalibrate subpixel centering offset.

---

## Using with AI Agents (MCP)

XHAIR ships with a Model Context Protocol server exposing all crosshair features.

### Claude Desktop Configuration
Add to `%APPDATA%\Claude\claude_desktop_config.json`:
```json
{
  "mcpServers": {
    "xhair": {
      "command": "C:\\Program Files\\xhair\\xhair-cli.exe",
      "args": ["mcp"]
    }
  }
}
```

### Cursor & Antigravity IDE
Configure the MCP server to point to `xhair-cli.exe mcp` over `stdio`. Agents can list crosshairs, generate symmetric reticles, calculate contrast ratios against game backdrops, and adjust subpixel nudges autonomously.

---

## Architecture & Documentation

Full architectural specifications and worked technical examples:

- [ARCHITECTURE.md](docs/ARCHITECTURE.md) — Architectural diagrams, crate responsibilities, and data flows.
- [GEOMETRY.md](docs/GEOMETRY.md) — Coordinate systems, even/odd parity math, aspect ratio stretch, and DPI scaling.
- [FORMAT.md](docs/FORMAT.md) — `.xhair`, `.xhairpack`, Brotli `XH1-...` share codes, and Valorant import syntax.
- [MCP.md](docs/MCP.md) — Complete 32-tool MCP catalog, schemas, resources, and prompt templates.
- [PROPERTIES.md](docs/PROPERTIES.md) — Auto-generated property catalog and tooltip descriptions.
- [COMMANDS.md](docs/COMMANDS.md) — Command bus registry, undo/redo mechanics, and patch diffs.
- [COMPLIANCE.md](docs/COMPLIANCE.md) — Transparency audit of all Win32 APIs and proof of anti-cheat safety.
- [DYNAMICS.md](docs/DYNAMICS.md) — Reticle dynamics, spread accumulation formulas, and Spray Lab.
- [PERFORMANCE.md](docs/PERFORMANCE.md) — Memory and CPU budgets, benchmark measurements, and ETW profiles.
- [TESTING.md](docs/TESTING.md) — Testing matrix and instructions for running unit, golden, and parity tests.
- [CONTRIBUTING.md](docs/CONTRIBUTING.md) — Contributing guidelines, commit conventions, and "Add a new shape in 5 steps".
- [ERRORS.md](docs/ERRORS.md) — Comprehensive error catalog (`XH-1xxx` to `XH-9xxx`) with remediation hints.

---

## Building from Source

### Prerequisites
- Windows 10 or 11 (x64)
- Rust toolchain $\ge 1.80$ (`rustup default stable`)
- Node.js $\ge 20$ and `pnpm` $\ge 9$

### Build Commands
```bash
# Clone the repository
git clone https://github.com/jeelnadaa/xhair.git
cd xhair

# Build all Rust crates
cargo build --workspace

# Build the Svelte 5 frontend
pnpm -C ui install
pnpm -C ui build

# Run all tests
cargo test --workspace
pnpm -C ui test

# Package release artifacts (portable zip, CLI, installer script, SBOM, checksums)
cargo run --package xtask -- package --version 0.1.0
```

---

## Anti-Cheat Compliance

XHAIR is strictly an overlay. It operates at the Desktop Window Manager layer and never touches or reads game memory. See [docs/COMPLIANCE.md](docs/COMPLIANCE.md) for the exhaustive list of Win32 APIs used and forbidden API audits.

---

## License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
