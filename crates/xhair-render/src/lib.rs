//! Deterministic vector rasterization and visual effects for xhair.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

/// Maximum allowed overlay bounding box dimension in pixels.
pub const MAX_OVERLAY_DIMENSION: u32 = 2048;

/// Maximum allowed blur scratch buffer dimension in pixels.
pub const MAX_BLUR_BUFFER_DIMENSION: u32 = 1024;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_limits() {
        assert!(MAX_BLUR_BUFFER_DIMENSION <= MAX_OVERLAY_DIMENSION);
    }
}
