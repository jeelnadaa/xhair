//! Format migration engine for backwards compatibility across schema versions.

use crate::error::FormatError;
use serde_json::Value;
use xhair_model::crosshair::CURRENT_FORMAT_VERSION;

/// Migrates a raw JSON value from its stored version up to `CURRENT_FORMAT_VERSION`.
pub fn migrate_document_json(doc: Value) -> Result<Value, FormatError> {
    let version = doc
        .get("format_version")
        .and_then(|v| v.as_u64())
        .map(|v| v as u32)
        .unwrap_or(1);

    if version > CURRENT_FORMAT_VERSION {
        return Err(FormatError::UnsupportedVersion {
            version,
            max_supported: CURRENT_FORMAT_VERSION,
        });
    }

    Ok(doc)
}
