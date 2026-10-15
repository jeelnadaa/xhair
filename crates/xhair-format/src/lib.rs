//! File serialization, share codes, and external profile converters for xhair.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

/// Share code magic header prefix.
pub const SHARE_CODE_PREFIX: &str = "XH1-";

/// Maximum allowed uncompressed JSON size for share codes in bytes.
pub const MAX_SHARE_CODE_PAYLOAD_BYTES: usize = 8192;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prefix() {
        assert_eq!(SHARE_CODE_PREFIX, "XH1-");
    }
}
