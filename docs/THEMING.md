# Theming System & Neutral Color Rules (§12)

XHAIR enforces a strict visual standard: **pure neutral chrome surfaces with zero blue/cyan tinting**, restrained accent colors, and precise corner curvature.

---

## 1. The Neutrality Rule (CI Enforced)

All chrome surfaces, borders, and text tokens must be pure neutral grays:
$$\max(|R - G|, |G - B|, |R - B|) \le 2$$

Color appears **only** in:
1. The crosshair itself.
2. The color picker swatch matrix.
3. The theme accent color (used sparingly for active tabs, primary actions, and focus indicators).
4. Critical state indicators (warnings, errors, alignment tags).

### Base Neutral Ramps

| Token | Graphite (Default Dark) | White (Light) | Pure Black (OLED) |
|---|---|---|---|
| `bg` | `#0A0A0A` | `#FFFFFF` | `#000000` |
| `canvas` | `#080808` | `#FFFFFF` | `#000000` |
| `surface` | `#111111` | `#F5F5F5` | `#0A0A0A` |
| `raised` | `#171717` | `#EDEDED` | `#121212` |
| `hover` | `#1F1F1F` | `#E4E4E4` | `#1A1A1A` |
| `border` | `#262626` | `#D4D4D4` | `#1F1F1F` |
| `borderStrong` | `#3A3A3A` | `#A3A3A3` | `#333333` |
| `text` | `#EDEDED` | `#0A0A0A` | `#EDEDED` |
| `textMuted` | `#A0A0A0` | `#525252` | `#888888` |
| `textDim` | `#808080` | `#6B6B6B` | `#555555` |
| `gridMinor` | `#141414` | `#EDEDED` | `#0D0D0D` |
| `gridMajor` | `#1E1E1E` | `#D4D4D4` | `#171717` |

---

## 2. Supported Themes

| Theme ID | Base Ramp | Accent | On-Accent | Guide Lines | Contrast |
|---|---|---|---|---|---|
| `mono-dark` (**default**) | Graphite | `#FFFFFF` | `#0A0A0A` | `#FFFFFF` | $\ge 14.2:1$ |
| `mono-light` | White | `#0A0A0A` | `#FFFFFF` | `#0A0A0A` | $\ge 18.5:1$ |
| `oled-black` | Pure Black | `#FFFFFF` | `#000000` | `#FFFFFF` | $\ge 19.8:1$ |
| `graphite-mint` | Graphite | `#3DDBB0` | `#04130F` | `#F5B83D` | $\ge 14.2:1$ |
| `graphite-violet` | Graphite | `#A78BFA` | `#0F0A1F` | `#F472B6` | $\ge 14.2:1$ |
| `graphite-amber` | Graphite | `#F5B83D` | `#1A1100` | `#3DDBB0` | $\ge 14.2:1$ |
| `graphite-crimson` | Graphite | `#FF5A5F` | `#1F0507` | `#FFFFFF` | $\ge 14.2:1$ |
| `light-indigo` | White | `#4F46E5` | `#FFFFFF` | `#DB2777` | $\ge 18.5:1$ |
| `high-contrast` | Pure Black (2px) | `#FFE600` | `#000000` | `#FFE600` | $\ge 21.0:1$ |

Themes switch live without window reload or layout shifts. Theme crossfade duration is $\le 150\text{ ms}$ (instantaneous when `prefers-reduced-motion` is active).

---

## 3. Corner Radius Token Table (§12.3)

| Token | Default | Usage |
|---|---|---|
| `radius.none` | `0px` | Canvas edges, dock split lines, underline tabs |
| `radius.xs` | `2px` | Slider tracks, mini tags, scrollbar thumbs |
| `radius.sm` | `3px` | Chips, badges, shortcut key caps, swatches |
| `radius.md` | `4px` | Buttons, number inputs, text fields, dropdown menus |
| `radius.lg` | `6px` | Dock panels, floating cards, popovers, tooltips |
| `radius.xl` | `8px` | Modal dialogs and major overlay windows |

**Hard Rule:** No fully rounded capsules or pills (`9999px` or `50%`) anywhere in the application chrome, except for true geometric circles (radio inputs, slider thumb handles, small 6px status dots). Maximum radius anywhere is strictly $10\text{ px}$.

---

## 4. How to Add a New Theme in 4 Steps

1. **Declare Tokens:** Add theme entry into `design/tokens.json` under `color.themes.<id>` referencing one of the 3 base neutral ramps (`graphite`, `white`, or `pureBlack`).
2. **Run Contrast & Neutrality CI Test:**
   ```bash
   node scratch/check_tokens_v5.cjs
   ```
   Ensures channel spread $\le 2$, body text contrast $\ge 4.5:1$, and outlines $\ge 3:1$.
3. **Register Preview Card:** Add card metadata to `ui/src/components/AppearanceModal.svelte`.
4. **Capture Visual Regression:** Run headless screen capture to record theme screenshot.
