# GLOSSARY — Domain Concepts & Terminology

- **Game Pixel (`GamePx`)**: One pixel of the game's internal render resolution. All crosshair authoring coordinates, offsets, thickness, gaps, and lengths are defined in `GamePx(f32)`.
- **Edge Coordinates**: A coordinate system where integer coordinates land on the grid edges between pixels. Pixel `(i, j)` occupies $[i, i+1) \times [j, j+1)$.
- **Screen Center ($C$)**: The exact midpoint of the render resolution $(W/2, H/2)$ in edge coordinates. If $W$ is even, $C$ lies on an edge between two pixels. If $W$ is odd, $C$ lies exactly in the middle of a pixel.
- **Physical Pixel**: The actual hardware pixel on the physical monitor display.
- **Scale Factor ($(s_x, s_y)$)**: Ratio of output dimensions to game render dimensions. In stretched mode (e.g. 4:3 stretched on 16:9), $s_x \ne s_y$.
- **Crisp AA**: Anti-aliasing mode where edges snap deterministically to integer output pixels to prevent sub-pixel blur.
- **Smooth AA**: Standard sub-pixel anti-aliased rendering supporting fractional widths, nudges, and rotations.
- **Arm**: One ray or branch in a multi-arm or symmetric shape (e.g. arms 0..3 of a standard 4-way crosshair, indexed clockwise starting from the top or `start_angle`).
- **Arm Override**: Independent adjustments to offset, length, thickness, color, or visibility of a single arm within a symmetric layer.
- **Resolution Profile**: Stored monitor, game resolution, stretch mode, calibration offset, and display settings.
- **Calibration Offset**: Fine adjustment (in game pixels, down to 0.25 px) applied to align the overlay center exactly with the in-game crosshair.
- **Slot**: One of 20 quick-access presets that can be switched mid-game via hotkey or MCP.
- **Command Bus**: The single bottleneck through which all state mutations flow. Emits atomic events and supports undo/redo and coalescing.
- **Coalescing**: Automatic merging of rapid sequential mutations (such as slider drags or nudge bursts within 400 ms) into a single undo/redo step.
- **Dynamic Reticle**: Simulated spread/recoil response driven by user-bound inputs (flagged opt-in).
- **Spray Lab**: Standalone non-overlay simulation window for testing gun profiles and recoil curves safely.
- **Share Code**: Compact string format `XH1-<base64url(brotli(canonical_json))>-<crc32>` used for sharing crosshairs without cloud accounts.
