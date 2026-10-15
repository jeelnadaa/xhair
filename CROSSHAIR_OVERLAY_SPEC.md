# XHAIR — Crosshair Overlay App: Complete Build Specification

> **Audience:** an autonomous coding agent with a FigCraft MCP connection.
> **Goal:** build a Windows crosshair overlay with an extremely customizable editor, pixel-accurate centering at any resolution, and a full MCP interface so an AI agent can do everything a human can.
> **Working name:** `xhair` (rename freely; keep it in one constant/config).

---

## 0. Agent Execution Protocol (READ FIRST)

You build this in **phases**. Do not skip ahead.

1. **Phase 0 — Design (FigCraft).** Use the FigCraft MCP to generate the complete app design (Section 17). Export everything to the `design/` folder. **Then STOP and wait for the human to approve or request changes.** Do not write application code before approval.
2. **Phases 1–10 — Implementation** (Section 16). After each milestone: run all tests, update docs, commit, and report what was done and what is next.
3. **The `design/` folder is the single source of truth for UI.** Do not invent layouts, colors, spacing, or components that contradict it. If something is missing from the design, add it to the design via FigCraft first, tell the human, then implement.
4. **Ask when blocked**, but do not ask about things this document already decides.
5. **Never claim a feature works without a test or a manual verification note** in the milestone report.
6. When this spec says "verify", it means you must check current documentation (crate docs, Windows API docs, MCP spec) because APIs change. Do not rely on memory for exact signatures.

### Non-negotiable principles
| # | Principle |
|---|-----------|
| P1 | **Overlay only.** Never inject into, attach to, read memory of, hook, or modify the game process. Never capture the game's screen. |
| P2 | **Low RAM.** Idle overlay target ≤ 20 MB private working set. Editor open ≤ 150 MB. Zero CPU when the crosshair is static. |
| P3 | **One source of truth.** One core library owns all state and logic. UI, MCP, CLI, and hotkeys are thin clients of the same command bus. |
| P4 | **Human/agent parity.** Anything a human can do in the UI exists as a command and therefore as an MCP tool. Enforced by a CI test. |
| P5 | **Schema-driven.** Properties are declared once (label, tooltip, type, range, unit, default). UI controls, tooltips, validation, MCP schemas, and docs are generated from that declaration. |
| P6 | **Pixel accuracy over prettiness.** The center must be provably correct and verifiable by the user (calibration). |
| P7 | **Clean, documented code.** See Section 15. |

---

## 1. Product Summary

### 1.1 What it is
A tray-resident Windows app that draws a customizable crosshair on top of a borderless/windowed game. A separate editor window (opened on demand) lets the user design crosshairs on a pixel grid that matches the game's actual render resolution, save unlimited crosshairs, switch them mid-game by hotkey, and share them as files or text codes. An embedded MCP server lets AI agents create and manage crosshairs, with or without the GUI running.

### 1.2 Scope: in
Everything in Sections 4–14.

### 1.3 Scope: out (do not build)
- Cloud storage, accounts, online gallery, telemetry, auto-update servers (sharing is via files/codes only).
- Anything that reads game state, game memory, or game pixels (hit markers, kill flashes, ammo, contrast-aware color from the live screen).
- Mac/Linux (Valorant is Windows-only). Keep platform code behind a trait so a port is possible, but do not build one.
- Fullscreen-exclusive overlay support (not technically possible for a normal overlay window; document that the game must be in Windowed or Windowed Fullscreen/borderless).

### 1.4 Runtime modes
| Mode | Command | Windows shown | Overlay | MCP |
|------|---------|---------------|---------|-----|
| **App** | `xhair.exe` | Tray + editor on demand | Yes | Yes (HTTP + stdio proxy) |
| **Headless** | `xhair.exe --headless [--overlay]` | None (optional tray) | Optional | Yes (stdio and/or HTTP) |
| **CLI** | `xhair-cli <cmd>` | None | No | Optional stdio server; render/convert/validate commands |

Headless with `--overlay` must be able to show/hide/switch the real overlay so an agent can fully operate the product without the GUI.

If the app is already running and a headless/CLI instance starts, the second instance must **proxy to the running instance** (lock file + local endpoint) instead of opening the library concurrently.

---

## 2. Compliance & Risk Guardrails

You must implement these as code-level constraints, not just documentation.

