//! Visual effects: glow, drop shadow, gaussian softening, and bloom.

use crate::units::{BlendMode, Rgba, Vec2};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Outer radiant glow effect.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct GlowEffect {
    /// Tint color of the glow.
    pub color: Rgba,
    /// Blur radius in game pixels (maximum 64.0 px).
    pub radius: f32,
    /// Glow multiplier intensity (0.0 to 4.0).
    pub intensity: f32,
    /// Spread extension factor (0.0 to 1.0).
    pub spread: f32,
    /// Blending mode (e.g. Add or Normal).
    pub blend: BlendMode,
    /// Rendering quality pass count (1 to 3 box blur iterations).
    pub quality: u8,
}

impl Default for GlowEffect {
    fn default() -> Self {
        Self {
            color: Rgba::CYAN,
            radius: 4.0,
            intensity: 1.0,
            spread: 0.0,
            blend: BlendMode::Add,
            quality: 3,
        }
    }
}

/// Drop shadow effect.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ShadowEffect {
    /// Shadow color (usually black with fractional opacity).
    pub color: Rgba,
    /// Directional pixel offset.
    pub offset: Vec2,
    /// Softening blur radius in game pixels.
    pub blur: f32,
    /// Normalized shadow opacity.
    pub opacity: f32,
}

impl Default for ShadowEffect {
    fn default() -> Self {
        Self {
            color: Rgba::BLACK,
            offset: Vec2::new(1.0, 1.0),
            blur: 2.0,
            opacity: 0.8,
        }
    }
}

/// Global composite bloom effect.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct BloomEffect {
    /// Luminance threshold above which bloom triggers (0.0 to 1.0).
    pub threshold: f32,
    /// Bloom dispersion radius.
    pub radius: f32,
    /// Intensity multiplier.
    pub intensity: f32,
}

impl Default for BloomEffect {
    fn default() -> Self {
        Self {
            threshold: 0.7,
            radius: 8.0,
            intensity: 1.2,
        }
    }
}

/// Composite effects configuration container.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Default)]
pub struct Effects {
    /// Optional glow effect.
    pub glow: Option<GlowEffect>,
    /// Optional drop shadow.
    pub shadow: Option<ShadowEffect>,
    /// Optional blur softening of the layer itself.
    pub blur: Option<f32>,
    /// Optional global bloom.
    pub bloom: Option<BloomEffect>,
}
