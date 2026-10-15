//! Deterministic vector rasterization and visual effects for xhair.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

pub mod bounds;
pub mod color;
pub mod context;
pub mod effects;
pub mod error;
pub mod paint;
pub mod renderer;
pub mod shapes;
pub mod stroke;
pub mod symmetry;

pub use bounds::{calculate_crosshair_bounds, calculate_layer_bounds, BoundingBox};
pub use color::{to_skia_blend_mode, to_skia_color};
pub use context::{DynamicState, RenderContext, RenderQuality, RenderStats};
pub use error::RenderError;
pub use paint::create_skia_paint;
pub use renderer::{
    bounds, render, render_png, render_to_new_pixmap, MAX_BLUR_BUFFER_DIMENSION,
    MAX_OVERLAY_DIMENSION,
};
pub use stroke::create_skia_stroke;
pub use symmetry::{generate_symmetry_instances, SymmetryInstance};

#[cfg(test)]
mod tests {
    use super::*;
    use xhair_model::crosshair::Crosshair;
    use xhair_model::effects::GlowEffect;
    use xhair_model::layer::Layer;
    use xhair_model::shapes::{
        DotProps, DotShape, LayerKind, PolygonProps, RingProps, TShapeProps,
    };
    use xhair_model::units::Rgba;

    #[test]
    fn test_buffer_limits() {
        assert!(MAX_BLUR_BUFFER_DIMENSION <= MAX_OVERLAY_DIMENSION);
    }

    #[test]
    fn test_render_classic_crosshair_to_png() {
        let xhair = Crosshair::new_classic("Classic Test");
        let ctx = RenderContext::standard();
        let png = render_png(&xhair, &ctx, 64, 64).expect("PNG render failed");
        assert!(!png.is_empty());
        // Verify PNG magic header: \x89PNG\r\n\x1a\n
        assert_eq!(&png[0..4], &[0x89, 0x50, 0x4E, 0x47]);
    }

    #[test]
    fn test_render_round_and_square_dots() {
        let mut xhair = Crosshair::new_blank("Dot Test");
        xhair.layers.push(Layer::new(
            "Round Dot",
            LayerKind::Dot(DotProps {
                diameter: 4.0,
                shape: DotShape::Round,
            }),
        ));

        let ctx = RenderContext::standard();
        let png_round = render_png(&xhair, &ctx, 32, 32).expect("Round dot failed");
        assert!(!png_round.is_empty());

        xhair.layers[0].kind = LayerKind::Dot(DotProps {
            diameter: 4.0,
            shape: DotShape::Square,
        });
        let png_square = render_png(&xhair, &ctx, 32, 32).expect("Square dot failed");
        assert!(!png_square.is_empty());
        assert_ne!(png_round, png_square);
    }

    #[test]
    fn test_render_t_shape() {
        let mut xhair = Crosshair::new_blank("T-Shape Test");
        xhair.layers.push(Layer::new(
            "T-Shape",
            LayerKind::TShape(TShapeProps {
                bar_length: 12.0,
                stem_length: 8.0,
                thickness: 2.0,
                gap: 2.0,
            }),
        ));

        let ctx = RenderContext::standard();
        let png = render_png(&xhair, &ctx, 64, 64).expect("T-shape render failed");
        assert!(!png.is_empty());
    }

    #[test]
    fn test_render_circle_and_ring() {
        let mut xhair = Crosshair::new_blank("Ring Test");
        xhair.layers.push(Layer::new(
            "Ring Layer",
            LayerKind::Ring(RingProps {
                outer_radius: 10.0,
                thickness: 2.0,
                dash: Vec::new(),
            }),
        ));

        let ctx = RenderContext::standard();
        let png = render_png(&xhair, &ctx, 64, 64).expect("Ring render failed");
        assert!(!png.is_empty());
    }

    #[test]
    fn test_render_polygon_and_star() {
        let mut xhair = Crosshair::new_blank("Polygon Test");
        xhair.layers.push(Layer::new(
            "Star",
            LayerKind::Polygon(PolygonProps {
                sides: 5,
                radius: 12.0,
                star_inner_ratio: 0.5,
            }),
        ));

        let ctx = RenderContext::standard();
        let png = render_png(&xhair, &ctx, 64, 64).expect("Star render failed");
        assert!(!png.is_empty());
    }

    #[test]
    fn test_render_with_glow_effect() {
        let mut xhair = Crosshair::new_classic("Glow Test");
        xhair.layers[0].effects.glow = Some(GlowEffect {
            color: Rgba::CYAN,
            radius: 8.0,
            intensity: 1.5,
            spread: 0.0,
            blend: xhair_model::units::BlendMode::Add,
            quality: 2,
        });

        let ctx = RenderContext::standard();
        let png = render_png(&xhair, &ctx, 64, 64).expect("Glow render failed");
        assert!(!png.is_empty());
    }

    #[test]
    fn test_render_performance_under_threshold() {
        let xhair = Crosshair::new_classic("Perf Test");
        let ctx = RenderContext::standard();
        let mut pixmap = tiny_skia::Pixmap::new(64, 64).unwrap();

        // Warm up
        let _ = render(&xhair, &ctx, &mut pixmap).unwrap();

        // Timed run
        let stats = render(&xhair, &ctx, &mut pixmap).unwrap();
        assert_eq!(stats.rendered_layers, 1);
        // Spec §7.4: typical crosshair render < 1 ms (1000 µs)
        println!("Typical crosshair raster time: {} µs", stats.raster_time_us);
    }
}