1. **No game process access.** Do not call `OpenProcess`, `ReadProcessMemory`, `WriteProcessMemory`, `CreateRemoteThread`, `SetWindowsHookEx` targeting the game, DLL injection, or driver code. Do not use screen/desktop duplication or `BitBlt`/`PrintWindow` on the game window.
2. **Allowed OS interactions (exhaustive list — publish it in the app's Transparency Panel):**
   - Create own layered, click-through, topmost window; `UpdateLayeredWindow`.
   - `EnumDisplayMonitors`, `GetMonitorInfo`, `GetDpiForMonitor`, `EnumDisplaySettings`.
   - `GetForegroundWindow`, `GetWindowRect`/`DwmGetWindowAttribute` (window **rectangle only**), and the process **image name** of the foreground window (via `GetWindowThreadProcessId` + `QueryFullProcessImageName` with limited-query rights) to know whether the game is focused.
   - `SetWinEventHook(EVENT_SYSTEM_FOREGROUND, ...)` out-of-context for foreground changes.
   - `RegisterHotKey` for global hotkeys.
   - Raw Input (`RegisterRawInputDevices`) **only** if the opt-in Dynamic Reticle is enabled (Section 12), restricted to the keys/buttons the user bound; no logging, no persistence of input.
   - Files in the app's own data directory and user-chosen import/export paths.
3. **Forbidden:** `WH_KEYBOARD_LL`/`WH_MOUSE_LL` low-level hooks, `GetAsyncKeyState` polling of arbitrary keys, keystroke logging.
4. **Dynamic Reticle (recoil/movement-reactive) is flagged as risky.** It is a gameplay aid and may conflict with the game publisher's rules or anti-cheat. Therefore:
   - **Off by default**, hidden behind an explicit opt-in dialog that states this plainly and requires confirmation each session.
   - Ships **together with a Spray Lab** (standalone window, no overlay, 100% safe) which is the recommended way to use the feature.
   - Never reads game data. Reacts only to the user's own bound inputs.
   - Gun/recoil numbers are **user-entered**. Do not ship numbers claiming to match the game.
   - Show a persistent "Dynamic Reticle ON" indicator in the tray tooltip.
   - Put a plain-language disclaimer in `docs/COMPLIANCE.md` and the first-run flow: *"Anti-cheat behavior can change without notice. No overlay can be guaranteed safe. Check the publisher's current rules before using in ranked."*
5. **MCP security:** see Section 13.7.

---

## 3. Tech Stack & Architecture

### 3.1 Decision
| Layer | Choice | Reason |
|-------|--------|--------|
| Core / logic | **Rust** | Low RAM, no GC, one binary |
| Overlay renderer | **tiny-skia** (CPU, anti-aliased vector raster) + custom blur for glow | No GPU context (saves tens of MB), deterministic output, compiles to WASM |
| Overlay window | **Win32 layered window** via the `windows` crate, `UpdateLayeredWindow` with premultiplied BGRA | Per-pixel alpha, tiny window = tiny memory |
| Editor UI | **Tauri 2 + WebView2**, **Svelte 5 + TypeScript + Vite**, canvas for the grid | Figma designs translate directly; framework is tiny |
| Preview parity | Renderer crate compiled to **WASM**, used by the editor canvas | Editor preview is pixel-identical to the overlay |
| MCP | Official Rust MCP SDK (**verify current crate name/version**, e.g. `rmcp`) | stdio + Streamable HTTP |
| SVG import | **resvg/usvg** | Safe (no scripts, no external fetches) |
| Image decode | `image` crate with only `png` feature | Small |
| Serialization | `serde`, `serde_json`; `brotli` for share codes | |
| Schema | `schemars` (JSON Schema from Rust types) | Drives validation + MCP |
| Async | `tokio` with **current-thread + minimal features** where possible | Keep RAM low |
| Tests | `cargo test`, `insta` (snapshots), `proptest`, `cargo-fuzz`, Vitest, Playwright (UI smoke) | |

If you find a concrete reason a choice fails the RAM budget, **measure, report, and propose** an alternative rather than silently switching.

### 3.2 Process model (important for RAM)
- **One process `xhair.exe`** stays resident in the tray. It contains: core, overlay thread, hotkey thread, MCP server, library watcher.
- The **editor is a Tauri webview window created on demand and fully destroyed on close** so WebView2 child processes exit and memory is released. The resident part must not link or preload the webview until the editor is requested (lazy init).
- Overlay window is sized to the **crosshair's bounding box only** (plus glow margin), never the whole screen.
- No render loop when static. The overlay thread blocks in `GetMessage`. Animation uses a timer only while an animated layer or Dynamic Reticle is active, capped at a configurable FPS (default 30, max 120).

### 3.3 Workspace layout
```
xhair/
├─ design/                      # FigCraft exports (source of truth for UI)
├─ docs/                        # See Section 15.2
├─ crates/
│  ├─ xhair-model/              # Types, schema, validation, migrations (no I/O)
│  ├─ xhair-commands/           # Command bus, undo/redo, registry (pure logic)
│  ├─ xhair-render/             # Rasterizer, effects, symmetry (native + wasm)
│  ├─ xhair-geometry/           # Resolution, DPI, center, snapping math (pure)
│  ├─ xhair-format/             # .xhair, .xhairpack, share codes, Valorant import
│  ├─ xhair-library/            # Disk storage, folders, tags, watcher
│  ├─ xhair-platform-win/       # Win32: overlay window, monitors, hotkeys, foreground, raw input
│  ├─ xhair-dynamics/           # Dynamic Reticle simulation (pure, testable)
│  ├─ xhair-mcp/                # MCP server generated from command registry
│  ├─ xhair-app/                # Tauri shell, tray, wiring, runtime modes
│  └─ xhair-cli/                # CLI front-end
├─ ui/                          # Svelte app (editor)
│  └─ src/{lib,components,routes,stores,wasm}/
├─ tests/                       # Cross-crate integration, golden images, fixtures
├─ scripts/                     # Build, codegen, packaging
└─ .github/workflows/ci.yml
```
Dependency rule: arrows point downward only. `model` depends on nothing internal. `platform-win` never imports `ui`. `xhair-app` is the only crate that wires everything.

### 3.4 Data flow
```
 UI / MCP / CLI / Hotkeys
          │  Command (typed, serializable)
          ▼
   ┌───────────────┐   emits Events     ┌──────────────┐
   │  Command Bus  │ ─────────────────► │ Subscribers: │
   │ (xhair-cmds)  │                    │ UI, MCP, Overlay, Library │
   └──────┬────────┘                    └──────────────┘
          │ mutates
          ▼
   Document Store (xhair-model) + Undo/Redo + Library persistence
```
- Every mutation is a `Command`. Every command has: stable name, JSON Schema for args, result type, undo behavior, `actor` (human/agent/hotkey/system), and a docs string.
- Subscribers receive `Event`s (`DocumentChanged`, `ActiveCrosshairChanged`, `OverlayStateChanged`, `LibraryChanged`, `SettingsChanged`).
- The overlay re-renders only on events that affect the active crosshair or geometry.

---

## 4. Domain Model

Define in `xhair-model`. All types derive `Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq`. Use newtypes for IDs (UUID v7) and for units (`GamePx(f32)`, `Degrees(f32)`, `Alpha(f32)`) to avoid unit mix-ups.

### 4.1 Crosshair
```text
Crosshair {
  format_version: u32,
  id: CrosshairId, name: String, description: String, author: String,
  tags: Vec<String>, favorite: bool,
  created_at, updated_at,
  global: GlobalTransform { offset: Vec2, scale: f32, rotation: Degrees,
                            opacity: Alpha, blend: BlendMode },
  symmetry: Symmetry,            // none | mirror_h | mirror_v | mirror_hv | radial(n: 2..=32)
  layers: Vec<Layer>,            // ordered bottom → top; groups nest
  global_effects: Effects,       // applied to the whole composite
  animation: Vec<AnimTrack>,
  dynamics: Option<DynamicsConfig>,
  render_hints: RenderHints { aa_mode: Crisp|Smooth|Auto, snap_to_pixel: bool },
  assets: Map<AssetHash, Asset>  // embedded PNG/SVG, content-addressed
}
```

### 4.2 Layer
```text
Layer {
  id, name, visible, locked, solo,
  kind: LayerKind,               // see 4.3 (includes Group { children })
  transform: { offset: Vec2, rotation: Degrees, scale: Vec2, anchor: Anchor },
  fill: Paint, stroke: Option<Stroke>,
  opacity: Alpha, blend: BlendMode,
  effects: Effects,              // per-layer glow/shadow/blur
  symmetry: SymmetryOverride,    // inherit | none | explicit
  arm_overrides: Vec<ArmOverride>, // per-arm offset/length/thickness/color/visibility
  bindings: Vec<DynamicBinding>,
  render_hints_override: Option<RenderHints>
}
```

### 4.3 Shape kinds (all must be implemented)
| Kind | Key parameters |
|------|----------------|
| `Line` (arms) | `arm_count` (1–16), `start_angle`, `angle_step`, `length`, `thickness`, `gap` (distance of near end from center), `cap` (butt/round/square), `taper` (0–1) |
| `Dot` | `diameter`, `shape` (square/round) |
| `Rect` | `width`, `height`, per-corner `radius` |
| `Circle` | `diameter` |
| `Ring` | `outer_radius`, `inner_radius` (or `thickness`), optional `dash` pattern |
| `Arc` | `radius`, `thickness`, `start_angle`, `end_angle`, `cap` |
| `Polygon` | `sides` (3–64), `radius`, `star_inner_ratio` (0 = regular polygon) |
| `Triangle` | `base`, `height` |
| `Chevron` | `arm_length`, `thickness`, `angle`, `gap` |
| `TShape` | `bar_length`, `stem_length`, `thickness`, `gap` |
| `Plus` | `length`, `thickness`, `gap`, `center_hole` |
| `Path` | SVG path data (cubic/quadratic béziers, arcs), `fill_rule` |
| `Pixels` | `width`, `height` (≤ 256×256), RGBA grid (run-length encoded in storage) |
| `Image` | `asset_hash`, `sampling` (nearest/bilinear), `tint` |
| `Group` | `children`, isolates blend/effects |

### 4.4 Paint
`Solid(Rgba)`, `Linear{angle, stops[]}`, `Radial{center, radius, stops[]}`, `Conic{center, start_angle, stops[]}`. Stops: position 0–1, color, ≥ 2, ≤ 16. Colors stored as **sRGB 8-bit + alpha 0–1 float**; internally blend in premultiplied sRGB (match what the user sees; document it). Provide an optional "linear-light blending" render hint for effects only.

### 4.5 Stroke / Outline
`width` (0–64, 0.25 step), `paint`, `position` (inside/center/outside), `dash` (array), `join` (miter/round/bevel), `cap`.

### 4.6 Effects
```text
Effects {
  glow:   Option<Glow   { color, radius, intensity, spread, blend: Add|Screen|Normal, quality }>,
  shadow: Option<Shadow { color, offset: Vec2, blur, opacity }>,
  blur:   Option<f32>,                // softening of the shape itself
  bloom:  Option<Bloom  { threshold, radius, intensity }>  // global only
}
```
Glow = shape alpha → blur → tint → composite *behind or additive over* the shape. Implement blur as 3 successive box blurs (approximate Gaussian) on a bounded scratch buffer; cap radius at 64 px and buffer size at 1024×1024 to bound memory. Reuse buffers between renders (arena).

### 4.7 Animation
Tracks target any numeric/color property by path (e.g. `layers[3].fill.stops[0].color`, `layers[1].transform.rotation`).
Kinds: `Pulse`, `Breathe`, `RainbowCycle`, `Rotate`, `Blink`, `Keyframes` (with easing: linear/ease-in/out/in-out/cubic-bezier/steps). Global **"Static only" lock** (setting) disables all animation and Dynamic Reticle motion.

### 4.8 Library entities
- `Folder { id, name, parent, order }`
- `Slot { index 1..=20, crosshair_id, hotkey }`
- `ResolutionProfile` (Section 6)
- `GunProfile` (Section 12)
- `Settings` (Section 14)

---

## 5. Command Bus & Schema-Driven Properties

### 5.1 Command registry
In `xhair-commands`, define a `Command` trait/enum with a **macro-generated registry**:
```text
register_command! {
  name: "layer.update",
  summary: "Update fields of a layer",
  args: LayerUpdateArgs,        // JsonSchema
  result: LayerUpdateResult,
  undoable: true,
  scope: Document | Library | Overlay | Settings | System,
  availability: App | HeadlessAndApp | AppOnly   // see 13.4
}
```
Requirements:
- Commands are **pure functions over state + side-effect requests**. Side effects (disk, Win32) are emitted as `Effect`s executed by the host, so commands are unit-testable.
- **Batch**: `batch([commands])` is atomic (all or nothing, single undo step).
- **Undo/redo**: per-document stacks (default depth 500), with **coalescing** of repeated same-target commands within 400 ms (slider drags, nudge key repeats). Each entry records `actor`.
- `dry_run: true` returns the would-be diff without applying.
- Commands return structured **diffs** (JSON Patch RFC 6902) so subscribers update incrementally.

### 5.2 Property descriptors (single source for UI + tooltips + MCP)
```text
#[derive(Property)]
struct LineProps {
  #[prop(label="Length", tip="How far each arm extends from its starting point, in game pixels.",
         unit="px", min=0.0, max=512.0, step=0.5, fine_step=0.1, default=6.0, group="Geometry")]
  length: f32,
  ...
}
```
A build step (`scripts/gen-schema`) emits `ui/src/generated/properties.json` and `docs/PROPERTIES.md`. The UI **renders inspector controls from this descriptor**; no hand-written inspector fields.

### 5.3 Tooltip requirements (applies to EVERY interactive control)
- Every toolbar button, tool, tab, toggle, slider, input, dropdown, color swatch, menu item, and icon-only control has a tooltip.
- Content: **Title** (bold), **description** (1–2 sentences, plain language), **unit/range/default** where applicable, **shortcut** badge where applicable, optional "Learn more" deep link into `docs/`.
- Behavior: show on hover after 400 ms, on keyboard focus immediately, dismiss on `Esc`, never obscure the control, reposition near viewport edges, `aria-describedby` wired, respects reduced-motion.
- Text lives in the property descriptors (for properties) and `ui/src/lib/actions.ts` (for actions), never inline in components.
- **CI test:** a script walks the action registry and property descriptors and fails if any entry lacks `tip`. A Playwright smoke test hovers a sample of controls.

---

## 6. Grid, Coordinates, Resolution, DPI, Stretch, Center (the accuracy core)

Implement in `xhair-geometry` as **pure, heavily tested functions**.

### 6.1 Coordinate system
- **Crosshair space:** origin at the *game's screen center*, +x right, +y down, unit = **game pixel** (one pixel of the game's render resolution). `f32`.
- **Game raster:** `W × H` pixels. Pixel `(i, j)` occupies `[i, i+1) × [j, j+1)` in **edge coordinates**.
- **Center point** `C = (W/2, H/2)` in edge coordinates. For **even** `W` the center lies on a pixel *edge*; for **odd** `W` it lies in a pixel *middle* (same for `H`).

