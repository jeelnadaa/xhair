//! Symmetry instance generation and arm overrides.

use tiny_skia::Transform;
use xhair_model::crosshair::Symmetry;
use xhair_model::layer::{ArmOverride, Layer, SymmetryOverride};
use xhair_model::units::Vec2;

/// A single rendered instance of a layer with its derived transform and arm overrides.
#[derive(Debug, Clone)]
pub struct SymmetryInstance<'a> {
    /// 0-indexed arm identifier.
    pub arm_index: u32,
    /// 2D affine transform matrix for tiny-skia.
    pub transform: Transform,
    /// Optional arm override for this instance.
    pub arm_override: Option<&'a ArmOverride>,
}

/// Computes the effective symmetry mode for a layer.
pub fn effective_symmetry(
    crosshair_symmetry: &Symmetry,
    layer_override: &SymmetryOverride,
) -> Symmetry {
    match layer_override {
        SymmetryOverride::None => Symmetry::None,
        SymmetryOverride::Inherit => crosshair_symmetry.clone(),
    }
}

/// Generates all symmetry instances for a layer given the active symmetry mode.
pub fn generate_symmetry_instances<'a>(
    layer: &'a Layer,
    crosshair_symmetry: &Symmetry,
    center_offset: Vec2,
) -> Vec<SymmetryInstance<'a>> {
    let sym = effective_symmetry(crosshair_symmetry, &layer.symmetry);
    let mut instances = Vec::new();

    let base_offset = layer.transform.offset + center_offset;
    let base_rot = layer.transform.rotation.0;
    let base_scale = layer.transform.scale;

    match sym {
        Symmetry::None => {
            let arm_override = layer.arm_overrides.iter().find(|o| o.arm_index == 0);
            if arm_override.and_then(|o| o.visible).unwrap_or(true) {
                let (final_offset, final_scale, final_rot) =
                    apply_override_transforms(base_offset, base_scale, base_rot, arm_override);
                let transform = make_transform(final_offset, final_scale, final_rot);
                instances.push(SymmetryInstance {
                    arm_index: 0,
                    transform,
                    arm_override,
                });
            }
        }
        Symmetry::MirrorH => {
            for (idx, sx_mult) in [1.0f32, -1.0f32].iter().enumerate() {
                let arm_override = layer
                    .arm_overrides
                    .iter()
                    .find(|o| o.arm_index == idx as u32);
                if arm_override.and_then(|o| o.visible).unwrap_or(true) {
                    let scale = Vec2::new(base_scale.x * sx_mult, base_scale.y);
                    let (final_offset, final_scale, final_rot) =
                        apply_override_transforms(base_offset, scale, base_rot, arm_override);
                    let transform = make_transform(final_offset, final_scale, final_rot);
                    instances.push(SymmetryInstance {
                        arm_index: idx as u32,
                        transform,
                        arm_override,
                    });
                }
            }
        }
        Symmetry::MirrorV => {
            for (idx, sy_mult) in [1.0f32, -1.0f32].iter().enumerate() {
                let arm_override = layer
                    .arm_overrides
                    .iter()
                    .find(|o| o.arm_index == idx as u32);
                if arm_override.and_then(|o| o.visible).unwrap_or(true) {
                    let scale = Vec2::new(base_scale.x, base_scale.y * sy_mult);
                    let (final_offset, final_scale, final_rot) =
                        apply_override_transforms(base_offset, scale, base_rot, arm_override);
                    let transform = make_transform(final_offset, final_scale, final_rot);
                    instances.push(SymmetryInstance {
                        arm_index: idx as u32,
                        transform,
                        arm_override,
                    });
                }
            }
        }
        Symmetry::MirrorHV => {
            let multipliers = [(1.0, 1.0), (-1.0, 1.0), (1.0, -1.0), (-1.0, -1.0)];
            for (idx, (sx_mult, sy_mult)) in multipliers.iter().enumerate() {
                let arm_override = layer
                    .arm_overrides
                    .iter()
                    .find(|o| o.arm_index == idx as u32);
                if arm_override.and_then(|o| o.visible).unwrap_or(true) {
                    let scale = Vec2::new(base_scale.x * sx_mult, base_scale.y * sy_mult);
                    let (final_offset, final_scale, final_rot) =
                        apply_override_transforms(base_offset, scale, base_rot, arm_override);
                    let transform = make_transform(final_offset, final_scale, final_rot);
                    instances.push(SymmetryInstance {
                        arm_index: idx as u32,
                        transform,
                        arm_override,
                    });
                }
            }
        }
        Symmetry::Radial { arms } => {
            let n = arms.clamp(1, 32);
            let angle_step = 360.0 / n as f32;
            for i in 0..n {
                let arm_override = layer.arm_overrides.iter().find(|o| o.arm_index == i);
                if arm_override.and_then(|o| o.visible).unwrap_or(true) {
                    let rot = base_rot + (i as f32 * angle_step);
                    let (final_offset, final_scale, final_rot) =
                        apply_override_transforms(base_offset, base_scale, rot, arm_override);
                    let transform = make_transform(final_offset, final_scale, final_rot);
                    instances.push(SymmetryInstance {
                        arm_index: i,
                        transform,
                        arm_override,
                    });
                }
            }
        }
    }

    instances
}

fn apply_override_transforms(
    base_offset: Vec2,
    base_scale: Vec2,
    base_rot: f32,
    arm_override: Option<&ArmOverride>,
) -> (Vec2, Vec2, f32) {
    if let Some(over) = arm_override {
        let offset = if let Some(nudge) = over.offset {
            base_offset + nudge
        } else {
            base_offset
        };
        (offset, base_scale, base_rot)
    } else {
        (base_offset, base_scale, base_rot)
    }
}

/// Builds a `tiny_skia::Transform` from offset, scale, and rotation angle in degrees.
#[inline]
pub fn make_transform(offset: Vec2, scale: Vec2, rotation_deg: f32) -> Transform {
    let mut t = Transform::identity();
    if scale.x != 1.0 || scale.y != 1.0 {
        t = t.post_concat(Transform::from_scale(scale.x, scale.y));
    }
    if rotation_deg != 0.0 {
        t = t.post_concat(Transform::from_rotate(rotation_deg));
    }
    if offset.x != 0.0 || offset.y != 0.0 {
        t = t.post_concat(Transform::from_translate(offset.x, offset.y));
    }
    t
}
