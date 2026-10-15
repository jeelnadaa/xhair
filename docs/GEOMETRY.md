# Coordinate Math & Geometry Reference — XHAIR

This document defines the mathematical foundations implemented in `crates/xhair-geometry`, detailing pixel centering, even/odd parity, aspect ratio stretch, DPI scaling, and offline calibration (§6).

---

## 1. Pixel Grid & Parity Conventions

In a 2D digital raster, a screen of dimension $W \times H$ has discrete pixel coordinates $(x, y)$ where $x \in [0, W-1]$ and $y \in [0, H-1]$.

### 1.1 The Center Problem: Even vs. Odd
The true geometric center of a monitor of width $W$ and height $H$ is:
$$x_c = \frac{W}{2}, \quad y_c = \frac{H}{2}$$

- **Even Dimensions (e.g., $1920 \times 1080$):**
  $$x_c = \frac{1920}{2} = 960.0, \quad y_c = \frac{1080}{2} = 540.0$$
  The geometric center falls **exactly on the boundary (edge)** between pixels $959$ and $960$, and between pixels $539$ and $540$.
  - An even-width crosshair feature (e.g. 2 px stroke) spans pixels $[959, 960]$ symmetrically without anti-aliasing blur.
  - An odd-width crosshair feature (e.g. 1 px dot) at $x_c = 960.0$ would split across two pixels (subpixel anti-aliasing) unless snapped or offset by $0.5\text{ px}$.

- **Odd Dimensions (e.g., $1921 \times 1081$):**
  $$x_c = \frac{1921}{2} = 960.5, \quad y_c = \frac{1081}{2} = 540.5$$
  The geometric center falls **in the exact center of pixel $(960, 540)$**.
  - A 1 px dot centered at $(960.5, 540.5)$ fills pixel $(960, 540)$ perfectly with $100\%$ alpha and zero blur.

### 1.2 Subpixel Snapping Rules
`xhair-geometry` supports two explicit parity modes:
1. **Geometric True Center (Float):** Exact mathematical center $(W/2, H/2)$ with subpixel anti-aliasing enabled.
2. **Pixel-Snapped Center (Crisp):** Integer or half-integer alignment:
   - For odd-stroke layers on even displays: snaps to $x_c - 0.5$ or $x_c + 0.5$ based on user preference or calibration offset.
   - For even-stroke layers on even displays: remains at integer boundary $x_c$.

---

## 2. Aspect Ratio Stretch & Black Bars

When a competitive gamer runs a non-native resolution (e.g. $1280 \times 960$ 4:3 stretched on a $1920 \times 1080$ 16:9 monitor):
- **Stretched Mode:** The GPU or display scaler expands the $1280 \times 960$ framebuffer non-uniformly across $1920 \times 1080$.
  - Horizontal scale factor: $S_x = \frac{1920}{1280} = 1.5$
  - Vertical scale factor: $S_y = \frac{1080}{960} = 1.125$
  - Any crosshair rendered at desktop coordinates must account for the stretch ratio $S_x / S_y$ if it desires to match the in-game perceived crosshair geometry.

- **Black Bars (Pillarbox / Letterbox):**
  - Render target is centered in the native monitor with aspect-ratio preserving scaling:
    $$S = \min\left(\frac{W_{\text{mon}}}{W_{\text{game}}}, \frac{H_{\text{mon}}}{H_{\text{game}}}\right)$$
  - Pillarbox offsets:
    $$\Delta x = \frac{W_{\text{mon}} - W_{\text{game}} \cdot S}{2}, \quad \Delta y = \frac{H_{\text{mon}} - H_{\text{game}} \cdot S}{2}$$

---

## 3. Per-Monitor DPI Scaling

Windows desktop DPI scales range from $100\%$ ($96\text{ DPI}$, scale $1.0$) to $500\%$ ($480\text{ DPI}$, scale $5.0$).

Given a logical coordinate $(x_l, y_l)$ in the crosshair designer:
$$\text{Physical Coordinates: } x_p = x_l \times \text{DPI Scale}, \quad y_p = y_l \times \text{DPI Scale}$$

The overlay daemon (`xhair-platform-win`) declares per-monitor V2 DPI awareness:
- Window dimensions match physical monitor pixels directly (`GetSystemMetrics(SM_CXSCREEN)`).
- Crosshairs are rasterized at physical pixels directly, avoiding OS bilinear scaling blur.

---

## 4. Worked Numerical Examples

### Example 1: 4-Arm Crosshair on 1080p ($1920 \times 1080$)
- **Properties:** Length = 6 px, Thickness = 2 px, Gap = 3 px.
- **Center:** $(960.0, 540.0)$.
- **Top Arm:**
  - $X$ span: $[960 - 1, 960 + 1] = [959, 961]$ (width = 2 px).
  - $Y$ span: from $540 - 3 - 6 = 531$ to $540 - 3 = 537$ (length = 6 px).
  - Completely lands on integer pixel borders with zero anti-aliasing blur.

### Example 2: 1-Pixel Dot on 1440p ($2560 \times 1440$)
- **Properties:** Radius = 0.5 px (diameter = 1 px).
- **Geometric Center:** $(1280.0, 720.0)$.
- **Pixel boundaries:** An unshifted dot at $1280.0$ touches pixels $1279$ and $1280$, resulting in two 50% opacity pixels.
- **Applying Crisp Snapping:**
  - Shift by $-0.5\text{ px} \to (1279.5, 719.5)$.
  - A 1 px rectangle centered at $(1279.5, 719.5)$ covers $[1279.0, 1280.0]$ and $[719.0, 720.0]$: exactly 1 physical pixel with 100% opacity.

---

## 5. Offline Screenshot Alignment Algorithm

Implemented in `xhair_geometry::screenshot_align`:
1. **User Input:** Static in-game screenshot taken with native weapon ADS or static crosshair.
2. **Region of Interest (ROI):** Crops a $64 \times 64$ window around estimated screen center.
3. **Thresholding & Centroid Calculation:**
   $$\bar{x} = \frac{\sum x \cdot I(x, y)}{\sum I(x, y)}, \quad \bar{y} = \frac{\sum y \cdot I(x, y)}{\sum I(x, y)}$$
4. **Calculated Offset:**
   $$\Delta x = \bar{x} - x_{\text{expected}}, \quad \Delta y = \bar{y} - y_{\text{expected}}$$
5. **1-Click Apply:** Stores $(\Delta x, \Delta y)$ as subpixel calibration offsets in the active resolution profile.
