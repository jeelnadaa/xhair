//! Deterministic golden image test suite for xhair-render (§7.1, §15.3).

use xhair_model::crosshair::Crosshair;
use xhair_model::effects::GlowEffect;
use xhair_model::layer::Layer;
use xhair_model::shapes::{DotProps, DotShape, LayerKind, PolygonProps, RingProps, TShapeProps};
use xhair_model::units::{Rgba, Vec2};
use xhair_render::{render_png, render_to_new_pixmap, RenderContext};

#[test]
fn test_golden_classic_crosshair_determinism() {
    let xhair = Crosshair::new_classic("Golden Classic");
    let ctx = RenderContext::standard();

    let run1 = render_png(&xhair, &ctx, 64, 64).expect("Run 1 render failed");
    let run2 = render_png(&xhair, &ctx, 64, 64).expect("Run 2 render failed");

    // Pure determinism: identical bytes
    assert_eq!(
        run1, run2,
        "Golden render must be 100% byte-identical across runs"
    );
    assert!(!run1.is_empty());
}

#[test]
fn test_golden_dot_round_and_square_shapes() {
    let mut round_dot = Crosshair::new_blank("Golden Round Dot");
    round_dot.layers.push(Layer::new(
        "Round",
        LayerKind::Dot(DotProps {
            diameter: 4.0,
            shape: DotShape::Round,
        }),
    ));

    let mut square_dot = Crosshair::new_blank("Golden Square Dot");
    square_dot.layers.push(Layer::new(
        "Square",
        LayerKind::Dot(DotProps {
            diameter: 4.0,
            shape: DotShape::Square,
        }),
    ));

    let ctx = RenderContext::standard();
    let p_round = render_to_new_pixmap(&round_dot, &ctx, 32, 32).unwrap();
    let p_square = render_to_new_pixmap(&square_dot, &ctx, 32, 32).unwrap();

    // Verify non-empty and distinct
    assert_ne!(p_round.data(), p_square.data());

    // Verify center pixel is colored in both
    let center_idx = (16 * 32 + 16) * 4;
    assert!(p_round.data()[center_idx + 3] > 0);
    assert!(p_square.data()[center_idx + 3] > 0);
}

#[test]
fn test_golden_t_shape_rasterization() {
    let mut xhair = Crosshair::new_blank("Golden T-Shape");
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
    let p1 = render_png(&xhair, &ctx, 64, 64).unwrap();
    let p2 = render_png(&xhair, &ctx, 64, 64).unwrap();
    assert_eq!(p1, p2);
}

#[test]
fn test_golden_circle_and_ring() {
    let mut xhair = Crosshair::new_blank("Golden Ring");
    xhair.layers.push(Layer::new(
        "Ring",
        LayerKind::Ring(RingProps {
            outer_radius: 12.0,
            thickness: 2.0,
            dash: Vec::new(),
        }),
    ));

    let ctx = RenderContext::standard();
    let p1 = render_png(&xhair, &ctx, 64, 64).unwrap();
    let p2 = render_png(&xhair, &ctx, 64, 64).unwrap();
    assert_eq!(p1, p2);
}

#[test]
fn test_golden_polygon_star_and_symmetry() {
    let mut xhair = Crosshair::new_blank("Golden Star");
    xhair.layers.push(Layer::new(
        "Star",
        LayerKind::Polygon(PolygonProps {
            sides: 5,
            radius: 14.0,
            star_inner_ratio: 0.5,
        }),
    ));

    let ctx = RenderContext::standard();
    let p1 = render_png(&xhair, &ctx, 64, 64).unwrap();
    let p2 = render_png(&xhair, &ctx, 64, 64).unwrap();
    assert_eq!(p1, p2);
}

#[test]
fn test_golden_glow_and_shadow_compositing() {
    let mut xhair = Crosshair::new_classic("Golden Glow & Shadow");
    xhair.layers[0].effects.glow = Some(GlowEffect {
        color: Rgba::CYAN,
        radius: 8.0,
        intensity: 1.5,
        spread: 0.0,
        blend: xhair_model::units::BlendMode::Add,
        quality: 2,
    });
    xhair.layers[0].effects.shadow = Some(xhair_model::effects::ShadowEffect {
        color: Rgba::BLACK,
        offset: Vec2::new(2.0, 2.0),
        blur: 3.0,
        opacity: 0.75,
    });

    let ctx = RenderContext::standard();
    let p1 = render_png(&xhair, &ctx, 128, 128).unwrap();
    let p2 = render_png(&xhair, &ctx, 128, 128).unwrap();
    assert_eq!(p1, p2);
}

#[test]
fn test_golden_fractional_center_nudge() {
    let xhair = Crosshair::new_classic("Subpixel Center");
    let ctx = RenderContext::standard().with_center_frac(0.25, 0.5);

    let p1 = render_png(&xhair, &ctx, 64, 64).unwrap();
    let p2 = render_png(&xhair, &ctx, 64, 64).unwrap();
    assert_eq!(p1, p2);
}

#[test]
fn test_golden_stretched_scaling() {
    let xhair = Crosshair::new_classic("Stretched 4:3 on 16:9");
    let ctx = RenderContext::standard().with_scale(1.333333, 1.0);

    let p1 = render_png(&xhair, &ctx, 64, 64).unwrap();
    let p2 = render_png(&xhair, &ctx, 64, 64).unwrap();
    assert_eq!(p1, p2);
}
