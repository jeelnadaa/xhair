# Right-Panel Dock System & Layout Architecture (§10)

XHAIR replaces static inspector columns with a modular, high-performance **Tabbed Dock System**.

---

## 1. Dock Capabilities

- **Tabbed Panels:** Multiple panels coexist in a shared dock header with horizontal scrolling and active underline indicators.
- **Independent Multiple Instances:** Users or agents can open several instances of any panel type (e.g. comparing two layer inspectors side-by-side).
- **Dock Areas:** Left, Right, Bottom, and Center-Top docks.
- **Split View:** Panels can be split horizontally or vertically within their dock area.
- **In-App Floating Panels:** Any dock tab (e.g. the Nudge Pad or Color Picker) can be popped out and floated over the canvas grid with pin and dock-back controls.
- **OS Window Detachment:** Detach panels to secondary monitors for dual-screen esports workflows.
- **Safe Tab Closure:** Closing a dock tab never deletes or resets crosshair document data.

---

## 2. Default Tabs Catalog

1. **`Design`**: Global reticle position, offset coordinates, bounding-box centers, relative/absolute mode, recenter controls.
2. **`Shape`**: Micro-properties tailored specifically to the active shape kind (arms, dot, rect, polygon, chevron, t-shape, etc.).
3. **`Fill & Stroke`**: Paint models (solid, linear, radial, conic), stroke widths, dash arrays, join/cap styles.
4. **`Effects`**: Real-time composite glow, drop shadow, and global bloom parameters.
5. **`Color`**: Saturation/brightness square, hue & alpha sliders, hex inputs, and saved palettes.
6. **`Layers`**: Complete layer stack, reordering, grouping, lock, and solo toggles.
7. **`Nudge`**: 8-direction virtual pad, step chips (`0.1`, `0.25`, `0.5`, `1`, `5`, `10`), and custom numeric steps.
8. **`Animation`**: Parameter animation tracks, keyframe interpolators, and playback speed.
9. **`Presets`**: Visual style presets, copy/paste style, and procedural variations (`crosshair.randomize`).
10. **`Properties`**: Comprehensive catalog of all schema descriptors with live search and "Show modified only" filter.
11. **`MCP Activity`**: Real-time streaming log of loopback agent tool executions and status badges.
12. **`Dynamics`**: Spray Lab visualizer and restricted Raw Input binding simulator.

---

## 3. Layout Presets

| Preset | Target Workflow | Active Panels & Arrangements |
|---|---|---|
| **`Designer`** (Default) | Full customization | Right dock with all 11 tabs, full canvas rulers and guides |
| **`Compact`** | Narrow laptop displays | Condensed `Design`, `Layers`, and `Nudge` |
| **`Pixel Art`** | 1px precision reticles | Floating nudge pad, floating color picker, 32× canvas zoom |
| **`Agent Watch`** | Autonomous pair programming | Split dock with live `MCP Activity`, `History`, and `Layers` |
| **`Minimal`** | Distraction-free preview | Full-screen canvas with auto-hiding dock |

Layouts persist in local storage, survive application restarts, and automatically fall back to standard dimensions if a previously detached display is disconnected.

---

## 4. Keyboard & Accessibility Navigation

- `Ctrl+P`: Quick dock panel switcher.
- `F6`: Cycle focus between Left Sidebar, Center Canvas, and Right Dock.
- `Ctrl+Tab`: Next dock tab; `Ctrl+Shift+Tab`: Previous dock tab.
- `Ctrl+W`: Close active dock tab.
- `Ctrl+Space`: Maximize / restore current dock area width.
- `Escape`: Close floating panel or dismiss popover.
- **Accessibility:** Full ARIA roles (`role="tablist"`, `role="tab"`, `aria-selected`), visible focus rings, and reduced-motion transitions.
