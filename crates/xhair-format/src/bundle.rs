//! `.xhairpack` bundle archive encoder and decoder using ZIP format.

use crate::canonical::to_canonical_json;
use crate::error::FormatError;
use serde::{Deserialize, Serialize};
use std::io::{Cursor, Read, Write};
use xhair_model::crosshair::Crosshair;
use zip::write::SimpleFileOptions;
use zip::{ZipArchive, ZipWriter};

/// Current pack format version.
pub const PACK_FORMAT_VERSION: u32 = 1;

/// Entry item within a crosshair pack manifest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PackEntry {
    /// Crosshair unique UUID string.
    pub id: String,
    /// Human-facing crosshair name.
    pub name: String,
    /// Creator or author.
    pub author: String,
    /// Archive internal relative path (e.g. `crosshairs/<uuid>.xhair`).
    pub path: String,
}

/// Metadata header describing a crosshair collection pack.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PackManifest {
    /// Manifest schema format version.
    pub pack_version: u32,
    /// Title of the crosshair pack.
    pub title: String,
    /// Collection description or notes.
    pub description: String,
    /// Author attribution.
    pub author: String,
    /// Contained crosshair entries.
    pub entries: Vec<PackEntry>,
}

/// Exports a list of crosshairs into a `.xhairpack` ZIP archive.
pub fn export_pack(
    crosshairs: &[Crosshair],
    title: &str,
    description: &str,
    author: &str,
) -> Result<Vec<u8>, FormatError> {
    let mut buffer = Vec::new();
    {
        let mut zip = ZipWriter::new(Cursor::new(&mut buffer));
        let options =
            SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);

        let mut entries = Vec::with_capacity(crosshairs.len());

        for xhair in crosshairs {
            let id_str = xhair.id.0.to_string();
            let relative_path = format!("crosshairs/{}.xhair", id_str);

            entries.push(PackEntry {
                id: id_str,
                name: xhair.name.clone(),
                author: xhair.author.clone(),
                path: relative_path.clone(),
            });

            let json_str = to_canonical_json(xhair)?;
            zip.start_file(relative_path, options)
                .map_err(|e| FormatError::ZipError(e.to_string()))?;
            zip.write_all(json_str.as_bytes())
                .map_err(|e| FormatError::ZipError(e.to_string()))?;
        }

        let manifest = PackManifest {
            pack_version: PACK_FORMAT_VERSION,
            title: title.to_string(),
            description: description.to_string(),
            author: author.to_string(),
            entries,
        };

        let manifest_json = serde_json::to_string_pretty(&manifest)
            .map_err(|e| FormatError::Serialization(e.to_string()))?;

        zip.start_file("manifest.json", options)
            .map_err(|e| FormatError::ZipError(e.to_string()))?;
        zip.write_all(manifest_json.as_bytes())
            .map_err(|e| FormatError::ZipError(e.to_string()))?;

        zip.finish()
            .map_err(|e| FormatError::ZipError(e.to_string()))?;
    }

    Ok(buffer)
}

/// Imports and extracts a `.xhairpack` ZIP archive into manifest metadata and validated crosshairs.
pub fn import_pack(zip_bytes: &[u8]) -> Result<(PackManifest, Vec<Crosshair>), FormatError> {
    let reader = Cursor::new(zip_bytes);
    let mut archive = ZipArchive::new(reader)
        .map_err(|e| FormatError::ZipError(format!("Invalid zip archive: {e}")))?;

    // Read manifest.json
    let manifest: PackManifest = {
        let mut manifest_file = archive.by_name("manifest.json").map_err(|_| {
            FormatError::ZipError("Pack lacks 'manifest.json' metadata file".into())
        })?;
        let mut content = String::new();
        manifest_file
            .read_to_string(&mut content)
            .map_err(|e| FormatError::ZipError(format!("Failed to read manifest.json: {e}")))?;
        serde_json::from_str(&content)
            .map_err(|e| FormatError::Deserialization(format!("Invalid manifest.json: {e}")))?
    };

    let mut crosshairs = Vec::with_capacity(manifest.entries.len());

    for entry in &manifest.entries {
        let mut file = archive.by_name(&entry.path).map_err(|_| {
            FormatError::ZipError(format!("Pack entry file '{}' missing", entry.path))
        })?;

        let mut content = String::new();
        file.read_to_string(&mut content).map_err(|e| {
            FormatError::ZipError(format!("Failed to read entry '{}': {e}", entry.path))
        })?;

        let crosshair: Crosshair = serde_json::from_str(&content).map_err(|e| {
            FormatError::Deserialization(format!("Failed to parse '{}': {e}", entry.path))
        })?;

        crosshair
            .validate()
            .map_err(|e| FormatError::ValidationFailed(format!("'{}': {e}", entry.name)))?;

        crosshairs.push(crosshair);
    }

    Ok((manifest, crosshairs))
}
