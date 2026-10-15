//! Crash-safe atomic file writing with fsync and backup preservation (§10.1).

use crate::error::LibraryError;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

/// Writes data to a file atomically: writes to a temporary file in the same directory,
/// calls `sync_all()`, updates the `.bak` file, and renames atomically over the target.
pub fn atomic_write_file(target_path: &Path, data: &[u8]) -> Result<(), LibraryError> {
    let parent = target_path.parent().unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent).map_err(|e| LibraryError::Io {
        path: parent.to_string_lossy().into(),
        source: e,
    })?;

    // Create temp file in same directory for atomic rename on the same volume
    let temp_name = format!(
        ".{}.tmp.{}",
        target_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("file"),
        uuid::Uuid::now_v7()
    );
    let temp_path = parent.join(temp_name);

    {
        let mut file = File::create(&temp_path).map_err(|e| LibraryError::Io {
            path: temp_path.to_string_lossy().into(),
            source: e,
        })?;

        file.write_all(data).map_err(|e| LibraryError::Io {
            path: temp_path.to_string_lossy().into(),
            source: e,
        })?;

        // Flush and fsync
        file.sync_all().map_err(|e| LibraryError::Io {
            path: temp_path.to_string_lossy().into(),
            source: e,
        })?;
    }

    // Preserve one backup if the file already exists
    if target_path.exists() {
        let bak_path = target_path.with_extension("bak");
        let _ = fs::copy(target_path, &bak_path);
    }

    // Atomic rename over target
    fs::rename(&temp_path, target_path).map_err(|e| {
        // Clean up temp file on failure
        let _ = fs::remove_file(&temp_path);
        LibraryError::Io {
            path: target_path.to_string_lossy().into(),
            source: e,
        }
    })?;

    Ok(())
}
