//! T-shaped reticle rasterization (top horizontal bar + vertical bottom stem).

use crate::paint::create_skia_paint;
use crate::stroke::create_skia_stroke;
use tiny_skia::{PathBuilder, Pixmap, Transform};
use xhair_model::layer::Layer;
use xhair_model::shapes::TShapeProps;

/// Rasterizes T-shaped reticle.
pub fn render_t_shape(
    props: &TShapeProps,
    layer: &Layer,
    instance_transform: Transform,
    opacity: f32,
    anti_alias: bool,
    pixmap: &mut Pixmap,
) {
    let half_bar = props.bar_length.max(0.1) * 0.5;
    let stem_len = props.stem_length.max(0.1);
    let t = props.thickness.max(0.1);
    let half_t = t * 0.5;
    let gap = props.gap.max(0.0);

    let skia_fill = create_skia_paint(&layer.fill, opacity, anti_alias, None);

    let mut pb = PathBuilder::new();

    // Top horizontal bar (split with center gap or solid bar above center)
    let bar_y = -(gap + half_t);
    pb.move_to(-half_bar, bar_y - half_t);
    pb.line_to(half_bar, bar_y - half_t);
    pb.line_to(half_bar, bar_y + half_t);
    pb.line_to(-half_bar, bar_y + half_t);
    pb.close();

    // Vertical bottom stem
    let stem_top = gap;
    let stem_bottom = gap + stem_len;
    pb.move_to(-half_t, stem_top);
    pb.line_to(half_t, stem_top);
    pb.line_to(half_t, stem_bottom);
    pb.line_to(-half_t, stem_bottom);
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
