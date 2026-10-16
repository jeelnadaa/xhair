# XHAIR Design Specification & Screen Mapping (v5)

Source of truth for all application UI, layouts, spacing, themes, typography, and interaction patterns.
Implements the **XHAIR — Complete Design & Spec Update (v5: v2 + v3 + v4 merged)**.

## Core Rules & Invariants
1. **Neutral Monochrome Ramps (§12.1):** All surface, border, and text colors are pure neutral grays ($R = G = B$, channel spread $\le 2$). Saturated color is strictly forbidden in the chrome and is reserved exclusively for the reticle itself, the color picker, and functional cues (selection outline `#38BDF8`, center guide, and state warnings).
2. **Tabbed Dock System (§10):** Flexible dock supporting tabs (`Design`, `Shape`, `Fill & Stroke`, `Effects`, `Color`, `Layers`, `Nudge`, `Animation`, `Symmetry & Arms`, `Dynamics`, `Analysis`, `History`, `Library`, `Presets`, `Properties`, `MCP Activity`, `Resolution & Preview`), split docking, floating in-app panels, detached windows, and persistent layout presets (`Designer`, `Agent Watch`, `Pixel Art`, `Minimal`).
3. **Corner Radius Scale (§12.3):** Strictly $\le 10\text{ px}$. No pill rounding ($9999\text{ px}$ or `50%`) anywhere except true circles (radio buttons, slider thumbs, color dots). Corner styles: `sharp` ($0/1/2/2/3/4$), `default` ($0/2/3/4/6/8$), `soft` ($3/4/5/6/8/10$).
4. **Bundled Typography Packs (§13):** 6 bundled self-hosted WOFF2 font packs ($\le 1.5\text{ MB}$ total) with zero external runtime CDN requests. Tabular figures and slashed zeros for monospace numbers.

---

## v5 Screen Catalog & Command Bus Map

