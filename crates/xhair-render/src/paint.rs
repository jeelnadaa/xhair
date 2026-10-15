//! Paint, gradient, and shader conversion for tiny-skia.

use crate::color::to_skia_color;
use tiny_skia::{
    Color, GradientStop, LinearGradient, Paint as SkiaPaint, Point, RadialGradient, SpreadMode,
    Transform,
};
use xhair_model::paint::Paint;

/// Converts a domain `Paint` to a configured `tiny_skia::Paint`.
pub fn create_skia_paint<'a>(
    paint: &Paint,
    opacity: f32,
    anti_alias: bool,
    bounds: Option<[f32; 4]>,
) -> SkiaPaint<'a> {
    let mut skia_paint = SkiaPaint {
        anti_alias,
        ..Default::default()
    };

    match paint {
        Paint::Solid { color } => {
            skia_paint.set_color(to_skia_color(*color, opacity));
        }
        Paint::Linear(lin) => {
            let stops: Vec<GradientStop> = lin
                .stops
                .iter()
                .map(|s| GradientStop::new(s.position, to_skia_color(s.color, opacity)))
                .collect();

            let (start, end) = if let Some([min_x, min_y, max_x, max_y]) = bounds {
                let cx = (min_x + max_x) * 0.5;
                let cy = (min_y + max_y) * 0.5;
                let w = (max_x - min_x).max(1.0);
                let h = (max_y - min_y).max(1.0);
                let r = (w * w + h * h).sqrt() * 0.5;

                let rad = lin.angle.0.to_radians();
                let dx = rad.cos() * r;
                let dy = rad.sin() * r;

                (
                    Point::from_xy(cx - dx, cy - dy),
                    Point::from_xy(cx + dx, cy + dy),
                )
            } else {
                let rad = lin.angle.0.to_radians();
                let dx = rad.cos() * 16.0;
                let dy = rad.sin() * 16.0;
                (Point::from_xy(-dx, -dy), Point::from_xy(dx, dy))
            };

            if let Some(shader) =
                LinearGradient::new(start, end, stops, SpreadMode::Pad, Transform::identity())
            {
                skia_paint.shader = shader;
            } else if let Some(first) = lin.stops.first() {
                skia_paint.set_color(to_skia_color(first.color, opacity));
            }
        }
        Paint::Radial(rad) => {
            let stops: Vec<GradientStop> = rad
                .stops
                .iter()
                .map(|s| GradientStop::new(s.position, to_skia_color(s.color, opacity)))
                .collect();

            let center = Point::from_xy(rad.center.x, rad.center.y);
            let radius = rad.radius.max(0.1);

            if let Some(shader) = RadialGradient::new(
                center,
                center,
                radius,
                stops,
                SpreadMode::Pad,
                Transform::identity(),
            ) {
                skia_paint.shader = shader;
            } else if let Some(first) = rad.stops.first() {
                skia_paint.set_color(to_skia_color(first.color, opacity));
            }
        }
        Paint::Conic(con) => {
            // Conic fallback to first stop color or approximated solid
            if let Some(first) = con.stops.first() {
                skia_paint.set_color(to_skia_color(first.color, opacity));
            } else {
                skia_paint.set_color(Color::from_rgba8(255, 255, 255, 255));
            }
        }
    }

    skia_paint
}