### 6.2 Pixel alignment rule
A symmetric stroke of thickness `t` centered on `C` has edges at `C ± t/2`. Edges land on pixel boundaries iff `(W + t)` is even (and `(H + t)` for the vertical axis). Provide:
- `alignment_report(crosshair, W, H) -> Vec<AlignmentIssue>` listing parts whose edges fall on half pixels, with a suggested fix (±0.5 px offset or ±1 px thickness).
- Editor badge per layer: ✅ aligned / ⚠ off-grid (click for the fix).
- **Snap to pixel grid** option (per crosshair and per layer): in `Crisp` AA mode, geometry edges are rounded to integer output pixels deterministically.

### 6.3 Resolution Profiles
```text
ResolutionProfile {
  id, name,
  game_resolution: { w: u32, h: u32 },      // ANY custom value, 1..=16384
  output: Monitor binding (stable monitor ID/hash, or "primary", or "follow game window"),
  display_mode: Native | Stretched | Letterbox | Pillarbox,
  stretch: { auto: bool, sx: f32, sy: f32 },// auto = output_size / game_resolution
  center_override: Option<Vec2>,            // in output pixels, rare
  calibration_offset: Vec2,                 // game px, 0.25 granularity, from calibration (6.7)
  dpi_mode: Auto | Manual(f32),
  follow_game_window: bool                  // use the focused game window's client rect as the output rect
}
```
- User can create unlimited profiles, pick from presets (1920×1080, 2560×1440, 1280×1024 stretched, 1440×1080 4:3, 1680×1050 16:10, 1920×1200, 3840×2160, etc.) **or type any custom width/height**. Validate and show derived aspect ratio.
- Profiles are selectable manually, via hotkey, via MCP, and **auto-switchable by monitor** (when the game window is on a given monitor).

### 6.4 Mapping to physical pixels
```
out_center = out_origin + (W/2, H/2) * (sx, sy) + calibration_offset * (sx, sy)
physical_px(gx, gy) = out_center + (gx, gy) * (sx, sy)
sx = out_w / W ; sy = out_h / H     (non-uniform when stretched)
```
- **Native:** `sx = sy = 1`.
- **Stretched (GPU/display scaling, e.g. 4:3 → 16:9):** non-uniform scale; crosshair appears stretched exactly like the game's rendering. Provide a toggle **"Draw in game-pixel space (looks stretched like the game)"** vs **"Draw in native pixels (undistorted)"**; default = game-pixel space so the grid matches the game.
- **Letterbox/Pillarbox (black bars):** uniform scale, center stays at the output center.
- If the game changes the real desktop display mode (true stretched via mode change), the overlay sees the new monitor size; listen to `WM_DISPLAYCHANGE` and recompute.
- Keep all math in `f64` until the final rounding to avoid drift; store and expose `f32` in the model.

### 6.5 DPI scaling
- App manifest: **Per-Monitor-V2 DPI awareness**. The overlay works **only in physical pixels**; never in DIPs. Windows scaling 100–500% must not shift the center.
- On `WM_DPICHANGED` and monitor changes, recompute geometry and re-place the overlay.
- Editor UI uses CSS pixels scaled by `devicePixelRatio`; the **grid canvas must render at device pixels** (set canvas backing store = css size × dpr, draw with integer-snapped lines) so grid lines remain crisp at 125%/150%/175%.
- Multi-monitor with different DPIs: each profile binds to a monitor; moving the editor between monitors must not corrupt the grid scale.

### 6.6 Overlay placement algorithm
1. Resolve active `ResolutionProfile` → output rect (monitor rect, or the game window client rect if `follow_game_window`).
2. Compute `out_center` (6.4).
3. Compute the crosshair's **bounding box** in game px (include stroke, glow radius, shadow offset, animation/dynamics max excursion) → transform to output px → expand outward to integers → this is the overlay window rect.
4. Rasterize into a premultiplied BGRA buffer of exactly that size, with the fractional part of the center applied as a sub-pixel translate inside the buffer.
5. `UpdateLayeredWindow` with the buffer and position. Do not recreate the window on change; only resize/move when the bounding box changes.

