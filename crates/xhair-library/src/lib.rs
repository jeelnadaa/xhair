//! Disk persistence, slots, tagging, and folder tree management for xhair.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

/// Maximum allowed quick-access hotkey slots.
pub const MAX_SLOTS: usize = 20;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_slots() {
        assert_eq!(MAX_SLOTS, 20);
    }
}
