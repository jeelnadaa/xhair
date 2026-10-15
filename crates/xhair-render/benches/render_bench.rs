//! Benchmark for typical and heavy crosshair rasterization performance (§7.4).
//!
//! Enforces:
//! - Typical crosshair render < 1 ms (1000 µs)
//! - Heavy crosshair with multi-layer glow, shadow, and bloom < 8 ms (8000 µs)

use std::time::Instant;
use xhair_model::crosshair::Crosshair;
use xhair_model::effects::{BloomEffect, GlowEffect, ShadowEffect};
use xhair_model::layer::Layer;
use xhair_model::shapes::{LayerKind, RingProps};
use xhair_model::units::{Rgba, Vec2};
use xhair_render::{render, RenderContext};

fn create_heavy_crosshair() -> Crosshair {
    let mut xhair = Crosshair::new_classic("Heavy Crosshair Benchmark");
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

    xhair
}

fn main() {
    println!("=== Running xhair-render Benchmark (§7.4) ===");

    let typical_xhair = Crosshair::new_classic("Classic Typical");
    let heavy_xhair = create_heavy_crosshair();
    let ctx = RenderContext::standard();

    let iterations = 100;

    // 1. Benchmark Typical Crosshair (budget < 1 ms = 1000 µs)
    let mut typical_pixmap = tiny_skia::Pixmap::new(64, 64).unwrap();
    // Warmup
    for _ in 0..10 {
        let _ = render(&typical_xhair, &ctx, &mut typical_pixmap);
    }
    let t0 = Instant::now();
    for _ in 0..iterations {
        let _ = render(&typical_xhair, &ctx, &mut typical_pixmap);
    }
    let typical_avg_us = t0.elapsed().as_micros() as f64 / iterations as f64;
    println!(
        "Typical crosshair average: {:.2} µs (budget: 1000 µs)",
        typical_avg_us
    );
    assert!(
        typical_avg_us < 1000.0,
        "Typical crosshair average ({:.2} µs) exceeds 1000 µs budget",
        typical_avg_us
    );

    // 2. Benchmark Heavy Crosshair (budget < 8 ms = 8000 µs)
    let mut heavy_pixmap = tiny_skia::Pixmap::new(128, 128).unwrap();
    // Warmup
    for _ in 0..10 {
        let _ = render(&heavy_xhair, &ctx, &mut heavy_pixmap);
    }
    let t1 = Instant::now();
    for _ in 0..iterations {
        let _ = render(&heavy_xhair, &ctx, &mut heavy_pixmap);
    }
    let heavy_avg_us = t1.elapsed().as_micros() as f64 / iterations as f64;
    println!(
        "Heavy crosshair average: {:.2} µs (budget: 8000 µs)",
        heavy_avg_us
    );
    assert!(
        heavy_avg_us < 8000.0,
        "Heavy crosshair average ({:.2} µs) exceeds 8000 µs budget",
        heavy_avg_us
    );

    println!("All §7.4 performance budgets PASSED successfully.");
}
