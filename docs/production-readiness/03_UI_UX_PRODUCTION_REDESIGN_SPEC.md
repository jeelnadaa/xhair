# UI / UX Production Redesign Specification

**Project:** XHAIR — Studio Interface & User Experience  
**Target:** Production-Grade Professional Desktop Application (Figma / Linear / Raycast standard)

---

## 1. Problem Statement: Why the Current UI Needs Redesign

While the application features comprehensive domain coverage, the current user interface suffers from significant ergonomic friction:

1. **The 11-Tab Dock Nightmare:**
   The right sidebar requires users to horizontally scroll through 11 separate tabs (`Design`, `Shape`, `Fill & Stroke`, `Effects`, `Color`, `Layers`, `Nudge`, `Animation`, `Presets`, `Properties`, `MCP Activity`).
   - To change a line's length and color, the user must click `Shape`, edit the length, click `Color`, pick a color, click `Design`, and adjust the position.
   - No professional design tool (Figma, Spline, Blender, Photoshop) separates basic object properties across disjoint tabs.
2. **Canvas Interaction Deadlock:**
   Direct mouse manipulation on the canvas is blocked by a state collision where `isMarquee` is permanently true, preventing users from dragging shapes.
3. **Modal Sprawl:**
   Ten separate modal windows pop over the canvas, fragmenting the workflow instead of integrating naturally into collapsible docks or trays.
4. **Browser Primitive Leaks:**
   Using browser `window.prompt()` for creating folders and uncoalesced mousemove events that flood undo history.

---

## 2. The 3-Column Studio Blueprint

The redesigned interface adopts a **modern, unified studio architecture**:

```
┌────────────────────────────────────────────────────────────────────────────────────────────────────────┐
│  [XHAIR] / Valorant / Cyan Gap [Saved ✓]          [ Zoom: 8× ▾ ] [ Grid ] [ Rings ]   [Overlay ON] [⚙] │
├──────────────────────┬──────────────────────────────────────────────────────────┬──────────────────────┤
│  EXPLORER (260px)    │                  CANVAS VIEWPORT (Center)                │   INSPECTOR (320px)  │
│                      │                                                          │                      │
│  ▼ GROUPS            │   ┌────────┐                                             │  ▼ TRANSFORM         │
│  📁 Valorant (8)     │   │ V L D  │   ┌───┐                                      │    X: [ +0.0 ] px    │
│  📁 CS2 (4)          │   │ R C P  │   │ + │ Interactive Crosshair with           │    Y: [ +0.0 ] px    │
│  📁 Apex Legends (3) │   └────────┘   └───┘ Bounding Box Handles                 │    Rotation: 0°      │
│                      │                                                          │    Opacity: 100%     │
│  ▼ LAYERS            │                                                          │                      │
│  👁 🔒 Arms ×4 (Line)│                                                          │  ▼ GEOMETRY (Line)   │
│  👁 🔒 Dot (Center)  │                                                          │    Length: 6.0 px    │
│  [+ Add Layer ▾]     │                                                          │    Thickness: 2.0 px │
│                      │                                                          │    Gap: 4.0 px       │
│                      │                                                          │                      │
│                      │                                                          │  ▼ FILL & STROKE     │
│                      │                                                          │    Color: [#36F2CB]  │
│                      │                                                          │    Cap: [ Butt ▾ ]   │
│                      │                                                          │                      │
│                      │                                                          │  ▼ EFFECTS           │
│                      │                                                          │    Glow: 0.0 px      │
│                      │                                                          │    Shadow: None      │
├──────────────────────┴──────────────────────────────────────────────────────────┴──────────────────────┤
│  ▼ BOTTOM DOCK (Collapsible): [ Timeline ]  [ Spray Lab ]  [ Calibration ]  [ MCP Agent Log ]          │
└────────────────────────────────────────────────────────────────────────────────────────────────────────┘
```

---

## 3. Left Explorer Rail (260px)

The Left Rail unifies **Project Management** and **Document Hierarchy** into two clean vertical sections:

### 3.1 Crosshair Library & Groups
- **Group Folders:** Expandable folder rows with color badges and game process associations (`VALORANT-Win64-Shipping.exe`).
- **In-Place Group Creation:** Clicking `+ Group` displays an inline input box with Enter/Esc handling (replacing `window.prompt()`).
- **Profile Search:** Instant fuzzy search by profile name, tag, or author.
- **Quick Action Menu:** Right-click context menu: Duplicate, Export Share Code, Move to Group, Delete (move to Trash).

### 3.2 Layers Panel
- **Tree Hierarchy:** Shows layers and layer groups with drag-and-drop reordering.
- **Quick Controls:**
  - Visibility toggle (eye icon).
  - Lock toggle (padlock icon).
  - Solo toggle (isolates layer for focused editing).
  - Alignment Badge: `ALIGNED` (cyan) or `OFF-GRID` (warning pill with 1-click automatic fix).
