//! Complete Crosshair document structure, symmetry, render hints, and validation.

use crate::effects::Effects;
use crate::error::XhairError;
use crate::layer::Layer;
use crate::shapes::LayerKind;
use crate::units::{Alpha, BlendMode, CrosshairId, Degrees, Vec2};
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Format version identifier for .xhair files.
pub const CURRENT_FORMAT_VERSION: u32 = 1;

/// Maximum allowable characters in crosshair name.
pub const MAX_NAME_LENGTH: usize = 256;

/// Maximum allowable characters in crosshair description.
pub const MAX_DESCRIPTION_LENGTH: usize = 4096;

/// Maximum allowable layer count.
pub const MAX_LAYERS: usize = 256;

/// Maximum group nesting depth.
pub const MAX_NESTING_DEPTH: usize = 8;

/// Maximum allowable total assets size in bytes (4 MB).
pub const MAX_ASSETS_BYTES: usize = 4 * 1024 * 1024;

/// Maximum allowable path command points.
pub const MAX_PATH_POINTS: usize = 10_000;

/// Anti-aliasing rasterization mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "snake_case")]
pub enum AaMode {
    /// Snap axis-aligned geometry to integer pixels for maximum sharpness.
    Crisp,
    /// Full sub-pixel anti-aliasing for fractional nudges and rotations.
    #[default]
    Smooth,
    /// Crisp when scale is integer and geometry is axis-aligned; otherwise Smooth.
    Auto,
}

/// Hints supplied to the rasterizer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderHints {
    /// Anti-aliasing quality mode.
    pub aa_mode: AaMode,
    /// Deterministically snap coordinates to output pixel boundaries.
    pub snap_to_pixel: bool,
}

impl Default for RenderHints {
    fn default() -> Self {
        Self {
            aa_mode: AaMode::Smooth,
            snap_to_pixel: false,
        }
    }
}

/// Global symmetry rule applied to crosshair layers.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Symmetry {
    /// No global symmetry.
    #[default]
    None,
    /// Horizontal bilateral mirror.
    MirrorH,
    /// Vertical bilateral mirror.
    MirrorV,
    /// Four-way horizontal and vertical mirror.
    MirrorHV,
    /// N-fold radial symmetry around center (2 to 32 arms).
    Radial {
        /// Number of symmetric radial repetitions.
        arms: u32,
    },
}

/// Global composite transform applied to all layers.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct GlobalTransform {
    /// Translation offset in game pixels.
    pub offset: Vec2,
    /// Global uniform scaling factor.
    pub scale: f32,
    /// Global rotation angle in degrees.
    pub rotation: Degrees,
    /// Master opacity factor.
    pub opacity: Alpha,
    /// Global blend mode.
    pub blend: BlendMode,
}

impl Default for GlobalTransform {
    fn default() -> Self {
        Self {
            offset: Vec2::ZERO,
            scale: 1.0,
            rotation: Degrees(0.0),
            opacity: Alpha::default(),
            blend: BlendMode::Normal,
        }
    }
}

/// Keyframe in an animation track.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Keyframe {
    /// Normalized time position (0.0 to 1.0).
    pub time: f32,
    /// Value at this keyframe.
    pub value: f32,
}

/// Procedural animation track.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct AnimTrack {
    /// Dot-separated target property path.
    pub target_path: String,
    /// Cycle duration in seconds.
    pub duration_seconds: f32,
    /// Keyframe sequence.
    pub keyframes: Vec<Keyframe>,
}

/// Embedded content-addressed binary asset (PNG or SVG).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Asset {
    /// SHA-256 hash digest of the asset content.
    pub hash: String,
    /// MIME type (e.g. "image/png" or "image/svg+xml").
    pub mime: String,
    /// Base64-encoded raw binary data.
    pub data_base64: String,
}

/// Dynamic reticle configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Default)]
pub struct DynamicsConfig {
    /// Associated gun profile name.
    pub gun_profile: Option<String>,
    /// Master multiplier for spread response.
    pub spread_scale: f32,
}

/// Complete Crosshair definition document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Crosshair {
    /// Document format version.
    pub format_version: u32,
    /// Unique Crosshair UUID v7.
    pub id: CrosshairId,
    /// User-facing crosshair name.
    pub name: String,
    /// Optional description or notes.
    pub description: String,
    /// Author name or creator attribution.
    pub author: String,
    /// Categorization tags (e.g. "rifle", "dot", "favorite").
    pub tags: Vec<String>,
    /// Quick-access favorite flag.
    pub favorite: bool,
    /// Creation timestamp (UTC).
    pub created_at: DateTime<Utc>,
    /// Last modification timestamp (UTC).
    pub updated_at: DateTime<Utc>,
    /// Global master transform.
    pub global: GlobalTransform,
    /// Symmetry engine mode.
    pub symmetry: Symmetry,
    /// Ordered list of layers (bottom to top).
    pub layers: Vec<Layer>,
    /// Composite visual effects applied to the entire reticle.
    pub global_effects: Effects,
    /// Procedural animation tracks.
    pub animation: Vec<AnimTrack>,
    /// Optional Dynamic Reticle simulation binding.
    pub dynamics: Option<DynamicsConfig>,
    /// Visual rendering hints.
    pub render_hints: RenderHints,
    /// Embedded content-addressed assets table.
    pub assets: HashMap<String, Asset>,
}

