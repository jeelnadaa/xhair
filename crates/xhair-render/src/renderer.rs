//! Central deterministic rendering orchestrator.

use crate::bounds::{calculate_crosshair_bounds, BoundingBox};
use crate::context::{RenderContext, RenderStats};
use crate::effects::{apply_bloom, apply_glow, apply_shadow};
use crate::error::RenderError;
use crate::shapes::render_shape;
use crate::symmetry::generate_symmetry_instances;
use std::time::Instant;
use tiny_skia::{BlendMode, Pixmap, PixmapPaint, Transform};
use xhair_model::crosshair::{AaMode, Crosshair};
use xhair_model::units::Vec2;

/// Maximum allowed overlay bounding box dimension in pixels.
pub const MAX_OVERLAY_DIMENSION: u32 = 2048;

/// Maximum allowed blur scratch buffer dimension in pixels.
pub const MAX_BLUR_BUFFER_DIMENSION: u32 = 1024;

/// Computes conservative bounding box of the crosshair.
pub fn bounds(crosshair: &Crosshair, ctx: &RenderContext) -> BoundingBox {
    let evaluated_doc;
    let ch = if !ctx.static_only && ctx.time > 0.0 && !crosshair.animation.is_empty() {
        evaluated_doc = crosshair.evaluate_animation(ctx.time);
        &evaluated_doc
    } else {
        crosshair
    };

    let mut b = calculate_crosshair_bounds(ch);
    let sx = ctx.scale.x.abs().max(0.01);
    let sy = ctx.scale.y.abs().max(0.01);
    b.min_x = b.min_x * sx + ctx.center_frac.x;
    b.max_x = b.max_x * sx + ctx.center_frac.x;
    b.min_y = b.min_y * sy + ctx.center_frac.y;
    b.max_y = b.max_y * sy + ctx.center_frac.y;
    b
}

/// Renders a crosshair into an allocated pixmap buffer.
pub fn render(
    crosshair: &Crosshair,
    ctx: &RenderContext,
    out: &mut Pixmap,
) -> Result<RenderStats, RenderError> {
    let start_time = Instant::now();
    let w = out.width();
    let h = out.height();

    if w > MAX_OVERLAY_DIMENSION || h > MAX_OVERLAY_DIMENSION {
        return Err(RenderError::DimensionTooLarge {
            width: w,
            height: h,
            max: MAX_OVERLAY_DIMENSION,
        });
    }

    let evaluated_doc;
    let ch = if !ctx.static_only && ctx.time > 0.0 && !crosshair.animation.is_empty() {
        evaluated_doc = crosshair.evaluate_animation(ctx.time);
        &evaluated_doc
    } else {
        crosshair
    };

    let anti_alias = match ctx.aa {
        AaMode::Crisp => false,
        AaMode::Smooth => true,
        AaMode::Auto => !(ctx.scale.x.fract() == 0.0 && ctx.scale.y.fract() == 0.0),
    };

    let mut center_x = (w as f32 * 0.5) + ctx.center_frac.x;
    let mut center_y = (h as f32 * 0.5) + ctx.center_frac.y;

    if ctx.aa == AaMode::Crisp {
        center_x = center_x.round();
        center_y = center_y.round();
    }

    let global_tx = center_x + ch.global.offset.x * ctx.scale.x;
    let global_ty = center_y + ch.global.offset.y * ctx.scale.y;
    let global_sx = ch.global.scale * ctx.scale.x;
    let global_sy = ch.global.scale * ctx.scale.y;
    let global_rot = ch.global.rotation.0;
    let global_opacity = ch.global.opacity.0;

    let root_transform = Transform::from_translate(global_tx, global_ty)
        .post_concat(Transform::from_rotate(global_rot))
        .post_concat(Transform::from_scale(global_sx, global_sy));

    let solo_active = ch.layers.iter().any(|l| l.solo);
    let mut rendered_layer_count = 0;

    for layer in &ch.layers {
        if !layer.visible {
            continue;
        }
        if solo_active && !layer.solo {
            continue;
        }

        let layer_opacity = (layer.opacity.0 * global_opacity).clamp(0.0, 1.0);
        if layer_opacity <= 0.0 {
            continue;
        }

        let has_effects = layer.effects.glow.is_some() || layer.effects.shadow.is_some();
        let instances = generate_symmetry_instances(layer, &ch.symmetry, Vec2::ZERO);

        if has_effects {
            // Render layer into an isolated scratch buffer
            let mut layer_pixmap = Pixmap::new(w, h).ok_or(RenderError::AllocationFailed {
                width: w,
                height: h,
            })?;

            for inst in instances {
                let combined_transform = inst.transform.post_concat(root_transform);
                render_shape(
                    layer,
                    inst.arm_override,
                    combined_transform,
                    layer_opacity,
                    anti_alias,
                    &mut layer_pixmap,
                );
            }

            // Apply shadow underneath
            if let Some(ref shadow) = layer.effects.shadow {
                apply_shadow(&layer_pixmap, shadow, out);
            }

            // Draw layer
            let paint = PixmapPaint {
                opacity: 1.0,
                blend_mode: BlendMode::SourceOver,
                quality: tiny_skia::FilterQuality::Bilinear,
            };
            out.draw_pixmap(
                0,
                0,
                layer_pixmap.as_ref(),
                &paint,
                Transform::identity(),
                None,
            );

            // Apply glow
            if let Some(ref glow) = layer.effects.glow {
                apply_glow(&layer_pixmap, glow, out);
            }
        } else {
            // Render directly into destination
            for inst in instances {
                let combined_transform = inst.transform.post_concat(root_transform);
                render_shape(
                    layer,
                    inst.arm_override,
                    combined_transform,
                    layer_opacity,
                    anti_alias,
                    out,
                );
            }
        }

        rendered_layer_count += 1;
    }

    // Apply global glow if present
    if let Some(ref global_glow) = ch.global_effects.glow {
        let snapshot = out.clone();
        apply_glow(&snapshot, global_glow, out);
    }

    // Apply global bloom if present (§4.6)
    if let Some(ref bloom) = ch.global_effects.bloom {
        apply_bloom(out, bloom);
    }

    let elapsed_us = start_time.elapsed().as_micros() as u64;
    let b = bounds(ch, ctx);

    Ok(RenderStats {
        rendered_layers: rendered_layer_count,
        raster_time_us: elapsed_us,
        bounds: [b.min_x, b.min_y, b.max_x, b.max_y],
    })
}

/// Allocates a new pixmap and renders the crosshair.
pub fn render_to_new_pixmap(
    crosshair: &Crosshair,
    ctx: &RenderContext,
    width: u32,
    height: u32,
) -> Result<Pixmap, RenderError> {
    let mut pixmap =
        Pixmap::new(width, height).ok_or(RenderError::AllocationFailed { width, height })?;
    render(crosshair, ctx, &mut pixmap)?;
    Ok(pixmap)
}

/// Renders a crosshair and encodes the result directly to PNG bytes.
pub fn render_png(
    crosshair: &Crosshair,
    ctx: &RenderContext,
    width: u32,
    height: u32,
) -> Result<Vec<u8>, RenderError> {
    let pixmap = render_to_new_pixmap(crosshair, ctx, width, height)?;
    pixmap
        .encode_png()
        .map_err(|e| RenderError::PngEncoding(e.to_string()))
}
