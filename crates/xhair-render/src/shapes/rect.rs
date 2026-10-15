//! Rectangle shape rasterization with per-corner radii.

use crate::paint::create_skia_paint;
use crate::stroke::create_skia_stroke;
use tiny_skia::{PathBuilder, Pixmap, Transform};
use xhair_model::layer::Layer;
use xhair_model::shapes::RectProps;

/// Rasterizes rectangle with optional corner radii.
pub fn render_rect(
    props: &RectProps,
    layer: &Layer,
    instance_transform: Transform,
    opacity: f32,
    anti_alias: bool,
    pixmap: &mut Pixmap,
) {
    let w = props.width.max(0.1);
    let h = props.height.max(0.1);
    let hw = w * 0.5;
    let hh = h * 0.5;

    let [tl, tr, br, bl] = props.corner_radii;
    let max_r = (hw).min(hh);
    let tl = tl.clamp(0.0, max_r);
    let tr = tr.clamp(0.0, max_r);
    let br = br.clamp(0.0, max_r);
    let bl = bl.clamp(0.0, max_r);

    let skia_fill = create_skia_paint(&layer.fill, opacity, anti_alias, None);

    let mut pb = PathBuilder::new();
    if tl == 0.0 && tr == 0.0 && br == 0.0 && bl == 0.0 {
        pb.move_to(-hw, -hh);
        pb.line_to(hw, -hh);
        pb.line_to(hw, hh);
        pb.line_to(-hw, hh);
        pb.close();
    } else {
        let k = 0.552_284_8;
        // Top edge
        pb.move_to(-hw + tl, -hh);
        pb.line_to(hw - tr, -hh);
        // Top-right corner
        if tr > 0.0 {
            pb.cubic_to(hw - tr + tr * k, -hh, hw, -hh + tr - tr * k, hw, -hh + tr);
        }
        // Right edge
        pb.line_to(hw, hh - br);
        // Bottom-right corner
        if br > 0.0 {
            pb.cubic_to(hw, hh - br + br * k, hw - br + br * k, hh, hw - br, hh);
        }
        // Bottom edge
        pb.line_to(-hw + bl, hh);
        // Bottom-left corner
        if bl > 0.0 {
            pb.cubic_to(-hw + bl - bl * k, hh, -hw, hh - bl + bl * k, -hw, hh - bl);
        }
        // Left edge
        pb.line_to(-hw, -hh + tl);
        // Top-left corner
        if tl > 0.0 {
            pb.cubic_to(
                -hw,
                -hh + tl - tl * k,
                -hw + tl - tl * k,
                -hh,
                -hw + tl,
                -hh,
            );
        }
        pb.close();
    }

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
