//! Shape primitives and their parameters.

use crate::paint::StrokeCap;
use crate::units::{Degrees, Rgba};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Parameters for symmetric line arms (classic crosshair rays).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct LineProps {
    /// Number of arms (1 to 16, default 4).
    pub arm_count: u32,
    /// Starting angle in degrees (default 0.0, pointing right or top).
    pub start_angle: Degrees,
    /// Angular separation step between arms in degrees.
    pub angle_step: Option<Degrees>,
    /// Arm length in game pixels.
    pub length: f32,
    /// Arm thickness in game pixels.
    pub thickness: f32,
    /// Distance of inner arm end from the center point.
    pub gap: f32,
    /// Line end cap style.
    pub cap: StrokeCap,
    /// Width taper along arm (0.0 = uniform, 1.0 = sharp tip).
    pub taper: f32,
}

impl Default for LineProps {
    fn default() -> Self {
        Self {
            arm_count: 4,
            start_angle: Degrees(0.0),
            angle_step: None,
            length: 6.0,
            thickness: 2.0,
            gap: 4.0,
            cap: StrokeCap::Butt,
            taper: 0.0,
        }
    }
}

/// Shape variant of a center dot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "snake_case")]
pub enum DotShape {
    /// Square dot snapped to grid.
    Square,
    /// Round anti-aliased dot.
    #[default]
    Round,
}

/// Parameters for a center dot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct DotProps {
    /// Diameter in game pixels.
    pub diameter: f32,
    /// Round or square dot.
    pub shape: DotShape,
}

impl Default for DotProps {
    fn default() -> Self {
        Self {
            diameter: 2.0,
            shape: DotShape::Round,
        }
    }
}

/// Parameters for rectangle shape.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RectProps {
    /// Width in game pixels.
    pub width: f32,
    /// Height in game pixels.
    pub height: f32,
    /// Per-corner radius `[top_left, top_right, bottom_right, bottom_left]`.
    pub corner_radii: [f32; 4],
}

impl Default for RectProps {
    fn default() -> Self {
        Self {
            width: 8.0,
            height: 8.0,
            corner_radii: [0.0; 4],
        }
    }
}

/// Parameters for circle shape.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CircleProps {
    /// Diameter in game pixels.
    pub diameter: f32,
}

impl Default for CircleProps {
    fn default() -> Self {
        Self { diameter: 10.0 }
    }
}

/// Parameters for hollow ring / donut shape.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RingProps {
    /// Outer radius in game pixels.
    pub outer_radius: f32,
    /// Ring wall thickness.
    pub thickness: f32,
    /// Optional dash pattern (e.g. `[4.0, 2.0]`).
    pub dash: Vec<f32>,
}

impl Default for RingProps {
    fn default() -> Self {
        Self {
            outer_radius: 8.0,
            thickness: 2.0,
            dash: Vec::new(),
        }
    }
}

/// Parameters for circular arc segment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ArcProps {
    /// Radius in game pixels.
    pub radius: f32,
    /// Arc stroke thickness.
    pub thickness: f32,
    /// Starting angle in degrees.
    pub start_angle: Degrees,
    /// Ending angle in degrees.
    pub end_angle: Degrees,
    /// End cap style.
    pub cap: StrokeCap,
}

impl Default for ArcProps {
    fn default() -> Self {
        Self {
            radius: 8.0,
            thickness: 2.0,
            start_angle: Degrees(0.0),
            end_angle: Degrees(180.0),
            cap: StrokeCap::Round,
        }
    }
}

/// Parameters for regular polygon or star shape.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct PolygonProps {
    /// Side count (3 to 64).
    pub sides: u32,
    /// Outer radius in game pixels.
    pub radius: f32,
    /// Ratio of inner star vertices to outer radius (0.0 = regular polygon).
    pub star_inner_ratio: f32,
}

impl Default for PolygonProps {
    fn default() -> Self {
        Self {
            sides: 3,
            radius: 8.0,
            star_inner_ratio: 0.0,
        }
    }
}

/// Parameters for isosceles triangle shape.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct TriangleProps {
    /// Base width in game pixels.
    pub base: f32,
    /// Height in game pixels.
    pub height: f32,
}

impl Default for TriangleProps {
    fn default() -> Self {
        Self {
            base: 6.0,
            height: 6.0,
        }
    }
}

