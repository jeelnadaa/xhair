//! Color and blend mode conversion between model and tiny-skia.

use tiny_skia::{BlendMode as SkiaBlendMode, Color as SkiaColor};
use xhair_model::units::{BlendMode, Rgba};

/// Converts a domain model `Rgba` to `tiny_skia::Color`.
#[inline]
pub fn to_skia_color(rgba: Rgba, opacity_multiplier: f32) -> SkiaColor {
    let alpha = (rgba.a * opacity_multiplier).clamp(0.0, 1.0);
    SkiaColor::from_rgba8(rgba.r, rgba.g, rgba.b, (alpha * 255.0).round() as u8)
}

/// Converts a domain model `BlendMode` to `tiny_skia::BlendMode`.
#[inline]
pub fn to_skia_blend_mode(mode: BlendMode) -> SkiaBlendMode {
    match mode {
        BlendMode::Normal => SkiaBlendMode::SourceOver,
        BlendMode::Add => SkiaBlendMode::Plus,
        BlendMode::Screen => SkiaBlendMode::Screen,
        BlendMode::Multiply => SkiaBlendMode::Multiply,
        BlendMode::Overlay => SkiaBlendMode::Overlay,
    }
}
