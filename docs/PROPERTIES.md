# Crosshair Property Catalog

*Generated automatically from descriptor registry — single source of truth for UI, tooltips, and MCP.*

| Property | Label | Group | Unit | Min..Max | Default | Tooltip Description |
|---|---|---|---|---|---|---|
| `line.length` | Length | Geometry | px | 0..512 | `6.0` | How far each arm extends from its starting point in game pixels. |
| `line.thickness` | Thickness | Geometry | px | 0.25..64 | `2.0` | Width of each crosshair line in game pixels. |
| `line.gap` | Gap | Geometry | px | 0..256 | `4.0` | Inner distance between the center point and the start of each arm. |
| `line.arm_count` | Arm Count | Geometry | - | 1..16 | `4` | Number of radially distributed arms around the center. |
| `dot.diameter` | Center Dot Diameter | Geometry | px | 0.5..32 | `2.0` | Size of the central reticle dot in game pixels. |
| `glow.radius` | Glow Radius | Effects | px | 0..64 | `4.0` | Outer radiant dispersion radius for glow effects (max 64 px). |
| `glow.intensity` | Glow Intensity | Effects | - | 0..4 | `1.0` | Brightness intensity multiplier of the glow bloom. |
| `global.scale` | Master Scale | Transform | - | 0.1..10 | `1.0` | Uniform scaling multiplier applied to the entire composite crosshair. |
| `global.opacity` | Master Opacity | Appearance | % | 0..1 | `1.0` | Overall visibility transparency of the reticle. |
