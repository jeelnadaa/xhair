//! Vector path rasterization for custom shapes.

use crate::paint::create_skia_paint;
use crate::stroke::create_skia_stroke;
use tiny_skia::{FillRule, Path, PathBuilder, Pixmap, Transform};
use xhair_model::layer::Layer;
use xhair_model::shapes::{FillRule as ModelFillRule, PathProps};

/// Parses a subset of SVG path syntax (M, L, C, Z commands).
pub fn parse_svg_path(data: &str) -> Option<Path> {
    let mut pb = PathBuilder::new();
    let mut current_cmd = 'M';
    let mut current_pt = (0.0f32, 0.0f32);

    let tokens: Vec<&str> = data
        .split(|c: char| c.is_whitespace() || c == ',')
        .filter(|s| !s.is_empty())
        .collect();

    let mut i = 0;
    while i < tokens.len() {
        let tok = tokens[i];
        let first_char = tok.chars().next()?;

        if first_char.is_alphabetic() {
            current_cmd = first_char;
            i += 1;
            if current_cmd == 'Z' || current_cmd == 'z' {
                pb.close();
                continue;
            }
        }

        match current_cmd {
            'M' | 'm' => {
                if i + 1 < tokens.len() {
                    let x: f32 = tokens[i].parse().ok()?;
                    let y: f32 = tokens[i + 1].parse().ok()?;
                    i += 2;
                    let (target_x, target_y) = if current_cmd == 'm' {
                        (current_pt.0 + x, current_pt.1 + y)
                    } else {
                        (x, y)
                    };
                    pb.move_to(target_x, target_y);
                    current_pt = (target_x, target_y);
                    current_cmd = if current_cmd == 'm' { 'l' } else { 'L' };
                } else {
                    break;
                }
            }
            'L' | 'l' => {
                if i + 1 < tokens.len() {
                    let x: f32 = tokens[i].parse().ok()?;
                    let y: f32 = tokens[i + 1].parse().ok()?;
                    i += 2;
                    let (target_x, target_y) = if current_cmd == 'l' {
                        (current_pt.0 + x, current_pt.1 + y)
                    } else {
                        (x, y)
                    };
                    pb.line_to(target_x, target_y);
                    current_pt = (target_x, target_y);
                } else {
                    break;
                }
            }
            'C' | 'c' => {
                if i + 5 < tokens.len() {
                    let x1: f32 = tokens[i].parse().ok()?;
                    let y1: f32 = tokens[i + 1].parse().ok()?;
                    let x2: f32 = tokens[i + 2].parse().ok()?;
                    let y2: f32 = tokens[i + 3].parse().ok()?;
                    let x: f32 = tokens[i + 4].parse().ok()?;
                    let y: f32 = tokens[i + 5].parse().ok()?;
                    i += 6;
                    if current_cmd == 'c' {
                        pb.cubic_to(
                            current_pt.0 + x1,
                            current_pt.1 + y1,
                            current_pt.0 + x2,
                            current_pt.1 + y2,
                            current_pt.0 + x,
                            current_pt.1 + y,
                        );
                        current_pt = (current_pt.0 + x, current_pt.1 + y);
                    } else {
                        pb.cubic_to(x1, y1, x2, y2, x, y);
                        current_pt = (x, y);
                    }
                } else {
                    break;
                }
            }
            'Z' | 'z' => {
                pb.close();
            }
            _ => {
                i += 1;
            }
        }
    }

    pb.finish()
}

/// Rasterizes vector SVG path.
pub fn render_path(
    props: &PathProps,
    layer: &Layer,
    instance_transform: Transform,
    opacity: f32,
    anti_alias: bool,
    pixmap: &mut Pixmap,
) {
    if let Some(path) = parse_svg_path(&props.data) {
        let fill_rule = match props.fill_rule {
            ModelFillRule::NonZero => FillRule::Winding,
            ModelFillRule::EvenOdd => FillRule::EvenOdd,
        };

        let skia_fill = create_skia_paint(&layer.fill, opacity, anti_alias, None);
        pixmap.fill_path(&path, &skia_fill, fill_rule, instance_transform, None);

        if let Some(ref stroke) = layer.stroke {
            let skia_stroke = create_skia_stroke(stroke);
            let stroke_paint = create_skia_paint(&stroke.paint, opacity, anti_alias, None);
            pixmap.stroke_path(&path, &stroke_paint, &skia_stroke, instance_transform, None);
        }
    }
}
