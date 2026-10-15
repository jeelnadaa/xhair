//! Error catalog for Windows platform APIs (§8, §15.1).

use thiserror::Error;

/// Errors produced during Windows platform, layered window, monitor, and hotkey operations.
#[derive(Debug, Error)]
pub enum PlatformError {
    /// Window class registration or creation failed.
    #[error("Failed to create Win32 overlay window: {0}")]
    WindowCreation(String),

    /// UpdateLayeredWindow failed.
    #[error("UpdateLayeredWindow failed: {0}")]
    LayeredWindowUpdate(String),

    /// Display monitor query failed.
    #[error("Monitor query failed: {0}")]
    MonitorQuery(String),

    /// Global hotkey registration conflict or OS error.
    #[error("Failed to register hotkey (ID {id}, vk: {vk:#x}): {message}")]
    HotkeyConflict {
        /// Hotkey ID.
        id: i32,
        /// Virtual key code.
        vk: u32,
        /// Error message.
        message: String,
    },

    /// Failed to query process image name or elevation status.
    #[error("Process query failed for PID {pid}: {message}")]
    ProcessQuery {
        /// Process ID.
        pid: u32,
        /// Description of failure.
        message: String,
    },

    /// System tray icon error.
    #[error("System tray error: {0}")]
    TrayError(String),

    /// Internal thread or channel communication error.
    #[error("Platform message channel error: {0}")]
    ChannelError(String),

    /// General Windows API or system registry error.
    #[error("Windows API error: {0}")]
    Windows(String),
}
