//! Preview and analysis tools: render.preview, analyze.alignment, analyze.visibility, validate.

use crate::context::ServerContext;
use crate::protocol::CallToolResult;
use base64::Engine;
use serde_json::Value;
use std::time::Instant;
use tiny_skia::Pixmap;
use xhair_geometry::alignment::alignment_report;
use xhair_model::crosshair::{AaMode, Crosshair};
use xhair_model::paint::Paint;
use xhair_model::units::{Rgba, Vec2};
use xhair_render::context::{RenderContext, RenderQuality};
use xhair_render::renderer::render;

/// Renders a crosshair and returns PNG image content (§13.3).
pub fn render_preview(ctx: &ServerContext, args: Value) -> CallToolResult {
    let cid = match ctx.resolve_crosshair_id(&args) {
        Ok(id) => id,
        Err(e) => return CallToolResult::error(e),
    };

    let crosshair = match ctx.with_state(|s| s.bus.documents.get(&cid).cloned()) {
        Some(c) => c,
        None => return CallToolResult::error(format!("Crosshair not found: {}", cid)),
    };

    let size = args.get("size").and_then(|v| v.as_u64()).unwrap_or(256) as u32;
    let scale = args.get("scale").and_then(|v| v.as_f64()).unwrap_or(1.0) as f32;

    let mut pixmap = match Pixmap::new(size, size) {
        Some(p) => p,
        None => return CallToolResult::error("Failed to allocate render pixmap"),
    };

    // Fill subtle dark background if requested
    if args
        .get("dark_background")
        .and_then(|v| v.as_bool())
        .unwrap_or(true)
    {
        pixmap.fill(tiny_skia::Color::from_rgba8(14, 19, 24, 255));
    }

    let render_ctx = RenderContext {
        scale: Vec2::new(scale, scale),
        center_frac: Vec2::ZERO,
        aa: AaMode::Auto,
        time: 0.0,
        dynamics: None,
        quality: RenderQuality::Standard,
    };

    let start = Instant::now();
    let stats = match render(&crosshair, &render_ctx, &mut pixmap) {
        Ok(s) => s,
        Err(e) => return CallToolResult::error(format!("Render error: {}", e)),
    };
    let elapsed_us = start.elapsed().as_micros();

    let png_bytes = match pixmap.encode_png() {
        Ok(b) => b,
        Err(e) => return CallToolResult::error(format!("PNG encode error: {}", e)),
    };

    let base64_data = base64::engine::general_purpose::STANDARD.encode(&png_bytes);

    let mut result = CallToolResult::image(base64_data, "image/png");
    result.content.push(crate::protocol::ToolContent::Text {
        text: format!(
            "Rendered in {} µs ({} layers, bounds: {:?})",
            elapsed_us, stats.rendered_layers, stats.bounds
        ),
    });

    result
}

/// Analyzes center alignment and parity report (§6.2).
pub fn analyze_alignment_tool(ctx: &ServerContext, args: Value) -> CallToolResult {
    let cid = match ctx.resolve_crosshair_id(&args) {
        Ok(id) => id,
        Err(e) => return CallToolResult::error(e),
    };

    let crosshair = match ctx.with_state(|s| s.bus.documents.get(&cid).cloned()) {
        Some(c) => c,
        None => return CallToolResult::error(format!("Crosshair not found: {}", cid)),
    };

    let width = args.get("width").and_then(|v| v.as_u64()).unwrap_or(1920) as u32;
    let height = args.get("height").and_then(|v| v.as_u64()).unwrap_or(1080) as u32;

    let issues = alignment_report(&crosshair, width, height);

    CallToolResult::json(&serde_json::json!({
        "width": width,
        "height": height,
        "center_mode": if width % 2 == 0 { "pixel-edge (even W)" } else { "pixel-middle (odd W)" },
        "issues_count": issues.len(),
        "issues": issues
    }))
}

/// Computes WCAG-style contrast and visibility ratio.
pub fn analyze_visibility(ctx: &ServerContext, args: Value) -> CallToolResult {
    let cid = match ctx.resolve_crosshair_id(&args) {
        Ok(id) => id,
        Err(e) => return CallToolResult::error(e),
    };

    let crosshair = match ctx.with_state(|s| s.bus.documents.get(&cid).cloned()) {
        Some(c) => c,
        None => return CallToolResult::error(format!("Crosshair not found: {}", cid)),
    };

    // Calculate primary layer color vs standard gaming backdrops
    let primary_color = crosshair
        .layers
        .first()
        .and_then(|l| match &l.fill {
            Paint::Solid { color } => Some(*color),
            _ => None,
        })
        .unwrap_or(Rgba::new(54, 242, 203, 1.0));

    // Primary luminance
    let l1 = 0.2126 * (primary_color.r as f32 / 255.0)
        + 0.7152 * (primary_color.g as f32 / 255.0)
        + 0.0722 * (primary_color.b as f32 / 255.0);

    let dark_bg_lum = 0.005_f32; // #0B0D10
    let light_bg_lum = 0.85_f32; // Light foliage / bright sky

    let contrast_dark = (l1 + 0.05) / (dark_bg_lum + 0.05);
    let contrast_light = (light_bg_lum + 0.05) / (l1 + 0.05);

    CallToolResult::json(&serde_json::json!({
        "primary_color": format!("#{:02X}{:02X}{:02X}", primary_color.r, primary_color.g, primary_color.b),
        "contrast_dark_backdrop": ((contrast_dark * 10.0_f32).round() / 10.0_f32) as f64,
        "contrast_light_backdrop": ((contrast_light * 10.0_f32).round() / 10.0_f32) as f64,
        "wcag_rating": if contrast_dark >= 7.0 { "AAA" } else if contrast_dark >= 4.5 { "AA" } else { "Fail" }
    }))
}

/// Validates crosshair structure against spec constraints.
pub fn validate(ctx: &ServerContext, args: Value) -> CallToolResult {
    let crosshair_val = match args.get("crosshair") {
        Some(v) => v.clone(),
        None => {
            let cid = match ctx.resolve_crosshair_id(&args) {
                Ok(id) => id,
                Err(e) => return CallToolResult::error(e),
            };
            match ctx.with_state(|s| s.bus.documents.get(&cid).cloned()) {
                Some(c) => serde_json::to_value(c).unwrap(),
                None => return CallToolResult::error(format!("Crosshair not found: {}", cid)),
            }
        }
    };

    match serde_json::from_value::<Crosshair>(crosshair_val) {
        Ok(ch) => match ch.validate() {
            Ok(_) => CallToolResult::json(&serde_json::json!({
                "valid": true,
                "layer_count": ch.layers.len()
            })),
            Err(e) => CallToolResult::json(&serde_json::json!({
                "valid": false,
                "error": e.to_string()
            })),
        },
        Err(e) => CallToolResult::error(format!("Deserialization error: {}", e)),
    }
}
