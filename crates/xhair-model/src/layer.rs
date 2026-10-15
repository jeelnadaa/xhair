//! Layer representation, transforms, symmetry overrides, and bindings.

use crate::effects::Effects;
use crate::paint::{Paint, Stroke};
use crate::shapes::LayerKind;
use crate::units::{Alpha, Anchor, BlendMode, Degrees, LayerId, Rgba, Vec2};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// 2D affine transform parameters for a layer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Transform {
    /// Position offset in game pixels.
    pub offset: Vec2,
    /// Rotation angle in degrees.
    pub rotation: Degrees,
    /// Non-uniform scale factor.
    pub scale: Vec2,
    /// Pivot anchor point for rotation and scaling.
    pub anchor: Anchor,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            offset: Vec2::ZERO,
            rotation: Degrees(0.0),
            scale: Vec2::ONE,
            anchor: Anchor::Center,
        }
    }
}

/// Symmetry override options for an individual layer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SymmetryOverride {
    /// Inherit crosshair-level symmetry.
    #[default]
    Inherit,
    /// Disable symmetry for this layer.
    None,
}

/// Override settings applied to a single arm of a symmetric/multi-arm shape.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ArmOverride {
    /// 0-indexed arm identifier, ordered clockwise starting from `start_angle`.
    pub arm_index: u32,
    /// Positional fine-nudge offset for this specific arm.
    pub offset: Option<Vec2>,
    /// Length difference relative to base shape length.
    pub length_delta: Option<f32>,
    /// Thickness override for this arm.
    pub thickness: Option<f32>,
    /// Color override for this arm.
    pub color: Option<Rgba>,
    /// Visibility toggle for this arm (hide an arm).
    pub visible: Option<bool>,
}

/// Property binding mutation mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "snake_case")]
pub enum BindingMode {
    /// Add driver value to current property value.
    #[default]
    Add,
    /// Overwrite property value with driver output.
    Set,
    /// Multiply current property value by driver output.
    Multiply,
}

/// Dynamic reticle property binding.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct DynamicBinding {
    /// Dot-separated target property path (e.g. "kind.line.gap").
    pub property_path: String,
    /// Simulation driver name (e.g. "spread", "movement", "firing").
    pub driver: String,
    /// Minimum and maximum input driver range.
    pub in_range: (f32, f32),
    /// Minimum and maximum mapped property output range.
    pub out_range: (f32, f32),
    /// Arithmetic combination mode.
    pub mode: BindingMode,
}

/// Individual reticle layer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Layer {
    /// Unique layer ID.
    pub id: LayerId,
    /// Human-readable layer name.
    pub name: String,
    /// Visibility flag.
    pub visible: bool,
    /// Edit lock flag.
    pub locked: bool,
    /// Solo isolation preview flag.
    pub solo: bool,
    /// Geometric shape kind and parameters.
    pub kind: LayerKind,
    /// Transform properties (offset, rotation, scale, anchor).
    pub transform: Transform,
    /// Surface fill paint.
    pub fill: Paint,
    /// Optional outline stroke.
    pub stroke: Option<Stroke>,
    /// Overall layer opacity (0.0 to 1.0).
    pub opacity: Alpha,
    /// Blend mode for this layer.
    pub blend: BlendMode,
    /// Layer-specific effects (glow, shadow, blur).
    pub effects: Effects,
    /// Symmetry inheritance or override.
    pub symmetry: SymmetryOverride,
    /// Per-arm overrides for symmetric shapes.
    pub arm_overrides: Vec<ArmOverride>,
    /// Dynamic Reticle simulated property bindings.
    pub bindings: Vec<DynamicBinding>,
}

impl Default for Layer {
    fn default() -> Self {
        Self {
            id: LayerId::new_v7(),
            name: "New Layer".to_string(),
            visible: true,
            locked: false,
            solo: false,
            kind: LayerKind::default(),
            transform: Transform::default(),
            fill: Paint::default(),
            stroke: None,
            opacity: Alpha::default(),
            blend: BlendMode::Normal,
            effects: Effects::default(),
            symmetry: SymmetryOverride::Inherit,
            arm_overrides: Vec::new(),
            bindings: Vec::new(),
        }
    }
}

impl Layer {
    /// Creates a new named layer with a given shape kind.
    pub fn new(name: impl Into<String>, kind: LayerKind) -> Self {
        Self {
            name: name.into(),
            kind,
            ..Default::default()
        }
    }
}
