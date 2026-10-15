//! Circle, Ring, and Arc shape rasterization.

use crate::paint::create_skia_paint;
use crate::stroke::create_skia_stroke;
use tiny_skia::{
    LineCap, LineJoin, Path, PathBuilder, Pixmap, Stroke as SkiaStroke, StrokeDash, Transform,
};
use xhair_model::layer::Layer;
use xhair_model::shapes::{ArcProps, CircleProps, RingProps};

fn make_circle_path(r: f32) -> Option<Path> {
    let k = r * 0.552_284_8;
    let mut pb = PathBuilder::new();
    pb.move_to(0.0, -r);
    pb.cubic_to(k, -r, r, -k, r, 0.0);
    pb.cubic_to(r, k, k, r, 0.0, r);
    pb.cubic_to(-k, r, -r, k, -r, 0.0);
    pb.cubic_to(-r, -k, -k, -r, 0.0, -r);
    pb.close();
    pb.finish()
}

/// Rasterizes filled circle.
pub fn render_circle(
    props: &CircleProps,
    layer: &Layer,
    instance_transform: Transform,
    opacity: f32,
    anti_alias: bool,
    pixmap: &mut Pixmap,
) {
    let r = props.diameter.max(0.2) * 0.5;
    if let Some(path) = make_circle_path(r) {
        let skia_fill = create_skia_paint(&layer.fill, opacity, anti_alias, None);
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

/// Rasterizes hollow ring / donut.
pub fn render_ring(
    props: &RingProps,
    layer: &Layer,
    instance_transform: Transform,
    opacity: f32,
    anti_alias: bool,
    pixmap: &mut Pixmap,
) {
    let outer_r = props.outer_radius.max(0.5);
    let thickness = props.thickness.max(0.1);
    let center_r = outer_r - thickness * 0.5;

    if center_r > 0.0 {
        if let Some(path) = make_circle_path(center_r) {
            let mut skia_stroke = SkiaStroke {
                width: thickness,
                miter_limit: 4.0,
                line_cap: LineCap::Butt,
                line_join: LineJoin::Round,
                dash: None,
            };
            if !props.dash.is_empty() && props.dash.len() % 2 == 0 {
                skia_stroke.dash = StrokeDash::new(props.dash.clone(), 0.0);
            }

            let stroke_paint = create_skia_paint(&layer.fill, opacity, anti_alias, None);
            pixmap.stroke_path(&path, &stroke_paint, &skia_stroke, instance_transform, None);
        }
    }
}

/// Rasterizes circular arc segment.
pub fn render_arc(
    props: &ArcProps,
    layer: &Layer,
    instance_transform: Transform,
    opacity: f32,
    anti_alias: bool,
    pixmap: &mut Pixmap,
) {
    let r = props.radius.max(0.5);
    let start_deg = props.start_angle.0;
    let end_deg = props.end_angle.0;
    let sweep = (end_deg - start_deg).abs();

    if sweep < 0.1 {
        return;
    }

    // Tessellate arc with small angular steps (e.g. 5 degrees per segment)
    let segments = ((sweep / 5.0).ceil() as usize).max(4);
    let step = (end_deg - start_deg) / segments as f32;

    let mut pb = PathBuilder::new();
    let start_rad = start_deg.to_radians();
    pb.move_to(start_rad.cos() * r, start_rad.sin() * r);

    for s in 1..=segments {
        let angle = (start_deg + (s as f32 * step)).to_radians();
        pb.line_to(angle.cos() * r, angle.sin() * r);
    }

    if let Some(path) = pb.finish() {
        let line_cap = match props.cap {
            xhair_model::paint::StrokeCap::Butt => LineCap::Butt,
            xhair_model::paint::StrokeCap::Round => LineCap::Round,
            xhair_model::paint::StrokeCap::Square => LineCap::Square,
        };

        let skia_stroke = SkiaStroke {
            width: props.thickness.max(0.1),
            miter_limit: 4.0,
            line_cap,
            line_join: LineJoin::Round,
            dash: None,
        };

        let stroke_paint = create_skia_paint(&layer.fill, opacity, anti_alias, None);
        pixmap.stroke_path(&path, &stroke_paint, &skia_stroke, instance_transform, None);
    }
}
