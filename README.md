# XHAIR — High-Precision Crosshair Overlay & Editor

A low-resource Windows crosshair overlay with pixel-accurate centering at any game resolution, a reactive editor, and full Model Context Protocol (MCP) server automation.

## Key Features
- **Overlay Only**: Zero game process memory reads, DLL injection, or screen capture. 100% compliant overlay.
- **Low RAM**: Idle overlay target $\le$ 20 MB private working set, zero CPU when static.
- **Human & Agent Parity**: All editor actions are command-bus driven and exposed 1:1 via MCP tools.
- **Pixel Accuracy**: Comprehensive coordinate math accounting for even/odd resolutions, DPI scaling, and custom stretch modes.

## Architecture
See `docs/ARCHITECTURE.md` and `docs/spec/` for detailed architectural specifications.
