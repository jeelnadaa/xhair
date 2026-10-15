//! Desktop application entry point, system tray daemon, and webview window management.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

/// Application default window title.
pub const APP_TITLE: &str = "XHAIR";

/// Runs the main application event loop.
pub fn run() -> anyhow::Result<()> {
    // Install local panic crash handler with zero telemetry (§16, M10)
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
