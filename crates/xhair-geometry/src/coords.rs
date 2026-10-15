//! Coordinate system definitions, edge coordinates, and screen midpoint math.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Indicates whether screen center falls on a pixel boundary edge or pixel middle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CenterAlignment {
    /// Dimension is even: center falls on the boundary edge between two pixels.
    PixelEdge,
    /// Dimension is odd: center falls squarely in the middle of a pixel.
    PixelMiddle,
}

/// Calculated screen center coordinates in continuous edge coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ScreenCenter {
    /// Center X coordinate.
    pub cx: f64,
    /// Center Y coordinate.
    pub cy: f64,
    /// Horizontal alignment classification.
    pub horizontal_alignment: CenterAlignment,
    /// Vertical alignment classification.
    pub vertical_alignment: CenterAlignment,
}

/// Computes the exact screen center in edge coordinates.
///
/// - For an even dimension (e.g. 1920), center lies on an edge: 960.0
/// - For an odd dimension (e.g. 1921), center lies in the middle of a pixel: 960.5
pub fn compute_screen_center(width: u32, height: u32) -> ScreenCenter {
    let cx = width as f64 / 2.0;
    let cy = height as f64 / 2.0;

    let horizontal_alignment = if width % 2 == 0 {
        CenterAlignment::PixelEdge
    } else {
        CenterAlignment::PixelMiddle
    };

    let vertical_alignment = if height % 2 == 0 {
        CenterAlignment::PixelEdge
    } else {
        CenterAlignment::PixelMiddle
    };

    ScreenCenter {
        cx,
        cy,
        horizontal_alignment,
        vertical_alignment,
    }
}
