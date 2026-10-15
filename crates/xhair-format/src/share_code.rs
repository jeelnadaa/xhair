//! Share code encoder and decoder with Brotli compression and CRC32 verification.

use crate::canonical::to_canonical_json;
use crate::error::FormatError;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use brotli::{CompressorWriter, Decompressor};
use std::io::{Read, Write};
use xhair_model::crosshair::Crosshair;

/// Canonical share code prefix.
pub const SHARE_CODE_PREFIX: &str = "XH1";

/// Maximum allowable length of a share code in bytes (8 KB).
pub const MAX_SHARE_CODE_LEN: usize = 8192;

/// Encodes a crosshair document into a share code: `XH1-<base64url(brotli(canonical_json))>-<crc32>`.
pub fn encode_share_code(crosshair: &Crosshair) -> Result<String, FormatError> {
    crosshair
        .validate()
        .map_err(|e| FormatError::ValidationFailed(e.to_string()))?;

    let canonical_json = to_canonical_json(crosshair)?;
    let json_bytes = canonical_json.as_bytes();

    // Compute CRC32 checksum over the canonical JSON
    let mut hasher = crc32fast::Hasher::new();
    hasher.update(json_bytes);
    let checksum = hasher.finalize();

    // Compress using Brotli (quality level 9)
    let mut compressed = Vec::new();
    {
        let mut compressor = CompressorWriter::new(&mut compressed, 4096, 9, 22);
        compressor
            .write_all(json_bytes)
            .map_err(|e| FormatError::Serialization(e.to_string()))?;
    }

    let b64_payload = URL_SAFE_NO_PAD.encode(&compressed);
    let code = format!("{}-{}-{:08x}", SHARE_CODE_PREFIX, b64_payload, checksum);

    if code.len() > MAX_SHARE_CODE_LEN {
        return Err(FormatError::CodeTooLarge {
            size: code.len(),
            max: MAX_SHARE_CODE_LEN,
        });
    }

    Ok(code)
}

/// Decodes a share code into a validated Crosshair document.
pub fn decode_share_code(code: &str) -> Result<Crosshair, FormatError> {
    let trimmed = code.trim();

    if trimmed.len() > MAX_SHARE_CODE_LEN {
        return Err(FormatError::CodeTooLarge {
            size: trimmed.len(),
            max: MAX_SHARE_CODE_LEN,
        });
    }

    let without_prefix = trimmed
        .strip_prefix(&format!("{}-", SHARE_CODE_PREFIX))
        .ok_or_else(|| {
            FormatError::InvalidShareCode(format!(
                "Share code must start with '{}-'",
                SHARE_CODE_PREFIX
            ))
        })?;

    let split_pos = without_prefix.rfind('-').ok_or_else(|| {
        FormatError::InvalidShareCode("Share code missing CRC32 delimiter '-'".to_string())
    })?;

    let b64_payload = &without_prefix[..split_pos];
    let crc_str = &without_prefix[split_pos + 1..];

    if crc_str.len() != 8 {
        return Err(FormatError::InvalidShareCode(format!(
            "CRC32 must be 8 hex characters, got '{}'",
            crc_str
        )));
    }

    let expected_crc = u32::from_str_radix(crc_str, 16).map_err(|_| {
        FormatError::InvalidShareCode(format!("Invalid hex checksum '{}'", crc_str))
    })?;

    let compressed_bytes = URL_SAFE_NO_PAD
        .decode(b64_payload)
        .map_err(|e| FormatError::InvalidShareCode(format!("Base64 decoding failed: {e}")))?;

    let mut decompressed = Vec::new();
    {
        let mut decompressor = Decompressor::new(compressed_bytes.as_slice(), 4096);
        decompressor.read_to_end(&mut decompressed).map_err(|e| {
            FormatError::Deserialization(format!("Brotli decompression failed: {e}"))
        })?;
    }

    // Verify CRC32
    let mut hasher = crc32fast::Hasher::new();
    hasher.update(&decompressed);
    let computed_crc = hasher.finalize();

    if computed_crc != expected_crc {
        return Err(FormatError::CorruptedCrc {
            expected: expected_crc,
            computed: computed_crc,
        });
    }

    let json_str = std::str::from_utf8(&decompressed)
        .map_err(|e| FormatError::Deserialization(format!("UTF-8 decode failed: {e}")))?;

    let crosshair: Crosshair =
        serde_json::from_str(json_str).map_err(|e| FormatError::Deserialization(e.to_string()))?;

    crosshair
        .validate()
        .map_err(|e| FormatError::ValidationFailed(e.to_string()))?;

    Ok(crosshair)
}
