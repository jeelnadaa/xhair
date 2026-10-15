//! Best-effort importer for Valorant in-game crosshair profile codes (§11.2).

use crate::error::FormatError;
use std::collections::HashMap;
use xhair_model::crosshair::Crosshair;
use xhair_model::layer::Layer;
use xhair_model::paint::{Paint, Stroke, StrokeCap, StrokePosition};
use xhair_model::shapes::{DotProps, DotShape, LayerKind, LineProps};
use xhair_model::units::{Degrees, Rgba};

/// Result of importing a Valorant crosshair string.
#[derive(Debug, Clone, PartialEq)]
pub struct ValorantImportResult {
    /// Resulting Crosshair domain document.
    pub crosshair: Crosshair,
    /// Non-fatal parsing and mapping warnings.
    pub warnings: Vec<String>,
}

/// Parses a color index into domain `Rgba`.
fn parse_color(code: u32, custom_hex: Option<&str>) -> Rgba {
    match code {
        0 => Rgba::WHITE,
        1 => Rgba::new(0, 255, 0, 1.0),    // Green
        2 => Rgba::new(127, 255, 0, 1.0),  // Yellow-Green
        3 => Rgba::new(223, 255, 0, 1.0),  // Green-Yellow
        4 => Rgba::new(255, 255, 0, 1.0),  // Yellow
        5 => Rgba::CYAN,                   // Cyan
        6 => Rgba::new(255, 20, 147, 1.0), // Pink
        7 => Rgba::new(255, 0, 0, 1.0),    // Red
        8 => {
            if let Some(hex) = custom_hex {
                parse_hex_color(hex).unwrap_or(Rgba::WHITE)
            } else {
                Rgba::WHITE
            }
        }
        _ => Rgba::WHITE,
    }
}

fn parse_hex_color(hex: &str) -> Option<Rgba> {
    let clean = hex.trim_start_matches('#');
    if clean.len() >= 6 {
        let r = u8::from_str_radix(&clean[0..2], 16).ok()?;
        let g = u8::from_str_radix(&clean[2..4], 16).ok()?;
        let b = u8::from_str_radix(&clean[4..6], 16).ok()?;
        let a = if clean.len() >= 8 {
            u8::from_str_radix(&clean[6..8], 16).ok()? as f32 / 255.0
        } else {
            1.0
        };
        Some(Rgba::new(r, g, b, a))
    } else {
        None
    }
}

