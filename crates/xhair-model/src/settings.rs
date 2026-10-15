//! Application configuration and persistent user preferences.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// UI Theme appearance mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "snake_case")]
pub enum ThemeMode {
    /// Dark pro tool aesthetic (default).
    #[default]
    Dark,
    /// Light theme.
    Light,
    /// High-contrast accessible theme.
    HighContrast,
}

/// Overlay visibility trigger rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "snake_case")]
pub enum VisibilityMode {
    /// Always visible on top of all windows.
    Always,
    /// Automatically show only when target game process is in foreground.
    #[default]
    OnlyWhenForeground,
    /// Manually toggled via hotkey or MCP tool.
    Manual,
}

/// General application preferences.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct GeneralSettings {
    /// Start minimized to system tray.
    pub start_minimized: bool,
    /// Launch on Windows user login (via Run registry key).
    pub start_with_windows: bool,
    /// UI theme mode.
    pub theme: ThemeMode,
    /// UI scaling factor (0.8 to 1.5, default 1.0).
    pub ui_scale: f32,
    /// Respect system reduced-motion preferences.
    pub reduce_motion: bool,
}

impl Default for GeneralSettings {
    fn default() -> Self {
        Self {
            start_minimized: false,
            start_with_windows: false,
            theme: ThemeMode::Dark,
            ui_scale: 1.0,
            reduce_motion: false,
        }
    }
}

/// Overlay window and rendering preferences.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct OverlaySettings {
    /// Overlay visibility triggering behavior.
    pub visibility_mode: VisibilityMode,
    /// List of target game process executable image names.
    pub game_processes: Vec<String>,
    /// Global lock disabling all procedural animation and dynamic recoil.
    pub static_only_lock: bool,
    /// Exclude overlay window from screen capture and streaming (WDA_EXCLUDEFROMCAPTURE).
    pub exclude_from_capture: bool,
    /// Track focused game window client bounds when not running fullscreen borderless.
    pub follow_game_window: bool,
    /// Maximum rendering frame rate when animating (default 30, maximum 120).
    pub max_fps: u32,
}

impl Default for OverlaySettings {
    fn default() -> Self {
        Self {
            visibility_mode: VisibilityMode::OnlyWhenForeground,
            game_processes: vec!["VALORANT-Win64-Shipping.exe".to_string()],
            static_only_lock: false,
            exclude_from_capture: false,
            follow_game_window: false,
            max_fps: 30,
        }
    }
}

/// Hotkey binding definition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct HotkeyBinding {
    /// Action identifier (e.g. "overlay.toggle", "slot.1").
    pub action: String,
    /// Key combination string (e.g. "Ctrl+Shift+X").
    pub combination: String,
    /// Whether hotkey is currently active.
    pub enabled: bool,
}

/// Library storage and caching preferences.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct LibrarySettings {
    /// Custom data directory override (null uses default %APPDATA%/xhair).
    pub data_directory: Option<String>,
    /// Autosave interval in seconds after modifications.
    pub autosave_interval_seconds: u32,
    /// Retention period for deleted items in trash before purge.
    pub trash_retention_days: u32,
    /// Maximum in-memory LRU thumbnail cache in megabytes.
    pub max_thumbnail_cache_mb: u32,
}

impl Default for LibrarySettings {
    fn default() -> Self {
        Self {
            data_directory: None,
            autosave_interval_seconds: 2,
            trash_retention_days: 30,
            max_thumbnail_cache_mb: 8,
        }
    }
}

/// Model Context Protocol (MCP) server settings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct McpSettings {
    /// Enable loopback HTTP transport.
    pub enable_http: bool,
    /// HTTP port (0 = auto-assign free port).
    pub port: u16,
    /// 256-bit bearer authentication token.
    pub bearer_token: String,
    /// Allowed import/export filesystem directories.
    pub allowed_directories: Vec<String>,
    /// Require interactive human confirmation before destructive tool execution.
    pub ask_before_delete: bool,
    /// Maximum entries retained in the live MCP activity log.
    pub activity_log_capacity: usize,
}

impl Default for McpSettings {
    fn default() -> Self {
        Self {
            enable_http: false,
            port: 0,
            bearer_token: String::new(),
            allowed_directories: Vec::new(),
            ask_before_delete: true,
            activity_log_capacity: 100,
        }
    }
}

/// Dynamic Reticle opt-in state and safety configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Default)]
pub struct DynamicsSettings {
    /// Session opt-in confirmation flag (must be explicitly granted each session).
    pub opt_in_confirmed: bool,
    /// Bound input key identifiers allowed for Raw Input listening.
    pub bound_inputs: Vec<String>,
}

/// Complete application settings bundle.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Default)]
pub struct Settings {
    /// General application settings.
    pub general: GeneralSettings,
    /// Overlay rendering and visibility settings.
    pub overlay: OverlaySettings,
    /// Global hotkey bindings.
    pub hotkeys: Vec<HotkeyBinding>,
    /// Library and filesystem settings.
    pub library: LibrarySettings,
    /// MCP server integration settings.
    pub mcp: McpSettings,
    /// Dynamic reticle safety preferences.
    pub dynamics: DynamicsSettings,
}
