//! Rendering context, execution options, and performance statistics.

use xhair_model::crosshair::AaMode;
use xhair_model::units::Vec2;

/// Rendering quality preset controlling blur and supersampling passes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RenderQuality {
    /// Faster rasterization for high-framerate dynamic animations.
    Draft,
    /// Balanced standard quality with full anti-aliasing.
    #[default]
    Standard,
    /// High-precision quality with 3-pass gaussian blur.
    High,
}

/// Dynamic reticle state input passed into the renderer.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct DynamicState {
    /// Spread multiplier (e.g. 1.0 = base, 2.5 = full spray).
    pub spread_multiplier: f32,
    /// Movement displacement offset.
    pub movement_offset: Vec2,
}

/// Rendering context parameters supplied to pure rasterizer.
#[derive(Debug, Clone, PartialEq)]
pub struct RenderContext {
    /// Non-uniform display scaling factor (output_px / game_px).
    pub scale: Vec2,
    /// Sub-pixel fractional center offset in output pixels (0.0 to 1.0).
    pub center_frac: Vec2,
    /// Anti-aliasing quality mode.
    pub aa: AaMode,
    /// Elapsed animation time in seconds.
    pub time: f32,
    /// Dynamic reticle simulation state.
    pub dynamics: Option<DynamicState>,
    /// Quality preset.
    pub quality: RenderQuality,
    /// Global lock disabling animation and dynamic recoil (§4.7).
    pub static_only: bool,
}

impl Default for RenderContext {
    fn default() -> Self {
        Self {
            scale: Vec2::ONE,
            center_frac: Vec2::ZERO,
            aa: AaMode::Smooth,
            time: 0.0,
            dynamics: None,
            quality: RenderQuality::Standard,
            static_only: false,
        }
    }
}

impl RenderContext {
    /// Creates a 1:1 pixel-exact standard render context.
    pub fn standard() -> Self {
        Self::default()
    }

    /// Sets non-uniform scale factors.
    pub fn with_scale(mut self, sx: f32, sy: f32) -> Self {
        self.scale = Vec2::new(sx, sy);
        self
    }

    /// Sets fractional sub-pixel center offset.
    pub fn with_center_frac(mut self, fx: f32, fy: f32) -> Self {
        self.center_frac = Vec2::new(fx, fy);
        self
    }

    /// Sets anti-aliasing mode.
    pub fn with_aa(mut self, aa: AaMode) -> Self {
        self.aa = aa;
        self
    }

    /// Sets animation timestamp.
    pub fn with_time(mut self, time: f32) -> Self {
        self.time = time;
        self
    }

    /// Sets static-only lock flag.
    pub fn with_static_only(mut self, static_only: bool) -> Self {
        self.static_only = static_only;
        self
    }
}

/// Performance telemetry and statistics collected during rasterization.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct RenderStats {
    /// Number of visual layers rasterized.
    pub rendered_layers: usize,
    /// Duration of rasterization pass in microseconds.
    pub raster_time_us: u64,
    /// Axis-aligned bounding box of drawn geometry `[min_x, min_y, max_x, max_y]`.
    pub bounds: [f32; 4],
}