### 6.7 Calibration (user-verifiable center)
Calibration Wizard (design it in Phase 0):
1. Choose resolution profile; confirm game is in windowed/borderless.
2. Overlay shows a **test pattern**: 1-game-px lines through the computed center with ticks every 1/5/10 px and distinct colored markers for the exact center edge/pixel.
3. User sets the **game's own crosshair** to something simple (e.g., 1-px dot/lines) in the practice range and compares against the overlay. A hotkey toggles the pattern on/off for flicker comparison.
4. User nudges the pattern with the nudge pad (0.25 px steps) until aligned; **Save** stores `calibration_offset` in the profile.
5. Show the final numbers and whether the profile is "even/odd" aligned.

**Screenshot Align (offline helper):** user imports a screenshot *they took themselves* of the game's crosshair into the editor; the tool finds the in-game crosshair's center (simple symmetry detection) and suggests a `calibration_offset`. Runs only on a user-supplied file; never captures the screen.

### 6.8 Editor grid view
- Draws the **game raster region around the center** (never allocates a full W×H image; compute visible cells only). Works for any `W×H` up to 16384.
- Zoom: integer steps 1×, 2×, 4×, 8×, 16×, 32×, 64× plus "fit" and scroll-wheel zoom anchored on the cursor; pan with middle-drag/space-drag; **"True size (1:1)"** toggle respecting DPI.
- Overlays (each toggleable, remembered): pixel grid, 5/10 px major lines, center lines, center marker that visibly indicates edge-vs-pixel-middle, concentric distance rings (5/10/20 px), rulers with pixel numbers, screen-edge indicators, 3×3 guides, bounding-box, alignment badges.
- Pixel hover readout: game-px coordinate relative to center (e.g., `x −3.5, y +0.0`) and absolute `(i,j)`.
- **Preview backgrounds:** solid colors, gradients, built-in noise/foliage-like patterns, plus user-imported images (stored locally, editor-only) so the user can test visibility. A "visibility score" (contrast vs the current background, WCAG-style) shown as a number with tooltip. This operates only on the preview background, never on the live screen.
- **Resolution selector** in the grid header switches the profile; the grid and aspect immediately change.

---

## 7. Renderer (`xhair-render`)

### 7.1 API
```text
render(crosshair: &Crosshair, ctx: &RenderContext, out: &mut Pixmap) -> RenderStats
RenderContext { scale: (sx, sy), center_frac: Vec2, aa: AaMode, time: f32, dynamics: DynState, quality }
bounds(crosshair, ctx) -> Rect     // conservative bbox for window sizing
```
- Pure and deterministic given `(crosshair, ctx, time)`. No global state. Compiles to `wasm32-unknown-unknown` and native; the editor uses the WASM build.
- **Golden-image tests** (`insta` + PNG diff with a per-channel tolerance of ≤1) for every shape, symmetry, gradient, stroke, glow, at several scales and fractional centers. Native and WASM must match.

### 7.2 AA modes
- **Crisp:** edges snapped to integer *output* pixels; no partial coverage on axis-aligned geometry. For rotated geometry fall back to AA with a note in the UI.
- **Smooth:** full sub-pixel anti-aliasing (supports 0.5 px strokes and fractional nudges).
- **Auto:** Crisp when `sx, sy` are integers and all layers are axis-aligned; otherwise Smooth.

### 7.3 Symmetry
Applied at build time by instancing the layer geometry: `mirror_h`, `mirror_v`, `mirror_hv`, `radial(n)`. Per-layer `SymmetryOverride`. **Arm overrides** let the user break symmetry on a single arm (e.g., different length for the top arm, per-arm nudge, per-arm color, hide an arm), keyed by arm index with a clearly documented ordering (clockwise from the start angle).

### 7.4 Performance rules
- Re-render only on events affecting the active crosshair, profile, or time (if animated).
- Cache layer rasters keyed by `(layer hash, scale, frac)` for animations that only change global opacity/color.
- Scratch buffers pooled; hard caps: overlay bitmap ≤ 2048×2048, blur buffer ≤ 1024×1024. If bounds exceed caps, scale down quality and show a warning badge.
- Add a benchmark (`cargo bench`) and a CI threshold for a typical crosshair (< 1 ms) and a heavy one with glow (< 8 ms).

---

## 8. Overlay Window (`xhair-platform-win`)

### 8.1 Window setup
- Extended styles: `WS_EX_LAYERED | WS_EX_TRANSPARENT | WS_EX_TOPMOST | WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE`. Style `WS_POPUP`. Never takes focus; clicks pass through.
- Own thread with a message loop. Updates arrive via a channel + `PostMessage` to a hidden message window; **no polling**.
- Re-assert topmost on `EVENT_SYSTEM_FOREGROUND` (via `SetWinEventHook`) and on `WM_DISPLAYCHANGE`/`WM_DPICHANGED`, not on a timer.
- **Visibility rules (settings):** Always show / Only when the game window is foreground (default) / Manual. Detect the game by process image name (configurable list, default `VALORANT-Win64-Shipping.exe`). Hide when the game isn't focused so the crosshair doesn't float over other apps.
- Optional **exclude from capture** (`SetWindowDisplayAffinity(WDA_EXCLUDEFROMCAPTURE)`) as a user setting (default off), with a tooltip explaining it hides the crosshair from recordings/streams.
- Handle: monitor hot-plug, sleep/resume, explorer restart (tray re-add), session lock/unlock, UAC secure desktop (just hide).
- Verify behavior when the game runs **elevated**: if the game window belongs to a higher-integrity process, detection of its rect may fail; surface a clear message and a "Run as administrator" help link in the UI.

### 8.2 Game window following
When `follow_game_window` is on, query the game window's client rect on foreground/move/size events (`EVENT_OBJECT_LOCATIONCHANGE` filtered to that HWND) and recompute the center. Rect only; no content access.

### 8.3 Tray
Menu: current crosshair name, toggle overlay, next/previous, recent crosshairs, open editor, resolution profile submenu, calibration, MCP status, settings, quit. Tooltip shows state flags (Overlay ON/OFF, Dynamic Reticle ON, MCP ON).

### 8.4 Hotkeys
- Registered via `RegisterHotKey`; configurable; **conflict detection** (already registered / OS-reserved) with inline error.
- Actions: toggle overlay, next/previous crosshair, jump to slot 1–20, cycle resolution profile, toggle calibration pattern, toggle animations lock, and (opt-in, off by default) **live nudge** of the active crosshair (8 directions, step setting) so users can adjust mid-game.
- **Hold-to-swap** (change while a key is held) cannot be done with `RegisterHotKey` alone. Implement only if Raw Input mode is enabled by the user (same opt-in as Dynamic Reticle, Section 12) and clearly label it. If not enabled, hide the option.
- All hotkey actions are ordinary commands (so they are also MCP tools).

---

## 9. Editor UI (follow `design/` exactly)

### 9.1 Layout (final layout comes from design; this is the functional contract)
- **Left:** Library sidebar (folders, tags, search, favorites, thumbnails rendered live via WASM).
- **Center:** Grid canvas with tool palette and view controls.
- **Right:** Inspector (properties generated from descriptors), Layers panel, Nudge Pad, Effects, Animation.
- **Top:** Document bar (name, undo/redo, zoom, resolution profile, background, preview-on-overlay toggle, share/export).
- **Bottom/dock:** Animation timeline (collapsible), history panel (with actor badges: human/agent).

### 9.2 Tools (each with tooltip + shortcut)
Select/Move, Direct select (vertices/anchors), Line-arms, Dot, Rect, Circle, Ring, Arc, Polygon, Chevron, T-shape, Plus, Pen (path), Pixel brush, Pixel eraser, Pixel fill, Eyedropper, Hand/Pan, Zoom, Ruler/Measure, Mirror guide. Tools create layers with sensible defaults and can be configured via the inspector.