/// Parses a semicolon-separated Valorant profile code into a `Crosshair`.
pub fn import_valorant_code(code: &str) -> Result<ValorantImportResult, FormatError> {
    let trimmed = code.trim();
    if trimmed.is_empty() {
        return Err(FormatError::ValorantParseError("Empty code".to_string()));
    }

    let tokens: Vec<&str> = trimmed.split(';').collect();
    if tokens.is_empty() {
        return Err(FormatError::ValorantParseError(
            "Invalid format".to_string(),
        ));
    }

    let mut map: HashMap<&str, &str> = HashMap::new();
    let mut i = 0;

    // First token is profile version (usually 0)
    if tokens.first() == Some(&"0") {
        i += 1;
    }

    while i < tokens.len() {
        let key = tokens[i];
        if i + 1 < tokens.len() && !tokens[i + 1].is_empty() && !is_known_key_alone(tokens[i + 1]) {
            map.insert(key, tokens[i + 1]);
            i += 2;
        } else {
            map.insert(key, "1");
            i += 1;
        }
    }

    let mut warnings = Vec::new();

    // Color resolution
    let color_idx: u32 = map.get("c").and_then(|v| v.parse().ok()).unwrap_or(0);
    let custom_hex = map.get("u").copied();
    let base_color = parse_color(color_idx, custom_hex);

    // Outlines
    let outlines_enabled = map.get("h").map(|v| *v == "1").unwrap_or(true);
    let outline_thickness: f32 = map.get("t").and_then(|v| v.parse().ok()).unwrap_or(1.0);
    let outline_opacity: f32 = map.get("o").and_then(|v| v.parse().ok()).unwrap_or(1.0);

    let outline_stroke = if outlines_enabled && outline_opacity > 0.0 {
        Some(Stroke {
            width: outline_thickness,
            paint: Paint::Solid {
                color: Rgba::new(0, 0, 0, outline_opacity),
            },
            position: StrokePosition::Center,
            dash: Vec::new(),
            join: xhair_model::paint::StrokeJoin::Miter,
            cap: StrokeCap::Butt,
        })
    } else {
        None
    };

    let mut crosshair = Crosshair::new_blank("Valorant Import");

    // Center dot
    let dot_enabled = map.get("d").map(|v| *v == "1").unwrap_or(false);
    if dot_enabled {
        let size: f32 = map.get("z").and_then(|v| v.parse().ok()).unwrap_or(2.0);
        let opacity: f32 = map.get("a").and_then(|v| v.parse().ok()).unwrap_or(1.0);

        let mut dot_layer = Layer::new(
            "Center Dot",
            LayerKind::Dot(DotProps {
                diameter: size,
                shape: DotShape::Square,
            }),
        );
        let mut color = base_color;
        color.a = opacity;
        dot_layer.fill = Paint::Solid { color };
        dot_layer.stroke = outline_stroke.clone();
        crosshair.layers.push(dot_layer);
    }

    // Inner lines
    let inner_enabled = map.get("0b").map(|v| *v == "1").unwrap_or(true);
    let inner_len: f32 = map.get("0l").and_then(|v| v.parse().ok()).unwrap_or(0.0);
    let inner_thick: f32 = map.get("0t").and_then(|v| v.parse().ok()).unwrap_or(0.0);
    let inner_gap: f32 = map
        .get("0g")
        .or_else(|| map.get("0o"))
        .and_then(|v| v.parse().ok())
        .unwrap_or(0.0);
    let inner_opacity: f32 = map.get("0a").and_then(|v| v.parse().ok()).unwrap_or(1.0);

    if inner_enabled && (inner_len > 0.0 || inner_thick > 0.0) {
        let mut inner_layer = Layer::new(
            "Inner Lines",
            LayerKind::Line(LineProps {
                arm_count: 4,
                start_angle: Degrees(0.0),
                angle_step: None,
                length: inner_len,
                thickness: inner_thick,
                gap: inner_gap,
                cap: StrokeCap::Butt,
                taper: 0.0,
            }),
        );
        let mut color = base_color;
        color.a = inner_opacity;
        inner_layer.fill = Paint::Solid { color };
        inner_layer.stroke = outline_stroke.clone();
        crosshair.layers.push(inner_layer);
    }

    // Outer lines
    let outer_enabled = map.get("1b").map(|v| *v == "1").unwrap_or(false);
    let outer_len: f32 = map.get("1l").and_then(|v| v.parse().ok()).unwrap_or(0.0);
    let outer_thick: f32 = map.get("1t").and_then(|v| v.parse().ok()).unwrap_or(0.0);
    let outer_gap: f32 = map
        .get("1g")
        .or_else(|| map.get("1o"))
        .and_then(|v| v.parse().ok())
        .unwrap_or(0.0);
    let outer_opacity: f32 = map.get("1a").and_then(|v| v.parse().ok()).unwrap_or(1.0);

    if outer_enabled && (outer_len > 0.0 || outer_thick > 0.0) {
        let mut outer_layer = Layer::new(
            "Outer Lines",
            LayerKind::Line(LineProps {
                arm_count: 4,
                start_angle: Degrees(0.0),
                angle_step: None,
                length: outer_len,
                thickness: outer_thick,
                gap: outer_gap,
                cap: StrokeCap::Butt,
                taper: 0.0,
            }),
        );
        let mut color = base_color;
        color.a = outer_opacity;
        outer_layer.fill = Paint::Solid { color };
        outer_layer.stroke = outline_stroke;
        crosshair.layers.push(outer_layer);
    }

    // Capture unmappable error multiplier fields as warnings
    if map.contains_key("0m") || map.contains_key("0f") {
        warnings
            .push("Dynamic movement/firing error on inner lines converted to static layer".into());
    }
    if map.contains_key("1m") || map.contains_key("1f") {
        warnings
            .push("Dynamic movement/firing error on outer lines converted to static layer".into());
    }

    Ok(ValorantImportResult {
        crosshair,
        warnings,
    })
}

fn is_known_key_alone(tok: &str) -> bool {
    matches!(tok, "P" | "s" | "c" | "h" | "d" | "0b" | "1b")
}
