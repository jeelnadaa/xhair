//! Core library storage and catalog management engine (§10).

use crate::atomic::atomic_write_file;
use crate::error::LibraryError;
use crate::index::{CrosshairSummary, Folder, LibraryIndex, TrashEntry, MAX_SLOTS};
use chrono::Utc;
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;
use xhair_model::crosshair::Crosshair;
use xhair_model::units::CrosshairId;

/// Sorting criteria for crosshair summaries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SortBy {
    /// Most recently updated first.
    #[default]
    Recent,
    /// Alphabetical ascending by name.
    Name,
    /// Chronological descending by creation time (newest first).
    Created,
}

/// Disk-backed library storage engine.
#[derive(Debug)]
pub struct LibraryEngine {
    root_dir: PathBuf,
    index: LibraryIndex,
}

impl LibraryEngine {
    /// Resolves the default data directory path based on portable mode or OS conventions.
    pub fn resolve_data_dir() -> PathBuf {
        // Portable mode: check if `xhair.portable` marker exists beside current executable
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(parent) = exe_path.parent() {
                if parent.join("xhair.portable").exists() {
                    return parent.join("data");
                }
            }
        }

        // Windows standard AppData
        #[cfg(target_os = "windows")]
        if let Ok(appdata) = std::env::var("APPDATA") {
            return PathBuf::from(appdata).join("xhair");
        }

        // Unix / fallback home directory
        if let Ok(home) = std::env::var("HOME") {
            return PathBuf::from(home)
                .join(".local")
                .join("share")
                .join("xhair");
        }

