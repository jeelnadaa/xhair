//! Structured application error types with unique error codes (XH-1xxx).

use thiserror::Error;

/// Root domain and validation error type.
#[derive(Debug, Error, Clone, PartialEq)]
pub enum XhairError {
    /// XH-1001: General domain validation failed.
    #[error("[XH-1001] Validation error: {0}")]
    Validation(String),

    /// XH-1002: Maximum layer count exceeded.
    #[error(
        "[XH-1002] Layer limit exceeded: maximum allowed is {max}, attempted to add {attempted}"
    )]
    LayerLimitExceeded {
        /// Maximum allowed layers.
        max: usize,
        /// Total layers after attempted addition.
        attempted: usize,
    },

    /// XH-1003: Numeric value outside permissible bounds.
    #[error("[XH-1003] Range error on property '{property}': value {value} outside allowed [{min}, {max}]")]
    OutOfRange {
        /// Property name.
        property: String,
        /// Actual attempted value.
        value: f32,
        /// Minimum allowed.
        min: f32,
        /// Maximum allowed.
        max: f32,
    },

    /// XH-1004: Serialization / deserialization format error.
    #[error("[XH-1004] Serialization error: {0}")]
    Serialization(String),

    /// XH-1005: Unsupported document version.
    #[error(
        "[XH-1005] Unsupported document format version {version} (latest supported is {latest})"
    )]
    UnsupportedVersion {
        /// Version in file.
        version: u32,
        /// Latest supported version.
        latest: u32,
    },

    /// XH-1006: Target entity not found.
    #[error("[XH-1006] Entity not found: {entity_type} with ID '{id}'")]
    NotFound {
        /// Type of entity (e.g. "layer", "crosshair").
        entity_type: String,
        /// Identifier string.
        id: String,
    },

    /// XH-1007: Embedded asset size exceeds permissible budget.
    #[error(
        "[XH-1007] Asset budget exceeded: asset size {size_bytes} exceeds limit of {max_bytes}"
    )]
    AssetTooLarge {
        /// Asset size in bytes.
        size_bytes: usize,
        /// Maximum allowable bytes.
        max_bytes: usize,
    },
}

impl XhairError {
    /// Returns the unique error code (e.g. "XH-1001").
    pub fn code(&self) -> &'static str {
        match self {
            Self::Validation(_) => "XH-1001",
            Self::LayerLimitExceeded { .. } => "XH-1002",
            Self::OutOfRange { .. } => "XH-1003",
            Self::Serialization(_) => "XH-1004",
            Self::UnsupportedVersion { .. } => "XH-1005",
            Self::NotFound { .. } => "XH-1006",
            Self::AssetTooLarge { .. } => "XH-1007",
        }
    }
}
