//! Plus / Cross reticle rasterization.

use crate::paint::create_skia_paint;
use crate::stroke::create_skia_stroke;
use tiny_skia::{PathBuilder, Pixmap, Transform};
use xhair_model::layer::Layer;
use xhair_model::shapes::PlusProps;

/// Rasterizes plus/cross shape with optional center hole.
pub fn render_plus(
    props: &PlusProps,
    layer: &Layer,
    instance_transform: Transform,
    opacity: f32,
    anti_alias: bool,
    pixmap: &mut Pixmap,
) {
    let length = props.length.max(0.1);
    let t = props.thickness.max(0.1);
    let half_t = t * 0.5;
    let gap = props.gap.max(props.center_hole * 0.5);

    let skia_fill = create_skia_paint(&layer.fill, opacity, anti_alias, None);

    let mut pb = PathBuilder::new();

    // 4 arms: Top, Bottom, Left, Right
    // Top arm
    pb.move_to(-half_t, -(gap + length));
    pb.line_to(half_t, -(gap + length));
    pb.line_to(half_t, -gap);
    pb.line_to(-half_t, -gap);
    pb.close();

    // Bottom arm
    pb.move_to(-half_t, gap);
    pb.line_to(half_t, gap);
    pb.line_to(half_t, gap + length);
    pb.line_to(-half_t, gap + length);
    pb.close();

    // Left arm
    pb.move_to(-(gap + length), -half_t);
    pb.line_to(-gap, -half_t);
    pb.line_to(-gap, half_t);
    pb.line_to(-(gap + length), half_t);
    pb.close();

    // Right arm
    pb.move_to(gap, -half_t);
    pb.line_to(gap + length, -half_t);
    pb.line_to(gap + length, half_t);
    pb.line_to(gap, half_t);
    pb.close();

    if let Some(path) = pb.finish() {
        pixmap.fill_path(
            &path,
            &skia_fill,
            tiny_skia::FillRule::Winding,
            instance_transform,
            None,
        );

        if let Some(ref stroke) = layer.stroke {
            let skia_stroke = create_skia_stroke(stroke);
            let stroke_paint = create_skia_paint(&stroke.paint, opacity, anti_alias, None);
            pixmap.stroke_path(&path, &stroke_paint, &skia_stroke, instance_transform, None);
        }
    }
}