        PathBuf::from("./xhair_data")
    }

    /// Initializes or loads a library engine at the given root directory.
    pub fn new(root_dir: impl Into<PathBuf>) -> Result<Self, LibraryError> {
        let root_dir = root_dir.into();
        let engine = Self {
            root_dir,
            index: LibraryIndex::default(),
        };

        // Ensure directory structure exists
        fs::create_dir_all(engine.crosshairs_dir()).map_err(|e| LibraryError::Io {
            path: engine.crosshairs_dir().to_string_lossy().into(),
            source: e,
        })?;
        fs::create_dir_all(engine.trash_dir()).map_err(|e| LibraryError::Io {
            path: engine.trash_dir().to_string_lossy().into(),
            source: e,
        })?;
        fs::create_dir_all(engine.assets_dir()).map_err(|e| LibraryError::Io {
            path: engine.assets_dir().to_string_lossy().into(),
            source: e,
        })?;
        fs::create_dir_all(engine.profiles_dir()).map_err(|e| LibraryError::Io {
            path: engine.profiles_dir().to_string_lossy().into(),
            source: e,
        })?;
        fs::create_dir_all(engine.backgrounds_dir()).map_err(|e| LibraryError::Io {
            path: engine.backgrounds_dir().to_string_lossy().into(),
            source: e,
        })?;
        fs::create_dir_all(engine.logs_dir()).map_err(|e| LibraryError::Io {
            path: engine.logs_dir().to_string_lossy().into(),
            source: e,
        })?;

        let mut engine = engine;
        engine.load_or_init_index()?;
        Ok(engine)
    }

    /// Root storage directory path.
    pub fn root_dir(&self) -> &Path {
        &self.root_dir
    }

    /// `library/` directory path.
    pub fn library_dir(&self) -> PathBuf {
        self.root_dir.join("library")
    }

    /// `library/crosshairs/` directory path.
    pub fn crosshairs_dir(&self) -> PathBuf {
        self.library_dir().join("crosshairs")
    }

    /// `library/trash/` directory path.
    pub fn trash_dir(&self) -> PathBuf {
        self.library_dir().join("trash")
    }

    /// `library/assets/` directory path.
    pub fn assets_dir(&self) -> PathBuf {
        self.library_dir().join("assets")
    }

    /// `library/index.json` path.
    pub fn index_path(&self) -> PathBuf {
        self.library_dir().join("index.json")
    }

    /// `profiles/` directory path.
    pub fn profiles_dir(&self) -> PathBuf {
        self.root_dir.join("profiles")
    }

    /// `backgrounds/` directory path.
    pub fn backgrounds_dir(&self) -> PathBuf {
        self.root_dir.join("backgrounds")
    }

    /// `logs/` directory path.
    pub fn logs_dir(&self) -> PathBuf {
        self.root_dir.join("logs")
    }

    /// Current in-memory catalog index.
    pub fn index(&self) -> &LibraryIndex {
        &self.index
    }

    /// Loads index from disk or initializes a default index if not present.
    fn load_or_init_index(&mut self) -> Result<(), LibraryError> {
        let index_path = self.index_path();
        if index_path.exists() {
            match fs::read_to_string(&index_path) {
                Ok(content) => match serde_json::from_str::<LibraryIndex>(&content) {
                    Ok(parsed) => {
                        self.index = parsed;
                        return Ok(());
                    }
                    Err(e) => {
                        // Check if backup exists
                        let bak_path = index_path.with_extension("bak");
                        if bak_path.exists() {
                            if let Ok(bak_content) = fs::read_to_string(&bak_path) {
                                if let Ok(bak_parsed) =
                                    serde_json::from_str::<LibraryIndex>(&bak_content)
                                {
                                    self.index = bak_parsed;
                                    let _ = self.save_index();
                                    return Ok(());
                                }
                            }
                        }
                        return Err(LibraryError::CorruptedIndex(format!(
                            "Failed to parse index at {}: {}",
                            index_path.to_string_lossy(),
                            e
                        )));
                    }
                },
                Err(e) => {
                    return Err(LibraryError::Io {
                        path: index_path.to_string_lossy().into(),
                        source: e,
                    })
                }
            }
        }

        // Initialize default empty index
        self.index = LibraryIndex::default();
        self.save_index()?;
        Ok(())
    }

    /// Reloads the catalog index from disk.
    pub fn reload_index(&mut self) -> Result<(), LibraryError> {
        self.load_or_init_index()
    }

    /// Atomically persists the in-memory catalog index to disk.
    pub fn save_index(&self) -> Result<(), LibraryError> {
        let data = serde_json::to_vec_pretty(&self.index)
            .map_err(|e| LibraryError::Serialization(e.to_string()))?;
        atomic_write_file(&self.index_path(), &data)
    }

    // ─── Crosshair Persistence ───────────────────────────────────────────────

    /// Saves a crosshair to disk atomically and updates the catalog index.
    pub fn save_crosshair(
        &mut self,
        crosshair: &Crosshair,
        folder_id: Option<String>,
    ) -> Result<(), LibraryError> {
        // Validate folder if specified
        if let Some(ref fid) = folder_id {
            if !self.index.folders.iter().any(|f| f.id == *fid) {
                return Err(LibraryError::FolderNotFound(fid.clone()));
            }
        }

        let file_path = self
            .crosshairs_dir()
            .join(format!("{}.xhair", crosshair.id.0));
        let data = serde_json::to_vec_pretty(crosshair)
            .map_err(|e| LibraryError::Serialization(e.to_string()))?;

        atomic_write_file(&file_path, &data)?;

        // Update or insert summary
        let summary = CrosshairSummary::from_crosshair(crosshair, folder_id);
        if let Some(pos) = self
            .index
            .crosshairs
            .iter()
            .position(|c| c.id == crosshair.id)
        {
            self.index.crosshairs[pos] = summary;
        } else {
            self.index.crosshairs.push(summary);
        }

        self.save_index()?;
        Ok(())
    }

    /// Loads a full crosshair document from disk on demand (lazy loading).
    pub fn load_crosshair(&self, id: CrosshairId) -> Result<Crosshair, LibraryError> {
        let file_path = self.crosshairs_dir().join(format!("{}.xhair", id.0));
        if !file_path.exists() {
            return Err(LibraryError::CrosshairNotFound(id.0.to_string()));
        }

        let content = fs::read(&file_path).map_err(|e| LibraryError::Io {
            path: file_path.to_string_lossy().into(),
            source: e,
        })?;

        let crosshair = serde_json::from_slice::<Crosshair>(&content)
            .map_err(|e| LibraryError::Serialization(e.to_string()))?;

        Ok(crosshair)
    }

    /// Soft-deletes a crosshair: moves its file to `trash/` and updates the index.
    pub fn delete_crosshair(&mut self, id: CrosshairId) -> Result<(), LibraryError> {
        let summary_pos = self
            .index
            .crosshairs
            .iter()
            .position(|c| c.id == id)
            .ok_or_else(|| LibraryError::CrosshairNotFound(id.0.to_string()))?;

        let summary = self.index.crosshairs.remove(summary_pos);
        let src = self.crosshairs_dir().join(&summary.file_name);
        let dst = self.trash_dir().join(&summary.file_name);

        if src.exists() {
            fs::rename(&src, &dst).map_err(|e| LibraryError::Io {
                path: dst.to_string_lossy().into(),
                source: e,
            })?;
        }

        // Clear from any active quick-select slots
        for slot in self.index.slots.iter_mut() {
            if *slot == Some(id) {
                *slot = None;
            }
        }

        // Add to trash list
        self.index.trash.push(TrashEntry {
            id,
            name: summary.name,
            deleted_at: Utc::now(),
            file_name: summary.file_name,
        });

        self.save_index()?;
        Ok(())
    }

    /// Restores a soft-deleted crosshair from `trash/` back into `crosshairs/`.
    pub fn restore_crosshair(&mut self, id: CrosshairId) -> Result<(), LibraryError> {
        let trash_pos = self
            .index
            .trash
            .iter()
            .position(|t| t.id == id)
            .ok_or_else(|| LibraryError::CrosshairNotFound(id.0.to_string()))?;

        let entry = self.index.trash.remove(trash_pos);
        let src = self.trash_dir().join(&entry.file_name);
        let dst = self.crosshairs_dir().join(&entry.file_name);

        if src.exists() {
            fs::rename(&src, &dst).map_err(|e| LibraryError::Io {
                path: dst.to_string_lossy().into(),
                source: e,
            })?;
        }

        // Reload crosshair to reconstruct summary
        let crosshair = self.load_crosshair(id)?;
        let summary = CrosshairSummary::from_crosshair(&crosshair, None);
        self.index.crosshairs.push(summary);

        self.save_index()?;
        Ok(())
    }

    /// Permanently deletes trash items older than `max_days` days (§10.2).
    pub fn purge_expired_trash(&mut self, max_days: i64) -> Result<usize, LibraryError> {
        let now = Utc::now();
        let mut purged_count = 0;
        let mut remaining = Vec::new();
        let trash_dir = self.trash_dir();

        for entry in self.index.trash.drain(..) {
            let age_days = (now - entry.deleted_at).num_days();
            if age_days >= max_days {
                let file_path = trash_dir.join(&entry.file_name);
                if file_path.exists() {
                    let _ = fs::remove_file(file_path);
                }
                purged_count += 1;
            } else {
                remaining.push(entry);
            }
        }

        self.index.trash = remaining;
        if purged_count > 0 {
            self.save_index()?;
        }
        Ok(purged_count)
    }

    /// Permanently deletes all items in the trash.
    pub fn empty_trash(&mut self) -> Result<usize, LibraryError> {
        let count = self.index.trash.len();
        let trash_dir = self.trash_dir();
        for entry in self.index.trash.drain(..) {
            let file_path = trash_dir.join(&entry.file_name);
            if file_path.exists() {
                let _ = fs::remove_file(file_path);
            }
        }
        self.save_index()?;
        Ok(count)
    }

    /// Duplicates an existing crosshair with a new unique ID and optional name override.
    pub fn duplicate_crosshair(
        &mut self,
        source_id: CrosshairId,
        new_name: Option<String>,
    ) -> Result<Crosshair, LibraryError> {
        let mut source = self.load_crosshair(source_id)?;
        let folder_id = self
            .index
            .crosshairs
            .iter()
            .find(|c| c.id == source_id)
            .and_then(|c| c.folder_id.clone());

        let new_id = CrosshairId(Uuid::now_v7());
        let now = Utc::now();
        source.id = new_id;
        source.name = new_name.unwrap_or_else(|| format!("{} (Copy)", source.name));
        source.created_at = now;
        source.updated_at = now;

        self.save_crosshair(&source, folder_id)?;
        Ok(source)
    }

    // ─── Folders Management ──────────────────────────────────────────────────

    /// Creates a new organizational folder.
    pub fn create_folder(
        &mut self,
        name: &str,
        parent_id: Option<String>,
    ) -> Result<Folder, LibraryError> {
        if let Some(ref pid) = parent_id {
            if !self.index.folders.iter().any(|f| f.id == *pid) {
                return Err(LibraryError::FolderNotFound(pid.clone()));
            }
        }

        let folder = Folder {
            id: Uuid::now_v7().to_string(),
            name: name.to_string(),
            parent_id,
        };

        self.index.folders.push(folder.clone());
        self.save_index()?;
        Ok(folder)
    }

    /// Renames an existing folder.
    pub fn rename_folder(&mut self, id: &str, new_name: &str) -> Result<(), LibraryError> {
        let folder = self
            .index
            .folders
            .iter_mut()
            .find(|f| f.id == id)
            .ok_or_else(|| LibraryError::FolderNotFound(id.to_string()))?;

        folder.name = new_name.to_string();
        self.save_index()?;
        Ok(())
    }

    /// Deletes a folder. If `delete_contents` is false, child crosshairs and folders
    /// are unassigned (moved to root). If true, crosshairs are moved to trash.
    pub fn delete_folder(&mut self, id: &str, delete_contents: bool) -> Result<(), LibraryError> {
        let folder_pos = self
            .index
            .folders
            .iter()
            .position(|f| f.id == id)
            .ok_or_else(|| LibraryError::FolderNotFound(id.to_string()))?;

        self.index.folders.remove(folder_pos);

        if delete_contents {
            let to_delete: Vec<CrosshairId> = self
                .index
                .crosshairs
                .iter()
                .filter(|c| c.folder_id.as_deref() == Some(id))
                .map(|c| c.id)
                .collect();

            for ch_id in to_delete {
                self.delete_crosshair(ch_id)?;
            }
        } else {
            // Unassign crosshairs
            for ch in self.index.crosshairs.iter_mut() {
                if ch.folder_id.as_deref() == Some(id) {
                    ch.folder_id = None;
                }
            }
            // Reparent child folders
            for f in self.index.folders.iter_mut() {
                if f.parent_id.as_deref() == Some(id) {
                    f.parent_id = None;
                }
            }
        }

        self.save_index()?;
        Ok(())
    }

    /// Moves a crosshair into a different folder (or to root if `folder_id` is None).
    pub fn move_crosshair_to_folder(
        &mut self,
        crosshair_id: CrosshairId,
        folder_id: Option<String>,
    ) -> Result<(), LibraryError> {
        if let Some(ref fid) = folder_id {
            if !self.index.folders.iter().any(|f| f.id == *fid) {
                return Err(LibraryError::FolderNotFound(fid.clone()));
            }
        }

        let summary = self
            .index
            .crosshairs
            .iter_mut()
            .find(|c| c.id == crosshair_id)
            .ok_or_else(|| LibraryError::CrosshairNotFound(crosshair_id.0.to_string()))?;

        summary.folder_id = folder_id;
        self.save_index()?;
        Ok(())
    }

    // ─── Tags & Favorites ────────────────────────────────────────────────────

    /// Toggles the favorite status of a crosshair.
    pub fn toggle_favorite(&mut self, id: CrosshairId) -> Result<bool, LibraryError> {
        let new_state = {
            let summary = self
                .index
                .crosshairs
                .iter_mut()
                .find(|c| c.id == id)
                .ok_or_else(|| LibraryError::CrosshairNotFound(id.0.to_string()))?;

            summary.favorite = !summary.favorite;
            summary.favorite
        };

        // Also update full crosshair file
        if let Ok(mut crosshair) = self.load_crosshair(id) {
            crosshair.favorite = new_state;
            let file_path = self.crosshairs_dir().join(format!("{}.xhair", id.0));
            if let Ok(data) = serde_json::to_vec_pretty(&crosshair) {
                let _ = atomic_write_file(&file_path, &data);
            }
        }

        self.save_index()?;
        Ok(new_state)
    }

    /// Adds a tag to a crosshair.
    pub fn add_tag(&mut self, id: CrosshairId, tag: &str) -> Result<(), LibraryError> {
        let tag = tag.trim().to_string();
        if tag.is_empty() {
            return Ok(());
        }

        let summary = self
            .index
            .crosshairs
            .iter_mut()
            .find(|c| c.id == id)
            .ok_or_else(|| LibraryError::CrosshairNotFound(id.0.to_string()))?;

        if !summary.tags.contains(&tag) {
            summary.tags.push(tag.clone());
        }

        if let Ok(mut crosshair) = self.load_crosshair(id) {
            if !crosshair.tags.contains(&tag) {
                crosshair.tags.push(tag);
                let file_path = self.crosshairs_dir().join(format!("{}.xhair", id.0));
                if let Ok(data) = serde_json::to_vec_pretty(&crosshair) {
                    let _ = atomic_write_file(&file_path, &data);
                }
            }
        }

        self.save_index()?;
        Ok(())
    }

    /// Removes a tag from a crosshair.
    pub fn remove_tag(&mut self, id: CrosshairId, tag: &str) -> Result<(), LibraryError> {
        let summary = self
            .index
            .crosshairs
            .iter_mut()
            .find(|c| c.id == id)
            .ok_or_else(|| LibraryError::CrosshairNotFound(id.0.to_string()))?;

        summary.tags.retain(|t| t != tag);

        if let Ok(mut crosshair) = self.load_crosshair(id) {
            crosshair.tags.retain(|t| t != tag);
            let file_path = self.crosshairs_dir().join(format!("{}.xhair", id.0));
            if let Ok(data) = serde_json::to_vec_pretty(&crosshair) {
                let _ = atomic_write_file(&file_path, &data);
            }
        }

        self.save_index()?;
        Ok(())
    }

    // ─── Slot Management (§10.3) ─────────────────────────────────────────────

    /// Assigns a crosshair (or clears) quick-select slot 1 to 20 (1-indexed).
    pub fn set_slot(
        &mut self,
        slot_1_based: usize,
        crosshair_id: Option<CrosshairId>,
    ) -> Result<(), LibraryError> {
        if slot_1_based == 0 || slot_1_based > MAX_SLOTS {
            return Err(LibraryError::SlotOutOfBounds {
                index: slot_1_based,
            });
        }

        if let Some(id) = crosshair_id {
            if !self.index.crosshairs.iter().any(|c| c.id == id) {
                return Err(LibraryError::CrosshairNotFound(id.0.to_string()));
            }
        }

        self.index.slots[slot_1_based - 1] = crosshair_id;
        self.save_index()?;
        Ok(())
    }

    /// Gets the crosshair ID assigned to quick-select slot 1 to 20 (1-indexed).
    pub fn get_slot(&self, slot_1_based: usize) -> Result<Option<CrosshairId>, LibraryError> {
        if slot_1_based == 0 || slot_1_based > MAX_SLOTS {
            return Err(LibraryError::SlotOutOfBounds {
                index: slot_1_based,
            });
        }
        Ok(self.index.slots[slot_1_based - 1])
    }

    /// Returns an immutable reference to the full slot mapping array.
    pub fn slots(&self) -> &[Option<CrosshairId>; MAX_SLOTS] {
        &self.index.slots
    }

    // ─── Search, Filter, Sort ────────────────────────────────────────────────

    /// Searches crosshairs by query text (case-insensitive substring match against name and tags).
    pub fn search(&self, query: &str) -> Vec<&CrosshairSummary> {
        let q = query.trim().to_lowercase();
        if q.is_empty() {
            return self.index.crosshairs.iter().collect();
        }

        self.index
            .crosshairs
            .iter()
            .filter(|c| {
                c.name.to_lowercase().contains(&q)
                    || c.tags.iter().any(|t| t.to_lowercase().contains(&q))
            })
            .collect()
    }

    /// Filters and sorts crosshairs based on folder, tag, favorite, and sort order.
    pub fn filter(
        &self,
        folder_id: Option<&str>,
        tag: Option<&str>,
        favorite_only: bool,
        sort_by: SortBy,
    ) -> Vec<&CrosshairSummary> {
        let mut results: Vec<&CrosshairSummary> = self
            .index
            .crosshairs
            .iter()
            .filter(|c| {
                if let Some(fid) = folder_id {
                    if c.folder_id.as_deref() != Some(fid) {
                        return false;
                    }
                }
                if let Some(t) = tag {
                    if !c.tags.iter().any(|item| item.eq_ignore_ascii_case(t)) {
                        return false;
                    }
                }
                if favorite_only && !c.favorite {
                    return false;
                }
                true
            })
            .collect();

        match sort_by {
            SortBy::Recent => results.sort_by_key(|a| std::cmp::Reverse(a.updated_at)),
            SortBy::Name => results.sort_by_key(|a| a.name.to_lowercase()),
            SortBy::Created => results.sort_by_key(|a| std::cmp::Reverse(a.created_at)),
        }

        results
    }

    // ─── Bulk Operations ─────────────────────────────────────────────────────

    /// Bulk moves multiple crosshairs to a target folder.
    pub fn bulk_move(
        &mut self,
        ids: &[CrosshairId],
        folder_id: Option<String>,
    ) -> Result<(), LibraryError> {
        for &id in ids {
            self.move_crosshair_to_folder(id, folder_id.clone())?;
        }
        Ok(())
    }

    /// Bulk soft-deletes multiple crosshairs to trash.
    pub fn bulk_delete(&mut self, ids: &[CrosshairId]) -> Result<(), LibraryError> {
        for &id in ids {
            let _ = self.delete_crosshair(id);
        }
        Ok(())
    }

    /// Bulk updates tags across multiple crosshairs.
    pub fn bulk_tag(
        &mut self,
        ids: &[CrosshairId],
        add_tags: &[String],
        remove_tags: &[String],
    ) -> Result<(), LibraryError> {
        for &id in ids {
            for tag in add_tags {
                let _ = self.add_tag(id, tag);
            }
            for tag in remove_tags {
                let _ = self.remove_tag(id, tag);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_engine_init_and_crud() {
        let dir = tempdir().unwrap();
        let mut engine = LibraryEngine::new(dir.path()).unwrap();

        let xhair = Crosshair::new_classic("Test Reticle");
        let id = xhair.id;

        // Save
        engine.save_crosshair(&xhair, None).unwrap();
        assert_eq!(engine.index().crosshairs.len(), 1);
        assert_eq!(engine.index().crosshairs[0].id, id);

        // Load
        let loaded = engine.load_crosshair(id).unwrap();
        assert_eq!(loaded.name, "Test Reticle");

        // Duplicate
        let dup = engine.duplicate_crosshair(id, None).unwrap();
        assert_eq!(dup.name, "Test Reticle (Copy)");
        assert_eq!(engine.index().crosshairs.len(), 2);

        // Delete (moves to trash)
        engine.delete_crosshair(id).unwrap();
        assert_eq!(engine.index().crosshairs.len(), 1);
        assert_eq!(engine.index().trash.len(), 1);

        // Restore
        engine.restore_crosshair(id).unwrap();
        assert_eq!(engine.index().crosshairs.len(), 2);
        assert_eq!(engine.index().trash.len(), 0);
    }

    #[test]
    fn test_folder_lifecycle() {
        let dir = tempdir().unwrap();
        let mut engine = LibraryEngine::new(dir.path()).unwrap();

        let folder = engine.create_folder("Rifles", None).unwrap();
        let fid = folder.id.clone();

        let xhair = Crosshair::new_classic("Vandal");
        engine.save_crosshair(&xhair, Some(fid.clone())).unwrap();

        assert_eq!(
            engine.index().crosshairs[0].folder_id.as_deref(),
            Some(fid.as_str())
        );

        // Filter by folder
        let filtered = engine.filter(Some(&fid), None, false, SortBy::Recent);
        assert_eq!(filtered.len(), 1);

        // Delete folder without deleting contents
        engine.delete_folder(&fid, false).unwrap();
        assert_eq!(engine.index().crosshairs[0].folder_id, None);
    }

    #[test]
    fn test_slot_assignment() {
        let dir = tempdir().unwrap();
        let mut engine = LibraryEngine::new(dir.path()).unwrap();

        let xhair = Crosshair::new_classic("Slot Reticle");
        let id = xhair.id;
        engine.save_crosshair(&xhair, None).unwrap();

        engine.set_slot(1, Some(id)).unwrap();
        assert_eq!(engine.get_slot(1).unwrap(), Some(id));

        // Invalid slot
        assert!(engine.set_slot(0, Some(id)).is_err());
        assert!(engine.set_slot(21, Some(id)).is_err());

        // Deleting crosshair clears slot
        engine.delete_crosshair(id).unwrap();
        assert_eq!(engine.get_slot(1).unwrap(), None);
    }
}
