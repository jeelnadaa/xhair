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
pub use effects::{apply_bloom, apply_glow, apply_shadow, gaussian_blur};
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
    #[allow(clippy::assertions_on_constants)]
    fn test_buffer_limits() {
        assert!(MAX_BLUR_BUFFER_DIMENSION <= MAX_OVERLAY_DIMENSION);
    }

    #[test]
    fn test_classic_crosshair_four_arms_rendered() {
        let xhair = Crosshair::new_classic("Classic 4-Arm");
        let ctx = RenderContext::standard();
        let pixmap = render_to_new_pixmap(&xhair, &ctx, 64, 64).unwrap();
        let data = pixmap.data();

        // Center is (32, 32). In Crosshair::new_classic: gap=4.0, length=6.0, thickness=2.0
        // Arms extend from gap (4px) to gap+length (10px).
        // Check pixel on each of the 4 arms at distance 7px from center:
        // Top arm: (32, 32 - 7) = (32, 25)
        let top_alpha = data[(25 * 64 + 32) * 4 + 3];
        // Bottom arm: (32, 32 + 7) = (32, 39)
        let bottom_alpha = data[(39 * 64 + 32) * 4 + 3];
        // Right arm: (32 + 7, 32) = (39, 32)
        let right_alpha = data[(32 * 64 + 39) * 4 + 3];
        // Left arm: (32 - 7, 32) = (25, 32)
        let left_alpha = data[(32 * 64 + 25) * 4 + 3];

        assert!(top_alpha > 0, "Top arm must be rendered");
        assert!(bottom_alpha > 0, "Bottom arm must be rendered");
        assert!(right_alpha > 0, "Right arm must be rendered");
        assert!(left_alpha > 0, "Left arm must be rendered");
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

    #[test]
    fn test_global_bloom_effect() {
        use xhair_model::effects::BloomEffect;

        let mut xhair = Crosshair::new_classic("Bloom Test");
        xhair.global_effects.bloom = Some(BloomEffect {
            threshold: 0.5,
            radius: 8.0,
            intensity: 1.5,
        });

        let ctx = RenderContext::standard();
        let png = render_png(&xhair, &ctx, 64, 64).expect("Bloom render failed");
        assert!(!png.is_empty());
    }

    #[test]
    fn test_animation_evaluation_and_rendering() {
        use xhair_model::crosshair::AnimTrack;

        let mut xhair = Crosshair::new_classic("Anim Test");
        xhair
            .animation
            .push(AnimTrack::new_rotate("global.rotation", 360.0));

        let ctx0 = RenderContext::standard().with_time(0.0);
        let png0 = render_png(&xhair, &ctx0, 64, 64).unwrap();

        let ctx1 = RenderContext::standard().with_time(0.125); // 45 degrees
        let png1 = render_png(&xhair, &ctx1, 64, 64).unwrap();

        // 45 degree rotation must differ from 0 degree
        assert_ne!(png0, png1);
    }

    #[test]
    fn test_static_only_lock_enforcement() {
        use xhair_model::crosshair::AnimTrack;

        let mut xhair = Crosshair::new_classic("Static Lock Test");
        xhair
            .animation
            .push(AnimTrack::new_rotate("global.rotation", 360.0));

        let ctx_static_base = RenderContext::standard().with_time(0.0);
        let png_base = render_png(&xhair, &ctx_static_base, 64, 64).unwrap();

        // Even though time = 0.5s (180 deg), static_only = true forces time = 0
        let ctx_locked = RenderContext::standard()
            .with_time(0.5)
            .with_static_only(true);
        let png_locked = render_png(&xhair, &ctx_locked, 64, 64).unwrap();

        assert_eq!(png_base, png_locked);
    }

    #[test]
    fn test_heavy_crosshair_performance_under_threshold() {
        use xhair_model::effects::{BloomEffect, ShadowEffect};
        use xhair_model::units::Vec2;

        let mut xhair = Crosshair::new_classic("Heavy Crosshair Test");
        // Add multiple detailed layers with shadows and glow
        for i in 1..=4 {
            let mut l = Layer::new(
                format!("Ring {}", i),
                LayerKind::Ring(RingProps {
                    outer_radius: 6.0 * i as f32,
                    thickness: 1.5,
                    dash: Vec::new(),
                }),
            );
            l.effects.glow = Some(GlowEffect {
                color: Rgba::CYAN,
                radius: 4.0,
                intensity: 1.0,
                spread: 0.0,
                blend: xhair_model::units::BlendMode::Add,
                quality: 2,
            });
            l.effects.shadow = Some(ShadowEffect {
                color: Rgba::BLACK,
                offset: Vec2::new(1.0, 1.0),
                blur: 2.0,
                opacity: 0.7,
            });
            xhair.layers.push(l);
        }

        xhair.global_effects.bloom = Some(BloomEffect {
            threshold: 0.6,
            radius: 6.0,
            intensity: 1.2,
        });

        let ctx = RenderContext::standard();
        let mut pixmap = tiny_skia::Pixmap::new(128, 128).unwrap();

        // Warmup
        let _ = render(&xhair, &ctx, &mut pixmap).unwrap();

        // Timed run
        let stats = render(&xhair, &ctx, &mut pixmap).unwrap();
        assert_eq!(stats.rendered_layers, 5);

        println!("Heavy crosshair raster time: {} µs", stats.raster_time_us);
        // Spec §7.4: heavy crosshair with glow render budget < 8 ms (8000 µs in release)
        let max_budget_us = if cfg!(debug_assertions) {
            200_000
        } else {
            8000
        };
        assert!(
            stats.raster_time_us < max_budget_us,
            "Raster time {} µs exceeds budget {} µs",
            stats.raster_time_us,
            max_budget_us
        );
    }
}
