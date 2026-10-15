# XHAIR — High-Precision Crosshair Overlay & Editor

<div align="center">

**A low-resource Windows crosshair overlay with subpixel-accurate centering, a reactive Svelte 5 editor, and full Model Context Protocol (MCP) server automation.**

[![CI](https://github.com/jeelnadaa/xhair/actions/workflows/ci.yml/badge.svg)](https://github.com/jeelnadaa/xhair/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![RAM](https://img.shields.io/badge/idle%20RAM-%E2%89%A4%2020%20MB-brightgreen.svg)](#performance)
[![CPU](https://img.shields.io/badge/static%20CPU-0.00%25-brightgreen.svg)](#performance)

</div>

---

## Table of Contents
1. [Key Features](#key-features)
2. [Quick Start & Running Locally](#quick-start--running-locally)
3. [How to Use the UI (Editor Walkthrough)](#how-to-use-the-ui-editor-walkthrough)
4. [How to Connect MCP (AI Agent Automation)](#how-to-connect-mcp-ai-agent-automation)
5. [In-Game Overlay & Hotkeys](#in-game-overlay--hotkeys)
6. [CLI & Startup Settings](#cli--startup-settings)
7. [Architecture & Technical Docs](#architecture--technical-docs)
8. [Building from Source](#building-from-source)
9. [Anti-Cheat Compliance](#anti-cheat-compliance)

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

## Quick Start & Running Locally

### Option 1: Launch the Web UI Editor (Quickest Visual Access)
You can launch the Svelte 5 editor in your browser immediately using Vite:
```bash
# Navigate to the ui folder and start dev server
pnpm -C ui dev
```
Open **`http://localhost:5173`** in Chrome, Edge, or Firefox. The full editor is interactive with the canvas grid, layers panel, inspector, nudge pad, and Spray Lab!

### Option 2: Run the Compiled Windows Application
To run the native Windows application shell with system tray and layered overlay:
```bash
# Run the native app binary via Cargo
cargo run --bin xhair
```
Or from the packaged portable zip (`dist/xhair-0.1.0-x64-portable.zip`), extract and double-click **`xhair.exe`**.

---

## How to Use the UI (Editor Walkthrough)

The XHAIR interface is organized into modular panels designed for subpixel precision:

### 1. Canvas & Grid (Center Viewport)
- **Zoom & Pan:** Scroll the mouse wheel to zoom (`1×` to `32×`). Hold the middle mouse button (or Space + drag) to pan. Click `1:1` or `Fit` to reset.
- **Precision Visual Guides:**
  - High-contrast magenta lines (`#FF4FD8`) show the exact subpixel geometric center.
  - Concentric distance rings show radii in pixels.
  - Coordinate rulers on top and left reflect exact physical pixel dimensions.
- **Floating Tool Palette (`V`, `A`, `L`, `D`, `R`, `C`, `P`):**
  - `V` (Select): Click any layer on canvas to select it.
  - `L` (Line): Add crosshair arms (length, thickness, gap).
  - `D` (Dot): Add center dot (circle or square, radius, crisp snapping).
  - `R` (Ring): Add concentric reticle ring.
  - `C` (Circle): Add circular reticle outline.
  - `P` (Polygon): Add multi-point star or polygon shapes.

### 2. Layers Panel (Left Sidebar)
- **Layer Stack:** Displays all active shapes ordered by render z-index. Click and drag or use the up/down arrows to reorder.
- **Visibility & Solo:** Click the Eye icon to hide/show layers; click `S` to solo a layer during editing.
- **Alignment Diagnostics:** Each layer displays an alignment pill:
  - `ALIGNED` (Green): Layer lies exactly on pixel boundaries without anti-aliasing blur.
  - `OFF-GRID` (Amber): Layer falls across subpixel boundaries. Click the pill to apply a **1-click suggested auto-fix**.

### 3. Property Inspector (Right Sidebar)
- **Descriptor Sliders:** Drag horizontal scrub sliders to adjust geometry (Thickness, Length, Gap, Radius, Corner Rounding).
- **Color Picker & Opacity:** Full hex picker (`#RRGGBBAA`) and quick-select esports swatches (Cyan, Lime, White, Red, Magenta).
- **Effects & Symmetry:** Toggle radial symmetry (2-way, 4-way, 6-way, 8-way), bloom threshold, and glow intensity.
- **Dynamic Spread Scrubber:** Drag the simulated spread slider to test how reticle expansion bindings react to firing in real time.

### 4. 8-Way Nudge Pad (Precision Positioning)
Located in the lower-right corner:
- **Directional Buttons:** 8 arrows (`↖`, `↑`, `↗`, `←`, `•`, `→`, `↙`, `↓`, `↘`) shift elements with subpixel precision.
- **Target Toggle:** Choose whether nudges apply to the `Whole Crosshair`, `Selected Layer`, or a `Single Arm`.
- **Step Size Chips:** Select `0.1 px`, `0.25 px`, `0.5 px`, `1.0 px`, `5.0 px`, or `10.0 px`.
- **Keyboard Shortcuts:** Use `Arrow` keys for standard nudges, `Shift + Arrow` for $5\times$, and `Ctrl + Arrow` for subpixel ($0.5\text{ px}$).

### 5. Calibration Wizard & Offline Screenshot Align
Access via the **Calibrate** button in the TopBar or `Ctrl + Shift + C`:
1. **Resolution Profile:** Select your active monitor resolution (1080p, 1440p, 4K, or 4:3 stretched).
2. **Screenshot Alignment Tool:** Drop a static in-game screenshot (e.g. weapon ADS or static target).
3. **Centroid Auto-Detection:** The bilateral symmetry algorithm detects the weapon reticle center and calculates the exact subpixel offset $(\Delta x, \Delta y)$. Click **Apply Calibration** to store the offset permanently.

### 6. Standalone Spray Lab
Access via the **Spray Lab** button in the TopBar:
- **Offline Recoil Visualizer:** Safe training environment executing purely on synthetic input sources.
- **Shot Scrubber:** Scrub through a 30-round spray pattern frame-by-frame.
- **Error Discs:** View $1\sigma$ and $2\sigma$ dispersion circles and export high-resolution trajectory PNGs.

### 7. Exporting & Sharing
- **Brotli Share Code:** Click **Share** in the TopBar to copy a compact `XH1-...` code to clipboard.
- **Importing:** Click **Import** and paste either:
  - An `XH1-...` XHAIR code, or
  - An in-game Valorant crosshair string (e.g. `0;P;c;5;o;1;d;1;z;3;...`).

---

## How to Connect MCP (AI Agent Automation)

XHAIR features a native **Model Context Protocol (MCP)** server providing 32 tools, 5 resources, and 5 prompt templates. AI assistants can design crosshairs, check contrast ratios against game backgrounds, adjust subpixel nudges, and manage profiles autonomously with **100% human/agent parity**.

### 1. Connecting via Claude Desktop
Edit your Claude Desktop configuration file at `%APPDATA%\Claude\claude_desktop_config.json`:
```json
{
  "mcpServers": {
    "xhair": {
      "command": "d:\\cross-aim\\target\\debug\\xhair-cli.exe",
      "args": ["mcp"]
    }
  }
}
```
*(If using the packaged release, change the path to `C:\\Program Files\\xhair\\xhair-cli.exe` or your portable folder path).*

### 2. Connecting via Cursor IDE
1. Open Cursor Settings $\to$ **Features** $\to$ **MCP Servers**.
2. Click **+ Add New MCP Server**.
3. Fill in:
   - **Name:** `xhair`
   - **Type:** `command`
   - **Command:** `d:\cross-aim\target\debug\xhair-cli.exe mcp`
4. The green indicator will confirm connection with 32 registered tools.

### 3. Connecting via Antigravity IDE / VS Code
Add to your workspace `.gemini/config/mcp_config.json` or VS Code MCP configuration:
```json
{
  "mcpServers": {
    "xhair": {
      "command": "d:\\cross-aim\\target\\debug\\xhair-cli.exe",
      "args": ["mcp"],
      "env": {
        "RUST_LOG": "info"
      }
    }
  }
}
```

### 4. Loopback HTTP Transport (Multi-Instance Proxy)
When `xhair.exe` is running, secondary CLI or stdio invocations automatically proxy JSON-RPC calls over authenticated loopback HTTP (`127.0.0.1`) via discovery of `%APPDATA%/xhair/xhair-endpoint.json`. The server enforces Bearer authentication and strict DNS-rebinding protection.

### 5. Example Prompts to Ask Your AI Agent
Once connected, try asking your assistant:
- *"Design a clean cyan 1px dot with a 1px black outline for 1080p."*
- *"Import this Valorant crosshair string: `0;P;c;5;o;1;d;1;z;3;0t;1;0l;4;0o;2;0a;1` and optimize its contrast for dark maps."*
- *"Check the alignment of my active crosshair and nudge any off-grid layers to integer boundaries."*
- *"Create a 6-arm radial crosshair with a subtle magenta glow."*
- *"Simulate a 30-round spray pattern using the Rifle template and report the maximum spread radius."*

---

## In-Game Overlay & Hotkeys

The overlay is a hardware-composited Windows layered window that floats transparently over your game:

| Hotkey | Action |
|---|---|
| `Ctrl + Shift + X` | **Toggle Overlay:** Show or hide crosshair overlay instantly mid-game. |
| `Ctrl + Shift + 1` .. `9` | **Quick Slot Switch:** Switch between 9 pre-configured crosshair slots. |
| `Ctrl + Shift + C` | **Recalibrate Center:** Launch calibration wizard or reset subpixel offsets. |
| `Ctrl + K` | **Command Palette:** Open spotlight quick-actions dialog in the editor. |

*Note: Hotkeys can be customized in Settings $\to$ Keybindings.*

---

## CLI & Startup Settings

The standalone `xhair-cli.exe` binary provides terminal utilities:

```bash
# Start the Model Context Protocol stdio server
xhair-cli mcp

# Render a crosshair document to a PNG file offline
xhair-cli render input.xhair --output preview.png --width 256 --height 256

# Import a Valorant string and save as .xhair JSON
xhair-cli import "0;P;c;5;o;1;d;1;z;3" --output my_reticle.xhair

# Query Windows startup status
xhair-cli startup --status

# Enable running xhair minimized on Windows user login (HKCU Run key)
xhair-cli startup --enable

# Disable running at Windows startup
xhair-cli startup --disable
```

---

## Architecture & Technical Docs

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
