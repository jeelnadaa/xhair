//! Windows Win32 platform layer for overlay window, monitors, foreground detection, and hotkeys (§8).

#![forbid(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

pub mod calibration;
pub mod error;
pub mod foreground;
pub mod hotkey;
pub mod monitor;
pub mod overlay;
pub mod tray;

pub use calibration::create_calibration_crosshair;
pub use error::PlatformError;
pub use foreground::{get_foreground_window_info, is_game_foreground, ForegroundWindowInfo};
pub use hotkey::{HotkeyAction, HotkeyBinding, HotkeyManager, HotkeyModifiers};
pub use monitor::{enumerate_monitors, get_monitor_at_point, get_primary_monitor, MonitorInfo};
pub use overlay::{
    render_and_update, spawn_overlay, OverlayCommand, OverlayConfig, OverlayHandle, VisibilityMode,
};
pub use tray::{TrayIcon, TrayStatus, WM_TRAY_CALLBACK};

/// Default game executable name for foreground auto-visibility (§8.1).
pub const DEFAULT_GAME_IMAGE_NAME: &str = "VALORANT-Win64-Shipping.exe";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_game_image() {
        assert_eq!(DEFAULT_GAME_IMAGE_NAME, "VALORANT-Win64-Shipping.exe");
    }

    #[test]
    fn test_tray_status_formatting() {
        let status = TrayStatus {
            overlay_enabled: true,
            crosshair_name: "Test Reticle".to_string(),
            dynamic_enabled: false,
            mcp_enabled: true,
        };
        let tip = status.format_tooltip();
        assert!(tip.contains("Overlay: ON"));
        assert!(tip.contains("Reticle: Test Reticle"));
        assert!(tip.contains("MCP: ON"));
    }

    #[test]
    fn test_calibration_pattern_creation() {
        let xhair = create_calibration_crosshair(1920, 1080);
        assert!(!xhair.layers.is_empty());
        assert_eq!(xhair.name, "Calibration Test Pattern");
    }
}