| # | Screen Name | Route / View | Primary Components | Commands / MCP Tools | Screen Asset (1x & 2x) |
|---|---|---|---|---|---|
| **1** | **Editor (mono-dark)** | `/editor` | TopBar, LibrarySidebar, ToolRail, CanvasGrid, DockPanel (`Design`) | `crosshair.get`, `edit.batch`, `layer.set_position`, `layer.recenter` | `Editor-1x.png`, `Editor-2x.png` |
| **2** | **Editor (mono-light)** | `/editor?theme=mono-light` | White ramp chrome, dark reticle, CanvasGrid, DockPanel (`Shape`) | `theme.set {id: "mono-light"}`, `property.set` | `Editor_mono_light-1x.png`, `Editor_mono_light-2x.png` |
| **3** | **Editor (graphite-mint)** | `/editor?theme=graphite-mint` | Mint accent `#3DDBB0`, amber guide, DockPanel (`Fill & Stroke`) | `theme.set {id: "graphite-mint"}`, `paint.set` | `Editor_graphite_mint-1x.png`, `Editor_graphite_mint-2x.png` |
| **4** | **Library & Groups** | `/library` | GroupTree, ProcessBinding display, ReticleCard grid, Blank button | `group.list`, `group.create`, `group.set_game_binding`, `crosshair.duplicate` | `Library-1x.png`, `Library-2x.png` |
| **5** | **Resolution Profiles** | `/settings/resolutions` | ResolutionCard grid, AspectRatio preview, Parity diagnostics | `resolution.list`, `resolution.create`, `resolution.activate` | `Resolution_profiles-1x.png`, `Resolution_profiles-2x.png` |
| **6** | **Calibration Wizard** | `/calibration` | 4-step wizard modal, High-contrast test pattern, Symmetry optimizer | `calibration.show`, `calibration.set_offset`, `analyze.alignment` | `Calibration-1x.png`, `Calibration-2x.png` |
| **7** | **MCP Settings & Transports** | `/settings/mcp` | Transport status (stdio/HTTP), Live activity log, Bearer security | `meta.capabilities`, `meta.schema`, `settings.get` | `MCP_settings-1x.png`, `MCP_settings-2x.png` |
| **8** | **Dock States & Split Views** | `/editor#dock-split` | Drop zone overlays, Split dock panels, Floating NudgePad window | `ui.panel.split`, `ui.panel.float`, `ui.panel.move` | `Dock_states-1x.png`, `Dock_states-2x.png` |
| **9** | **Dock Layout Presets** | `/editor#layouts` | Layout switcher (Designer, Agent Watch, Pixel Art, Minimal) | `ui.layout.list`, `ui.layout.apply`, `ui.layout.save` | `Dock_layout_presets-1x.png`, `Dock_layout_presets-2x.png` |
| **10** | **Properties Tab (Search & Filter)** | `/editor#properties` | Property search input, Modified-only toggle, Group cards, Reset | `property.list`, `property.search`, `property.reset` | `Properties_tab_search-1x.png`, `Properties_tab_search-2x.png` |
| **11** | **Per-Shape Inspector Variants** | `/editor#shapes` | Micro-inspectors for all 14 shapes (Line, Dot, Rect, Ring, etc.) | `property.get`, `property.set`, `layer.update` | `Per_shape_inspector_variants-1x.png`, `Per_shape_inspector_variants-2x.png` |
| **12** | **Overrides & Style Presets** | `/editor#overrides` | Per-arm override badges, Reset override, Copy/Paste style | `property.set` (arm overrides), `style.copy`, `style.paste`, `style.save_preset` | `Override_indicators_and_styles-1x.png`, `Override_indicators_and_styles-2x.png` |
| **13** | **Color Picker & Gradient Editor** | Popover `/editor#color` | Large SV square, Hue slider, Alpha checkerboard, Hex/RGB/HSL, Swatches | `paint.set {color: "#RRGGBBAA"}` | `Color_picker_popover-1x.png`, `Color_picker_popover-2x.png` |
| **14** | **Group Settings Popover** | Modal `/group/settings` | Process binding input, Group color, Resolution default, Hotkeys | `group.set_game_binding`, `group.set_resolution_profile`, `group.rename` | `Group_settings_popover-1x.png`, `Group_settings_popover-2x.png` |
| **15** | **Blank Start Empty State** | `/editor?new=blank` | 0 layers canvas, Guide center marker, Quick add shape prompt | `crosshair.create {template: "blank"}`, `layer.add {kind, at: "center"}` | `Blank_start_empty_state-1x.png`, `Blank_start_empty_state-2x.png` |
| **16** | **Multi-Select & Marquee** | `/editor#select` | Canvas marquee drag, Bounding box center marker, Multi-action bar | `selection.set`, `selection.all`, `selection.invert`, `layer.group` | `Multi_select_state-1x.png`, `Multi_select_state-2x.png` |
| **17** | **Layer Groups (Hierarchy)** | `/editor#layers` | Expandable folder rows, Indented child rows, Group/Ungroup | `layer.group`, `layer.ungroup`, `layer.rename` | `Layer_groups_in_layers_panel-1x.png`, `Layer_groups_in_layers_panel-2x.png` |
| **18** | **Keyboard Shortcuts Dialog** | Modal `/dialog/shortcuts` | Full catalog cheat sheet (Navigation, Editing, Selection, Nudge) | `meta.shortcuts` | `Keyboard_shortcuts_dialog-1x.png`, `Keyboard_shortcuts_dialog-2x.png` |
| **19** | **Tooltip Gallery** | Showcase | Property tooltips with diagrams, Action tooltips, Disabled hints | Schema property descriptors & `actions.ts` | `Tooltip_gallery-1x.png`, `Tooltip_gallery-2x.png` |
| **20** | **In-App MCP Docs Viewer** | `/docs/mcp` | Tool schemas, parameter tables, live JSON-RPC request preview | `meta.schema`, `meta.capabilities` | `In_app_mcp_docs_viewer-1x.png`, `In_app_mcp_docs_viewer-2x.png` |
| **21** | **Theme Gallery (9 Themes)** | `/gallery/themes` | 9 theme cards (mono-dark, mono-light, oled, mint, violet, amber, crimson, indigo, high-contrast) | `theme.list`, `theme.preview`, `theme.set` | `Theme_gallery-1x.png`, `Theme_gallery-2x.png` |
| **22** | **Appearance Settings** | `/settings/appearance` | Follow system toggle, Corner style selector, Font pack picker | `appearance.get`, `appearance.set` | `Appearance_settings-1x.png`, `Appearance_settings-2x.png` |
| **23** | **Radius & Component Spec** | Spec Sheet | Primary/Secondary/Ghost buttons, Status tags, Inputs, Sliders across styles | Design tokens verification | `Radius_and_component_spec-1x.png`, `Radius_and_component_spec-2x.png` |
| **24** | **Typography Specimens** | Spec Sheet | Wordmark, display, UI body, uppercase labels (+0.06em), tabular mono figures | Self-hosted WOFF2 font packs | `Typography_specimens-1x.png`, `Typography_specimens-2x.png` |

---

## Token Verification Status
- **Neutrality Check:** 100% of chrome surface, border, and text tokens have $R = G = B$ (channel spread $= 0 \le 2$).
- **Contrast Check:** Text on backgrounds $\ge 4.5:1$ ($7:1$ on High Contrast).
- **Corner Radii:** Strictly $\le 10\text{ px}$. Zero pill rounding anywhere.
