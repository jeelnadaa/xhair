//! Coordinate systems, DPI scaling, and screen center math for xhair.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

/// Computes the exact screen center in edge coordinates.
///
/// For an even dimension (e.g. 1920), center lies on an edge (960.0).
/// For an odd dimension (e.g. 1921), center lies in the middle of a pixel (960.5).
pub fn screen_center(width: u32, height: u32) -> (f64, f64) {
    (width as f64 / 2.0, height as f64 / 2.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_screen_center_even_and_odd() {
        let (cx, cy) = screen_center(1920, 1080);
        assert_eq!(cx, 960.0);
        assert_eq!(cy, 540.0);

        let (cx_odd, cy_odd) = screen_center(1921, 1081);
        assert_eq!(cx_odd, 960.5);
        assert_eq!(cy_odd, 540.5);
    }
}
