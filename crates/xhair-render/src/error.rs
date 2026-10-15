//! Errors produced during rasterization and visual rendering.

use thiserror::Error;

/// Rendering and rasterization error types.
#[derive(Debug, Error, Clone, PartialEq)]
pub enum RenderError {
    /// Dimension exceeds permissible maximum (2048x2048).
    #[error("Output dimension ({width}x{height}) exceeds maximum limit of {max}x{max}")]
    DimensionTooLarge {
        /// Attempted width.
        width: u32,
        /// Attempted height.
        height: u32,
        /// Maximum allowed dimension.
        max: u32,
    },

    /// Pixmap buffer allocation failed.
    #[error("Failed to allocate pixmap buffer for dimensions {width}x{height}")]
    AllocationFailed {
        /// Requested width.
        width: u32,
        /// Requested height.
        height: u32,
    },

    /// Vector path syntax invalid.
    #[error("Invalid SVG path data: {0}")]
    InvalidPath(String),

    /// Shape geometry generation failed.
    #[error("Geometry error in layer '{layer_name}': {reason}")]
    GeometryError {
        /// Name of layer.
        layer_name: String,
        /// Specific failure reason.
        reason: String,
    },

    /// Asset resolution failed.
    #[error("Embedded asset with hash '{hash}' not found")]
    AssetNotFound {
        /// Hash identifier.
        hash: String,
    },

    /// PNG encoding failure.
    #[error("Failed to encode PNG image: {0}")]
    PngEncoding(String),
}
