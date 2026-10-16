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
| `position.x` | X Offset | Transform | px | -1024..1024 | `0.0` | Horizontal position relative to center in game pixels. |
| `position.y` | Y Offset | Transform | px | -1024..1024 | `0.0` | Vertical position relative to center in game pixels. |
| `position.mode` | Position Mode | Transform | - | - | `"relative"` | Relative (delta) or Absolute (from reticle center) coordinates. |
| `nudge.step` | Nudge Step | Transform | px | 0.01..100 | `1.0` | Movement distance per nudge arrow press in game pixels. |
| `paint.color` | Reticle Color | Appearance | - | - | `"#36F2CBFF"` | Reticle fill color in hex format (#RGB, #RRGGBB, #RRGGBBAA). |
| `recenter.selection` | Recenter Selection | Transform | - | - | `null` | Moves selected layers back to exact center (0, 0). Shortcut: Home. |
| `rect.width` | Rectangle Width | Geometry | px | 1..256 | `8.0` | Width of rectangle layer in game pixels. |
| `rect.height` | Rectangle Height | Geometry | px | 1..256 | `8.0` | Height of rectangle layer in game pixels. |
| `rect.corner_radius` | Corner Radius | Geometry | px | 0..10 | `0.0` | Rounding radius of rectangle corners in game pixels (max 10 px). |
| `circle.diameter` | Circle Diameter | Geometry | px | 1..256 | `16.0` | Total outer diameter of circle in game pixels. |
| `ring.outer_radius` | Ring Outer Radius | Geometry | px | 1..256 | `10.0` | Outer radius of circular ring in game pixels. |
| `ring.inner_radius` | Ring Inner Radius | Geometry | px | 0..255 | `7.0` | Inner cutout radius of ring in game pixels. |
| `arc.radius` | Arc Radius | Geometry | px | 1..256 | `12.0` | Radius of circular arc in game pixels. |
| `polygon.sides` | Polygon Sides | Geometry | - | 3..64 | `6` | Number of sides or points for regular polygon / star (3 to 64). |
| `triangle.base` | Triangle Base | Geometry | px | 1..256 | `10.0` | Width of triangle base in game pixels. |
| `chevron.arm_length` | Chevron Arm Length | Geometry | px | 1..256 | `6.0` | Length of each chevron wing in game pixels. |
| `t_shape.bar_length` | T-Shape Bar Length | Geometry | px | 1..256 | `10.0` | Horizontal top bar length of T-shape in game pixels. |
| `plus.length` | Plus Arm Length | Geometry | px | 1..256 | `6.0` | Length of plus cross arms in game pixels. |
| `path.stroke_width` | Path Stroke Width | Geometry | px | 0.25..64 | `2.0` | Width of vector Bézier path stroke in game pixels. |
| `pixels.grid_size` | Pixel Grid Dimension | Geometry | - | 4..64 | `16` | Resolution of custom pixel matrix grid (e.g. 16 for 16x16). |
| `image.scale` | Image Scale | Geometry | x | 0.1..10 | `1.0` | Scaling factor for custom sprite image layer. |
| `glow.radius` | Glow Radius | Effects | px | 0..64 | `0.0` | Diffusion spread radius of Gaussian glow in game pixels. |
| `shadow.blur` | Drop Shadow Blur | Effects | px | 0..64 | `0.0` | Blur radius of drop shadow in game pixels. |
| `bloom.intensity` | Global Bloom Intensity | Effects | % | 0..200 | `0.0` | HDR luminance threshold boost for high-contrast reticle visibility. |
| `theme.id` | Active Theme | Appearance | - | - | `"mono-dark"` | Monochromatic UI chrome theme (mono-dark, mono-light, graphite-mint, etc.). |
| `appearance.font_pack` | Font Pack | Appearance | - | - | `"satoshi"` | Bundled WOFF2 typography pack (satoshi, technical, editorial, plex, space, system). |
| `appearance.corner_style` | Corner Style | Appearance | - | - | `"default"` | UI component corner radius scale: sharp, default (max 8px), or soft (max 10px). |