### 9.3 Layers panel
Reorder (drag + keyboard), duplicate, group/ungroup, rename, hide/solo/lock, multi-select, copy/paste (also across crosshairs), mirror/flip actions, alignment helpers (align to center, distribute), delete with undo.

### 9.4 Fine nudging (hard requirement)
Nudging must be available for: **the whole crosshair**, **selected layer(s)**, **a selected arm** of a symmetric/multi-arm layer, and **a selected vertex/anchor** (path/polygon) — in **every direction** (up, down, left, right, and the 4 diagonals).
- **Nudge Pad control:** 8-direction buttons + center "reset" button, step chips `0.1 | 0.25 | 0.5 | 1 | 5 | 10` px, a custom step input (min 0.01), a **Target** selector (Whole crosshair / Selection / Arm / Anchor), and a **Frame** selector (Screen axes / Layer-local axes after rotation).
- **Keyboard:** Arrow keys = current step; `Shift+Arrow` = ×10; `Alt+Arrow` = 0.1; hold to repeat (initial delay 300 ms, repeat 30 ms); `Ctrl+Arrow` = nudge the whole crosshair regardless of selection.
- **Numeric fields** for X/Y with scrub-drag, arrow up/down, and mouse-wheel stepping; also direct typing.
- Sub-pixel nudges only produce visible movement in **Smooth** mode or when scale ≥ 1 output pixel per step; in Crisp mode show a hint ("Crisp mode snaps to whole output pixels; use Smooth for sub-pixel movement") rather than silently doing nothing.
- Nudge commands coalesce into one undo step per burst.
- Same operations exposed via MCP `nudge` (Section 13).

### 9.5 Color
Picker with RGB/HSV/HSL/Hex/CMYK inputs, alpha, eyedropper (from the editor canvas/palette only), recent colors, saved swatches per library, colorblind-safe preset palettes, harmony helpers, copy/paste hex. Gradient editor with draggable stops, type switch (linear/radial/conic), angle/center controls, and stop alpha.

### 9.6 Other screens (all in design, Section 17)
Library grid view, Import/Export dialogs, Resolution Profiles manager, Calibration Wizard, Hotkeys settings, MCP settings (status, token, copy-paste client config, live activity log), Dynamic Reticle + Gun Profile editor, Spray Lab, Settings, Onboarding, Transparency/Compliance panel, About/Docs.

### 9.7 Accessibility & UX
Full keyboard operation, visible focus rings, ARIA roles, high-contrast theme, reduced-motion support, scalable UI (80–150%), undo/redo everywhere, never lose work (autosave every 2 s after change, crash-safe atomic writes).

### 9.8 Shortcut map
Define in one file `ui/src/lib/shortcuts.ts`; surfaced in tooltips and a searchable "Keyboard shortcuts" dialog. Include: undo/redo, copy/paste/duplicate, delete, group, select all, zoom in/out/fit/1:1, toggle grid/rulers/guides, tool keys, toggle overlay preview, save, export, command palette (`Ctrl+K`) — the **command palette lists every registered command** (a free human-side mirror of MCP).

---

## 10. Library, Profiles, Slots, Auto-activation

### 10.1 Storage (no cloud, plain files, human-readable)
```
%APPDATA%/xhair/
├─ settings.json
├─ library/
│  ├─ index.json                  # folders, ordering, slots (lightweight)
│  ├─ crosshairs/<uuid>.xhair     # one file per crosshair (JSON)
│  └─ assets/<hash>.png|svg       # content-addressed (also embedded on export)
├─ profiles/resolutions.json
├─ profiles/guns.json
├─ backgrounds/                   # user preview images
└─ logs/                          # rotating, no input data
```
- Atomic writes (temp file + rename), `fsync` where supported; keep one `.bak` per file.
- **Lazy loading:** keep only `index.json` + thumbnails metadata in memory; load a crosshair fully only when opened/activated. Thumbnails rendered on demand and cached in a bounded LRU (≤ 8 MB).
- File watcher (`notify`) reloads on external edits; emits `LibraryChanged`.
- Portable mode: if `xhair.portable` exists next to the exe, store data beside the exe.

### 10.2 Organization
Unlimited crosshairs; folders (nested), tags, favorites, search (name/tag/author/description), sort (recent/name/created), duplicate, bulk move/tag/delete/export, trash with restore for 30 days (setting).

### 10.3 Slots & contextual profiles
- **Slots 1–20** map to crosshairs and optional hotkeys → instant mid-game switching.
- "Per-agent / per-map / per-gun" is implemented as **tags + slot groups** (e.g., group "Vandal", "Operator") and optional **per-gun hotkeys**; the app cannot detect these from the game (by design), so switching is user-initiated by hotkey or MCP.
- Slot groups: a hotkey cycles within a group.

### 10.4 Auto-activation
- Show overlay only when a configured process is foreground (8.1).
- Optional rule: *profile per monitor*.
- Remember the last active crosshair and restore on launch.

---

## 11. Import / Export / Sharing (no cloud)

### 11.1 Formats
| Artifact | Extension | Content |
|----------|-----------|---------|
| Single crosshair | `.xhair` | JSON, versioned, assets embedded (base64) |
| Collection | `.xhairpack` | ZIP: `manifest.json` + `*.xhair` + optional previews |
| Share code | text | `XH1-<base64url(brotli(canonical_json))>-<crc32>` |
| Preview image | `.png` | Rendered preview (optionally with the `.xhair` embedded in a PNG text chunk so the image itself is importable) |
| Raw art in | `.png`, `.svg` | Becomes a `Pixels`/`Image`/`Path` layer |
| Game-code in | text | Best-effort import of the community-documented in-game crosshair profile code |

### 11.2 Requirements
- **Canonical serialization:** stable key order, no insignificant whitespace, floats normalized (max 4 decimals) so the same crosshair always yields the same code.
- **Share code limits:** if the compressed code exceeds 8 KB (e.g., large embedded images), show a clear message and fall back to file export.
- **Versioning:** `format_version` integer; `xhair-format` contains ordered **migrations** `vN → vN+1`; reject newer-than-known versions with an actionable message; keep old fixtures in `tests/fixtures/formats/` and test every migration.
- **Validation on import (untrusted input):** JSON Schema validation, numeric range clamping, limits (layers ≤ 256, nesting ≤ 8, total assets ≤ 4 MB, single pixel layer ≤ 256×256, path points ≤ 10 000, strings ≤ 256 chars for names, ≤ 4 KB for description). Reject, never panic. **Fuzz** the importers (`cargo-fuzz`) and property-test round trips (`export → import == original`).
- **Import dialog:** paste a code/drop a file/choose a file; show a **preview render**, name, author, any warnings/clamps, and a choice: *Import as new / Replace existing / Cancel*. Never auto-overwrite.
- **Export dialog:** choose single/multi, file vs code vs PNG-with-data; copy-to-clipboard button; optional "include author/description".
- **Drag & drop** files onto the app/tray window.
- **Valorant code import:** the in-game format is a community-documented, semicolon-separated key/value string. **Research the current format**, implement a parser in `xhair-format/src/valorant.rs` mapping fields (inner/outer lines, center dot, outline, colors, gaps, thickness, firing/movement error bits where meaningful) to editable layers, mark unmappable fields as warnings, and keep the parser **isolated and fully fixture-tested** because the format can change. Also implement the reverse export **only if** the mapping is lossless for the supported subset (otherwise skip and document why).
- No network access anywhere in import/export.

---

## 12. Dynamic Reticle & Spray Lab (flagged, opt-in)

> Read Section 2 point 4 first. Build **Spray Lab before** the in-overlay Dynamic Reticle.

