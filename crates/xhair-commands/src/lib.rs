//! Command bus, undo/redo, and command registry for xhair.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

/// Default maximum undo stack depth.
pub const DEFAULT_MAX_UNDO_DEPTH: usize = 500;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_undo_depth() {
        assert_eq!(DEFAULT_MAX_UNDO_DEPTH, 500);
    }
}
