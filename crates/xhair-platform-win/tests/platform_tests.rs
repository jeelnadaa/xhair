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
