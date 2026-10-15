# XHAIR Component Library Specification

All UI components designed for the desktop editor at `1440×900` and responsive down to `1100×700`.
Components reference design tokens exclusively (see `design/tokens.json`).

## States Matrix
Every interactive control implements standard states:
- **Default**: Base resting state.
- **Hover**: Cursor over control (subtle brightness increase or border highlight).
- **Active / Pressed**: Mouse button held down or keyboard actuation.
- **Focus**: Visible 2px focus ring (`--accent` / `#36F2CB`) with 2px offset for keyboard navigation.
- **Disabled**: 40% opacity, `cursor: not-allowed`, non-reactive to pointer/keyboard.
- **Error / Warning**: Border colored `--danger` (`#FF5C5C`) or `--warning` (`#FFB454`), with error helper text or tooltip.

---

## 1. Action & Navigation Controls

### Button (`.b`)
- **Variants**:
  - `Primary` (`.b.p`): Background `--accent` (`#36F2CB`), text `#04110E`, font-weight 800. For primary confirmations and creation.
  - `Secondary` (`.b`): Background `--surface2` (`#1A2027`), border `--border` (`#262E38`), text `--text` (`#E9EEF3`).
  - `Ghost`: Transparent background, hover background `--surface2`.
  - `Icon`: 32×32px or 36×36px square with centered SVG icon.
- **Props**: `label`, `icon?`, `shortcutBadge?`, `disabled?`, `variant`

### Segmented Control
- Horizontal grouping of connected buttons. Selected item has border `--accent` and background `--accentDim`.
- Used for Frame modes (Screen vs Local axes) and Target selectors (Arm vs Layer vs All).

### Tabs
- Horizontal tab headers with bottom border indicator in `--accent`.

---

## 2. Inputs & Parameter Controls

### Number Input with Scrub Handle (`.in`)
- Monospace font (`JetBrains Mono`, 12px), background `--bg` (`#0B0D10`), border `--border`.
- **Scrubbing**: Dragging horizontally on the label or icon increases/decreases the value continuously. Holding `Alt` enables fine stepping (0.01 or 0.1); holding `Shift` steps by 10.

### Slider Control
- Track height 4px, background `--border`, active fill `--accent`.
- Thumb: 14px circle, border 2px solid `--accent`. Accompanied by direct numeric input.

### Search Field
- Input with embedded magnifying glass icon and clear `×` button.

---

## 3. Specialized Editor Controls

### 8-Way Nudge Pad (`.pn .nudge-pad`)
- **Layout**: 3×3 grid of directional buttons (`↖`, `↑`, `↗`, `←`, `•`, `→`, `↙`, `↓`, `↘`).
  - Center button (`•`): Resets current target offset to `(0, 0)`.
- **Step Chips**: Fast step selector buttons: `0.1` | `0.25` | `0.5` | `1` | `5` | `10` px, plus custom step input.
- **Target Selector**: Segmented switch between `Arm` | `Layer` | `All` (Whole Crosshair).
- **Frame Selector**: Toggle between `Screen Axes` (world) and `Layer-Local Axes` (rotated).
- **Keyboard Shortcuts**: Mapped directly to arrow keys, `Shift+Arrow` (10x), `Alt+Arrow` (0.1x), `Ctrl+Arrow` (whole crosshair).

### Grid-Canvas Chrome
- Floating overlays at canvas edges:
  - **Tool Palette** (left): Tool buttons `V` (Select), `A` (Direct/Anchor), `L` (Line arms), `D` (Dot), `R` (Rect), `C` (Circle), `P` (Pen/Path), `B` (Pixel brush).
  - **View Controls Bar** (bottom): Zoom chips (`1×`, `2×`, `4×`, `8×`, `16×`, `32×`, `Fit`, `1:1`), toggle chips (`Grid`, `Rulers`, `Guides`, `Distance Rings`).
  - **Coordinate Readout**: Game-px relative offset (`x -3.5, y +0.0`) and absolute render coords `(i, j)`.
  - **Alignment Hint**: Status pill indicating `Center = pixel edge (even W)` or `Center = pixel middle (odd W)`.

### Layer Item Row
- Container in layers list showing:
  - Drag handle (6 dots)
  - Solo / Hide / Lock icons
  - Layer name (editable on double click)
  - **Alignment Badge**:
    - `ALIGNED` (`.bd`): Green/cyan tag (`#0F2A26`, text `#36F2CB`)
    - `OFF-GRID` (`.bd.wn`): Orange tag (`#33260F`, text `#FFB454`)

### Status Pills
- High-visibility runtime pills in the top document bar:
  - `Overlay ON` / `Overlay OFF` (active green/cyan border `#1D6B5C`)
  - `MCP ON` / `MCP OFF` (active cyan)
  - `Dynamic ON` / `Dynamic OFF` (opt-in indicator)
  - Resolution Profile badge (e.g. `1920×1080 · Native`)

---

## 4. Overlay & Inspection Feedback

### Tooltip (`Tooltip`)
- **Requirement**: Present on EVERY interactive control.
- **Contents**:
  1. Title in bold
  2. Description in plain language (1–2 sentences)
  3. Range / default / unit (e.g. `Default: 6 px, Range: 0–512 px`)
  4. Shortcut badge (e.g. `L` or `Ctrl+Z`)
  5. Optional "Learn more" link to `docs/`
- **Behavior**: Opens on hover after 400ms or immediately on keyboard focus. Closes on `Escape`.

### Command Palette (`Ctrl+K`)
- Spotlight modal listing all registered commands, their descriptions, and keyboard shortcuts.
