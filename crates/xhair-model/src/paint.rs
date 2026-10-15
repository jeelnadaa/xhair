//! Paints, gradients, outlines, and stroke styling.

use crate::units::{Degrees, Rgba, Vec2};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Color stop along a gradient line or curve.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct GradientStop {
    /// Normalized position from 0.0 to 1.0.
    pub position: f32,
    /// Color at this position.
    pub color: Rgba,
}

/// Linear color gradient.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct LinearGradient {
    /// Angle of the linear gradient axis in degrees.
    pub angle: Degrees,
    /// Ordered list of color stops (between 2 and 16 stops).
    pub stops: Vec<GradientStop>,
}

/// Radial color gradient.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RadialGradient {
    /// Center origin point.
    pub center: Vec2,
    /// Radius of the outer color stop in game pixels.
    pub radius: f32,
    /// Ordered list of color stops.
    pub stops: Vec<GradientStop>,
}

/// Conic (angular/sweep) color gradient.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ConicGradient {
    /// Center origin point.
    pub center: Vec2,
    /// Starting angle in degrees.
    pub start_angle: Degrees,
    /// Ordered list of color stops.
    pub stops: Vec<GradientStop>,
}

/// Surface fill or outline paint definition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Paint {
    /// Flat single color.
    Solid {
        /// RGBA color.
        color: Rgba,
    },
    /// Linear gradient fill.
    Linear(LinearGradient),
    /// Radial gradient fill.
    Radial(RadialGradient),
    /// Conic sweep gradient fill.
    Conic(ConicGradient),
}

impl Default for Paint {
    fn default() -> Self {
        Self::Solid { color: Rgba::CYAN }
    }
}

/// Position of outline relative to shape boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "snake_case")]
pub enum StrokePosition {
    /// Stroke drawn inside shape perimeter.
    Inside,
    /// Stroke centered along boundary.
    #[default]
    Center,
    /// Stroke drawn outside shape boundary.
    Outside,
}

/// End cap styling for open paths and lines.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "snake_case")]
pub enum StrokeCap {
    /// Flat squared-off end at exact coordinate.
    #[default]
    Butt,
    /// Semicircular rounded cap.
    Round,
    /// Squared end extending half stroke-width past coordinate.
    Square,
}

/// Corner join styling between connected segments.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "snake_case")]
pub enum StrokeJoin {
    /// Sharp corner with miter limit.
    #[default]
    Miter,
    /// Rounded corner join.
    Round,
    /// Beveled flat corner.
    Bevel,
}

/// Stroke / outline configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Stroke {
    /// Stroke width in game pixels (0.0 to 64.0).
    pub width: f32,
    /// Paint used to draw the stroke.
    pub paint: Paint,
    /// Inside, center, or outside positioning.
    pub position: StrokePosition,
    /// Dash array pattern (empty for solid line).
    pub dash: Vec<f32>,
    /// Corner join mode.
    pub join: StrokeJoin,
    /// Cap mode for line ends.
    pub cap: StrokeCap,
}

impl Default for Stroke {
    fn default() -> Self {
        Self {
            width: 1.0,
            paint: Paint::Solid { color: Rgba::BLACK },
            position: StrokePosition::Center,
            dash: Vec::new(),
            join: StrokeJoin::Miter,
            cap: StrokeCap::Butt,
        }
    }
}
