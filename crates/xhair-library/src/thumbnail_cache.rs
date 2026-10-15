//! Bounded LRU thumbnail cache for crosshair previews (§10.1).

use std::collections::HashMap;
use xhair_model::units::CrosshairId;

/// Default maximum memory budget for thumbnails (8 MB).
pub const DEFAULT_THUMBNAIL_CACHE_BYTES: usize = 8 * 1024 * 1024;

/// In-memory LRU cache for rendered thumbnail images.
#[derive(Debug)]
pub struct ThumbnailCache {
    max_bytes: usize,
    current_bytes: usize,
    entries: HashMap<CrosshairId, Vec<u8>>,
    access_order: Vec<CrosshairId>,
}

impl Default for ThumbnailCache {
    fn default() -> Self {
        Self::new(DEFAULT_THUMBNAIL_CACHE_BYTES)
    }
}

impl ThumbnailCache {
    /// Creates a new cache with the specified byte capacity limit.
    pub fn new(max_bytes: usize) -> Self {
        Self {
            max_bytes,
            current_bytes: 0,
            entries: HashMap::new(),
            access_order: Vec::new(),
        }
    }

    /// Returns the current total memory consumption in bytes.
    pub fn current_bytes(&self) -> usize {
        self.current_bytes
    }

    /// Returns the maximum allowed memory budget in bytes.
    pub fn max_bytes(&self) -> usize {
        self.max_bytes
    }

    /// Retrieves a cached thumbnail and marks it as most recently used.
    pub fn get(&mut self, id: &CrosshairId) -> Option<&[u8]> {
        if self.entries.contains_key(id) {
            // Update LRU ordering
            if let Some(pos) = self.access_order.iter().position(|x| x == id) {
                let item = self.access_order.remove(pos);
                self.access_order.push(item);
            }
            self.entries.get(id).map(|v| v.as_slice())
        } else {
            None
        }
    }

    /// Stores a thumbnail in the cache, evicting oldest items if exceeding capacity.
    pub fn insert(&mut self, id: CrosshairId, data: Vec<u8>) {
        let entry_size = data.len();

        // If replacing existing entry, subtract previous size
        if let Some(old) = self.entries.remove(&id) {
            self.current_bytes = self.current_bytes.saturating_sub(old.len());
            if let Some(pos) = self.access_order.iter().position(|x| *x == id) {
                self.access_order.remove(pos);
            }
        }

        // Evict until space is available
        while !self.access_order.is_empty() && self.current_bytes + entry_size > self.max_bytes {
            let oldest_id = self.access_order.remove(0);
            if let Some(evicted) = self.entries.remove(&oldest_id) {
                self.current_bytes = self.current_bytes.saturating_sub(evicted.len());
            }
        }

        if entry_size <= self.max_bytes {
            self.current_bytes += entry_size;
            self.entries.insert(id, data);
            self.access_order.push(id);
        }
    }

    /// Removes a cached thumbnail.
    pub fn remove(&mut self, id: &CrosshairId) {
        if let Some(data) = self.entries.remove(id) {
            self.current_bytes = self.current_bytes.saturating_sub(data.len());
            if let Some(pos) = self.access_order.iter().position(|x| x == id) {
                self.access_order.remove(pos);
            }
        }
    }

    /// Clears the entire thumbnail cache.
    pub fn clear(&mut self) {
        self.entries.clear();
        self.access_order.clear();
        self.current_bytes = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_thumbnail_cache_eviction() {
        let mut cache = ThumbnailCache::new(100);

        let id1 = CrosshairId(Uuid::now_v7());
        let id2 = CrosshairId(Uuid::now_v7());
        let id3 = CrosshairId(Uuid::now_v7());

        cache.insert(id1, vec![0u8; 40]);
        cache.insert(id2, vec![0u8; 40]);
        assert_eq!(cache.current_bytes(), 80);

        // Access id1 to make id2 the least recently used
        assert!(cache.get(&id1).is_some());

        // Insert id3 (40 bytes), which exceeds 100 bytes (80 + 40 = 120 > 100)
        // id2 was oldest access, so id2 should be evicted
        cache.insert(id3, vec![0u8; 40]);

        assert!(cache.get(&id1).is_some());
        assert!(cache.get(&id2).is_none());
        assert!(cache.get(&id3).is_some());
        assert_eq!(cache.current_bytes(), 80);
    }
}
