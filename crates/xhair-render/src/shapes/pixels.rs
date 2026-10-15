//! Pixel grid rasterization (up to 256x256 custom bitmap reticles).

use tiny_skia::{Color, Paint, Pixmap, Rect, Transform};
use xhair_model::shapes::PixelsProps;

/// Rasterizes custom pixel grid.
pub fn render_pixels(
    props: &PixelsProps,
    instance_transform: Transform,
    opacity: f32,
    pixmap: &mut Pixmap,
) {
    let w = props.width.min(256);
    let h = props.height.min(256);
    if w == 0 || h == 0 || props.rle_data.is_empty() {
        return;
    }

    let half_w = w as f32 * 0.5;
    let half_h = h as f32 * 0.5;

    // Decode RLE or raw RGBA bytes
    let mut pixels = Vec::with_capacity((w * h) as usize);

    if props.rle_data.len() == (w * h * 4) as usize {
        // Raw RGBA
        for chunk in props.rle_data.chunks_exact(4) {
            pixels.push((chunk[0], chunk[1], chunk[2], chunk[3]));
        }
    } else if props.rle_data.len() % 5 == 0 {
        // RLE [count, r, g, b, a]
        for chunk in props.rle_data.chunks_exact(5) {
            let count = chunk[0];
            let rgba = (chunk[1], chunk[2], chunk[3], chunk[4]);
            for _ in 0..count {
                if pixels.len() < (w * h) as usize {
                    pixels.push(rgba);
                }
            }
        }
    }

    let mut paint = Paint {
        anti_alias: false,
        ..Default::default()
    };

    for y in 0..h {
        for x in 0..w {
            let idx = (y * w + x) as usize;
            if let Some(&(r, g, b, a)) = pixels.get(idx) {
                if a == 0 {
                    continue;
                }
                let alpha = (a as f32 / 255.0) * opacity;
                paint.set_color(Color::from_rgba8(r, g, b, (alpha * 255.0).round() as u8));

                let px = x as f32 - half_w;
                let py = y as f32 - half_h;

                if let Some(rect) = Rect::from_xywh(px, py, 1.0, 1.0) {
                    pixmap.fill_rect(rect, &paint, instance_transform, None);
                }
            }
        }
    }
}
