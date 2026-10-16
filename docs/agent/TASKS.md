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
| **M0-006** | M0 Design | Present design summary and await human approval | done | §17.5 | Approved by user |
| **M1-001** | M1 Found. | Implement `xhair-model` domain types & validation | done | §4 | Complete |
| **M1-002** | M1 Found. | Implement `xhair-geometry` coordinate & alignment math | done | §6 | Complete |
| **M1-003** | M1 Found. | Implement `xhair-commands` command bus & undo/redo | done | §5.1 | Complete |
| **M1-004** | M1 Found. | Implement property descriptors macro and JSON export | done | §5.2 | Complete |
| **M1-005** | M1 Found. | Settings model and error types catalog | done | §14, §15.1 | Complete |
| **M1-006** | M1 Found. | Foundations property and unit test suites | done | §16 (M1) | Complete |
| **M2-001** | M2 Render | Implement `xhair-render` tiny-skia shape rasterizers | done | §7 | Complete |
| **M2-002** | M2 Render | Implement paints, gradients, and strokes | done | §4.4, §4.5 | Complete |
| **M2-003** | M2 Render | Implement effects (blur, glow, shadow) | done | §4.6 | Complete |
| **M2-004** | M2 Render | Compile renderer to WASM target | done | §7.1 | Complete |
| **M2-005** | M2 Render | Golden image tests (native vs wasm parity) | done | §7.1 | Complete |
| **M2-006** | M2 Render | Implement `xhair-cli render` command | done | §16 (M2) | Complete |
| **M3-001** | M3 Library | Implement disk storage, atomic writes, index.json | done | §10.1 | Complete |
| **M3-002** | M3 Library | Implement folders, tags, favorites, and slots | done | §10.2, §10.3 | Complete |
| **M3-003** | M3 Library | Implement `.xhair` and `.xhairpack` serialization | done | §11.1 | Complete |
| **M3-004** | M3 Library | Implement Brotli share codes and CRC32 checks | done | §11.1 | Complete |
| **M3-005** | M3 Library | Implement Valorant code import parser | done | §11.2 | Complete |
| **M3-006** | M3 Library | Round-trip tests and cargo-fuzz fuzzers | done | §11.2 | Complete |
| **M4-001** | M4 Overlay | Win32 transparent click-through layered window | done | §8.1 | Complete |
| **M4-002** | M4 Overlay | DPI awareness and multi-monitor handling | done | §6.5 | Complete |
| **M4-003** | M4 Overlay | Foreground game process detection and follow rect | done | §8.1, §8.2 | Complete |
| **M4-004** | M4 Overlay | Global hotkey registration and conflict checks | done | §8.4 | Complete |
| **M4-005** | M4 Overlay | System tray icon, context menu, and tooltip states | done | §8.3 | Complete |
| **M4-006** | M4 Overlay | Overlay calibration test pattern overlay | done | §6.7 | Complete |
| **M5-001** | M5 UI | Setup Svelte 5 + Vite + Tauri 2 shell | done | §3.1 | Complete |
| **M5-002** | M5 UI | High-performance canvas grid view with DPR scaling | done | §6.8 | Complete |
| **M5-003** | M5 UI | Descriptor-driven inspector controls and tooltips | done | §5.2, §5.3 | Complete |
| **M5-004** | M5 UI | Layers panel, ordering, solo/hide/lock | done | §9.3 | Complete |
| **M5-005** | M5 UI | 8-way Nudge Pad with step chips & frame selector | done | §9.4 | Complete |
| **M5-006** | M5 UI | Color picker, gradient editor, and presets | done | §9.5 | Complete |
| **M5-007** | M5 UI | Command palette and shortcut registry | done | §9.8 | Complete |
| **M5-008** | M5 UI | Playwright and Vitest UI test suite | done | §15.3 | Complete |
| **M6-001** | M6 MCP | MCP server generated from command registry | done | §13.1 | Complete (31 tools registered & dispatched) |
| **M6-002** | M6 MCP | stdio and loopback HTTP transports with auth token | done | §13.2, §13.7 | Complete (Bearer auth + DNS rebinding guard) |
| **M6-003** | M6 MCP | Proxy-to-running-app IPC mechanism | done | §1.4, §13.2 | Complete (endpoint discovery & forwarding) |
| **M6-004** | M6 MCP | Implement MCP resources and prompt templates | done | §13.5, §13.6 | Complete (5 resources, 5 prompt templates) |
| **M6-005** | M6 MCP | MCP live activity log and UI editing banner | done | §13.7 | Complete (McpModal drawer & pulsing TopBar badge) |
| **M6-006** | M6 MCP | Parity CI test between UI, commands, and tools | done | §13.8 | Complete (parity.rs verified in CI) |
| **M7-001** | M7 Polish | Global bloom and advanced glow compositing | done | §4.6 | Luminance threshold + 3-pass gaussian + additive composite |
| **M7-002** | M7 Polish | Animation tracks, keyframes, and timeline dock | done | §4.7 | Procedural tracks, evaluation engine, UI TimelineDock |
| **M7-003** | M7 Polish | Static-only lock and reduced-motion enforcement | done | §4.7 | Enforced in RenderContext, state store, and UI lock |
| **M7-004** | M7 Polish | Heavy crosshair performance optimization (<8ms) | done | §7.4 | Bounded scratch buffers, SIMD release bench passes 5.4ms < 8ms |
| **M8-001** | M8 Calib | 4-step Calibration Wizard dialog | done | §6.7 | CalibrationWizard.svelte modal, test pattern & profile binding |
| **M8-002** | M8 Calib | Screenshot Align tool (local file symmetry finder) | done | §6.7 | Offline bilateral symmetry optimizer in xhair-geometry & UI dropzone |
| **M8-003** | M8 Calib | Alignment issue report and automatic 1-click fixes | done | §6.2 | alignment_report for Line/Dot/Rect/Circle & apply_suggested_fix |
| **M8-004** | M8 Calib | Verification on native and stretched resolutions | done | §16 (M8) | Multi-resolution tests pass for 1080p, 1440p, 4:3 stretched, and odd res |
| **M9-001** | M9 Dynamic | Spray Lab standalone simulation window | done | §12.6 | SprayLab.svelte standalone simulation window, shot scrubber, dynamic reticle response, PNG export |
| **M9-002** | M9 Dynamic | Dynamic reticle state machine and input simulation | done | §12.1, §12.3 | SimulationEngine, PlayerMovementState, DynamicDrivers, temporal smoothing in xhair-dynamics |
| **M9-003** | M9 Dynamic | Gun profile editor with custom spread curves | done | §12.4 | GunProfile, SpreadConfig, templates (Rifle, SMG, Pistol) with neutral disclaimer |
| **M9-004** | M9 Dynamic | Property binding editor with scrubber preview | done | §12.5 | DynamicBinding, apply_bindings, live simulatedSpread scrubber in Inspector & CanvasGrid |
| **M9-005** | M9 Dynamic | Raw Input restricted capture with session opt-in | done | §2, §12.2 | Restricted InputState, DynamicOptInModal session confirmation and risk advisory |
| **M9-006** | M9 Dynamic | Transparency & Compliance panel API listing | done | §2 | CompliancePanel.svelte exhaustive audit of permitted and prohibited OS APIs |
| **M10-001** | M10 Release | NSIS installer script and portable zip bundling | done | §21.3 | xtask package generates NSIS script, portable zip, CLI zip, SBOM, SHA256SUMS |
| **M10-002** | M10 Release | Start with Windows and start minimized settings | done | §14 | HKCU Run registry helper in xhair-platform-win::startup and CLI commands |
| **M10-003** | M10 Release | Crash handler with local reports (no telemetry) | done | §16 (M10) | Local panic hook writes diagnostics to %APPDATA%/xhair/crashes/ with zero telemetry |
| **M10-004** | M10 Release | Complete documentation deliverables (§15.2) | done | §15.2 | All 14 docs created/updated including worked 5-step example in CONTRIBUTING.md |
| **M10-005** | M10 Release | Memory & performance budget verification | done | §15.3 | Enforced in tests, zero-CPU static loop, memory gate, and criterion benches |
| **M10-006** | M10 Release | Clean-machine install verification and final release | done | §18 | Packaging validated, release artifacts verified, SBOM and checksums generated |
| **V2-001** | Spec v2 | Phase 0 Design Update & Tokens | done | v2 §12 | 12 screens rendered in design/screens/, tokens.json updated to monochromatic gray ramp |
| **V2-002** | Spec v2 | Command Registry & Bus Inverse Mutations | done | v2 §1, §4–§7 | Layer/Crosshair/Group rename, groups, position/recenter, paint, selection commands |
| **V2-003** | Spec v2 | MCP Tools Suite & Parity Test | done | v2 §0, §10 | 30 v2 tools across edit/library, 100% human/agent parity test verified |
| **V2-004** | Spec v2 | Overlay Window Toggle & IPC Bridge | done | Bug Fix | Win32 SW_HIDE / SW_SHOWNOACTIVATE, Ctrl+Shift+X hotkey, wry IPC toggle handler |
| **V2-005** | Spec v2 | Frontend UI Components & Runes State | done | v2 §1–§9 | Multi-select, NudgePad 8-dir+chips, Layer groups, ColorPicker, Group modal, Shortcuts dialog |
| **V5-001** | Spec v5 | Phase 0 Design: 24 screens, pure neutral tokens & zero pills | done | v5 §0, §12, §17 | 24 screens at 1x & 2x in design/screens/, tokens.json updated to 3 neutral ramps, 9 themes, 6 font packs, radius <= 10px |
| **V5-002** | Spec v5 | Command Registry & Bus: 23 new v5 commands with pure inverses | done | v5 §1–§10 | PropertySet, PropertySetMany, PropertyReset, PropertyLink, Style*, Randomize, ThemeSet, AppearanceSet, UiPanel*, UiLayout* |
| **V5-003** | Spec v5 | Schema-Driven Descriptors & Properties Catalog | done | v5 §4, §9 | Descriptors for all 14 shapes, visual effects, and appearance; exported properties.json & PROPERTIES.md |
| **V5-004** | Spec v5 | MCP Tools Suite & Parity Test (P4) | done | v5 §3, §14 | 27 new MCP tools, 100 UI actions synced, 100% parity test verified |
| **V5-005** | Spec v5 | Tabbed Dock System & Layout Presets | done | v5 §10 | DockSystem.svelte with 11 default tabs, layout presets (Designer, Compact, Pixel Art, Agent Watch, Minimal), maximize, property search |
| **V5-006** | Spec v5 | Appearance Modal, Neutral Themes & Styling | done | v5 §12, §13 | AppearanceModal.svelte with live theme cards & font specimens, app.css neutral styling |
| **V5-007** | Spec v5 | Complete Documentation Deliverables | done | v5 §14.2 | AGENT_README.md, TYPOGRAPHY.md, LAYOUT.md, THEMING.md, MCP.md, COMMANDS.md |
| **PR-001** | Phase 1 | Overlay Calibration Offset Wiring | done | PR §1.1 | Added compute_overlay_window_rect applying calibration_offset.x/y to Win32 window positioning |
| **PR-002** | Phase 1 | Symmetrical Bounding Box & Zero-Clipping | done | PR §1.2 | Symmetrical bounds buffer allocation around (0,0) with 16px conservative padding |
| **PR-003** | Phase 1 | Canvas Mouse Drag & Marquee State Separation | done | PR §1.3 | Layer hit-testing separating layer drag from marquee selection with 120fps redraw |
| **PR-004** | Phase 1 | Tool Palette Mapping & Shape Creation Fixes | done | PR §1.4 | Tool 'R' inserts Rect instead of TShape, tool 'B' adds polygon |
| **PR-005** | Phase 1 | Color Picker Undo Coalescing | done | PR §1.5 | Single undo snapshot on drag start, continuous live color preview, sync on release |
| **PR-006** | Phase 2 | Embedded Loopback MCP Server in App | done | PR §2.1 | Loopback 127.0.0.1:0, 256-bit token, %TEMP%/xhair-endpoint.json lifecycle & cleanup |
| **PR-007** | Phase 2 | Dual-Mode Stdio Proxy in CLI | done | PR §2.2 | Stdio proxying to running app instance; fallback to headless LibraryEngine |
| **PR-008** | Phase 2 | Physical Win32 Overlay Controls for MCP | done | PR §2.3 | OverlayController trait wired to OverlaySender for overlay, calibration, display tools |
| **PR-009** | Phase 2 | Disk Storage Integration (xhair-library) | done | PR §2.4 | LibraryEngine in app, REST endpoints /api/library/*, UI syncs with disk |
| **PR-010** | Phase 2 | Real-Time UI Event Streaming for MCP | done | PR §2.5 | /api/mcp/activity polling, agent editing banner in TopBar & McpModal activity log |
| **PR-011** | Phase 3 | 3-Column Studio Layout Architecture | done | PR §3.1 | 260px LeftRail, CenterStage canvas, 320px Inspector, responsive 1024x640 to 4K |
| **PR-012** | Phase 3 | Unified Contextual Inspector | done | PR §3.2 | Collapsible accordions for Transform, Geometry, Fill/Stroke, Effects, Presets |
| **PR-013** | Phase 3 | Figma-Style Drag-to-Scrub Numerical Inputs | done | PR §3.3 | ScrubInput.svelte with horizontal label drag, Alt=0.1x, Shift=10x, direct type |
| **PR-014** | Phase 3 | Clean In-Place Modal & Group Creation | done | PR §3.4 | Inline group creation & renaming (no window.prompt), desktop context menus |
| **PR-015** | Phase 3 | Collapsible Diagnostics & Labs Bottom Drawer | done | PR §3.5 | BottomDock.svelte with Timeline, Spray Lab, Calibration, and MCP Live Stream |
| **PR-016** | Phase 4 | Full Workspace Test Suite Verification | done | PR §4.1 | 90 Rust tests + Vitest 24 tests + parity test + clippy + fmt + tsc 100% green |
| **PR-017** | Phase 4 | End-to-End Integration Tests | done | PR §4.2 | MCP mutation -> library -> overlay E2E and UI profile save -> disk E2E |
| **PR-018** | Phase 4 | Automated Production Packaging | done | PR §4.3 | cargo xtask package --version 1.0.0: NSIS installer, portable zip, CLI zip, SBOM, SHA256SUMS |
