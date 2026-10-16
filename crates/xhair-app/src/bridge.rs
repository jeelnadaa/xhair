//! Bridge for converting UI JSON representation into domain Crosshair documents.

use serde_json::Value;
use xhair_model::crosshair::Crosshair;
use xhair_model::effects::{GlowEffect, ShadowEffect};
use xhair_model::layer::{ArmOverride, Layer};
use xhair_model::paint::{Paint, StrokeCap};
use xhair_model::shapes::*;
use xhair_model::units::{Alpha, Degrees, Rgba, Vec2};

/// Parses a hex color string (e.g. "#36F2CB" or "#36F2CBFF") into Rgba.
pub fn parse_hex_color(hex: &str) -> Rgba {
    let clean = hex.trim_start_matches('#');
    let (r, g, b, a) = match clean.len() {
        6 => {
            let r = u8::from_str_radix(&clean[0..2], 16).unwrap_or(54);
            let g = u8::from_str_radix(&clean[2..4], 16).unwrap_or(242);
            let b = u8::from_str_radix(&clean[4..6], 16).unwrap_or(203);
            (r, g, b, 255)
        }
        8 => {
            let r = u8::from_str_radix(&clean[0..2], 16).unwrap_or(54);
            let g = u8::from_str_radix(&clean[2..4], 16).unwrap_or(242);
            let b = u8::from_str_radix(&clean[4..6], 16).unwrap_or(203);
            let a = u8::from_str_radix(&clean[6..8], 16).unwrap_or(255);
            (r, g, b, a)
        }
        3 => {
            let r = u8::from_str_radix(&clean[0..1], 16).unwrap_or(3) * 17;
            let g = u8::from_str_radix(&clean[1..2], 16).unwrap_or(15) * 17;
            let b = u8::from_str_radix(&clean[2..3], 16).unwrap_or(12) * 17;
            (r, g, b, 255)
        }
        _ => (54, 242, 203, 255),
    };
    Rgba::new(r, g, b, (a as f32) / 255.0)
}