/// Parameters for chevron (arrowhead/V-shape) reticle.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ChevronProps {
    /// Length of each arm.
    pub arm_length: f32,
    /// Arm stroke thickness.
    pub thickness: f32,
    /// Internal opening angle in degrees.
    pub angle: Degrees,
    /// Gap offset from center.
    pub gap: f32,
}

impl Default for ChevronProps {
    fn default() -> Self {
        Self {
            arm_length: 6.0,
            thickness: 2.0,
            angle: Degrees(90.0),
            gap: 2.0,
        }
    }
}

/// Parameters for T-shaped reticle.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct TShapeProps {
    /// Horizontal top bar length.
    pub bar_length: f32,
    /// Vertical stem length.
    pub stem_length: f32,
    /// Thickness of bar and stem.
    pub thickness: f32,
    /// Distance from center.
    pub gap: f32,
}

impl Default for TShapeProps {
    fn default() -> Self {
        Self {
            bar_length: 8.0,
            stem_length: 6.0,
            thickness: 2.0,
            gap: 2.0,
        }
    }
}

/// Parameters for cross/plus reticle with optional center hole.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct PlusProps {
    /// Length of each arm from center.
    pub length: f32,
    /// Thickness of arms.
    pub thickness: f32,
    /// Gap distance from center.
    pub gap: f32,
    /// Diameter of center hole.
    pub center_hole: f32,
}

impl Default for PlusProps {
    fn default() -> Self {
        Self {
            length: 8.0,
            thickness: 2.0,
            gap: 2.0,
            center_hole: 2.0,
        }
    }
}

/// SVG path fill rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "snake_case")]
pub enum FillRule {
    /// Non-zero winding rule.
    #[default]
    NonZero,
    /// Even-odd fill rule.
    EvenOdd,
}

/// Custom vector path shape.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Default)]
pub struct PathProps {
    /// SVG path string data (e.g. "M 0 0 L 10 10 Z").
    pub data: String,
    /// Fill rule.
    pub fill_rule: FillRule,
}

/// Run-length encoded RGBA pixel grid.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct PixelsProps {
    /// Grid width (maximum 256).
    pub width: u32,
    /// Grid height (maximum 256).
    pub height: u32,
    /// Pixel data as run-length encoded byte values.
    pub rle_data: Vec<u8>,
}

impl Default for PixelsProps {
    fn default() -> Self {
        Self {
            width: 16,
            height: 16,
            rle_data: Vec::new(),
        }
    }
}

/// Image sampling mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "snake_case")]
pub enum SamplingMode {
    /// Nearest neighbor (sharp pixel art).
    #[default]
    Nearest,
    /// Bilinear interpolation.
    Bilinear,
}

/// Embedded raster or vector asset reference.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Default)]
pub struct ImageProps {
    /// Content hash referencing embedded asset in Crosshair assets table.
    pub asset_hash: String,
    /// Texture filtering mode.
    pub sampling: SamplingMode,
    /// Optional color tint overlay.
    pub tint: Option<Rgba>,
}

/// Group container isolating transforms and visual effects.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Default)]
pub struct GroupProps {
    /// Nested child layer IDs.
    pub children: Vec<crate::units::LayerId>,
}

/// Kind of layer shape primitive.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum LayerKind {
    /// Symmetric line arms.
    Line(LineProps),
    /// Center dot.
    Dot(DotProps),
    /// Rectangle.
    Rect(RectProps),
    /// Circle.
    Circle(CircleProps),
    /// Ring.
    Ring(RingProps),
    /// Arc.
    Arc(ArcProps),
    /// Regular polygon or star.
    Polygon(PolygonProps),
    /// Triangle.
    Triangle(TriangleProps),
    /// Chevron / Arrow.
    Chevron(ChevronProps),
    /// T-Shape.
    TShape(TShapeProps),
    /// Plus cross.
    Plus(PlusProps),
    /// Vector path.
    Path(PathProps),
    /// Pixel grid.
    Pixels(PixelsProps),
    /// Embedded Image.
    Image(ImageProps),
    /// Group container.
    Group(GroupProps),
}

impl Default for LayerKind {
    fn default() -> Self {
        Self::Line(LineProps::default())
    }
}