### 12.1 Concept
Layers can be **bound** to a *driver* (e.g., "movement", "firing", "spread") so they move/scale/fade based on the user's own simulated state. `xhair-dynamics` is a **pure simulation** crate: input-state in → driver values out. It knows nothing about the game.

### 12.2 Inputs (only when opt-in is enabled)
- Raw Input restricted to **user-bound** keys/buttons (e.g., W/A/S/D, Shift, Ctrl, Space, Mouse1). Everything else is discarded at the earliest point. No logging, no persistence, no network.
- Input source is a trait (`InputSource`) so tests inject synthetic sequences.

### 12.3 State machine
States and timers: `Idle`, `Walking`, `ShiftWalking`, `Crouching`, `Airborne(timer)`, `Firing{shot_index, since_last_shot}`. Transitions are configurable per **Gun Profile**.

### 12.4 Gun Profile (user-editable, no shipped game numbers)
```text
GunProfile {
  id, name, notes,
  fire_rate_hz, magazine,
  spread: { base, per_shot, max, recovery_per_sec,
            move_penalty, shift_move_penalty, crouch_bonus, air_penalty },
  smoothing: { attack_ms, release_ms },
  pattern: Vec<Vec2>        // per-shot offset (x,y) for Spray Lab / pattern visualization
}
```
Provide a few **empty templates** (e.g., "Rifle", "SMG", "Pistol") with neutral placeholder values clearly labeled "example, not real game data".

### 12.5 Bindings
`DynamicBinding { property_path, driver, curve (linear/ease/custom points), in_range, out_range, mode: Add|Set|Multiply }`.
Examples: `arm.gap += spread × 1.0`; `global.scale *= 1 + spread × 0.02`; `layer.opacity` fades while moving; color lerps from A→B with spread. Each binding has a live **preview scrubber** in the editor (drag a fake "spread" slider to see the effect, no input needed).

### 12.6 Spray Lab (always safe)
A **standalone window** (not an overlay) with a plain backdrop: choose a gun profile, press "Fire" to animate the pattern, scrub shot index, show the pattern path, compare to the crosshair's dynamic response, export the pattern as PNG. Everything simulated.

### 12.7 Runtime behavior
Only active when: opt-in accepted this session, Static-only lock is off, and a dynamic crosshair is active. Simulation ticks at ≤ 120 Hz *only while a driver is non-idle*; returns to zero-cost idle afterwards. Re-render only when the output changes by ≥ 0.05 px.

---

## 13. MCP Server (`xhair-mcp`)

### 13.1 Principle
**Tools are generated from the command registry** (Section 5.1). Adding a command automatically adds an MCP tool with the right JSON Schema and description. A CI test enforces: *every command is exposed; every UI action maps to a command; every tool has examples in docs*.

### 13.2 Transports
- **stdio:** `xhair-cli mcp` or `xhair.exe --headless --mcp-stdio`. If the app is running, the stdio process becomes a **thin proxy** to it.
- **Streamable HTTP:** bound to `127.0.0.1` only, configurable port (default random free port, shown in the UI), off by default, enabled in MCP settings.
- Follow the current MCP spec (verify: initialization, capabilities, tools, resources, prompts, progress, cancellation, structured content, image content).

### 13.3 Tools (grouped; names are stable API — document each in `docs/MCP.md` with examples)
**Library:** `library.list`, `library.search`, `crosshair.get`, `crosshair.create` (from blank/template/description JSON), `crosshair.duplicate`, `crosshair.delete`, `crosshair.rename`, `crosshair.set_metadata` (tags, favorite, author, description), `folder.create|rename|move|delete`, `crosshair.move_to_folder`, `trash.list|restore|empty`.

**Editing:** `edit.batch` (atomic list of commands), `layer.add`, `layer.update`, `layer.remove`, `layer.reorder`, `layer.group`, `layer.ungroup`, `layer.duplicate`, `layer.set_visibility|lock`, `symmetry.set`, `global.set`, `paint.set` (fill/stroke/gradient), `effects.set` (glow/shadow/blur/bloom), `animation.set`, `pixels.paint` (rects/points/fill, with color), `asset.import_png|svg` (from path or base64), **`nudge`** (`target: crosshair|layer|arm|anchor`, `ids`, `dx`, `dy`, `step`, `frame`), `history.get`, `undo`, `redo`.

**Preview & analysis:** `render.preview` (returns **PNG image content**: options zoom, background color/pattern/imported image, show grid/center/rulers, resolution profile), `analyze.alignment` (the 6.2 report), `analyze.visibility` (contrast vs listed backgrounds), `analyze.bounds`, `validate` (schema + limits).

**Overlay (App or Headless+overlay):** `overlay.get_state`, `overlay.set_enabled`, `overlay.activate` (by id or slot), `overlay.next|previous`, `overlay.set_visibility_mode`, `display.list`, `calibration.show|hide`, `calibration.set_offset`.

**Resolution & hotkeys:** `resolution.list|create|update|delete|activate`, `slot.assign|clear|list`, `hotkey.set|clear|list`.

**Import/Export:** `export.file`, `export.code`, `export.png`, `export.pack`, `import.file`, `import.code`, `import.pack`, `import.valorant_code`, `import.preview` (parse + validate without saving).

**Dynamics:** `gun.list|create|update|delete`, `binding.set|remove`, `dynamics.simulate` (synthetic input sequence → driver samples), `dynamics.set_enabled` (requires explicit `confirm_risk: true` argument and the human-side opt-in).

**Settings & meta:** `settings.get|set`, `meta.capabilities` (mode, version, which tools are available), `meta.schema` (JSON Schema + tooltips for all properties), `meta.docs`, `meta.shortcuts`.

### 13.4 Availability matrix
Each command declares `availability`. Overlay/display/hotkey commands return a structured error `{code: "unavailable_in_mode", mode, hint}` in plain CLI mode; they work in Headless **with** `--overlay` and in App mode. Everything else works everywhere. `meta.capabilities` tells the agent exactly what it can do right now.

### 13.5 Resources
`xhair://library`, `xhair://crosshair/{id}` (the `.xhair` JSON), `xhair://crosshair/{id}/preview.png`, `xhair://schema/crosshair`, `xhair://settings`, `xhair://resolution-profiles`, `xhair://docs/{topic}`.

### 13.6 Prompts
`design_crosshair_from_description`, `tune_for_visibility`, `convert_game_code`, `build_calibration_plan`, `make_symmetric_variants`. Each returns a short instruction template that tells the agent which tools to call and to verify with `render.preview` + `analyze.alignment`.

### 13.7 Security & safety
- Bind to loopback; require a **bearer token** (random 256-bit, stored in settings, rotatable from the UI; stdio needs none). Validate `Origin`/`Host` headers to block DNS-rebinding. Rate-limit (default 60 calls/s). 
- **File access:** import/export paths limited to user-approved directories (default: Documents/xhair, Downloads, a user-chosen folder); path canonicalization and traversal checks; refuse symlink escapes.
- **Destructive commands** (`crosshair.delete`, `trash.empty`, bulk operations, overwrite-import) support `dry_run`; app mode can optionally require human confirmation (setting: *Ask me before agents delete things*, default on).
- The MCP panel shows a **live activity log** (tool, args summary, actor=agent, result), and edits made by an agent appear in the editor live with an "Agent is editing" indicator and `agent` badges in history.
- Concurrency: commands serialize through the bus; if human and agent edit the same crosshair, last-write-wins at the *command* level (not file level); the UI shows a toast and offers undo.

### 13.8 Parity test (CI)
`tests/parity.rs`: loads `ui/src/lib/actions.ts` manifest (exported to JSON at build), the command registry, and the MCP tool list; fails if any UI action has no command, any command has no tool, or any tool lacks docs/examples.

---

## 14. Settings

