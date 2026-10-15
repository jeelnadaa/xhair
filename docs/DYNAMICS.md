# Reticle Dynamics & Spray Lab Model — XHAIR

This document defines the mathematical models, dynamic driver bindings, and spray simulation algorithms implemented in `crates/xhair-dynamics` (§12, §15.2).

---

## 1. Overview & Ethical Boundaries

Competitive tactical shooters feature dynamic weapon inaccuracy caused by player movement (running, jumping) and continuous fire (spread and recoil expansion). 

**Ethical & Safety Boundary:** XHAIR never reads weapon state or spread from game memory. Instead, `xhair-dynamics` provides:
1. **Mathematical Simulation Engine:** A deterministic simulation model driven by neutral weapon profiles or user-configured parameters.
2. **Dynamic Property Bindings:** Bindings that mathematically scale visual crosshair properties (gap, stroke, opacity) based on simulated spread.
3. **Standalone Spray Lab:** An offline training tool for visualizing recoil patterns, error discs, and reset timings.

---

## 2. Weapon Spread & Recoil Mathematics

### 2.1 Spread Accumulation Model
During continuous firing sequence of duration $t$ or shot count $k$:
$$\text{Spread}(k) = \text{BaseSpread} + (\text{MaxSpread} - \text{BaseSpread}) \cdot \left(1 - e^{-\frac{k}{\tau_{\text{bloom}}}}\right)$$

- **$\text{BaseSpread}$:** Inherent weapon inaccuracy (deg or px).
- **$\text{MaxSpread}$:** Ceiling inaccuracy reached during full sustained fire.
- **$\tau_{\text{bloom}}$:** Bloom rate constant defining how rapidly spread expands.

### 2.2 Inaccuracy Recovery / Decay
When fire ceases, inaccuracy recovers asymptotically towards the stationary baseline:
$$\text{Spread}(t) = \text{BaseSpread} + (\text{CurrentSpread} - \text{BaseSpread}) \cdot e^{-\frac{t}{\tau_{\text{recovery}}}}$$

- **$\tau_{\text{recovery}}$:** Inaccuracy reset constant.

---

## 3. Dynamic Property Bindings

Dynamic bindings allow crosshair properties (e.g. `center_gap`, `stroke_width`, `opacity`) to respond fluidly to simulated spread $S(t) \in [0, 1]$:

$$P(t) = P_{\text{base}} + \Delta P \cdot f(S(t))$$

### 3.1 Transfer Curves $f(x)$
- **Linear:** $f(x) = x$
- **Quadratic:** $f(x) = x^2$ (subtle expansion for initial taps, dramatic expansion on sustained spray)
- **Square Root:** $f(x) = \sqrt{x}$ (immediate response on first shot)
- **Step Function:** $f(x) = 1 \text{ if } x \ge T \text{ else } 0$ (visual trigger when inaccuracy exceeds critical threshold)

### 3.2 Temporal Smoothing
To avoid jarring visual jitter, raw driver values are smoothed using an exponential moving average (EMA) filter:
$$S_{\text{smooth}}(t) = \alpha \cdot S(t) + (1 - \alpha) \cdot S_{\text{smooth}}(t - \Delta t)$$
where $\alpha = 1 - e^{-\frac{\Delta t}{\tau_{\text{filter}}}}$.

---

## 4. Synthetic Movement Drivers

Movement state modifies the baseline spread multiplier:
- **Stationary (`Still`):** Multiplier $= 1.0\times$
- **Walking (`Walk`):** Multiplier $= 1.4\times$
- **Sprinting (`Run`):** Multiplier $= 2.2\times$
- **Airborne (`Jump`):** Multiplier $= 3.5\times$

---

## 5. Spray Lab Architecture

Spray Lab is a 100% offline simulation environment hosted within the Svelte 5 editor:
- **Trajectory Visualizer:** Renders deterministic recoil offsets $(dx_i, dy_i)$ and circular $1\sigma$ / $2\sigma$ error discs.
- **Scrubber Timeline:** Allows frame-by-frame scrubbing through a 30-round spray pattern.
- **Safety Guarantee:** Executes in an isolated preview canvas with synthetic input sources; completely inert during active gameplay.
