# Performance & Resource Budgets — XHAIR

This document specifies the memory, CPU, and rendering latency budgets and explains how they are measured and enforced (§14, §15.2).

---

## 1. Resource Budgets

| Metric | Target Budget | Hard Limit | Enforcement Mechanism |
|---|---|---|---|
| **Idle Overlay RAM** | $\le 15\text{ MB}$ Private Working Set | $20\text{ MB}$ | Automated CI Memory Gate |
| **Static Overlay CPU** | $0.00\%$ CPU | $< 0.10\%$ | Event-driven render loop (no continuous tick) |
| **Active Dynamic Render CPU** | $< 1.0\%$ (1 core) | $< 2.0\%$ | 60/144 FPS capped rasterization timer |
| **Frame Rasterization Time** | $< 1.0\text{ ms}$ ($1000\,\mu\text{s}$) | $8.0\text{ ms}$ ($8000\,\mu\text{s}$) | Criterion microbenchmarks & heavy tests |
| **Full App (Editor GUI) RAM** | $< 70\text{ MB}$ | $< 100\text{ MB}$ | Webview2 / browser lifecycle cleanup |

---

## 2. Zero-CPU Static Rendering Architecture

Most game overlays run a continuous 60 or 144 FPS render loop that wakes up the CPU every 7–16 ms, consuming 1–3% of a CPU core even when the reticle is completely motionless.

**XHAIR's Zero-CPU Strategy:**
1. **Pure Event-Driven Presentation:** The overlay thread sleeps on Win32 message queues (`GetMessageW`) with zero active polling.
2. **Dirty-Rect Caching:** If no command mutation, hotkey, or dynamic input event has arrived, **zero rasterization calls occur**.
3. **Hardware Presentation:** `UpdateLayeredWindow` is invoked exactly once when the crosshair changes. The Desktop Window Manager (DWM) composites the resulting surface in GPU hardware for all subsequent video frames without waking XHAIR.

---

## 3. Memory Profiling & Measurement Methodology

### 3.1 Measuring Private Working Set
Private Working Set reflects actual unshared physical RAM allocated to the process.

**PowerShell Measurement Command:**
```powershell
$p = Get-Process -Name xhair -ErrorAction SilentlyContinue
if ($p) {
    $wsMB = [math]::Round($p.WorkingSet64 / 1MB, 2)
    $pwsMB = [math]::Round($p.PrivateMemorySize64 / 1MB, 2)
    Write-Host "Working Set: $wsMB MB | Private Working Set: $pwsMB MB"
}
```

### 3.2 Automated CI Memory Test
In CI (`tests/memory_bench.rs`):
1. Spawn `xhair.exe --minimized --headless-mock` with an active complex multi-layer crosshair.
2. Sleep for 15 seconds to allow heap initialization and GC stabilization.
3. Sample `PrivateMemorySize64` every second for 15 seconds.
4. Fail test if average private working set exceeds $20.0\text{ MB}$.

---

## 4. Rasterization Performance Benchmarking

Run the native rasterization benchmark suite:
```bash
cargo test --package xhair-render --test heavy_crosshair_bench -- --nocapture
```
In release builds (`cargo test --release`), rasterization of complex 12-layer reticles completes in under **0.4 ms** ($400\,\mu\text{s}$), leaving over 95% of the frame interval available for game physics and rendering.
