//! Newtype unit wrappers and primitive geometric types.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Crosshair identifier UUID v7 wrapper.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct CrosshairId(pub Uuid);

impl CrosshairId {
    /// Generates a new unique CrosshairId using UUID v7.
    pub fn new_v7() -> Self {
        Self(Uuid::now_v7())
    }
}

impl Default for CrosshairId {
    fn default() -> Self {
        Self::new_v7()
    }
}

impl std::fmt::Display for CrosshairId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for CrosshairId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Uuid::parse_str(s).map(Self)
    }
}

/// Layer identifier UUID v7 wrapper.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct LayerId(pub Uuid);

impl LayerId {
    /// Generates a new unique LayerId using UUID v7.
    pub fn new_v7() -> Self {
        Self(Uuid::now_v7())
    }
}

impl Default for LayerId {
    fn default() -> Self {
        Self::new_v7()
    }
}

impl std::fmt::Display for LayerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for LayerId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Uuid::parse_str(s).map(Self)
    }
}

/// Length or offset in game render pixels.
#[derive(
    Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, JsonSchema, Default,
)]
pub struct GamePx(pub f32);

impl From<f32> for GamePx {
    fn from(v: f32) -> Self {
        Self(v)
    }
}

impl From<GamePx> for f32 {
    fn from(v: GamePx) -> Self {
        v.0
    }
}

/// Angle in degrees (0.0 to 360.0).
#[derive(
    Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, JsonSchema, Default,
)]
pub struct Degrees(pub f32);

impl From<f32> for Degrees {
    fn from(v: f32) -> Self {
        Self(v)
    }
}

impl From<Degrees> for f32 {
    fn from(v: Degrees) -> Self {
        v.0
    }
}

/// Normalized opacity alpha value (0.0 to 1.0).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, JsonSchema)]
pub struct Alpha(pub f32);

impl Default for Alpha {
    fn default() -> Self {
        Self(1.0)
    }
}

impl From<f32> for Alpha {
    fn from(v: f32) -> Self {
        Self(v.clamp(0.0, 1.0))
    }
}

impl From<Alpha> for f32 {
    fn from(v: Alpha) -> Self {
        v.0
    }
}

/// 2D vector for offsets, positions, and coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema, Default)]
pub struct Vec2 {
    /// X component.
    pub x: f32,
    /// Y component.
    pub y: f32,
}

impl Vec2 {
    /// Creates a new 2D vector.
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Zero vector `(0.0, 0.0)`.
    pub const ZERO: Self = Self::new(0.0, 0.0);

    /// Unit scale vector `(1.0, 1.0)`.
    pub const ONE: Self = Self::new(1.0, 1.0);
}

impl std::ops::Add for Vec2 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl std::ops::Mul<f32> for Vec2 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

/// Color stored in sRGB 8-bit components with a float alpha (0.0 to 1.0).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Rgba {
    /// Red component (0–255).
    pub r: u8,
    /// Green component (0–255).
    pub g: u8,
    /// Blue component (0–255).
    pub b: u8,
    /// Alpha transparency (0.0 = fully transparent, 1.0 = fully opaque).
    pub a: f32,
}

impl Default for Rgba {
    fn default() -> Self {
        Self::WHITE
    }
}

impl Rgba {
    /// Creates a new RGBA color.
    pub fn new(r: u8, g: u8, b: u8, a: f32) -> Self {
        Self {
            r,
            g,
            b,
            a: a.clamp(0.0, 1.0),
        }
    }

    /// Fully opaque white (`#FFFFFF`).
    pub const WHITE: Self = Self {
        r: 255,
        g: 255,
        b: 255,
        a: 1.0,
    };

    /// Fully opaque black (`#000000`).
    pub const BLACK: Self = Self {
        r: 0,
        g: 0,
        b: 0,
        a: 1.0,
    };

    /// Brand accent cyan (`#36F2CB`).
    pub const CYAN: Self = Self {
        r: 54,
        g: 242,
        b: 203,
        a: 1.0,
    };

    /// Transparent black.
    pub const TRANSPARENT: Self = Self {
        r: 0,
        g: 0,
        b: 0,
        a: 0.0,
    };

    /// Returns hexadecimal representation (e.g. `#36F2CB` or `#36F2CBFF`).
    pub fn to_hex(self) -> String {
        if (self.a - 1.0).abs() < f32::EPSILON {
            format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
        } else {
            let alpha_byte = (self.a * 255.0).round() as u8;
            format!(
                "#{:02X}{:02X}{:02X}{:02X}",
                self.r, self.g, self.b, alpha_byte
            )
        }
    }
}

/// Visual layer and effect blending mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "snake_case")]
pub enum BlendMode {
    /// Standard alpha compositing over underlying layer.
    #[default]
    Normal,
    /// Additive blend (common for glowing energy reticles).
    Add,
    /// Screen blend mode.
    Screen,
    /// Multiply blend mode.
    Multiply,
    /// Overlay blend mode.
    Overlay,
}

/// Layer transformation anchor origin point.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "snake_case")]
pub enum Anchor {
    /// Exact center of the layer bounding box.
    #[default]
    Center,
    /// Top-left corner.
    TopLeft,
    /// Top-center edge.
    TopCenter,
    /// Top-right corner.
    TopRight,
    /// Center-left edge.
    CenterLeft,
    /// Center-right edge.
    CenterRight,
    /// Bottom-left corner.
    BottomLeft,
    /// Bottom-center edge.
    BottomCenter,
    /// Bottom-right corner.
    BottomRight,
}
