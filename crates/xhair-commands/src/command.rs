//! Typed command declarations for all document and application mutations.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use xhair_model::crosshair::{GlobalTransform, Symmetry};
use xhair_model::effects::Effects;
use xhair_model::layer::{Layer, Transform};
use xhair_model::paint::{Paint, Stroke};
use xhair_model::shapes::LayerKind;
use xhair_model::units::{Alpha, BlendMode, CrosshairId, LayerId};

/// Target scope of a command mutation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CommandScope {
    /// Modifies a crosshair document.
    Document,
    /// Modifies folders, library index, or slots.
    Library,
    /// Controls the physical overlay window.
    Overlay,
    /// Updates persistent application settings.
    Settings,
    /// System and runtime meta control.
    System,
}

/// Runtime mode availability requirement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Availability {
    /// Available in all modes (App, Headless, CLI).
    Everywhere,
    /// Available in App mode and Headless with --overlay flag.
    HeadlessAndApp,
    /// Only available when full GUI App is running.
    AppOnly,
}

/// Fine nudge target selection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum NudgeTarget {
    /// Nudge all layers simultaneously (global offset).
    WholeCrosshair,
    /// Nudge an entire single layer.
    Layer {
        /// Target layer ID.
        id: LayerId,
    },
    /// Nudge a single arm in a symmetric or multi-arm layer.
    Arm {
        /// Target layer ID.
        layer_id: LayerId,
        /// 0-indexed arm identifier.
        arm_index: u32,
    },
}

/// Directional frame axis for nudging.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "snake_case")]
pub enum NudgeFrame {
    /// Screen cardinal axes (world up/down/left/right).
    #[default]
    Screen,
    /// Layer-local axes after rotation is applied.
    Layer,
}

/// Layer field update parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Default)]
pub struct LayerUpdatePayload {
    /// Optional updated layer name.
    pub name: Option<String>,
    /// Optional visibility toggle.
    pub visible: Option<bool>,
    /// Optional edit lock toggle.
    pub locked: Option<bool>,
    /// Optional solo preview toggle.
    pub solo: Option<bool>,
    /// Optional updated shape parameters.
    pub kind: Option<LayerKind>,
    /// Optional updated transform.
    pub transform: Option<Transform>,
    /// Optional fill paint.
    pub fill: Option<Paint>,
    /// Optional stroke outline.
    pub stroke: Option<Option<Stroke>>,
    /// Optional layer opacity.
    pub opacity: Option<Alpha>,
    /// Optional blend mode.
    pub blend: Option<BlendMode>,
    /// Optional layer effects.
    pub effects: Option<Effects>,
}

/// Single executable command.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "command", content = "args", rename_all = "snake_case")]
pub enum Command {
    /// Add a new layer to a crosshair.
    LayerAdd {
        /// Crosshair ID.
        crosshair_id: CrosshairId,
        /// Layer to insert.
        layer: Layer,
    },
    /// Update existing layer properties.
    LayerUpdate {
        /// Crosshair ID.
        crosshair_id: CrosshairId,
        /// Target layer ID.
        layer_id: LayerId,
        /// Updated fields.
        payload: LayerUpdatePayload,
    },
    /// Remove a layer.
    LayerRemove {
        /// Crosshair ID.
        crosshair_id: CrosshairId,
        /// Target layer ID.
        layer_id: LayerId,
    },
    /// Reorder a layer's index in the stack.
    LayerReorder {
        /// Crosshair ID.
        crosshair_id: CrosshairId,
        /// Target layer ID.
        layer_id: LayerId,
        /// New 0-indexed position.
        new_index: usize,
    },
    /// Duplicate a layer.
    LayerDuplicate {
        /// Crosshair ID.
        crosshair_id: CrosshairId,
        /// Target layer ID to duplicate.
        layer_id: LayerId,
    },
    /// Update crosshair global master transform.
    GlobalSet {
        /// Crosshair ID.
        crosshair_id: CrosshairId,
        /// Global transform.
        transform: GlobalTransform,
    },
    /// Update symmetry mode.
    SymmetrySet {
        /// Crosshair ID.
        crosshair_id: CrosshairId,
        /// Symmetry rule.
        symmetry: Symmetry,
    },
    /// Update global composite effects.
    GlobalEffectsSet {
        /// Crosshair ID.
        crosshair_id: CrosshairId,
        /// Effects settings.
        effects: Effects,
    },
    /// Nudge crosshair, layer, or arm in game pixels.
    Nudge {
        /// Crosshair ID.
        crosshair_id: CrosshairId,
        /// Nudge target element.
        target: NudgeTarget,
        /// Horizontal offset delta in game pixels.
        dx: f32,
        /// Vertical offset delta in game pixels.
        dy: f32,
        /// Coordinate frame.
        frame: NudgeFrame,
    },
    /// Atomic batch execution of multiple commands.
    Batch {
        /// List of commands to execute atomically in a single undo step.
        commands: Vec<Command>,
    },
}

impl Command {
    /// Stable command name string.
    pub fn name(&self) -> &'static str {
        match self {
            Self::LayerAdd { .. } => "layer.add",
            Self::LayerUpdate { .. } => "layer.update",
            Self::LayerRemove { .. } => "layer.remove",
            Self::LayerReorder { .. } => "layer.reorder",
            Self::LayerDuplicate { .. } => "layer.duplicate",
            Self::GlobalSet { .. } => "global.set",
            Self::SymmetrySet { .. } => "symmetry.set",
            Self::GlobalEffectsSet { .. } => "effects.set",
            Self::Nudge { .. } => "nudge",
            Self::Batch { .. } => "edit.batch",
        }
    }

    /// Scope of the command.
    pub fn scope(&self) -> CommandScope {
        CommandScope::Document
    }

    /// Runtime availability.
    pub fn availability(&self) -> Availability {
        Availability::Everywhere
    }

    /// Target Crosshair ID affected by this command, if any.
    pub fn crosshair_id(&self) -> Option<CrosshairId> {
        match self {
            Self::LayerAdd { crosshair_id, .. }
            | Self::LayerUpdate { crosshair_id, .. }
            | Self::LayerRemove { crosshair_id, .. }
            | Self::LayerReorder { crosshair_id, .. }
            | Self::LayerDuplicate { crosshair_id, .. }
            | Self::GlobalSet { crosshair_id, .. }
            | Self::SymmetrySet { crosshair_id, .. }
            | Self::GlobalEffectsSet { crosshair_id, .. }
            | Self::Nudge { crosshair_id, .. } => Some(*crosshair_id),
            Self::Batch { commands } => commands.first().and_then(|c| c.crosshair_id()),
        }
    }
}
