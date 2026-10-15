# XHAIR Design Specification & Screen Mapping

Source of truth for all application UI, layouts, spacing, and interaction patterns.
Implemented from FigCraft bundle exports (`design/XHAIR — App Design.html`) and design tokens (`design/tokens.json`).

## Screens Architecture & Command Map

| # | Screen Name | Route / Modal | Primary Components | Commands / Data Used | Bundle Frame ID / Ref |
|---|---|---|---|---|---|
| **1** | **Onboarding / First Run** | `/onboarding` | Stepper, monitor select, compliance disclaimer, import action | `settings.set`, `display.list`, `resolution.activate` | `scr:onboarding` |
| **2** | **Tray Menu & Tooltip** | System Tray | Native Win32 menu, status pills, recent slots | `overlay.set_enabled`, `slot.assign`, `overlay.activate` | `scr:tray` |
| **3** | **Main Window Shell** | `/` | Top document bar, sidebars, dock, status indicators | `meta.capabilities`, `settings.get` | `scr:shell` |
| **4** | **Library View** | `/library` | Grid/list view, folder tree, tags, search bar, slot chips | `library.list`, `library.search`, `crosshair.duplicate`, `crosshair.delete` | `97bb8de0-a65a-48e8-bccc-3e5c9b69fa48` |
| **5** | **Editor — Default State** | `/editor` | Grid canvas, tools palette, inspector, layers list, nudge pad | `crosshair.get`, `edit.batch`, `layer.update`, `nudge` | `d2a3b957-bbc1-4791-bc04-3ff973a90e52` |
| **6** | **Editor — High Zoom (16×/32×)** | `/editor?zoom=32` | Sub-pixel grid, center edge-vs-middle visual indicator | `xhair-geometry::alignment_report`, `render.preview` | `scr:editor-zoom` |
| **7** | **Editor — Pixel Paint Mode** | `/editor?mode=pixel` | Pixel brush, eraser, bucket fill, palette swatches | `pixels.paint` | `scr:editor-pixel` |
| **8** | **Editor — Path/Anchor Mode** | `/editor?mode=path` | Direct selection tool, bezier handles, anchor nudge | `layer.update`, `nudge` (anchor target) | `scr:editor-path` |
| **9** | **Nudge Controls Panel** | `/editor#nudge` | 8-way directional buttons, step chips, target/frame toggles | `nudge` (target: crosshair/layer/arm/anchor) | `scr:editor-nudge` |
| **10** | **Color Picker & Gradients** | Popover | RGB/HSV sliders, hex input, gradient stop timeline | `paint.set` | `scr:color-picker` |
| **11** | **Effects Panel** | `/editor#effects` | Sliders for glow, shadow, blur, bloom, live toggle | `effects.set` | `scr:effects` |
| **12** | **Animation Timeline** | Dock `/editor#timeline` | Track rows, keyframes, scrub bar, static-only lock toggle | `animation.set`, `settings.set` | `scr:timeline` |
| **13** | **Preview Backgrounds** | Popover `/editor#bg` | Background presets, noise textures, WCAG contrast score | `analyze.visibility` | `scr:preview-bg` |
| **14** | **Resolution Profiles** | `/settings/resolutions` | Profile cards, custom W×H, stretch toggle, monitor binding | `resolution.list`, `resolution.create`, `resolution.update` | `65409862-15c5-4973-8513-0c873665453b` |
| **15** | **Calibration Wizard** | `/calibration` | 4-step wizard, interactive test pattern, nudge pad | `calibration.show`, `calibration.set_offset`, `analyze.alignment` | `a7874e07-aa6d-4ebe-ba73-ba72fb321288` |
| **16** | **Import / Export Dialogs** | Modal `/dialog/share` | Code input/output, file dropzone, Valorant code parser | `import.code`, `export.code`, `export.pack`, `import.valorant_code` | `scr:import-export` |
| **17** | **Hotkeys Settings** | `/settings/hotkeys` | Shortcut recorder, conflict warnings, slot assignments | `hotkey.set`, `hotkey.clear`, `slot.assign` | `scr:hotkeys` |
| **18** | **MCP Server Settings** | `/settings/mcp` | Loopback toggle, port, token reveal/rotate, activity log | `meta.capabilities`, `settings.get`, `settings.set` | `5417d769-3d6b-49a9-b1c1-b0201537ff0f` |
| **19** | **Dynamic Reticle Opt-in** | Modal `/dialog/dynamic` | Risk acknowledgment dialog, session confirmation | `dynamics.set_enabled` (`confirm_risk: true`) | `scr:dynamic-optin` |
| **20** | **Spray Lab** | Standalone Window | Recoil pattern scrub bar, fire test button, spread curve | `gun.list`, `dynamics.simulate`, `binding.set` | `scr:spray-lab` |
| **21** | **Application Settings** | `/settings` | General, start with Windows, theme, UI scale, logging | `settings.get`, `settings.set` | `scr:settings` |
| **22** | **Transparency Panel** | `/compliance` | Whitelisted OS APIs list, forbidden APIs assurance | Read-only compliance report | `scr:compliance` |
| **23** | **Shortcuts & Command Palette** | Modal (`Ctrl+K`) | Searchable action list with shortcuts, fuzzy filter | Action registry / command bus | `scr:command-palette` |
| **24** | **Error & Fallback States** | Banner / Modal | Elevated process warning, display disconnected, conflict | System error codes `XH-1xxx` | `scr:error-states` |
| **25** | **Performance HUD** | Editor Overlay | FPS counter, render latency (ms), private working set | `xhair-app` telemetry (editor-only) | `scr:perf-hud` |

## Acceptance Checklist Self-Check (§17.5)
- [x] Every tool/control in Section 9 appears in the screens.
- [x] Every tooltip variant has dedicated content and state in the design system.
- [x] The grid view explicitly displays pixel edges, pixel centers, and true center with status pills.
- [x] The 8-way nudge pad is directly embedded in the editor inspector and accessible globally.
- [x] All interactive states (default, hover, active, focus, disabled, error) are defined in `components.md`.
- [x] Design tokens are formalized in `design/tokens.json` without hard-coded color values.
- [x] All 25 screens are mapped to routes, components, and command-bus operations above.
