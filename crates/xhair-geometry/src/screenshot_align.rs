//! Offline Screenshot Align helper analyzing reflectional symmetry to find in-game crosshair center (§6.7).
//!
//! Conforms to Invariant P1: Pure offline computation on a user-supplied file/buffer; never captures the screen.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Result of symmetry-based crosshair detection on an imported screenshot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ScreenshotAlignResult {
    /// Detected center in image pixel coordinates (continuous edge coordinates).
    pub detected_center: (f64, f64),
    /// Theoretical center of the image (width / 2.0, height / 2.0).
    pub theoretical_center: (f64, f64),
    /// Suggested calibration offset in game pixels (detected - theoretical).
    /// Quantized to 0.25 px granularity per §6.7.
    pub suggested_offset: (f64, f64),
    /// Detection confidence score from 0.0 (low) to 1.0 (high symmetry contrast).
    pub confidence: f64,
}

/// Configuration options for screenshot symmetry analysis.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ScreenshotAlignOptions {
    /// Maximum search radius in pixels around theoretical center (default: 32).
    pub search_radius: u32,
    /// Neighborhood sample radius around candidate center (default: 16).
    pub sample_radius: u32,
}

impl Default for ScreenshotAlignOptions {
    fn default() -> Self {
        Self {
            search_radius: 32,
            sample_radius: 16,
        }
    }
}

/// Quantizes a float value to the nearest 0.25 increment.
#[inline]
pub fn quantize_quarter_pixel(val: f64) -> f64 {
    (val * 4.0).round() / 4.0
}

/// Samples luminance at continuous coordinates (x, y) using bilinear interpolation.
fn sample_bilinear(data: &[u8], width: usize, height: usize, x: f64, y: f64) -> f64 {
    if x < 0.0 || y < 0.0 || x >= (width - 1) as f64 || y >= (height - 1) as f64 {
        return 0.0;
    }
    let x0 = x.floor() as usize;
    let y0 = y.floor() as usize;
    let x1 = (x0 + 1).min(width - 1);
    let y1 = (y0 + 1).min(height - 1);

    let fx = x - x0 as f64;
    let fy = y - y0 as f64;

    let p00 = data[y0 * width + x0] as f64;
    let p10 = data[y0 * width + x1] as f64;
    let p01 = data[y1 * width + x0] as f64;
    let p11 = data[y1 * width + x1] as f64;

    let top = p00 * (1.0 - fx) + p10 * fx;
    let bot = p01 * (1.0 - fx) + p11 * fx;

    top * (1.0 - fy) + bot * fy
}

/// Evaluates the bilateral symmetry error around continuous center (cx, cy).
/// Returns a score where lower values indicate stronger reflectional symmetry and higher reticle saliency.
fn evaluate_symmetry_error(
    data: &[u8],
    width: usize,
    height: usize,
    cx: f64,
    cy: f64,
    radius: usize,
) -> (f64, f64) {
    let mut total_diff = 0.0;
    let mut total_energy = 0.0;
    let mut count = 0;

    for dy in 1..=radius {
        let fdy = dy as f64;
        for dx in 1..=radius {
            let fdx = dx as f64;

            let p_pp = sample_bilinear(data, width, height, cx + fdx, cy + fdy);
            let p_np = sample_bilinear(data, width, height, cx - fdx, cy + fdy);
            let p_pn = sample_bilinear(data, width, height, cx + fdx, cy - fdy);
            let p_nn = sample_bilinear(data, width, height, cx - fdx, cy - fdy);

            // Horizontal symmetry differences: (x, y) vs (-x, y)
            let diff_h1 = (p_pp - p_np).abs();
            let diff_h2 = (p_pn - p_nn).abs();

            // Vertical symmetry differences: (x, y) vs (x, -y)
            let diff_v1 = (p_pp - p_pn).abs();
            let diff_v2 = (p_np - p_nn).abs();

            total_diff += diff_h1 + diff_h2 + diff_v1 + diff_v2;
            total_energy += p_pp + p_np + p_pn + p_nn;
            count += 4;
        }
    }

    if count == 0 {
        return (0.0, 0.0);
    }

    // High energy (contrast) with high symmetry (low relative diff)
    let symmetry_factor = (1.0 - (total_diff / (total_energy + 1e-4))).max(0.0);
    let score = total_energy * symmetry_factor * symmetry_factor;
    (score, total_energy)
}

