//! Storage, file IO, and library management error types.

use thiserror::Error;

/// Errors produced during library management, disk IO, and atomic persistence.
#[derive(Debug, Error)]
pub enum LibraryError {
    /// Filesystem IO failure.
    #[error("IO error at '{path}': {source}")]
    Io {
        /// Affected path.
        path: String,
        /// Underlying IO error.
        #[source]
        source: std::io::Error,
    },

    /// Format serialization/deserialization failure.
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Crosshair entity not found in library.
    #[error("Crosshair with ID '{0}' not found in library")]
    CrosshairNotFound(String),

    /// Folder entity not found in library.
    #[error("Folder with ID '{0}' not found in library")]
    FolderNotFound(String),

    /// Slot index out of bounds (1..=20).
    #[error("Slot index {index} out of bounds (must be 1 to 20)")]
    SlotOutOfBounds {
        /// Requested index.
        index: usize,
    },

    /// Corrupted index file.
    #[error("Corrupted library index file at '{0}'")]
    CorruptedIndex(String),

    /// Validation error.
    #[error("Library validation error: {0}")]
    Validation(String),
}
