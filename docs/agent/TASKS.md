# TASKS — Implementation Task Matrix

| ID | Milestone | Title | Status | Spec Ref | Notes |
|---|---|---|---|---|---|
| **P-1-001** | Phase -1 | Create repository skeleton and workspace crates | done | §19 | Complete |
| **P-1-002** | Phase -1 | Copy specs to `docs/spec/` as read-only references | done | §19 | Complete |
| **P-1-003** | Phase -1 | Set up agent continuity files and pointer files | done | §20 | Complete |
| **P-1-004** | Phase -1 | Configure release pipeline (release-please, workflows) | done | §21 | Complete |
| **P-1-005** | Phase -1 | Set up project memory graph and `cargo xtask` | done | §22 | Complete |
| **P-1-006** | Phase -1 | Commit and verify baseline CI/tests green | done | §19 | Complete |
| **M0-001** | M0 Design | Review FigCraft tools and generate tokens.json | done | §17.1 | Complete |
| **M0-002** | M0 Design | Design component library with all states | done | §17.3 | Complete |
| **M0-003** | M0 Design | Design 25 application screens and flows | done | §17.4 | Complete |
| **M0-004** | M0 Design | Export screens & components to `design/` | done | §17.1 | Complete |
| **M0-005** | M0 Design | Complete `design/README.md` route & command map | done | §17.1 | Complete |
| **M0-006** | M0 Design | Present design summary and await human approval | doing | §17.5 | Awaiting human approval |
| **M1-001** | M1 Found. | Implement `xhair-model` domain types & validation | todo | §4 | |
| **M1-002** | M1 Found. | Implement `xhair-geometry` coordinate & alignment math | todo | §6 | |
| **M1-003** | M1 Found. | Implement `xhair-commands` command bus & undo/redo | todo | §5.1 | |
| **M1-004** | M1 Found. | Implement property descriptors macro and JSON export | todo | §5.2 | |
| **M1-005** | M1 Found. | Settings model and error types catalog | todo | §14, §15.1 | |
| **M1-006** | M1 Found. | Foundations property and unit test suites | todo | §16 (M1) | |
| **M2-001** | M2 Render | Implement `xhair-render` tiny-skia shape rasterizers | todo | §7 | |
| **M2-002** | M2 Render | Implement paints, gradients, and strokes | todo | §4.4, §4.5 | |
| **M2-003** | M2 Render | Implement effects (blur, glow, shadow) | todo | §4.6 | |
| **M2-004** | M2 Render | Compile renderer to WASM target | todo | §7.1 | |
| **M2-005** | M2 Render | Golden image tests (native vs wasm parity) | todo | §7.1 | |
| **M2-006** | M2 Render | Implement `xhair-cli render` command | todo | §16 (M2) | |
| **M3-001** | M3 Library | Implement disk storage, atomic writes, index.json | todo | §10.1 | |
| **M3-002** | M3 Library | Implement folders, tags, favorites, and slots | todo | §10.2, §10.3 | |
| **M3-003** | M3 Library | Implement `.xhair` and `.xhairpack` serialization | todo | §11.1 | |
| **M3-004** | M3 Library | Implement Brotli share codes and CRC32 checks | todo | §11.1 | |
| **M3-005** | M3 Library | Implement Valorant code import parser | todo | §11.2 | |
| **M3-006** | M3 Library | Round-trip tests and cargo-fuzz fuzzers | todo | §11.2 | |
| **M4-001** | M4 Overlay | Win32 transparent click-through layered window | todo | §8.1 | |
| **M4-002** | M4 Overlay | DPI awareness and multi-monitor handling | todo | §6.5 | |
| **M4-003** | M4 Overlay | Foreground game process detection and follow rect | todo | §8.1, §8.2 | |
| **M4-004** | M4 Overlay | Global hotkey registration and conflict checks | todo | §8.4 | |
| **M4-005** | M4 Overlay | System tray icon, context menu, and tooltip states | todo | §8.3 | |
| **M4-006** | M4 Overlay | Overlay calibration test pattern overlay | todo | §6.7 | |
| **M5-001** | M5 UI | Setup Svelte 5 + Vite + Tauri 2 shell | todo | §3.1 | |
| **M5-002** | M5 UI | High-performance canvas grid view with DPR scaling | todo | §6.8 | |
| **M5-003** | M5 UI | Descriptor-driven inspector controls and tooltips | todo | §5.2, §5.3 | |
| **M5-004** | M5 UI | Layers panel, ordering, solo/hide/lock | todo | §9.3 | |
| **M5-005** | M5 UI | 8-way Nudge Pad with step chips & frame selector | todo | §9.4 | |
| **M5-006** | M5 UI | Color picker, gradient editor, and presets | todo | §9.5 | |
| **M5-007** | M5 UI | Command palette and shortcut registry | todo | §9.8 | |
| **M5-008** | M5 UI | Playwright and Vitest UI test suite | todo | §15.3 | |
| **M6-001** | M6 MCP | MCP server generated from command registry | todo | §13.1 | |
| **M6-002** | M6 MCP | stdio and loopback HTTP transports with auth token | todo | §13.2, §13.7 | |
| **M6-003** | M6 MCP | Proxy-to-running-app IPC mechanism | todo | §1.4, §13.2 | |
| **M6-004** | M6 MCP | Implement MCP resources and prompt templates | todo | §13.5, §13.6 | |
| **M6-005** | M6 MCP | MCP live activity log and UI editing banner | todo | §13.7 | |
| **M6-006** | M6 MCP | Parity CI test between UI, commands, and tools | todo | §13.8 | |
| **M7-001** | M7 Polish | Global bloom and advanced glow compositing | todo | §4.6 | |
| **M7-002** | M7 Polish | Animation tracks, keyframes, and timeline dock | todo | §4.7 | |
| **M7-003** | M7 Polish | Static-only lock and reduced-motion enforcement | todo | §4.7 | |
| **M7-004** | M7 Polish | Heavy crosshair performance optimization (<8ms) | todo | §7.4 | |
| **M8-001** | M8 Calib | 4-step Calibration Wizard dialog | todo | §6.7 | |
| **M8-002** | M8 Calib | Screenshot Align tool (local file symmetry finder) | todo | §6.7 | |
| **M8-003** | M8 Calib | Alignment issue report and automatic 1-click fixes | todo | §6.2 | |
| **M8-004** | M8 Calib | Verification on native and stretched resolutions | todo | §16 (M8) | |
| **M9-001** | M9 Dynamic | Spray Lab standalone simulation window | todo | §12.6 | |
| **M9-002** | M9 Dynamic | Dynamic reticle state machine and input simulation | todo | §12.1, §12.3 | |
| **M9-003** | M9 Dynamic | Gun profile editor with custom spread curves | todo | §12.4 | |
| **M9-004** | M9 Dynamic | Property binding editor with scrubber preview | todo | §12.5 | |
| **M9-005** | M9 Dynamic | Raw Input restricted capture with session opt-in | todo | §2, §12.2 | |
| **M9-006** | M9 Dynamic | Transparency & Compliance panel API listing | todo | §2 | |
| **M10-001** | M10 Release | NSIS installer script and portable zip bundling | todo | §21.3 | |
| **M10-002** | M10 Release | Start with Windows and start minimized settings | todo | §14 | |
| **M10-003** | M10 Release | Crash handler with local reports (no telemetry) | todo | §16 (M10) | |
| **M10-004** | M10 Release | Complete documentation deliverables (§15.2) | todo | §15.2 | |
| **M10-005** | M10 Release | Memory & performance budget verification | todo | §15.3 | |
| **M10-006** | M10 Release | Clean-machine install verification and final release | todo | §18 | |
