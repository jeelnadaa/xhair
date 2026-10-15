//! Desktop application entry point, system tray daemon, and webview window management.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

pub mod server;

use std::process::Command;
use xhair_geometry::ResolutionProfile;
use xhair_model::crosshair::Crosshair;
use xhair_platform_win::{spawn_overlay, OverlayConfig, VisibilityMode};

/// Application default window title.
pub const APP_TITLE: &str = "XHAIR";

/// Runs the main application event loop.
pub fn run() -> anyhow::Result<()> {
    // 1. Install local panic crash handler with zero telemetry (§16, M10)
    let crash_dir = xhair_platform_win::install_crash_handler()
        .map_err(|e| anyhow::anyhow!("Failed to initialize crash handler: {}", e))?;
    tracing::debug!("Crash reports directory: {:?}", crash_dir);

    let version = env!("CARGO_PKG_VERSION");
    let args: Vec<String> = std::env::args().collect();
    let is_minimized = args.iter().any(|a| a == "--minimized");

    tracing::info!(
        "Starting {} v{} (minimized={})",
        APP_TITLE,
        version,
        is_minimized
    );

    // 2. Load default crosshair and resolution profile (§4, §6)
    let crosshair = Crosshair::new_classic("Default Crosshair");
    let profile = ResolutionProfile::default_1080p();

    // 3. Configure overlay: default to visible on screen (§8.1)
    let config = OverlayConfig {
        visibility_mode: VisibilityMode::AlwaysShow,
        ..Default::default()
    };

    // 4. Spawn the Win32 transparent click-through overlay window + tray icon + hotkeys (§8)
    let overlay_handle = spawn_overlay(crosshair, profile, config)
        .map_err(|e| anyhow::anyhow!("Failed to spawn overlay: {}", e))?;

    // 5. If not launched minimized, start the local UI server and launch default browser
    if !is_minimized {
        if let Some(port) = server::start_ui_server() {
            let url = format!("http://127.0.0.1:{}", port);
            tracing::info!("XHAIR Editor UI serving at {}", url);
            let _ = Command::new("cmd").args(["/c", "start", &url]).spawn();
        }
    }

    // 6. Keep daemon running until exit (blocks on overlay thread message loop / tray menu)
    overlay_handle.wait();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_title() {
        assert_eq!(APP_TITLE, "XHAIR");
    }
}