/// Parses either canonical .xhair JSON or editor UI crosshair JSON into a Crosshair domain object.
pub fn parse_ui_crosshair(json_str: &str) -> Result<Crosshair, String> {
    // 1. Try canonical Crosshair deserialization first
    if let Ok(ch) = serde_json::from_str::<Crosshair>(json_str) {
        return Ok(ch);
    }

    // 2. Parse as dynamic Value for UI crosshair format
    let val: Value = serde_json::from_str(json_str).map_err(|e| e.to_string())?;

    let name = val["name"].as_str().unwrap_or("Active Crosshair");
    let mut ch = Crosshair::new_blank(name);

    if let Some(desc) = val["description"].as_str() {
        ch.description = desc.to_string();
    }
    if let Some(author) = val["author"].as_str() {
        ch.author = author.to_string();
    }

    let ox = val["offsetX"].as_f64().unwrap_or(0.0) as f32;
    let oy = val["offsetY"].as_f64().unwrap_or(0.0) as f32;
    ch.global.offset = Vec2::new(ox, oy);

    let scale = val["scale"].as_f64().unwrap_or(1.0) as f32;
    ch.global.scale = scale.max(0.01);

    let opacity = val["opacity"].as_f64().unwrap_or(1.0) as f32;
    ch.global.opacity = Alpha::from(opacity);

    if let Some(layers) = val["layers"].as_array() {
        for l_val in layers {
            let l_name = l_val["name"].as_str().unwrap_or("Layer");
            let visible = l_val["visible"].as_bool().unwrap_or(true);
            let locked = l_val["locked"].as_bool().unwrap_or(false);

            let color_str = l_val["color"].as_str().unwrap_or("#36F2CB");
            let color = parse_hex_color(color_str);

            let l_opacity = l_val["opacity"].as_f64().unwrap_or(1.0) as f32;
            let l_ox = l_val["offsetX"].as_f64().unwrap_or(0.0) as f32;
            let l_oy = l_val["offsetY"].as_f64().unwrap_or(0.0) as f32;
            let l_rot = l_val["rotation"].as_f64().unwrap_or(0.0) as f32;

            let shape_val = &l_val["shape"];
            let shape_type = shape_val["type"].as_str().unwrap_or("LineArms");

            let mut arm_overrides = Vec::new();

            let kind = match shape_type {
                "LineArms" => {
                    let arm_count = shape_val["armCount"].as_u64().unwrap_or(4) as u32;
                    let length = shape_val["length"].as_f64().unwrap_or(6.0) as f32;
                    let thickness = shape_val["thickness"].as_f64().unwrap_or(2.0) as f32;
                    let gap = shape_val["gap"].as_f64().unwrap_or(4.0) as f32;

                    if let Some(offsets) = shape_val["armOffsets"].as_array() {
                        for (idx, off) in offsets.iter().enumerate() {
                            let ax = off["x"].as_f64().unwrap_or(0.0) as f32;
                            let ay = off["y"].as_f64().unwrap_or(0.0) as f32;
                            if ax != 0.0 || ay != 0.0 {
                                arm_overrides.push(ArmOverride {
                                    arm_index: idx as u32,
                                    offset: Some(Vec2::new(ax, ay)),
                                    length_delta: None,
                                    thickness: None,
                                    color: None,
                                    visible: None,
                                });
                            }
                        }
                    }

                    LayerKind::Line(LineProps {
                        arm_count,
                        length,
                        thickness,
                        gap,
                        ..Default::default()
                    })
                }
                "Dot" => {
                    let diameter = shape_val["diameter"].as_f64().unwrap_or(2.0) as f32;
                    let is_square = shape_val["isSquare"].as_bool().unwrap_or(true);
                    LayerKind::Dot(DotProps {
                        diameter,
                        shape: if is_square {
                            DotShape::Square
                        } else {
                            DotShape::Round
                        },
                    })
                }
                "Plus" | "Cross" => {
                    let length = shape_val["length"].as_f64().unwrap_or(6.0) as f32;
                    let thickness = shape_val["thickness"].as_f64().unwrap_or(2.0) as f32;
                    let gap = shape_val["gap"].as_f64().unwrap_or(2.0) as f32;
                    let center_hole = shape_val["centerHole"].as_f64().unwrap_or(0.0) as f32;
                    LayerKind::Plus(PlusProps {
                        length,
                        thickness,
                        gap,
                        center_hole,
                    })
                }
                "Circle" => {
                    let radius = shape_val["radius"].as_f64().unwrap_or(8.0) as f32;
                    LayerKind::Circle(CircleProps {
                        diameter: radius * 2.0,
                    })
                }
                "Ring" => {
                    let radius = shape_val["radius"].as_f64().unwrap_or(8.0) as f32;
                    let thickness = shape_val["thickness"].as_f64().unwrap_or(2.0) as f32;
                    LayerKind::Ring(RingProps {
                        outer_radius: radius,
                        thickness,
                        dash: Vec::new(),
                    })
                }
                "Arc" => {
                    let radius = shape_val["radius"].as_f64().unwrap_or(8.0) as f32;
                    let thickness = shape_val["thickness"].as_f64().unwrap_or(2.0) as f32;
                    let start = shape_val["startAngle"].as_f64().unwrap_or(0.0) as f32;
                    let end = shape_val["endAngle"].as_f64().unwrap_or(180.0) as f32;
                    LayerKind::Arc(ArcProps {
                        radius,
                        thickness,
                        start_angle: Degrees(start),
                        end_angle: Degrees(end),
                        cap: StrokeCap::Butt,
                    })
                }
                "Rect" => {
                    let width = shape_val["width"].as_f64().unwrap_or(8.0) as f32;
                    let height = shape_val["height"].as_f64().unwrap_or(8.0) as f32;
                    let corner_radius = shape_val["cornerRadius"].as_f64().unwrap_or(0.0) as f32;
                    LayerKind::Rect(RectProps {
                        width,
                        height,
                        corner_radii: [corner_radius; 4],
                    })
                }
                "Triangle" => {
                    let base = shape_val["base"].as_f64().unwrap_or(8.0) as f32;
                    let height = shape_val["height"].as_f64().unwrap_or(8.0) as f32;
                    LayerKind::Triangle(TriangleProps { base, height })
                }
                "Polygon" => {
                    let sides = shape_val["points"].as_u64().unwrap_or(5) as u32;
                    let radius = shape_val["radius"].as_f64().unwrap_or(8.0) as f32;
                    LayerKind::Polygon(PolygonProps {
                        sides,
                        radius,
                        star_inner_ratio: 0.0,
                    })
                }
                "Star" => {
                    let sides = shape_val["points"].as_u64().unwrap_or(5) as u32;
                    let radius = shape_val["radius"].as_f64().unwrap_or(8.0) as f32;
                    let inner_r = shape_val["innerRadius"].as_f64().unwrap_or(4.0) as f32;
                    let ratio = if radius > 0.0 { inner_r / radius } else { 0.5 };
                    LayerKind::Polygon(PolygonProps {
                        sides,
                        radius,
                        star_inner_ratio: ratio,
                    })
                }
                "Chevron" => {
                    let arm_length = shape_val["length"].as_f64().unwrap_or(8.0) as f32;
                    let thickness = shape_val["thickness"].as_f64().unwrap_or(2.0) as f32;
                    let gap = shape_val["gap"].as_f64().unwrap_or(2.0) as f32;
                    let angle = shape_val["angle"].as_f64().unwrap_or(90.0) as f32;
                    LayerKind::Chevron(ChevronProps {
                        arm_length,
                        thickness,
                        gap,
                        angle: Degrees(angle),
                    })
                }
                "TShape" => {
                    let bar_length = shape_val["length"].as_f64().unwrap_or(10.0) as f32;
                    let stem_length = shape_val["height"].as_f64().unwrap_or(6.0) as f32;
                    let thickness = shape_val["thickness"].as_f64().unwrap_or(2.0) as f32;
                    let gap = shape_val["gap"].as_f64().unwrap_or(2.0) as f32;
                    LayerKind::TShape(TShapeProps {
                        bar_length,
                        stem_length,
                        thickness,
                        gap,
                    })
                }
                "SvgPath" => {
                    let data = shape_val["pathData"].as_str().unwrap_or("").to_string();
                    LayerKind::Path(PathProps {
                        data,
                        fill_rule: FillRule::NonZero,
                    })
                }
                _ => LayerKind::Line(LineProps::default()),
            };

            let mut layer = Layer::new(l_name, kind);
            layer.visible = visible;
            layer.locked = locked;
            layer.fill = Paint::Solid { color };
            layer.opacity = Alpha::from(l_opacity);
            layer.transform.offset = Vec2::new(l_ox, l_oy);
            layer.transform.rotation = Degrees(l_rot);
            layer.arm_overrides = arm_overrides;

            if let Some(glow_r) = l_val["glowRadius"].as_f64() {
                if glow_r > 0.0 {
                    let glow_color = l_val["glowColor"]
                        .as_str()
                        .map(parse_hex_color)
                        .unwrap_or(color);
                    layer.effects.glow = Some(GlowEffect {
                        color: glow_color,
                        radius: glow_r as f32,
                        intensity: 1.0,
                        spread: 0.0,
                        blend: xhair_model::units::BlendMode::Add,
                        quality: 2,
                    });
                }
            }

            if let Some(s_blur) = l_val["shadowBlur"].as_f64() {
                let s_ox = l_val["shadowOffsetX"].as_f64().unwrap_or(1.0) as f32;
                let s_oy = l_val["shadowOffsetY"].as_f64().unwrap_or(1.0) as f32;
                if s_blur > 0.0 || s_ox != 0.0 || s_oy != 0.0 {
                    let s_col = l_val["shadowColor"]
                        .as_str()
                        .map(parse_hex_color)
                        .unwrap_or(Rgba::BLACK);
                    layer.effects.shadow = Some(ShadowEffect {
                        color: s_col,
                        offset: Vec2::new(s_ox, s_oy),
                        blur: s_blur as f32,
                        opacity: 0.75,
                    });
                }
            }

            ch.layers.push(layer);
        }
    }

    Ok(ch)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ui_crosshair_cyan_gap() {
        let json = "{\"id\":\"ch-cyan-gap-001\",\"name\":\"Cyan Gap\",\"offsetX\":0,\"offsetY\":0,\"scale\":1.0,\"opacity\":1.0,\"layers\":[{\"id\":\"layer-arms-1\",\"name\":\"Arms x4\",\"visible\":true,\"locked\":false,\"shape\":{\"type\":\"LineArms\",\"length\":6,\"thickness\":2,\"gap\":4,\"armCount\":4},\"color\":\"#36F2CB\",\"opacity\":1.0,\"offsetX\":0,\"offsetY\":0,\"rotation\":0},{\"id\":\"layer-dot-2\",\"name\":\"Center dot\",\"visible\":true,\"locked\":false,\"shape\":{\"type\":\"Dot\",\"diameter\":2,\"isSquare\":true},\"color\":\"#36F2CB\",\"opacity\":1.0,\"offsetX\":0,\"offsetY\":0,\"rotation\":0}]}";

        let ch = parse_ui_crosshair(json).expect("Failed to parse UI crosshair");
        assert_eq!(ch.name, "Cyan Gap");
        assert_eq!(ch.layers.len(), 2);
        assert_eq!(ch.layers[0].name, "Arms x4");
        assert_eq!(ch.layers[1].name, "Center dot");
        if let LayerKind::Line(props) = &ch.layers[0].kind {
            assert_eq!(props.arm_count, 4);
            assert_eq!(props.length, 6.0);
            assert_eq!(props.gap, 4.0);
        } else {
            panic!("Expected Line kind");
        }
    }
}
