//! Disk persistence, slots, tagging, and folder tree management for xhair (§10).

#![forbid(unsafe_code)]
#![deny(missing_docs)]

pub mod atomic;
pub mod engine;
pub mod error;
pub mod index;
pub mod thumbnail_cache;

pub use atomic::atomic_write_file;
pub use engine::{LibraryEngine, SortBy};
pub use error::LibraryError;
pub use index::{CrosshairSummary, Folder, LibraryIndex, TrashEntry, MAX_SLOTS};
pub use thumbnail_cache::{ThumbnailCache, DEFAULT_THUMBNAIL_CACHE_BYTES};
