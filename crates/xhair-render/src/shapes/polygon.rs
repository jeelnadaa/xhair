//! Polygon, Star, Triangle, and Chevron shape rasterization.

use crate::paint::create_skia_paint;
use crate::stroke::create_skia_stroke;
use tiny_skia::{PathBuilder, Pixmap, Transform};
use xhair_model::layer::Layer;
use xhair_model::shapes::{ChevronProps, PolygonProps, TriangleProps};

/// Rasterizes regular polygon or star shape.
pub fn render_polygon(
    props: &PolygonProps,
    layer: &Layer,
    instance_transform: Transform,
    opacity: f32,
    anti_alias: bool,
    pixmap: &mut Pixmap,
) {
    let sides = props.sides.clamp(3, 64);
    let outer_r = props.radius.max(0.2);
    let inner_ratio = props.star_inner_ratio.clamp(0.0, 0.95);

    let mut pb = PathBuilder::new();

    if inner_ratio > 0.0 {
        // Star polygon
        let total_points = sides * 2;
        let step = std::f32::consts::TAU / total_points as f32;
        let inner_r = outer_r * inner_ratio;

        for p in 0..total_points {
            let r = if p % 2 == 0 { outer_r } else { inner_r };
            let angle = (p as f32 * step) - std::f32::consts::FRAC_PI_2;
            let x = angle.cos() * r;
            let y = angle.sin() * r;

            if p == 0 {
                pb.move_to(x, y);
            } else {
                pb.line_to(x, y);
            }
        }
        pb.close();
    } else {
        // Regular polygon
        let step = std::f32::consts::TAU / sides as f32;
        for i in 0..sides {
            let angle = (i as f32 * step) - std::f32::consts::FRAC_PI_2;
            let x = angle.cos() * outer_r;
            let y = angle.sin() * outer_r;

            if i == 0 {
                pb.move_to(x, y);
            } else {
                pb.line_to(x, y);
            }
        }
        pb.close();
    }

    if let Some(path) = pb.finish() {
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

/// Rasterizes isosceles triangle shape.
pub fn render_triangle(
    props: &TriangleProps,
    layer: &Layer,
    instance_transform: Transform,
    opacity: f32,
    anti_alias: bool,
    pixmap: &mut Pixmap,
) {
    let half_base = props.base.max(0.1) * 0.5;
    let height = props.height.max(0.1);

    let mut pb = PathBuilder::new();
    pb.move_to(0.0, -height * 0.5);
    pb.line_to(half_base, height * 0.5);
    pb.line_to(-half_base, height * 0.5);
    pb.close();

    if let Some(path) = pb.finish() {
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

/// Rasterizes chevron / arrowhead reticle.
pub fn render_chevron(
    props: &ChevronProps,
    layer: &Layer,
    instance_transform: Transform,
    opacity: f32,
    anti_alias: bool,
    pixmap: &mut Pixmap,
) {
    let arm_len = props.arm_length.max(0.1);
    let half_angle = (props.angle.0.clamp(10.0, 170.0) * 0.5).to_radians();
    let gap = props.gap.max(0.0);

    let apex_y = -gap;
    let dx = half_angle.sin() * arm_len;
    let dy = half_angle.cos() * arm_len;

    let mut pb = PathBuilder::new();
    pb.move_to(-dx, apex_y + dy);
    pb.line_to(0.0, apex_y);
    pb.line_to(dx, apex_y + dy);

    if let Some(path) = pb.finish() {
        let skia_stroke = tiny_skia::Stroke {
            width: props.thickness.max(0.1),
            miter_limit: 4.0,
            line_cap: tiny_skia::LineCap::Butt,
            line_join: tiny_skia::LineJoin::Miter,
            dash: None,
        };

        let stroke_paint = create_skia_paint(&layer.fill, opacity, anti_alias, None);
        pixmap.stroke_path(&path, &stroke_paint, &skia_stroke, instance_transform, None);
    }
}
