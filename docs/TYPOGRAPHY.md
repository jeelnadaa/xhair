# Typography & Font System (§13)

XHAIR ships with **bundled, self-hosted WOFF2 font packs**. There is strictly **no user font import** (`font.import` does not exist), and **no external CDN/Google Fonts requests at runtime**. All fonts load exclusively inside the editor webview; the resident low-RAM overlay never touches webview fonts.

---

## 1. Typographic Hierarchy & Roles

| Role Token | Target Usage | Sizing & Weight | Features |
|---|---|---|---|
| `font.display` | Wordmark, window titles, empty states | 16–22 px, Bold (700) | Tight tracking (`-0.01em`) |
| `font.ui` | Body, buttons, tab labels, layer rows, tooltips | 12–13 px, Medium (500) | Normal tracking |
| `font.label` | Small uppercase section headers & badges | 10–11 px, Semibold (600) | Expanded tracking (`+0.06em`) |
| `font.mono` | Coordinates, hex codes, numeric inputs, MCP logs | 11–12 px, Regular/Medium | `tnum` (tabular), slashed zero |

---

## 2. Bundled Font Packs

| Pack ID | `display` | `ui` | `mono` | License Terms | Size Target |
|---|---|---|---|---|---|
| `satoshi` (**default**) | Clash Display | Satoshi | JetBrains Mono | Fontshare Free / OFL | ~280 KB |
| `technical` | Geist | Geist | Geist Mono | SIL Open Font License (OFL) | ~240 KB |
| `editorial` | Cabinet Grotesk | General Sans | IBM Plex Mono | Fontshare Free / OFL | ~290 KB |
| `plex` | IBM Plex Sans Condensed | IBM Plex Sans | IBM Plex Mono | SIL Open Font License (OFL) | ~310 KB |
| `space` | Space Grotesk | Space Grotesk | Space Mono | SIL Open Font License (OFL) | ~260 KB |
| `system` | Segoe UI Variable | Segoe UI Variable | Cascadia Mono | Windows OS System Built-in | 0 KB (Native) |

Total combined payload for all bundled packs is **≤ 1.5 MB WOFF2**. Only the active pack is loaded into the DOM; other packs load on demand during preview in Settings → Appearance.

---

## 3. Font Engineering & Subsetting Pipeline

To guarantee sub-second initial loads and eliminate bloat:

```bash
# Subsetting script using fonttools pyftsubset
pyftsubset Font-Variable.ttf \
  --unicodes="U+0020-007E,U+00A0-00FF,U+2010-2027,U+20AC" \
  --flavor=woff2 \
  --layout-features="kern,liga,tnum,zero,ss01" \
  --output-file=font.woff2
```

### CSS Self-Hosting Declaration
```css
@font-face {
  font-family: 'Satoshi';
  src: url('/fonts/Satoshi-Variable.woff2') format('woff2');
  font-weight: 300 900;
  font-display: swap;
  font-style: normal;
}
```

---

## 4. Making Fonts That Belong to XHAIR (§13.3)

### Custom Wordmark
The XHAIR wordmark features an integrated crosshair reticle inside the glyph **"X"**. To prevent font file dependencies during splash and window rendering, the wordmark is embedded as an **outlined, zero-dependency SVG** in `TopBar.svelte`.

### Option A: Licensed & Subsetted Fonts
Download variable weights, verify desktop redistribution license, subset Latin character sets, and encode as self-hosted WOFF2.

### Option B: Derived OFL Families
Fork an open-source OFL face (e.g. Geist or JetBrains Mono), customize specific gaming glyphs (squared zeroes, high-legibility numerals `1, 4, 7`), rename the font family to drop reserved names, and recompile via `fontmake`.

### Option C: Custom Type Foundry Brief
When commissioning proprietary brand fonts:
1. **Brief:** Ultra-crisp rendering at 11–13 px on Windows ClearType / DirectWrite.
2. **Metrics:** 1000 unitsPerEm, matching cap height and x-height between UI and Mono faces.
3. **Features:** Mandatory `tnum` (tabular numbers) and dotted/slashed zero for jitter-free live coordinate readouts.
