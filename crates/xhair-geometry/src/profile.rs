//! Resolution profiles, monitor bindings, and display scaling modes.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use xhair_model::units::Vec2;

/// Unique Profile ID newtype wrapper.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct ProfileId(pub Uuid);

impl ProfileId {
    /// Generates a new ProfileId.
    pub fn new_v7() -> Self {
        Self(Uuid::now_v7())
    }
}

impl Default for ProfileId {
    fn default() -> Self {
        Self::new_v7()
    }
}

/// Target display mode for non-native resolutions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "snake_case")]
pub enum DisplayMode {
    /// 1:1 pixel mapping with no scaling.
    #[default]
    Native,
    /// Stretched non-uniformly to fill the display aspect ratio (e.g. 4:3 stretched on 16:9).
    Stretched,
    /// Uniform scale with horizontal letterbox black bars.
    Letterbox,
    /// Uniform scale with vertical pillarbox black bars.
    Pillarbox,
}

/// Monitor association rule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MonitorBinding {
    /// Bind to Windows primary monitor.
    #[default]
    Primary,
    /// Bind to a specific physical monitor device hash/id.
    Specific {
        /// Stable monitor identifier string.
        monitor_id: String,
    },
    /// Dynamically follow the monitor hosting the active game window.
    FollowGameWindow,
}

/// DPI scaling computation mode.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DpiMode {
    /// Automatically query Windows Per-Monitor-V2 DPI.
    #[default]
    Auto,
    /// Override with fixed scaling multiplier (e.g. 1.25 for 125%).
    Manual {
        /// Fixed DPI scale factor.
        scale: f32,
    },
}

/// Game render resolution dimensions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Resolution {
    /// Width in pixels (1 to 16384).
    pub width: u32,
    /// Height in pixels (1 to 16384).
    pub height: u32,
}

impl Resolution {
    /// Common 1080p full HD resolution.
    pub const FHD_1080P: Self = Self {
        width: 1920,
        height: 1080,
    };

    /// Common 1440p QHD resolution.
    pub const QHD_1440P: Self = Self {
        width: 2560,
        height: 1440,
    };

    /// Common 4:3 stretched tournament resolution.
    pub const STRETCHED_4_3: Self = Self {
        width: 1280,
        height: 1024,
    };

    /// Computes derived aspect ratio string (e.g. "16:9" or "4:3").
    pub fn aspect_ratio_str(&self) -> String {
        fn gcd(a: u32, b: u32) -> u32 {
            if b == 0 {
                a
            } else {
                gcd(b, a % b)
            }
        }
        let d = gcd(self.width, self.height);
        if let (Some(w), Some(h)) = (self.width.checked_div(d), self.height.checked_div(d)) {
            format!("{w}:{h}")
        } else {
            "Custom".to_string()
        }
    }
}

/// Non-uniform stretch configuration.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct StretchConfig {
    /// Automatically derive scale from (output_dimension / game_dimension).
    pub auto: bool,
    /// Horizontal scale factor.
    pub sx: f32,
    /// Vertical scale factor.
    pub sy: f32,
}

impl Default for StretchConfig {
    fn default() -> Self {
        Self {
            auto: true,
            sx: 1.0,
            sy: 1.0,
        }
    }
}

/// Complete resolution profile definition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ResolutionProfile {
    /// Unique profile identifier.
    pub id: ProfileId,
    /// Profile display name (e.g. "1080p Native" or "1280x1024 Stretched").
    pub name: String,
    /// Game render resolution.
    pub game_resolution: Resolution,
    /// Bound monitor destination.
    pub output: MonitorBinding,
    /// Aspect scaling mode.
    pub display_mode: DisplayMode,
    /// Stretch multiplier settings.
    pub stretch: StretchConfig,
    /// Optional manual center override in physical output pixels.
    pub center_override: Option<Vec2>,
    /// Sub-pixel calibration offset in game pixels (0.25 px granularity).
    pub calibration_offset: Vec2,
    /// DPI scaling behavior.
    pub dpi_mode: DpiMode,
    /// Dynamic game window client rect following.
    pub follow_game_window: bool,
}

impl ResolutionProfile {
    /// Creates a standard 1080p native profile.
    pub fn default_1080p() -> Self {
        Self {
            id: ProfileId::new_v7(),
            name: "1080p Native".to_string(),
            game_resolution: Resolution::FHD_1080P,
            output: MonitorBinding::Primary,
            display_mode: DisplayMode::Native,
            stretch: StretchConfig::default(),
            center_override: None,
            calibration_offset: Vec2::ZERO,
            dpi_mode: DpiMode::Auto,
            follow_game_window: true,
        }
    }
}
