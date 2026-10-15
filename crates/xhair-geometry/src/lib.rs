//! Pure geometric calculations for screen centering, pixel alignment, stretch scaling, and DPI conversions.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

pub mod alignment;
pub mod coords;
pub mod mapping;
pub mod profile;
pub mod screenshot_align;

pub use alignment::*;
pub use coords::*;
pub use mapping::*;
pub use profile::*;
pub use screenshot_align::*;

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

    #[test]
    fn test_rect_and_circle_alignment_diagnostics() {
        use xhair_model::shapes::{CircleProps, RectProps};

        let mut xhair = Crosshair::new_blank("Test Rect & Circle");
        // Rect 7x8 on 1920x1080: 1920+7 is odd (misaligned horizontally), 1080+8 is even (aligned vertically)
        xhair.layers.push(Layer::new(
            "Rect",
            LayerKind::Rect(RectProps {
                width: 7.0,
                height: 8.0,
                ..Default::default()
            }),
        ));
        // Circle 5.0 on 1920x1080: 1920+5 is odd (misaligned)
        xhair.layers.push(Layer::new(
            "Circle",
            LayerKind::Circle(CircleProps { diameter: 5.0 }),
        ));

        let issues = alignment_report(&xhair, 1920, 1080);
        assert_eq!(issues.len(), 2);
        assert_eq!(issues[0].layer_name, "Rect");
        assert_eq!(issues[0].axis, AlignmentAxis::Horizontal);
        assert_eq!(issues[1].layer_name, "Circle");
        assert_eq!(issues[1].axis, AlignmentAxis::Both);
    }

    #[test]
    fn test_apply_suggested_fix() {
        let mut xhair = Crosshair::new_blank("Fix Test");
        xhair.layers.push(Layer::new(
            "1px Line",
            LayerKind::Line(LineProps {
                thickness: 1.0,
                ..Default::default()
            }),
        ));
        let issues = alignment_report(&xhair, 1920, 1080);
        assert_eq!(issues.len(), 1);

        // Apply 1-click fix
        let applied = apply_suggested_fix(&mut xhair, &issues[0]);
        assert!(applied);

        // Verify issue is now resolved
        let remaining_issues = alignment_report(&xhair, 1920, 1080);
        assert!(remaining_issues.is_empty());
    }

    #[test]
    fn test_multi_resolution_verification() {
        use xhair_model::units::Vec2;

        // Native 1080p
        let p_1080p = ResolutionProfile::default_1080p();
        let map_1080p = GeometryMapping::compute(&p_1080p, (0.0, 0.0), (1920.0, 1080.0));
        assert_eq!(map_1080p.sx, 1.0);
        assert_eq!(map_1080p.sy, 1.0);
        assert_eq!(
            (map_1080p.out_center_x, map_1080p.out_center_y),
            (960.0, 540.0)
        );

        // Native 1440p
        let p_1440p = ResolutionProfile::default_1440p();
        let map_1440p = GeometryMapping::compute(&p_1440p, (0.0, 0.0), (2560.0, 1440.0));
        assert_eq!(map_1440p.sx, 1.0);
        assert_eq!(map_1440p.sy, 1.0);
        assert_eq!(
            (map_1440p.out_center_x, map_1440p.out_center_y),
            (1280.0, 720.0)
        );

        // Stretched 4:3 1280x1024 on 1080p output (1920x1080)
        let mut p_stretched = ResolutionProfile::default_1080p();
        p_stretched.game_resolution = Resolution::STRETCHED_4_3;
        p_stretched.display_mode = DisplayMode::Stretched;
        let map_stretched = GeometryMapping::compute(&p_stretched, (0.0, 0.0), (1920.0, 1080.0));
        assert!((map_stretched.sx - 1.5).abs() < 1e-4);
        assert!((map_stretched.sy - (1080.0 / 1024.0)).abs() < 1e-4);

        // Calibration offset application
        let mut p_calib = ResolutionProfile::default_1080p();
        p_calib.calibration_offset = Vec2::new(0.5, -0.25);
        let map_calib = GeometryMapping::compute(&p_calib, (0.0, 0.0), (1920.0, 1080.0));
        assert_eq!(
            (map_calib.out_center_x, map_calib.out_center_y),
            (960.5, 539.75)
        );
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