General: start with Windows (via Run registry key, off by default), start minimized, language (i18n-ready, English first), theme (dark/light/high-contrast), UI scale, reduce motion, check nothing online (no update checks).
Overlay: visibility mode, game process list, AA default, max FPS, static-only lock, exclude-from-capture, follow game window, active monitor.
Hotkeys: all bindings (Section 8.4).
Library: data location, autosave interval, trash retention, thumbnail cache size.
MCP: enable HTTP, port, token (view/rotate), allowed directories, ask-before-delete, activity log size.
Advanced: log level, performance HUD (shows RSS, render ms, FPS — **only inside the editor**), reset, export/import settings, "Open data folder".
Dynamic Reticle: opt-in status, bound inputs list, gun profiles.

---

## 15. Code Quality, Documentation, Testing

### 15.1 Code rules
- Rust: `rustfmt` + `clippy -D warnings` (pedantic subset documented), `#![forbid(unsafe_code)]` everywhere **except** `xhair-platform-win`, where each `unsafe` block has a `// SAFETY:` comment and is wrapped in a safe API.
- No `unwrap()`/`expect()` in non-test code (use typed errors with `thiserror`; `anyhow` only in binaries).
- Functions ≤ 60 lines, files ≤ 400 lines, modules cohesive; no god objects; no global mutable state (pass context explicitly).
- Every public item has rustdoc with an example where useful; `#![deny(missing_docs)]` on library crates.
- TypeScript: `strict`, ESLint + Prettier, no `any`, components ≤ 200 lines, logic in stores/lib not in components, no inline tooltip strings.
- Naming: domain terms from Section 4 exactly; units in names or newtypes (`gap_px`).
- Errors: user-facing messages are human-readable with a recovery hint; internal errors carry codes (`XH-1xxx`) documented in `docs/ERRORS.md`.
- Logging: `tracing`, structured, **never** logs input events or file contents.
- Dependencies: justify each in `docs/DEPENDENCIES.md`; prefer fewer; run `cargo deny` + `cargo audit` in CI; pin versions with a lockfile.

### 15.2 Documentation deliverables (keep updated each milestone)
```
README.md                 Quick start, screenshots, install
docs/ARCHITECTURE.md      Diagrams from Section 3, crate responsibilities
docs/GEOMETRY.md          Section 6 math with worked examples (even/odd, stretch, DPI)
docs/FORMAT.md            .xhair / .xhairpack / share code specs + migrations
docs/MCP.md               Every tool, args, examples, error codes, client configs
docs/PROPERTIES.md        Generated from descriptors
docs/COMMANDS.md          Generated from registry
docs/COMPLIANCE.md        Section 2 + Transparency list of OS APIs
docs/DYNAMICS.md          Section 12 model and formulas
docs/PERFORMANCE.md       RAM/CPU budgets and how they are measured
docs/TESTING.md           Strategy and how to run each suite
docs/CONTRIBUTING.md      Style, commit conventions, how to add a shape/command
docs/ERRORS.md            Error code catalog
CHANGELOG.md              Keep a Changelog format
```
Include in `CONTRIBUTING.md` a worked example: "Add a new shape in 5 steps" (model → descriptor → renderer → tool → test) to prove the architecture stays simple.

### 15.3 Testing matrix
| Layer | What |
|-------|------|
| `geometry` | Property tests: alignment, mapping round trips, stretch, DPI 100–500%, odd/even W/H, calibration offsets |
| `model`/`format` | Schema validation, migrations, canonical form, round trips, fuzz |
| `commands` | Undo/redo invariants, coalescing, batch atomicity, dry-run equals real diff |
| `render` | Golden images (native + WASM identical), perf benchmarks |
| `dynamics` | Deterministic simulation with synthetic input, state-machine transitions |
| `mcp` | Spawn headless server, run scripted sessions over stdio and HTTP, token/origin/path-security tests |
| `platform-win` | Windows CI runner: window styles correct, click-through verified, DPI-aware manifest present, center equals expected for several mocked monitors |
| UI | Vitest for stores; Playwright smoke: open editor, draw a line, nudge 0.5 px, undo, export code, import it, tooltips present |
| Parity | Section 13.8 |
| **Memory** | CI step launches the app headless with an active crosshair, waits 30 s, asserts private working set ≤ 20 MB (warn at 15); opens/closes the editor and asserts memory returns near baseline |

### 15.4 CI (`.github/workflows/ci.yml`)
Jobs: fmt/lint, unit, wasm build + golden parity, fuzz smoke (60 s), Windows integration, UI build + tests, memory check, docs generation check (fails if generated docs are stale), release artifacts (installer + portable zip, unsigned unless a signing secret exists).

---

## 16. Milestones (execute in order; each ends with tests passing, docs updated, a short report)

**M0 — Design (FigCraft).** Section 17. **Stop for human approval.**

**M1 — Foundations.** Workspace, `model`, `geometry`, `commands`, undo/redo, schema + property descriptors, settings model, error types, tests.
*Accept:* geometry property tests pass; command batch/undo tests pass; schemas generated.

**M2 — Renderer + CLI.** `xhair-render` all shapes, paints, strokes, symmetry, AA modes, effects (glow/shadow/blur), `xhair-cli render` → PNG.
*Accept:* golden tests for every shape; native/WASM identical; benchmarks under thresholds.

**M3 — Library + format.** Storage, folders/tags/slots, `.xhair`, `.xhairpack`, share code, migrations, validation, fuzz, PNG/SVG import, Valorant code import (best effort).
*Accept:* round-trip property tests; fuzz 10 min clean; fixtures for every limit.

**M4 — Overlay daemon.** Layered window, resolution profiles, DPI, stretch, center math wired end-to-end, tray, hotkeys, foreground detection, game-window following, calibration pattern.
*Accept:* verified on 100%/125%/150% scaling, 1080p/1440p/stretched 4:3 profiles; idle RAM ≤ 20 MB; zero CPU idle (report numbers).

**M5 — Editor UI.** Implement the approved design: library, grid canvas, all tools, layers, inspector (descriptor-driven), color/gradient, nudge pad (all targets/frames), tooltips everywhere, shortcuts, command palette, backgrounds + visibility score, autosave, live overlay preview toggle.
*Accept:* Playwright smoke; tooltip coverage test; editor open/close memory test.

**M6 — MCP.** Registry-generated tools, stdio + HTTP, resources, prompts, security, proxy-to-running-app, headless (+overlay), activity log UI, docs with examples, parity test.
*Accept:* scripted agent session creates a crosshair, previews it, nudges a single arm by 0.5 px, activates it on the overlay, exports a code, and re-imports it — all via MCP, in both headless and app modes.

**M7 — Effects & animation polish.** Gradients (all types), bloom, animation timeline/tracks, static-only lock, colorblind palettes, performance tuning.
*Accept:* heavy-crosshair render < 8 ms; animated crosshair CPU usage reported and bounded.

**M8 — Calibration.** Wizard, offsets per profile, Screenshot Align, alignment badges and fixes.
*Accept:* documented manual verification with screenshots on at least two resolutions.

**M9 — Dynamics (flagged).** Spray Lab first, then opt-in Dynamic Reticle, gun profiles, bindings UI, Raw Input restricted source, indicators, compliance dialog.
*Accept:* deterministic simulation tests; verified that nothing is read or stored when disabled; Transparency panel lists exactly the APIs in use.

**M10 — Packaging & hardening.** Installer + portable build, start-with-Windows, crash handling (panic hook writes a local report, no network), final docs pass, performance pass, accessibility pass, `CHANGELOG`, release checklist.
*Accept:* clean-machine install/uninstall test; all CI green; docs complete and consistent with code.

---

## 17. Phase 0 — Design Brief for FigCraft

