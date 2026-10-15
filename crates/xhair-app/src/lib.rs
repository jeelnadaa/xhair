//! Desktop application entry point, system tray daemon, and webview window management.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

/// Application default window title.
pub const APP_TITLE: &str = "XHAIR";

/// Runs the main application event loop.
pub fn run() -> anyhow::Result<()> {
    tracing::info!("Starting {}", APP_TITLE);
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
