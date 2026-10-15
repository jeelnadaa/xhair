//! Structured JSON Patch (RFC 6902) style diffs emitted by command executions.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Atomic JSON Patch operation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum PatchOp {
    /// Add or insert value at target path.
    Add {
        /// Target path (e.g. "/layers/0").
        path: String,
        /// Value payload.
        value: serde_json::Value,
    },
    /// Remove item at target path.
    Remove {
        /// Target path.
        path: String,
    },
    /// Replace value at target path.
    Replace {
        /// Target path.
        path: String,
        /// New replacement value.
        value: serde_json::Value,
    },
}

/// Structured diff result returned from executing a command.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Default)]
pub struct CommandDiff {
    /// Ordered list of patch operations.
    pub patches: Vec<PatchOp>,
    /// Summary human-readable description of change.
    pub description: String,
}

impl CommandDiff {
    /// Creates a new empty diff.
    pub fn new(description: impl Into<String>) -> Self {
        Self {
            patches: Vec::new(),
            description: description.into(),
        }
    }

    /// Appends a patch operation to this diff.
    pub fn add_patch(&mut self, op: PatchOp) {
        self.patches.push(op);
    }
}
