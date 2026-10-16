//! Integration and contract tests for Windows platform overlay and system queries (§8).

use xhair_platform_win::{
    create_calibration_crosshair, enumerate_monitors, get_primary_monitor, is_game_foreground,
    HotkeyManager, HotkeyModifiers, OverlayConfig, VisibilityMode,
};

#[test]
fn test_monitor_enumeration() {
    let monitors = enumerate_monitors().expect("Monitor enumeration should succeed");
    assert!(
        !monitors.is_empty(),
        "At least one monitor must be detected"
    );

    let primary = get_primary_monitor().expect("Primary monitor must be resolved");
    assert!(primary.width() > 0);
    assert!(primary.height() > 0);
    assert!(primary.scale_factor >= 1.0);
    println!(
        "Primary monitor detected: {} ({}x{}, scale: {:.2})",
        primary.device_name,
        primary.width(),
        primary.height(),
        primary.scale_factor
    );
}

#[test]
fn test_foreground_game_detection_with_dummy_target() {
    let dummy_target = vec!["non_existent_game_binary_12345.exe".to_string()];
    let is_focused = is_game_foreground(&dummy_target);
    assert!(
        !is_focused,
        "Non-existent dummy binary should not be foreground"
    );
}

#[test]
fn test_hotkey_conflict_checking() {
    // Test a common key combination
    let mods = HotkeyModifiers {
        ctrl: true,
        alt: true,
        shift: false,
        win: false,
    };
    let res = HotkeyManager::test_conflict(mods, 0x58); // Ctrl+Alt+X
    assert!(
        res.is_ok(),
        "Conflict test should return a clean boolean result"
    );
}

#[test]
fn test_calibration_pattern_even_odd_alignments() {
    // 1080p (even x even)
    let pattern_even = create_calibration_crosshair(1920, 1080);
    assert!(pattern_even.validate().is_ok());

    // 1921x1081 (odd x odd)
    let pattern_odd = create_calibration_crosshair(1921, 1081);
    assert!(pattern_odd.validate().is_ok());
}

#[test]
fn test_overlay_config_defaults() {
    let config = OverlayConfig::default();
    assert_eq!(
        config.visibility_mode,
        VisibilityMode::OnlyWhenGameForeground
    );
    assert!(config.follow_game_window);
    assert!(!config.exclude_from_capture);
    assert!(!config.calibration_mode);
    assert_eq!(
        config.target_game_images,
        vec!["VALORANT-Win64-Shipping.exe"]
    );
}

#[test]
fn test_spawn_and_render_overlay() {
    let xhair = xhair_model::crosshair::Crosshair::new_classic("Test Overlay");
    let profile = xhair_geometry::ResolutionProfile::default_1080p();
    let config = OverlayConfig {
        visibility_mode: VisibilityMode::AlwaysShow,
        follow_game_window: false,
        ..Default::default()
    };
    let handle =
        xhair_platform_win::spawn_overlay(xhair, profile, config).expect("Failed to spawn overlay");
    std::thread::sleep(std::time::Duration::from_millis(100));
    handle.stop();
}

#[test]
fn test_calibration_offset_shifts_overlay_rect() {
    use xhair_geometry::{GeometryMapping, ResolutionProfile};
    use xhair_model::crosshair::Crosshair;
    use xhair_model::units::Vec2;
    use xhair_platform_win::compute_overlay_window_rect;

    let xhair = Crosshair::new_classic("Offset Test");
    let profile = ResolutionProfile::default_1080p();
    let mapping = GeometryMapping::compute(&profile, (0.0, 0.0), (1920.0, 1080.0));

    let mut config = OverlayConfig::default();
    config.calibration_offset = Vec2::ZERO;
    let (x0, y0, w0, h0) = compute_overlay_window_rect(&xhair, &mapping, &config);

    // Apply a calibration offset of +15px horizontal, -10px vertical
    config.calibration_offset = Vec2::new(15.0, -10.0);
    let (x1, y1, w1, h1) = compute_overlay_window_rect(&xhair, &mapping, &config);

    assert_eq!(w0, w1);
    assert_eq!(h0, h1);
    assert_eq!(x1 - x0, 15);
    assert_eq!(y1 - y0, -10);
}

#[test]
fn test_large_offset_crosshair_bounds_zero_clipping() {
    use xhair_geometry::{GeometryMapping, ResolutionProfile};
    use xhair_model::crosshair::Crosshair;
    use xhair_model::units::Vec2;
    use xhair_platform_win::compute_overlay_window_rect;

    let mut xhair = Crosshair::new_classic("Offset Reticle");
    // Move reticle 60px right, 40px up
    xhair.global.offset = Vec2::new(60.0, -40.0);

    let profile = ResolutionProfile::default_1080p();
    let mapping = GeometryMapping::compute(&profile, (0.0, 0.0), (1920.0, 1080.0));
    let config = OverlayConfig::default();

    let (win_x, win_y, win_w, win_h) = compute_overlay_window_rect(&xhair, &mapping, &config);

    // Buffer must be at least large enough to contain the 60px offset + reticle extent symmetrically
    assert!(win_w >= 140, "win_w ({}) should be at least 140px", win_w);
    assert!(win_h >= 100, "win_h ({}) should be at least 100px", win_h);

    // Reticle origin inside buffer is at (win_w/2, win_h/2).
    // On screen, window is at win_x.
    // The screen position of the reticle center (960.0) is exactly:
    let screen_reticle_x = win_x as f32 + (win_w as f32 / 2.0);
    let screen_reticle_y = win_y as f32 + (win_h as f32 / 2.0);
    assert!((screen_reticle_x - 960.0).abs() <= 1.0);
    assert!((screen_reticle_y - 540.0).abs() <= 1.0);
}
