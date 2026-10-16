//! Server runtime context managing the CommandBus, Library, and Overlay state.

use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use xhair_commands::CommandBus;
use xhair_library::LibraryEngine;
use xhair_model::crosshair::Crosshair;
use xhair_model::units::CrosshairId;

/// Single MCP action entry for the live activity log (§13.7).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ActivityEntry {
    /// Timestamp in ISO 8601.
    pub timestamp: String,
    /// Tool or action invoked.
    pub action: String,
    /// Arguments summary string.
    pub summary: String,
    /// Actor who invoked this ("agent").
    pub actor: String,
    /// Result status ("success" or "error: ...").
    pub result: String,
}

/// Trait for dispatching physical overlay actions from MCP tools (§13.3).
pub trait OverlayController: Send + Sync {
    /// Explicitly enable or disable the overlay window.
    fn set_enabled(&self, enabled: bool) -> Result<(), String>;
    /// Update the crosshair rendered on the overlay window.
    fn set_crosshair(&self, crosshair: Box<Crosshair>) -> Result<(), String>;
    /// Toggle overlay visibility.
    fn toggle_visibility(&self) -> Result<(), String>;
    /// Toggle calibration test pattern.
    fn toggle_calibration_pattern(&self) -> Result<(), String>;
    /// Nudge crosshair position or set calibration offset.
    fn nudge(&self, delta: xhair_model::Vec2) -> Result<(), String>;
    /// List connected display monitors with geometry and DPI.
    fn list_monitors(&self) -> Result<Vec<serde_json::Value>, String>;
}

/// Server runtime context shared across stdio and HTTP transports.
#[derive(Clone)]
pub struct ServerContext {
    inner: Arc<Mutex<ServerState>>,
}

/// Inner mutable state.
pub struct ServerState {
    /// Command bus owning documents and undo/redo stacks.
    pub bus: CommandBus,
    /// Library engine for disk catalog and slots.
    pub library: Option<LibraryEngine>,
    /// Optional physical overlay window controller.
    pub overlay_controller: Option<Arc<dyn OverlayController>>,
    /// Currently active crosshair ID.
    pub active_crosshair_id: Option<CrosshairId>,
    /// Overlay visibility state flag.
    pub overlay_enabled: bool,
    /// Overlay visibility mode ("Always", "ForegroundOnly", "Manual").
    pub visibility_mode: String,
    /// Currently active slot 1..=20.
    pub active_slot: Option<u8>,
    /// Calibration offset (x, y) in physical pixels.
    pub calibration_offset: (f32, f32),
    /// Calibration pattern visibility.
    pub calibration_active: bool,
    /// Dynamic reticle opt-in accepted this session.
    pub dynamics_confirmed: bool,
    /// Live activity log entries.
    pub activity_log: Vec<ActivityEntry>,
    /// Permitted directories for file import/export.
    pub allowed_directories: Vec<PathBuf>,
    /// In-memory crosshair groups fallback when no disk library is attached.
    pub in_memory_groups: Vec<xhair_library::Folder>,
}

impl Default for ServerContext {
    fn default() -> Self {
        Self::new(None)
    }
}

impl ServerContext {
    /// Creates a new ServerContext with an optional library engine.
    pub fn new(library: Option<LibraryEngine>) -> Self {
        let mut bus = CommandBus::default();
        let default_ch = Crosshair::new_classic("Cyan Gap");
        let active_id = default_ch.id;
        bus.register_document(default_ch);

        let state = ServerState {
            bus,
            library,
            overlay_controller: None,
            active_crosshair_id: Some(active_id),
            overlay_enabled: true,
            visibility_mode: "ForegroundOnly".to_string(),
            active_slot: Some(1),
            calibration_offset: (0.0, 0.0),
            calibration_active: false,
            dynamics_confirmed: false,
            activity_log: Vec::new(),
            allowed_directories: vec![std::env::temp_dir()],
            in_memory_groups: Vec::new(),
        };

        Self {
            inner: Arc::new(Mutex::new(state)),
        }
    }

    /// Sets an overlay controller for dispatching physical overlay updates.
    pub fn set_overlay_controller(&self, controller: Arc<dyn OverlayController>) {
        self.with_state(|state| {
            state.overlay_controller = Some(controller);
        });
    }

    /// Sync the active crosshair from the CommandBus to the physical overlay.
    pub fn sync_overlay(&self) {
        self.with_state(|state| {
            if let Some(ref controller) = state.overlay_controller {
                if let Some(active_id) = state.active_crosshair_id {
                    if let Some(doc) = state.bus.documents.get(&active_id) {
                        let _ = controller.set_crosshair(Box::new(doc.clone()));
                    }
                }
            }
        });
    }

    /// Execute a closure with mutable access to server state.
    pub fn with_state<R>(&self, f: impl FnOnce(&mut ServerState) -> R) -> R {
        let mut lock = self.inner.lock().expect("ServerContext mutex poisoned");
        f(&mut lock)
    }

    /// Resolves target crosshair ID from arguments or defaults to active crosshair.
    pub fn resolve_crosshair_id(&self, args: &serde_json::Value) -> Result<CrosshairId, String> {
        if let Some(id_str) = args.get("crosshair_id").and_then(|v| v.as_str()) {
            id_str
                .parse::<CrosshairId>()
                .map_err(|e| format!("Invalid crosshair_id: {}", e))
        } else {
            self.with_state(|s| s.active_crosshair_id)
                .ok_or_else(|| "No active crosshair and crosshair_id not provided".to_string())
        }
    }

    /// Logs an activity entry to the live log.
    pub fn log_activity(&self, action: &str, summary: &str, result: &str) {
        self.with_state(|state| {
            let entry = ActivityEntry {
                timestamp: chrono::Utc::now().to_rfc3339(),
                action: action.to_string(),
                summary: summary.to_string(),
                actor: "agent".to_string(),
                result: result.to_string(),
            };
            state.activity_log.push(entry);
            if state.activity_log.len() > 100 {
                state.activity_log.remove(0);
            }
        });
    }
}
