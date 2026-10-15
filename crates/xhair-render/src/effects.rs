//! Visual effects engine: glow, bloom, drop shadow, and gaussian blur.

use tiny_skia::{BlendMode, Pixmap, PixmapPaint, Transform};
use xhair_model::effects::{BloomEffect, GlowEffect, ShadowEffect};

/// Applies drop shadow underneath a layer pixmap.
pub fn apply_shadow(src: &Pixmap, shadow: &ShadowEffect, dest: &mut Pixmap) {
    let w = src.width();
    let h = src.height();
    if w == 0 || h == 0 || shadow.opacity <= 0.0 {
        return;
    }

    let mut shadow_pixmap = match Pixmap::new(w, h) {
        Some(p) => p,
        None => return,
    };

    // Extract alpha and colorize with shadow color * opacity
    let src_data = src.data();
    let shadow_data = shadow_pixmap.data_mut();

    let sr = (shadow.color.r as f32 / 255.0) * shadow.opacity;
    let sg = (shadow.color.g as f32 / 255.0) * shadow.opacity;
    let sb = (shadow.color.b as f32 / 255.0) * shadow.opacity;

    for i in 0..(w * h) as usize {
        let alpha = src_data[i * 4 + 3] as f32 / 255.0;
        if alpha > 0.0 {
            let final_a = (alpha * shadow.opacity * 255.0).round() as u8;
            let pr = (sr * alpha * 255.0).round() as u8;
            let pg = (sg * alpha * 255.0).round() as u8;
            let pb = (sb * alpha * 255.0).round() as u8;

            shadow_data[i * 4] = pr;
            shadow_data[i * 4 + 1] = pg;
            shadow_data[i * 4 + 2] = pb;
            shadow_data[i * 4 + 3] = final_a;
        }
    }

    // Blur shadow if radius > 0
    if shadow.blur > 0.5 {
        gaussian_blur(&mut shadow_pixmap, shadow.blur.min(32.0), 2);
    }

    // Draw shadow onto destination at offset (dx, dy)
    let paint = PixmapPaint {
        opacity: 1.0,
        blend_mode: BlendMode::SourceOver,
        quality: tiny_skia::FilterQuality::Bilinear,
    };

    dest.draw_pixmap(
        shadow.offset.x.round() as i32,
        shadow.offset.y.round() as i32,
        shadow_pixmap.as_ref(),
        &paint,
        Transform::identity(),
        None,
    );
}

/// Applies outer radiant glow on top of or behind a layer.
pub fn apply_glow(src: &Pixmap, glow: &GlowEffect, dest: &mut Pixmap) {
    let w = src.width();
    let h = src.height();
    if w == 0 || h == 0 || glow.radius <= 0.1 || glow.intensity <= 0.0 {
        return;
    }

    let mut glow_pixmap = match Pixmap::new(w, h) {
        Some(p) => p,
        None => return,
    };

    let src_data = src.data();
    let glow_data = glow_pixmap.data_mut();

    let gr = glow.color.r as f32 / 255.0;
    let gg = glow.color.g as f32 / 255.0;
    let gb = glow.color.b as f32 / 255.0;

    for i in 0..(w * h) as usize {
        let alpha = src_data[i * 4 + 3] as f32 / 255.0;
        if alpha > 0.0 {
            let mult = (alpha * glow.intensity).min(1.0);
            glow_data[i * 4] = (gr * mult * 255.0).round() as u8;
            glow_data[i * 4 + 1] = (gg * mult * 255.0).round() as u8;
            glow_data[i * 4 + 2] = (gb * mult * 255.0).round() as u8;
            glow_data[i * 4 + 3] = (mult * 255.0).round() as u8;
        }
    }

    let passes = glow.quality.clamp(1, 3) as usize;
    gaussian_blur(&mut glow_pixmap, glow.radius.min(64.0), passes);

    let blend_mode = match glow.blend {
        xhair_model::units::BlendMode::Add => BlendMode::Plus,
        xhair_model::units::BlendMode::Screen => BlendMode::Screen,
        _ => BlendMode::SourceOver,
    };

    let paint = PixmapPaint {
        opacity: (glow.intensity * 0.5).min(1.0),
        blend_mode,
        quality: tiny_skia::FilterQuality::Bilinear,
    };

    dest.draw_pixmap(
        0,
        0,
        glow_pixmap.as_ref(),
        &paint,
        Transform::identity(),
        None,
    );
}

/// Separable multi-pass gaussian blur approximation.
pub fn gaussian_blur(pixmap: &mut Pixmap, radius: f32, passes: usize) {
    let r = radius.round() as usize;
    if r == 0 {
        return;
    }

    let w = pixmap.width() as usize;
    let h = pixmap.height() as usize;
    if w == 0 || h == 0 {
        return;
    }

    let mut temp = vec![0u8; w * h * 4];

    for _ in 0..passes {
        // Horizontal pass
        box_blur_h(pixmap.data(), &mut temp, w, h, r);
        // Vertical pass
        box_blur_v(&temp, pixmap.data_mut(), w, h, r);
    }
}