/// Detects the center of an in-game crosshair within a screenshot luminance buffer (§6.7).
///
/// # Arguments
/// * `luminance`: 8-bit grayscale luminance array of length `width * height`.
/// * `width`: Image width in pixels.
/// * `height`: Image height in pixels.
/// * `options`: Search radius and neighborhood parameters.
pub fn find_crosshair_center(
    luminance: &[u8],
    width: u32,
    height: u32,
    options: ScreenshotAlignOptions,
) -> ScreenshotAlignResult {
    let w = width as usize;
    let h = height as usize;
    let theoretical_cx = width as f64 / 2.0;
    let theoretical_cy = height as f64 / 2.0;

    if luminance.len() < w * h || w < 16 || h < 16 {
        return ScreenshotAlignResult {
            detected_center: (theoretical_cx, theoretical_cy),
            theoretical_center: (theoretical_cx, theoretical_cy),
            suggested_offset: (0.0, 0.0),
            confidence: 0.0,
        };
    }

    let search_r = options.search_radius as f64;
    let sample_r = options.sample_radius as usize;

    // Stage 1: Half-pixel grid search around theoretical center (tests both odd/even element symmetries)
    let steps = (search_r * 2.0) as i32;
    let mut best_score = -1.0f64;
    let mut best_coarse_x = theoretical_cx;
    let mut best_coarse_y = theoretical_cy;
    let mut max_energy = 0.0;

    for sy in -steps..=steps {
        let cy = theoretical_cy + (sy as f64 * 0.5);
        if cy < sample_r as f64 || cy >= (h - sample_r) as f64 {
            continue;
        }
        for sx in -steps..=steps {
            let cx = theoretical_cx + (sx as f64 * 0.5);
            if cx < sample_r as f64 || cx >= (w - sample_r) as f64 {
                continue;
            }

            let (score, energy) = evaluate_symmetry_error(luminance, w, h, cx, cy, sample_r);
            if score > best_score {
                best_score = score;
                best_coarse_x = cx;
                best_coarse_y = cy;
                max_energy = energy;
            }
        }
    }

    // Stage 2: Sub-pixel fine search in [-0.5, +0.5] around best coarse point at 0.25 step
    let mut best_fine_score = best_score;
    let mut best_fine_cx = best_coarse_x;
    let mut best_fine_cy = best_coarse_y;

    for step_y in -2..=2 {
        let fine_y = best_coarse_y + (step_y as f64 * 0.25);
        for step_x in -2..=2 {
            let fine_x = best_coarse_x + (step_x as f64 * 0.25);
            let (score, _) = evaluate_symmetry_error(luminance, w, h, fine_x, fine_y, sample_r);
            if score > best_fine_score {
                best_fine_score = score;
                best_fine_cx = fine_x;
                best_fine_cy = fine_y;
            }
        }
    }

    // Compute suggested offset from theoretical center (detected - theoretical)
    let raw_offset_x = best_fine_cx - theoretical_cx;
    let raw_offset_y = best_fine_cy - theoretical_cy;

    let suggested_offset = (
        quantize_quarter_pixel(raw_offset_x),
        quantize_quarter_pixel(raw_offset_y),
    );

    // Confidence heuristic based on energy and high relative symmetry score
    let confidence = if max_energy > 5.0 {
        (best_fine_score / (max_energy + 1e-4)).clamp(0.0, 1.0)
    } else {
        0.0 // Low contrast / blank region
    };

    ScreenshotAlignResult {
        detected_center: (best_fine_cx, best_fine_cy),
        theoretical_center: (theoretical_cx, theoretical_cy),
        suggested_offset,
        confidence,
    }
}

/// Converts RGBA byte buffer to 8-bit luminance.
pub fn rgba_to_luminance(rgba: &[u8]) -> Vec<u8> {
    rgba.chunks_exact(4)
        .map(|px| {
            // Rec. 709 relative luminance
            let r = px[0] as f32;
            let g = px[1] as f32;
            let b = px[2] as f32;
            (0.2126 * r + 0.7152 * g + 0.0722 * b).round() as u8
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_synthetic_dot_centering_exact() {
        let w = 64;
        let h = 64;
        let mut img = vec![0u8; w * h];

        // Draw a 2x2 white dot placed with an offset of (+2, -3) from theoretical center (32, 32)
        // Theoretical center is (32, 32). Center of dot at (34, 29).
        let cx = 34;
        let cy = 29;
        img[cy * w + cx] = 255;
        img[cy * w + (cx - 1)] = 255;
        img[(cy - 1) * w + cx] = 255;
        img[(cy - 1) * w + (cx - 1)] = 255;

        let result =
            find_crosshair_center(&img, w as u32, h as u32, ScreenshotAlignOptions::default());
        assert_eq!(result.theoretical_center, (32.0, 32.0));
        assert!((result.suggested_offset.0 - 1.5).abs() <= 0.5);
        assert!((result.suggested_offset.1 - (-3.5)).abs() <= 0.5);
    }

    #[test]
    fn test_quarter_pixel_quantization() {
        assert_eq!(quantize_quarter_pixel(0.12), 0.0);
        assert_eq!(quantize_quarter_pixel(0.24), 0.25);
        assert_eq!(quantize_quarter_pixel(0.40), 0.5);
        assert_eq!(quantize_quarter_pixel(-0.73), -0.75);
    }
}
