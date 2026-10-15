//! Physical output pixel mapping, stretch scaling, and sub-pixel translation math.

use crate::profile::{DisplayMode, ResolutionProfile};

/// Calculated output geometry parameters.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeometryMapping {
    /// Horizontal scale factor from game px to physical px.
    pub sx: f64,
    /// Vertical scale factor from game px to physical px.
    pub sy: f64,
    /// Exact center position on physical display.
    pub out_center_x: f64,
    /// Exact center position on physical display.
    pub out_center_y: f64,
}

impl GeometryMapping {
    /// Computes physical mapping given a resolution profile and target output display rectangle.
    pub fn compute(
        profile: &ResolutionProfile,
        out_origin: (f64, f64),
        out_size: (f64, f64),
    ) -> Self {
        let (out_x, out_y) = out_origin;
        let (out_w, out_h) = out_size;
        let gw = profile.game_resolution.width as f64;
        let gh = profile.game_resolution.height as f64;

        let (sx, sy) = match profile.display_mode {
            DisplayMode::Native => (1.0, 1.0),
            DisplayMode::Stretched => {
                if profile.stretch.auto {
                    (out_w / gw, out_h / gh)
                } else {
                    (profile.stretch.sx as f64, profile.stretch.sy as f64)
                }
            }
            DisplayMode::Letterbox | DisplayMode::Pillarbox => {
                let scale = (out_w / gw).min(out_h / gh);
                (scale, scale)
            }
        };

        let cal_x = profile.calibration_offset.x as f64;
        let cal_y = profile.calibration_offset.y as f64;

        let out_center_x = if let Some(override_center) = profile.center_override {
            out_x + override_center.x as f64
        } else {
            out_x + (out_w / 2.0) + cal_x * sx
        };

        let out_center_y = if let Some(override_center) = profile.center_override {
            out_y + override_center.y as f64
        } else {
            out_y + (out_h / 2.0) + cal_y * sy
        };

        Self {
            sx,
            sy,
            out_center_x,
            out_center_y,
        }
    }

    /// Transforms a point from game pixel space (relative to crosshair center) into physical screen pixel coordinates.
    pub fn game_to_physical(&self, gx: f64, gy: f64) -> (f64, f64) {
        (
            self.out_center_x + gx * self.sx,
            self.out_center_y + gy * self.sy,
        )
    }

    /// Inverts a physical display pixel coordinate back into game pixel space relative to center.
    pub fn physical_to_game(&self, px: f64, py: f64) -> (f64, f64) {
        (
            (px - self.out_center_x) / self.sx,
            (py - self.out_center_y) / self.sy,
        )
    }
}

/// Converts device independent pixels (DIPs) to physical pixels given Windows monitor DPI.
/// Standard base 100% scale corresponds to 96 DPI.
pub fn dips_to_physical_px(dips: f64, dpi: u32) -> f64 {
    dips * (dpi as f64 / 96.0)
}

/// Converts physical pixels to device independent pixels (DIPs).
pub fn physical_px_to_dips(physical: f64, dpi: u32) -> f64 {
    physical * (96.0 / dpi as f64)
}
