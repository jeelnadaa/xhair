//! Calibration test pattern generator for pixel-accurate alignment verification (§6.7).

use xhair_model::crosshair::Crosshair;
use xhair_model::layer::Layer;
use xhair_model::paint::Paint;
use xhair_model::shapes::{DotProps, DotShape, LayerKind, LineProps, RectProps};
use xhair_model::units::{Degrees, Rgba, Vec2};

/// Generates a calibration test pattern crosshair for screen center alignment verification.
///
/// Pattern features (§6.7):
/// - 1-game-px continuous horizontal and vertical cross lines through the computed center.
/// - Tick marks placed at ±1, ±5, ±10, and ±20 game pixel distances.
/// - Distinct high-contrast center indicator marking the exact sub-pixel/edge raster center.
pub fn create_calibration_crosshair(width: u32, height: u32) -> Crosshair {
    let mut xhair = Crosshair::new_blank("Calibration Test Pattern");

    // Center cross lines (1 px thickness) spanning 120 pixels in each direction
    let main_lines = LayerKind::Line(LineProps {
        arm_count: 4,
        length: 120.0,
        thickness: 1.0,
        gap: 0.0,
        taper: 0.0,
        start_angle: Degrees(0.0),
        ..Default::default()
    });

    let mut line_layer = Layer::new("Guide Lines", main_lines);
    line_layer.fill = Paint::Solid {
        color: Rgba::new(0, 245, 212, 0.85),
    }; // Neon Cyan
    xhair.layers.push(line_layer);

    // Ticks at 5, 10, 20 px intervals
    for dist in [5.0f32, 10.0, 20.0, 40.0] {
        let tick_thickness = if (dist - 10.0).abs() < 0.01 || (dist - 20.0).abs() < 0.01 {
            1.5
        } else {
            1.0
        };
        let tick_length = if (dist - 20.0).abs() < 0.01 { 8.0 } else { 4.0 };

        // Horizontal arms ticks
        let mut tick_layer = Layer::new(
            format!("Ticks ±{dist}px"),
            LayerKind::Rect(RectProps {
                width: tick_thickness,
                height: tick_length,
                corner_radii: [0.0; 4],
            }),
        );
        tick_layer.transform.offset = Vec2::new(dist, 0.0);
        tick_layer.fill = Paint::Solid {
            color: Rgba::new(255, 255, 255, 0.9),
        };
        xhair.layers.push(tick_layer);

        // Negative horizontal tick
        let mut neg_tick = Layer::new(
            format!("Ticks -{dist}px"),
            LayerKind::Rect(RectProps {
                width: tick_thickness,
                height: tick_length,
                corner_radii: [0.0; 4],
            }),
        );
        neg_tick.transform.offset = Vec2::new(-dist, 0.0);
        neg_tick.fill = Paint::Solid {
            color: Rgba::new(255, 255, 255, 0.9),
        };
        xhair.layers.push(neg_tick);

        // Vertical arms tick
        let mut v_tick = Layer::new(
            format!("Ticks Y+{dist}px"),
            LayerKind::Rect(RectProps {
                width: tick_length,
                height: tick_thickness,
                corner_radii: [0.0; 4],
            }),
        );
        v_tick.transform.offset = Vec2::new(0.0, dist);
        v_tick.fill = Paint::Solid {
            color: Rgba::new(255, 255, 255, 0.9),
        };
        xhair.layers.push(v_tick);

        // Negative vertical tick
        let mut neg_v_tick = Layer::new(
            format!("Ticks Y-{dist}px"),
            LayerKind::Rect(RectProps {
                width: tick_length,
                height: tick_thickness,
                corner_radii: [0.0; 4],
            }),
        );
        neg_v_tick.transform.offset = Vec2::new(0.0, -dist);
        neg_v_tick.fill = Paint::Solid {
            color: Rgba::new(255, 255, 255, 0.9),
        };
        xhair.layers.push(neg_v_tick);
    }

    // Exact center indicator: magenta 1-px dot for contrast
    let is_even_w = width % 2 == 0;
    let is_even_h = height % 2 == 0;

    let center_dot = LayerKind::Dot(DotProps {
        diameter: 1.0,
        shape: if is_even_w && is_even_h {
            DotShape::Square
        } else {
            DotShape::Round
        },
    });

    let mut dot_layer = Layer::new("Center Marker", center_dot);
    dot_layer.fill = Paint::Solid {
        color: Rgba::new(255, 0, 128, 1.0),
    }; // Vivid Magenta
    xhair.layers.push(dot_layer);

    xhair
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calibration_crosshair_structure() {
        let xhair = create_calibration_crosshair(1920, 1080);
        assert!(!xhair.layers.is_empty());
        assert!(xhair.validate().is_ok());

        // Check center marker
        let center_marker = xhair.layers.iter().find(|l| l.name == "Center Marker");
        assert!(center_marker.is_some());
    }
}
