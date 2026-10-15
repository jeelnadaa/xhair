//! Center dot rasterization (round and square center reticles).

use crate::paint::create_skia_paint;
use crate::stroke::create_skia_stroke;
use tiny_skia::{PathBuilder, Pixmap, Transform};
use xhair_model::layer::Layer;
use xhair_model::shapes::{DotProps, DotShape};

/// Rasterizes center dot.
pub fn render_dot(
    props: &DotProps,
    layer: &Layer,
    instance_transform: Transform,
    opacity: f32,
    anti_alias: bool,
    pixmap: &mut Pixmap,
) {
    let d = props.diameter.max(0.25);
    let r = d * 0.5;

    let skia_fill = create_skia_paint(&layer.fill, opacity, anti_alias, None);

    match props.shape {
        DotShape::Square => {
            let mut pb = PathBuilder::new();
            pb.move_to(-r, -r);
            pb.line_to(r, -r);
            pb.line_to(r, r);
            pb.line_to(-r, r);
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
                    pixmap.stroke_path(
                        &path,
                        &stroke_paint,
                        &skia_stroke,
                        instance_transform,
                        None,
                    );
                }
            }
        }
        DotShape::Round => {
            // Circle using 4 cubic bezier splines with magic constant k = 0.5522847498
            let k = r * 0.552_284_8;
            let mut pb = PathBuilder::new();
            pb.move_to(0.0, -r);
            pb.cubic_to(k, -r, r, -k, r, 0.0);
            pb.cubic_to(r, k, k, r, 0.0, r);
            pb.cubic_to(-k, r, -r, k, -r, 0.0);
            pb.cubic_to(-r, -k, -k, -r, 0.0, -r);
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
                    pixmap.stroke_path(
                        &path,
                        &stroke_paint,
                        &skia_stroke,
                        instance_transform,
                        None,
                    );
                }
            }
        }
    }
}