- **Add Shape Dropdown:** Direct 1-click insertion for all 14 shapes: Line Arms, Center Dot, Circle, Ring, T-Shape, Plus, Rectangle, Chevron, Triangle, Polygon, Star, and Custom Path.

---

## 4. Center Canvas Viewport

### 4.1 Direct Manipulation Engine
- **Selection & Transform Box:** Clicking any layer renders an accent bounding box with:
  - 4 corner resize handles.
  - Center translation anchor.
  - Sub-pixel quarter-pixel (0.25 px) precision snapping.
- **Mouse Drag Workflow:**
  - Clicking on a layer and dragging moves it immediately with smooth 120fps feedback.
  - Holding `Shift` constrains movement to pure horizontal or vertical axes.
  - Holding `Alt` duplicates the layer on drag.
  - Dragging from an empty canvas area creates a marquee selection box.
- **Middle-Click or Space+Drag:** Smooth infinite pan.
- **Mouse Wheel:** Smooth geometric zoom (`1×` to `64×`) centered on cursor.

### 4.2 Floating HUD & Visual Tools
- **Tool Palette:** Compact floating pill on the left:
  - `V` Select / Transform
  - `L` Line Arms
  - `D` Center Dot
  - `R` Rectangle
  - `C` Circle / Ring
  - `T` T-Shape
  - `+` Plus
  - `P` Vector Path
- **Bottom Status Bar:**
  - Game coordinates: `X: +0.0 px · Y: +0.0 px`.
  - Screen pixel coordinates: `Pixel: [960, 540]`.
  - Contrast score: Live WCAG luminance score against active background (e.g. `14.2:1 AAA Pass`).
  - Tactical preview backdrop selector: `Dark`, `Light`, `Foliage`, `Desert`, `Urban`.

---

## 5. Right Unified Contextual Inspector (320px)

Eliminates the 11-tab horizontal dock. When a layer is selected, all its parameters appear in a single, beautifully organized, collapsible accordion:

### 5.1 Transform & Position Section
- **Figma-Style Scrubbable Inputs:** Dragging the `X` or `Y` label scrubs the value left and right; clicking allows direct numerical input.
- **Relative vs Absolute Toggle:** Inspect position relative to group parent or absolute screen center.
- **Recenter Buttons:** Instant 1-click recentering to `(0, 0)`.
- **Rotation:** Compact slider with degree readout (0°–360°).
- **Opacity:** Percentage slider (0%–100%).

### 5.2 Shape Geometry Section
Dynamically renders the parameters corresponding to the selected shape kind:
- **Line Arms:** Length, Thickness, Gap, Arm Count (1–8), Taper, Start Angle.
- **Dot:** Diameter, Shape (Square vs Round).
- **T-Shape:** Bar Length, Stem Length, Thickness, Gap.
- **Circle & Ring:** Radius, Thickness, Center Hole.
- **Rectangle:** Width, Height, Corner Radii (individual 4-corner controls).
- **Polygon & Star:** Points, Outer Radius, Inner Radius.
- **Path:** SVG Path data input with syntax validation.

### 5.3 Fill & Stroke Section
- **Color Swatch & Quick Presets:** 8 curated high-visibility competitive swatches + 8 colorblind-safe swatches.
- **Interactive HSV Popover:** Saturation/Value canvas, hue rainbow slider, and alpha transparency over checkerboard.
- **Coalesced Undo Transactions:** Mouse drag events in color pickers push a single undo snapshot upon mouse release, preventing history flooding.
- **Stroke Width & Cap:** Butt, Round, Square cap options.

### 5.4 Visual Effects Section
- **Glow:** Radius slider, glow color picker, additive bloom toggle.
- **Drop Shadow:** Blur radius, color, X and Y offsets.
- **Dynamic Spread Scrubber:** Test gap expansion dynamically without needing game input.

---

## 6. Collapsible Bottom Dock (Diagnostics & Labs)

The bottom dock remains collapsed by default to maximize canvas editing space, and expands upwards upon clicking bottom tabs:

1. **Animation Timeline:** Keyframe tracks, playhead scrubber, loop toggle, and static-only lock compliance indicator.
2. **Spray Lab:** Recoil spread simulator, weapon profile selection (Rifle, SMG, Pistol), burst firing test, and shot pattern analysis.
3. **Calibration Studio:** 4-step wizard, test pattern overlay, and offline bilateral symmetry Screenshot Align tool.
4. **MCP Agent Live Stream:** Real-time log of tool calls invoked by autonomous AI agents, with execution duration and parameter diffs.
