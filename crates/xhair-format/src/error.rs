//! Format serialization, decompression, and import errors.

use thiserror::Error;

/// Errors produced during encoding, decoding, compression, and migration.
#[derive(Debug, Error, Clone, PartialEq)]
pub enum FormatError {
    /// JSON serialization failed.
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// JSON deserialization failed.
    #[error("Deserialization error: {0}")]
    Deserialization(String),

    /// Share code prefix or syntax invalid.
    #[error("Invalid share code format: {0}")]
    InvalidShareCode(String),

    /// CRC32 checksum mismatch in share code.
    #[error(
        "Share code checksum verification failed: expected {expected:08x}, computed {computed:08x}"
    )]
    CorruptedCrc {
        /// Expected checksum from code.
        expected: u32,
        /// Computed checksum from payload.
        computed: u32,
    },

    /// Share code payload exceeds maximum allowable length (8192 bytes).
    #[error("Share code exceeds maximum length limit of {max} bytes (actual: {size} bytes)")]
    CodeTooLarge {
        /// Actual size in bytes.
        size: usize,
        /// Maximum permitted limit.
        max: usize,
    },

    /// Document version is newer than maximum supported version.
    #[error("Unsupported document version {version} (latest supported is {max_supported})")]
    UnsupportedVersion {
        /// File version.
        version: u32,
        /// Max version known to this build.
        max_supported: u32,
    },

    /// Validation error on untrusted imported crosshair.
    #[error("Imported crosshair failed validation: {0}")]
    ValidationFailed(String),

    /// ZIP bundle error.
    #[error("Zip archive error: {0}")]
    ZipError(String),

    /// In-game Valorant crosshair string parse error.
    #[error("Valorant crosshair parse error: {0}")]
    ValorantParseError(String),
}