impl Crosshair {
    /// Creates a new empty crosshair document.
    pub fn new_blank(name: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            format_version: CURRENT_FORMAT_VERSION,
            id: CrosshairId::new_v7(),
            name: name.into(),
            description: String::new(),
            author: String::new(),
            tags: Vec::new(),
            favorite: false,
            created_at: now,
            updated_at: now,
            global: GlobalTransform::default(),
            symmetry: Symmetry::None,
            layers: Vec::new(),
            global_effects: Effects::default(),
            animation: Vec::new(),
            dynamics: None,
            render_hints: RenderHints::default(),
            assets: HashMap::new(),
        }
    }

    /// Creates a classic 4-arm crosshair template.
    pub fn new_classic(name: impl Into<String>) -> Self {
        let mut xhair = Self::new_blank(name);
        xhair.layers.push(Layer::new(
            "Arms",
            LayerKind::Line(crate::shapes::LineProps {
                arm_count: 4,
                length: 6.0,
                thickness: 2.0,
                gap: 4.0,
                ..Default::default()
            }),
        ));
        xhair
    }

    /// Validates crosshair structure against architectural limits.
    pub fn validate(&self) -> Result<(), XhairError> {
        if self.name.is_empty() {
            return Err(XhairError::Validation(
                "Crosshair name cannot be empty".to_string(),
            ));
        }

        if self.name.len() > MAX_NAME_LENGTH {
            return Err(XhairError::Validation(format!(
                "Crosshair name exceeds maximum length of {} characters",
                MAX_NAME_LENGTH
            )));
        }

        if self.description.len() > MAX_DESCRIPTION_LENGTH {
            return Err(XhairError::Validation(format!(
                "Crosshair description exceeds maximum length of {} characters",
                MAX_DESCRIPTION_LENGTH
            )));
        }

        if self.layers.len() > MAX_LAYERS {
            return Err(XhairError::Validation(format!(
                "Crosshair exceeds maximum limit of {} layers",
                MAX_LAYERS
            )));
        }

        let total_assets: usize = self.assets.values().map(|a| a.data_base64.len()).sum();
        if total_assets > MAX_ASSETS_BYTES {
            return Err(XhairError::Validation(format!(
                "Total embedded assets size ({} bytes) exceeds maximum allowable limit of {} bytes (4 MB)",
                total_assets, MAX_ASSETS_BYTES
            )));
        }

        if let Symmetry::Radial { arms } = self.symmetry {
            if !(2..=32).contains(&arms) {
                return Err(XhairError::Validation(
                    "Radial symmetry arm count must be between 2 and 32".to_string(),
                ));
            }
        }

        // Validate individual layer properties
        let mut group_map = HashMap::new();
        for layer in &self.layers {
            match &layer.kind {
                LayerKind::Polygon(poly) if !(3..=64).contains(&poly.sides) => {
                    return Err(XhairError::Validation(
                        "Polygon sides must be between 3 and 64".to_string(),
                    ));
                }
                LayerKind::Pixels(pix) if pix.width > 256 || pix.height > 256 => {
                    return Err(XhairError::Validation(
                        "Pixel layer dimension cannot exceed 256x256".to_string(),
                    ));
                }
                LayerKind::Path(path) => {
                    let point_count = path.data.split_whitespace().count();
                    if point_count > MAX_PATH_POINTS {
                        return Err(XhairError::Validation(format!(
                            "Path points count ({}) exceeds maximum limit of {}",
                            point_count, MAX_PATH_POINTS
                        )));
                    }
                }
                LayerKind::Group(group) => {
                    group_map.insert(layer.id, &group.children);
                }
                _ => {}
            }
        }

        // Check group nesting depth
        for &group_id in group_map.keys() {
            let mut visited = Vec::new();
            let depth = compute_group_depth(group_id, &group_map, &mut visited)?;
            if depth > MAX_NESTING_DEPTH {
                return Err(XhairError::Validation(format!(
                    "Group nesting depth ({}) exceeds maximum limit of {}",
                    depth, MAX_NESTING_DEPTH
                )));
            }
        }

        Ok(())
    }
}

fn compute_group_depth(
    id: crate::units::LayerId,
    map: &HashMap<crate::units::LayerId, &Vec<crate::units::LayerId>>,
    visited: &mut Vec<crate::units::LayerId>,
) -> Result<usize, XhairError> {
    if visited.contains(&id) {
        return Err(XhairError::Validation(
            "Cycle detected in layer group hierarchy".to_string(),
        ));
    }
    visited.push(id);
    let mut max_child_depth = 0;
    if let Some(children) = map.get(&id) {
        for &child in *children {
            let d = compute_group_depth(child, map, visited)?;
            if d > max_child_depth {
                max_child_depth = d;
            }
        }
    }
    visited.pop();
    Ok(1 + max_child_depth)
}
