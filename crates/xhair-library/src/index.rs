//! Lightweight library catalog index (folders, metadata summaries, slots, and trash).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use xhair_model::crosshair::Crosshair;
use xhair_model::units::CrosshairId;

/// Number of quick-selection slots (§10.3).
pub const MAX_SLOTS: usize = 20;

/// Organizational folder/group entity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Folder {
    /// Unique folder UUID.
    pub id: String,
    /// Human-facing display name.
    pub name: String,
    /// Parent folder UUID for nested hierarchies.
    #[serde(default)]
    pub parent_id: Option<String>,
    /// Optional icon or symbol name.
    #[serde(default)]
    pub icon: Option<String>,
    /// Optional color dot hex code (e.g. "#EF4444").
    #[serde(default)]
    pub color: Option<String>,
    /// Collapsed toggle in library tree.
    #[serde(default)]
    pub collapsed: bool,
    /// Display sort order.
    #[serde(default)]
    pub order: usize,
    /// Bound game executable process name (e.g. "VALORANT-Win64-Shipping.exe").
    #[serde(default)]
    pub game_binding: Option<String>,
    /// Active crosshair ID within this group.
    #[serde(default)]
    pub active_crosshair_id: Option<CrosshairId>,
    /// Default resolution profile name for this group.
    #[serde(default)]
    pub default_resolution_profile: Option<String>,
}

/// Lightweight metadata summary of an on-disk crosshair document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CrosshairSummary {
    /// Crosshair unique ID.
    pub id: CrosshairId,
    /// Display name.
    pub name: String,
    /// Assigned folder ID.
    pub folder_id: Option<String>,
    /// Tags for filtering.
    pub tags: Vec<String>,
    /// Quick-access favorite flag.
    pub favorite: bool,
    /// Creation timestamp (UTC).
    pub created_at: DateTime<Utc>,
    /// Last modification timestamp (UTC).
    pub updated_at: DateTime<Utc>,
    /// Relative filename under `crosshairs/` directory.
    pub file_name: String,
}

impl CrosshairSummary {
    /// Extracts a metadata summary from a full Crosshair document.
    pub fn from_crosshair(crosshair: &Crosshair, folder_id: Option<String>) -> Self {
        Self {
            id: crosshair.id,
            name: crosshair.name.clone(),
            folder_id,
            tags: crosshair.tags.clone(),
            favorite: crosshair.favorite,
            created_at: crosshair.created_at,
            updated_at: crosshair.updated_at,
            file_name: format!("{}.xhair", crosshair.id.0),
        }
    }
}

/// Metadata for a crosshair in the recycle bin.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrashEntry {
    /// Crosshair ID.
    pub id: CrosshairId,
    /// Original name.
    pub name: String,
    /// Deletion timestamp.
    pub deleted_at: DateTime<Utc>,
    /// Filename under `trash/`.
    pub file_name: String,
}

/// Root index catalog saved to `library/index.json`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LibraryIndex {
    /// Index schema version.
    pub version: u32,
    /// Defined folder hierarchy.
    pub folders: Vec<Folder>,
    /// Crosshair summaries.
    pub crosshairs: Vec<CrosshairSummary>,
    /// Quick-select slots 1 to 20.
    pub slots: [Option<CrosshairId>; MAX_SLOTS],
    /// Soft-deleted crosshairs pending expiration.
    pub trash: Vec<TrashEntry>,
}

impl Default for LibraryIndex {
    fn default() -> Self {
        Self {
            version: 1,
            folders: Vec::new(),
            crosshairs: Vec::new(),
            slots: [None; MAX_SLOTS],
            trash: Vec::new(),
        }
    }
}
