//! Stroke configuration and styling conversion for tiny-skia.

use tiny_skia::{LineCap, LineJoin, Stroke as SkiaStroke, StrokeDash};
use xhair_model::paint::{Stroke, StrokeCap, StrokeJoin};

/// Converts domain `StrokeCap` to `tiny_skia::LineCap`.
#[inline]
pub fn to_skia_line_cap(cap: StrokeCap) -> LineCap {
    match cap {
        StrokeCap::Butt => LineCap::Butt,
        StrokeCap::Round => LineCap::Round,
        StrokeCap::Square => LineCap::Square,
    }
}

/// Converts domain `StrokeJoin` to `tiny_skia::LineJoin`.
#[inline]
pub fn to_skia_line_join(join: StrokeJoin) -> LineJoin {
    match join {
        StrokeJoin::Miter => LineJoin::Miter,
        StrokeJoin::Round => LineJoin::Round,
        StrokeJoin::Bevel => LineJoin::Bevel,
    }
}

/// Converts domain `Stroke` to `tiny_skia::Stroke`.
pub fn create_skia_stroke(stroke: &Stroke) -> SkiaStroke {
    let mut s = SkiaStroke {
        width: stroke.width.max(0.0),
        miter_limit: 4.0,
        line_cap: to_skia_line_cap(stroke.cap),
        line_join: to_skia_line_join(stroke.join),
        dash: None,
    };

    if !stroke.dash.is_empty() && stroke.dash.len() % 2 == 0 {
        s.dash = StrokeDash::new(stroke.dash.clone(), 0.0);
    }

    s
}
