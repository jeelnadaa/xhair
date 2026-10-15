//! Attribution actor identifying who or what initiated a state mutation.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Originating actor of a command.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Actor {
    /// Human user interacting with the editor UI or tray.
    #[default]
    Human,
    /// Automated AI agent invoking an MCP tool.
    Agent {
        /// Optional client or agent name identifier.
        client_name: Option<String>,
    },
    /// Global hotkey trigger pressed mid-game.
    Hotkey {
        /// Key combination string.
        combination: String,
    },
    /// Internal application system automation or migration.
    System,
}
