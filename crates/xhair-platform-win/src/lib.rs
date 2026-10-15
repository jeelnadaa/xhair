//! Windows Win32 platform layer for overlay window, monitors, foreground detection, and hotkeys.

#![deny(missing_docs)]

/// Default game executable name for foreground auto-visibility.
pub const DEFAULT_GAME_IMAGE_NAME: &str = "VALORANT-Win64-Shipping.exe";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_game_image() {
        assert_eq!(DEFAULT_GAME_IMAGE_NAME, "VALORANT-Win64-Shipping.exe");
    }
}
