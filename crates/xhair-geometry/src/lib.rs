//! Pure geometric calculations for screen centering, pixel alignment, stretch scaling, and DPI conversions.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

pub mod alignment;
pub mod coords;
pub mod mapping;
pub mod profile;

pub use alignment::*;
pub use coords::*;
pub use mapping::*;
pub use profile::*;

/// Quick helper returning the screen midpoint in continuous edge coordinates.
pub fn screen_center(width: u32, height: u32) -> (f64, f64) {
    let center = compute_screen_center(width, height);
    (center.cx, center.cy)
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use xhair_model::crosshair::Crosshair;
    use xhair_model::layer::Layer;
    use xhair_model::shapes::{DotProps, LayerKind, LineProps};

    #[test]
    fn test_appendix_a_worked_examples() {
        // 1920x1080, line thickness 2: (1920+2) even -> edges at 960±1 = integers (aligned)
        assert!(is_symmetric_stroke_aligned(1920, 2.0));
        assert!(is_symmetric_stroke_aligned(1080, 2.0));

        // Thickness 1 on 1920: (1920+1) odd -> misaligned
        assert!(!is_symmetric_stroke_aligned(1920, 1.0));

        // 1921x1081 (odd): thickness 1 -> (1921+1) even -> aligned
        assert!(is_symmetric_stroke_aligned(1921, 1.0));
        assert!(is_symmetric_stroke_aligned(1081, 1.0));

        // Stretched 1280x1024 on a 1920x1080 output: sx=1.5, sy≈1.0546875
        let mut profile = ResolutionProfile::default_1080p();
        profile.game_resolution = Resolution::STRETCHED_4_3; // 1280x1024
        profile.display_mode = DisplayMode::Stretched;

        let mapping = GeometryMapping::compute(&profile, (0.0, 0.0), (1920.0, 1080.0));
        assert!((mapping.sx - 1.5).abs() < 1e-4);
        assert!((mapping.sy - (1080.0 / 1024.0)).abs() < 1e-4);
    }

    #[test]
    fn test_alignment_diagnostics() {
        let mut xhair = Crosshair::new_blank("Test");
        // Thickness 1 on 1920x1080 should trigger an issue
        xhair.layers.push(Layer::new(
            "1px Line",
            LayerKind::Line(LineProps {
                thickness: 1.0,
                ..Default::default()
            }),
        ));
        let issues = alignment_report(&xhair, 1920, 1080);
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].axis, AlignmentAxis::Both);
    }

    #[test]
    fn test_dot_alignment_diagnostics() {
        let mut xhair = Crosshair::new_blank("Test");
        xhair.layers.push(Layer::new(
            "Dot",
            LayerKind::Dot(DotProps {
                diameter: 2.0,
                ..Default::default()
            }),
        ));
        let issues = alignment_report(&xhair, 1920, 1080);
        // Diameter 2 on 1920x1080 is aligned!
        assert!(issues.is_empty());
    }

    proptest! {
        #[test]
        fn prop_coordinate_mapping_round_trip(
            gx in -500.0f64..500.0,
            gy in -500.0f64..500.0,
            out_w in 800.0f64..3840.0,
            out_h in 600.0f64..2160.0
        ) {
            let mut profile = ResolutionProfile::default_1080p();
            profile.display_mode = DisplayMode::Stretched;
            let mapping = GeometryMapping::compute(&profile, (0.0, 0.0), (out_w, out_h));

            let (px, py) = mapping.game_to_physical(gx, gy);
            let (back_gx, back_gy) = mapping.physical_to_game(px, py);

            prop_assert!((gx - back_gx).abs() < 1e-6);
            prop_assert!((gy - back_gy).abs() < 1e-6);
        }

        #[test]
        fn prop_dpi_scaling_round_trip(
            dips in 0.0f64..2000.0,
            dpi in 96u32..480u32 // 100% to 500% Windows scaling
        ) {
            let physical = dips_to_physical_px(dips, dpi);
            let round_trip = physical_px_to_dips(physical, dpi);
            prop_assert!((dips - round_trip).abs() < 1e-6);
        }

        #[test]
        fn prop_even_odd_center_coordinates(
            w in 1u32..4000u32,
            h in 1u32..4000u32
        ) {
            let center = compute_screen_center(w, h);
            if w % 2 == 0 {
                prop_assert_eq!(center.horizontal_alignment, CenterAlignment::PixelEdge);
                prop_assert_eq!(center.cx.fract(), 0.0);
            } else {
                prop_assert_eq!(center.horizontal_alignment, CenterAlignment::PixelMiddle);
                prop_assert_eq!(center.cx.fract(), 0.5);
            }

            if h % 2 == 0 {
                prop_assert_eq!(center.vertical_alignment, CenterAlignment::PixelEdge);
                prop_assert_eq!(center.cy.fract(), 0.0);
            } else {
                prop_assert_eq!(center.vertical_alignment, CenterAlignment::PixelMiddle);
                prop_assert_eq!(center.cy.fract(), 0.5);
            }
        }
    }
}
