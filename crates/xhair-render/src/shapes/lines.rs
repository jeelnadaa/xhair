//! Line arms rasterization (classic crosshair rays with gap, taper, and arm overrides).

use crate::paint::create_skia_paint;
use crate::stroke::create_skia_stroke;
use tiny_skia::{PathBuilder, Pixmap, Transform};
use xhair_model::layer::{ArmOverride, Layer};
use xhair_model::paint::Paint;
use xhair_model::shapes::LineProps;

/// Rasterizes symmetric line arms.
pub fn render_lines(
    props: &LineProps,
    layer: &Layer,
    arm_override: Option<&ArmOverride>,
    instance_transform: Transform,
    opacity: f32,
    anti_alias: bool,
    pixmap: &mut Pixmap,
) {
    let arm_count = props.arm_count.clamp(1, 16);
    let step = props
        .angle_step
        .map(|d| d.0)
        .unwrap_or_else(|| 360.0 / arm_count as f32);

    for i in 0..arm_count {
        // Check per-arm override for this specific radial arm
        let override_for_arm = if let Some(over) = arm_override {
            Some(over)
        } else {
            layer.arm_overrides.iter().find(|o| o.arm_index == i)
        };

        if let Some(over) = override_for_arm {
            if over.visible == Some(false) {
                continue;
            }
        }

        let length =
            (props.length + override_for_arm.and_then(|o| o.length_delta).unwrap_or(0.0)).max(0.0);

        let thickness = override_for_arm
            .and_then(|o| o.thickness)
            .unwrap_or(props.thickness)
            .max(0.1);

        let fill_paint = if let Some(color) = override_for_arm.and_then(|o| o.color) {
            Paint::Solid { color }
        } else {
            layer.fill.clone()
        };

        let skia_fill = create_skia_paint(&fill_paint, opacity, anti_alias, None);
        let arm_angle = props.start_angle.0 + (i as f32 * step);
        let arm_rot = Transform::from_rotate(arm_angle);
        let final_transform = arm_rot.post_concat(instance_transform);

        let gap = props.gap.max(0.0);
        let taper = props.taper.clamp(0.0, 1.0);

        if taper > 0.0 {
            // Tapered arm polygon
            let w_start = thickness * 0.5;
            let w_end = thickness * (1.0 - taper) * 0.5;

            let mut pb = PathBuilder::new();
            pb.move_to(-w_start, -gap);
            pb.line_to(w_start, -gap);
            pb.line_to(w_end, -(gap + length));
            pb.line_to(-w_end, -(gap + length));
            pb.close();

            if let Some(path) = pb.finish() {
                pixmap.fill_path(
                    &path,
                    &skia_fill,
                    tiny_skia::FillRule::Winding,
                    final_transform,
                    None,
                );

                if let Some(ref stroke) = layer.stroke {
                    let skia_stroke = create_skia_stroke(stroke);
                    let stroke_paint = create_skia_paint(&stroke.paint, opacity, anti_alias, None);
                    pixmap.stroke_path(&path, &stroke_paint, &skia_stroke, final_transform, None);
                }
            }
        } else {
            // Standard uniform line arm (rect or stroke)
            let half_t = thickness * 0.5;
            let mut pb = PathBuilder::new();
            pb.move_to(-half_t, -gap);
            pb.line_to(half_t, -gap);
            pb.line_to(half_t, -(gap + length));
            pb.line_to(-half_t, -(gap + length));
            pb.close();

            if let Some(path) = pb.finish() {
                pixmap.fill_path(
                    &path,
                    &skia_fill,
                    tiny_skia::FillRule::Winding,
                    final_transform,
                    None,
                );

                if let Some(ref stroke) = layer.stroke {
                    let skia_stroke = create_skia_stroke(stroke);
                    let stroke_paint = create_skia_paint(&stroke.paint, opacity, anti_alias, None);
                    pixmap.stroke_path(&path, &stroke_paint, &skia_stroke, final_transform, None);
                }
            }
        }
    }
}