### 17.1 Instructions to the agent
1. Discover which FigCraft MCP tools are available (list them first; do not assume names).
2. Create **one Figma file/page set** named "XHAIR". Build **design tokens** (colors, typography, spacing, radii, shadows, motion), a **component library** (with variants/states), then **all screens** below at a **desktop size of 1440×900** and a **minimum size of 1100×700**, plus key flows as prototype links if supported.
3. Export into `design/`:
   - `design/tokens.json` (colors, type scale, spacing, radius, elevation, motion durations)
   - `design/components/` (PNG per component + `components.md` with props/states)
   - `design/screens/` (PNG per screen at 1× and 2×)
   - `design/README.md` — table mapping **screen → route → components → data/commands used**, plus FigCraft file/node IDs for every frame
4. If any export step isn't supported by the available tools, document node IDs and link in `design/README.md` and tell the human.
5. **Stop and present a summary** with screenshots. Ask for approval or changes. Iterate until approved.

### 17.2 Visual direction (adjust only if the human asks)
Dark, dense-but-clean "pro tool" aesthetic suited to gamers and designers: near-black neutral surfaces, one vivid accent (default a bright cyan/green; must be user-themeable later), high-contrast text, 8-pt spacing grid, 6–8 px radii, subtle 1 px borders, minimal shadows, crisp monospace for numeric fields, clear selected/hover/focus/disabled states, motion limited to ≤150 ms fades/slides. Provide **light** and **high-contrast** variants of the tokens.

### 17.3 Components to design (with all states: default/hover/active/focus/disabled/error)
Buttons (primary/secondary/ghost/icon), toggle, checkbox, radio, slider (with numeric field + fine-step affordance), number input with scrub handle, dropdown, segmented control, tabs, chips/tags, search field, tooltip (title + description + range/default + shortcut badge + learn-more), toast, modal, popover, context menu, command palette, tree/list rows (library + layers), thumbnail card, color swatch, **color picker**, **gradient editor**, **nudge pad** (8-way + center + step chips + target/frame selectors), grid-canvas chrome (rulers, zoom control, overlay toggles), layer row, timeline track, badge (aligned/off-grid/agent/human), status pill (Overlay ON, MCP ON, Dynamic ON), empty states, skeletons, error banners.

### 17.4 Screens to design
1. **Onboarding / first run** (3–4 steps: pick monitor & game resolution, windowed/borderless note, create first crosshair or import, hotkeys, compliance note).
2. **Tray menu** and tray tooltip states.
3. **Main window shell** (library sidebar, canvas, inspector, layers, document bar, dock).
4. **Library view** (grid + list), folders, tags, search, favorites, bulk-select toolbar, trash.
5. **Editor — default state** with a typical crosshair, grid, rulers, center marker, alignment badges.
6. **Editor — zoom 16×/32× pixel view** showing edge-vs-pixel-middle center indication.
7. **Editor — pixel paint mode** (brush, eraser, fill, palette).
8. **Editor — path/anchor edit mode** with anchor nudging.
9. **Nudge states:** whole crosshair, layer, single arm, single anchor (show Target/Frame selectors).
10. **Color picker popover** and **gradient editor**.
11. **Effects panel** (glow, shadow, blur, bloom) with live preview.
12. **Animation timeline** dock (tracks, keyframes, easing, static-only lock indicator).
13. **Preview backgrounds** picker + visibility score.
14. **Resolution Profiles manager** (presets + custom W×H, display mode, stretch, DPI info, monitor binding, even/odd alignment hint).
15. **Calibration Wizard** (4 steps) and **Screenshot Align**.
16. **Import dialog** (paste code/drop file/Valorant code) with preview and warnings; **Export dialog** (file/code/PNG/pack, copy button).
17. **Hotkeys settings** with conflict states and slot assignment.
18. **MCP settings:** status, mode, port, token (reveal/rotate), allowed folders, client config snippets (copy buttons), **live activity log**, "Agent is editing" banner state in the editor.
19. **Dynamic Reticle opt-in dialog** (clear risk language), **Gun Profile editor**, **Bindings editor** with scrubber preview.
20. **Spray Lab** window.
21. **Settings** (all groups from Section 14).
22. **Transparency / Compliance panel** (list of OS APIs used, what is never accessed).
23. **Keyboard shortcuts** dialog and **Command palette**.
24. **Error/empty/offline-like states** (no monitor found, game elevated warning, hotkey conflict, import failure, file too large).
25. **Performance HUD** (editor-only).

### 17.5 Design acceptance checklist (agent self-checks before presenting)
- [ ] Every tool/control in Section 9 appears in some screen.
- [ ] Every tooltip variant is shown at least once.
- [ ] The grid view clearly distinguishes pixel edges, pixel centers, and the true center.
- [ ] The nudge pad is reachable from every editing context.
- [ ] All states for each component exist.
- [ ] Tokens exported and referenced by every component (no hard-coded colors).
- [ ] `design/README.md` maps each screen to routes and commands.

**After presenting, STOP. Wait for the human's approval before M1.**

---

## 18. Definition of Done (whole project)

- [ ] All milestones M1–M10 accepted.
- [ ] Idle overlay ≤ 20 MB RAM, ~0% CPU static; editor closes and releases memory.
- [ ] Center verified at ≥ 3 resolutions (including one stretched and one custom) and ≥ 2 DPI scales with documented calibration.
- [ ] Any custom `W×H` accepted; any monitor/DPI combination handled without shifting the center.
- [ ] Fine nudge works for crosshair, layer, arm, anchor in all 8 directions with 0.1 px minimum step.
- [ ] Every control has a tooltip (CI enforced).
- [ ] Export/import via file, pack, share code, PNG-with-data all round-trip; Valorant code import works for the documented subset with fixtures.
- [ ] MCP parity test green; headless and app modes both pass the scripted agent session; security tests green.
- [ ] Dynamic Reticle opt-in only, Spray Lab functional, Transparency panel accurate.
- [ ] No network calls anywhere (verified by a test that blocks sockets except loopback MCP).
- [ ] All docs in 15.2 exist, are accurate, and generated docs are current.
- [ ] Clean `clippy`/`eslint`, no `unwrap` in production paths, no TODOs without linked issues.

---

## Appendix A — Worked Example: center alignment

`1920×1080`, line thickness `2`: `(1920+2)` even → edges at `960±1` = integers ✅ aligned.
Thickness `1`: `(1920+1)` odd → edges at `959.5`, `960.5` ⚠ straddles two pixels (blurry in Smooth, off by half a pixel in Crisp). Fix: offset by `0.5` px or use thickness 2.
`1921×1081` (odd): thickness `1` → `(1921+1)` even ✅ and the line sits exactly on the center pixel.
Stretched `1280×1024` on a `1920×1080` output: `sx=1.5`, `sy≈1.0547`; a 1 game-px line is 1.5 output px wide and ≈1.05 tall — this is expected and matches how the game renders; offer the "native pixels" toggle for users who want an undistorted look.

## Appendix B — Example MCP session (for docs/MCP.md)
1. `meta.capabilities` → confirm mode and available tools.
2. `crosshair.create` `{name:"Cyan Gap", template:"blank"}`.
3. `edit.batch` → add 4-arm `Line` (length 5, thickness 2, gap 4), add `Dot` (2 px), set fill cyan, add glow radius 3.
4. `analyze.alignment` `{resolution_profile:"1080p"}` → fix any ⚠ items.
5. `render.preview` `{zoom:16, background:"noise-green", grid:true}` → inspect the returned image.
6. `nudge` `{target:"arm", layer:"...", arm_index:0, dx:0, dy:-0.5, step:0.5}`.
7. `overlay.activate` `{id:"..."}` (app/headless+overlay).
8. `export.code` → share code string.

## Appendix C — Open questions the agent should NOT guess (ask the human once, early in M1)
1. Final product name and icon direction.
2. Preferred accent color for the default theme.
3. Whether to sign binaries (certificate available?).
4. Whether the installer should be MSI, NSIS, or MSIX (default: NSIS + portable ZIP).
