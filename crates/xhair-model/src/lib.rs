//! Domain model and validation rules for xhair.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

/// Initial schema format version.
pub const FORMAT_VERSION: u32 = 1;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_version() {
        assert_eq!(FORMAT_VERSION, 1);
    }
}