fn box_blur_h(src: &[u8], dst: &mut [u8], w: usize, h: usize, r: usize) {
    let div = (2 * r + 1) as u32;

    for y in 0..h {
        let row_start = y * w * 4;

        let mut sum_r = 0u32;
        let mut sum_g = 0u32;
        let mut sum_b = 0u32;
        let mut sum_a = 0u32;

        let first_idx = row_start;
        let f_r = src[first_idx] as u32;
        let f_g = src[first_idx + 1] as u32;
        let f_b = src[first_idx + 2] as u32;
        let f_a = src[first_idx + 3] as u32;

        for _ in 0..=r {
            sum_r += f_r;
            sum_g += f_g;
            sum_b += f_b;
            sum_a += f_a;
        }

        for x in 1..=r {
            let px = x.min(w - 1);
            let idx = row_start + px * 4;
            sum_r += src[idx] as u32;
            sum_g += src[idx + 1] as u32;
            sum_b += src[idx + 2] as u32;
            sum_a += src[idx + 3] as u32;
        }

        for x in 0..w {
            let out_idx = row_start + x * 4;
            dst[out_idx] = (sum_r / div) as u8;
            dst[out_idx + 1] = (sum_g / div) as u8;
            dst[out_idx + 2] = (sum_b / div) as u8;
            dst[out_idx + 3] = (sum_a / div) as u8;

            let next_x = (x + r + 1).min(w - 1);
            let prev_x = (x as isize - r as isize).max(0) as usize;

            let in_idx = row_start + next_x * 4;
            let out_p_idx = row_start + prev_x * 4;

            sum_r += src[in_idx] as u32;
            sum_g += src[in_idx + 1] as u32;
            sum_b += src[in_idx + 2] as u32;
            sum_a += src[in_idx + 3] as u32;

            sum_r = sum_r.saturating_sub(src[out_p_idx] as u32);
            sum_g = sum_g.saturating_sub(src[out_p_idx + 1] as u32);
            sum_b = sum_b.saturating_sub(src[out_p_idx + 2] as u32);
            sum_a = sum_a.saturating_sub(src[out_p_idx + 3] as u32);
        }
    }
}

fn box_blur_v(src: &[u8], dst: &mut [u8], w: usize, h: usize, r: usize) {
    let div = (2 * r + 1) as u32;

    for x in 0..w {
        let mut sum_r = 0u32;
        let mut sum_g = 0u32;
        let mut sum_b = 0u32;
        let mut sum_a = 0u32;

        let col_start = x * 4;
        let f_r = src[col_start] as u32;
        let f_g = src[col_start + 1] as u32;
        let f_b = src[col_start + 2] as u32;
        let f_a = src[col_start + 3] as u32;

        for _ in 0..=r {
            sum_r += f_r;
            sum_g += f_g;
            sum_b += f_b;
            sum_a += f_a;
        }

        for y in 1..=r {
            let py = y.min(h - 1);
            let idx = (py * w + x) * 4;
            sum_r += src[idx] as u32;
            sum_g += src[idx + 1] as u32;
            sum_b += src[idx + 2] as u32;
            sum_a += src[idx + 3] as u32;
        }

        for y in 0..h {
            let out_idx = (y * w + x) * 4;
            dst[out_idx] = (sum_r / div) as u8;
            dst[out_idx + 1] = (sum_g / div) as u8;
            dst[out_idx + 2] = (sum_b / div) as u8;
            dst[out_idx + 3] = (sum_a / div) as u8;

            let next_y = (y + r + 1).min(h - 1);
            let prev_y = (y as isize - r as isize).max(0) as usize;

            let in_idx = (next_y * w + x) * 4;
            let out_p_idx = (prev_y * w + x) * 4;

            sum_r += src[in_idx] as u32;
            sum_g += src[in_idx + 1] as u32;
            sum_b += src[in_idx + 2] as u32;
            sum_a += src[in_idx + 3] as u32;

            sum_r = sum_r.saturating_sub(src[out_p_idx] as u32);
            sum_g = sum_g.saturating_sub(src[out_p_idx + 1] as u32);
            sum_b = sum_b.saturating_sub(src[out_p_idx + 2] as u32);
            sum_a = sum_a.saturating_sub(src[out_p_idx + 3] as u32);
        }
    }
}

/// Applies global bloom to a finished composite pixmap (§4.6).
pub fn apply_bloom(pixmap: &mut Pixmap, bloom: &BloomEffect) {
    let w = pixmap.width();
    let h = pixmap.height();
    if w == 0 || h == 0 || bloom.intensity <= 0.0 || bloom.radius <= 0.1 {
        return;
    }

    let mut bloom_pixmap = match Pixmap::new(w, h) {
        Some(p) => p,
        None => return,
    };

    let src_data = pixmap.data();
    let bloom_data = bloom_pixmap.data_mut();
    let threshold_luma = bloom.threshold.clamp(0.0, 1.0);

    for i in 0..(w * h) as usize {
        let a = src_data[i * 4 + 3] as f32 / 255.0;
        if a > 0.0 {
            let r = src_data[i * 4] as f32 / 255.0;
            let g = src_data[i * 4 + 1] as f32 / 255.0;
            let b = src_data[i * 4 + 2] as f32 / 255.0;

            // Perceptual relative luminance (ITU-R BT.709)
            let luma = 0.2126 * r + 0.7152 * g + 0.0722 * b;
            if luma > threshold_luma {
                let range = (1.0 - threshold_luma).max(0.001);
                let over = (luma - threshold_luma) / range;
                let factor = (over * bloom.intensity).min(2.0);

                bloom_data[i * 4] = (r * factor * 255.0).min(255.0) as u8;
                bloom_data[i * 4 + 1] = (g * factor * 255.0).min(255.0) as u8;
                bloom_data[i * 4 + 2] = (b * factor * 255.0).min(255.0) as u8;
                bloom_data[i * 4 + 3] = (a * factor * 255.0).min(255.0) as u8;
            }
        }
    }

    let radius = bloom.radius.min(64.0);
    gaussian_blur(&mut bloom_pixmap, radius, 3);

    let paint = PixmapPaint {
        opacity: 1.0,
        blend_mode: BlendMode::Plus,
        quality: tiny_skia::FilterQuality::Bilinear,
    };

    pixmap.draw_pixmap(
        0,
        0,
        bloom_pixmap.as_ref(),
        &paint,
        Transform::identity(),
        None,
    );
}
