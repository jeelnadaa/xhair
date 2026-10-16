//! The central Command Bus: single source of truth for all state mutations.

use crate::actor::Actor;
use crate::command::{Command, LayerUpdatePayload, NudgeTarget};
use crate::diff::{CommandDiff, PatchOp};
use crate::history::HistoryStack;
use chrono::Utc;
use std::collections::HashMap;
use xhair_model::crosshair::Crosshair;
use xhair_model::error::XhairError;
use xhair_model::units::CrosshairId;

/// Event emitted to subscribers upon state mutation.
#[derive(Debug, Clone, PartialEq)]
pub enum BusEvent {
    /// Crosshair document contents modified.
    DocumentChanged {
        /// Modified crosshair ID.
        crosshair_id: CrosshairId,
        /// Applied diff.
        diff: CommandDiff,
        /// Actor who made the change.
        actor: Actor,
    },
    /// Active crosshair switched.
    ActiveCrosshairChanged {
        /// New active crosshair ID.
        crosshair_id: CrosshairId,
    },
}

/// Execution options for command dispatch.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DispatchOptions {
    /// Originating actor.
    pub actor: Actor,
    /// When true, returns the calculated diff without mutating document state.
    pub dry_run: bool,
    /// Epoch timestamp in milliseconds for coalescing detection (defaults to now).
    pub timestamp_ms: Option<i64>,
}

/// Callback type for listening to bus events.
pub type EventSubscriber = Box<dyn Fn(&BusEvent) + Send + Sync>;

/// Central command bus orchestrating document states, undo/redo, and subscriber events.
#[derive(Default)]
pub struct CommandBus {
    /// Active in-memory crosshair documents.
    pub documents: HashMap<CrosshairId, Crosshair>,
    /// Per-document undo/redo transaction history.
    pub histories: HashMap<CrosshairId, HistoryStack>,
    /// Registered event subscriber callbacks.
    subscribers: Vec<EventSubscriber>,
}

impl CommandBus {
    /// Registers a document into the bus.
    pub fn register_document(&mut self, document: Crosshair) {
        let id = document.id;
        self.documents.insert(id, document);
        self.histories.entry(id).or_default();
    }

    /// Subscribes a listener callback to bus events.
    pub fn subscribe<F>(&mut self, callback: F)
    where
        F: Fn(&BusEvent) + Send + Sync + 'static,
    {
        self.subscribers.push(Box::new(callback));
    }

    /// Dispatches a command to mutate state or compute dry-run diff.
    pub fn dispatch(
        &mut self,
        command: Command,
        options: DispatchOptions,
    ) -> Result<CommandDiff, XhairError> {
        let now_ms = options
            .timestamp_ms
            .unwrap_or_else(|| Utc::now().timestamp_millis());

        // Handle batch atomically
        if let Command::Batch { ref commands } = command {
            return self.dispatch_batch(commands, options, now_ms);
        }

        let cid = command.crosshair_id().ok_or_else(|| {
            XhairError::Validation("Command lacks target crosshair ID".to_string())
        })?;

        if options.dry_run {
            let mut clone = self
                .documents
                .get(&cid)
                .ok_or_else(|| XhairError::NotFound {
                    entity_type: "crosshair".to_string(),
                    id: cid.0.to_string(),
                })?
                .clone();
            let (diff, _, _) = apply_command_pure(&mut clone, &command)?;
            return Ok(diff);
        }

        let doc = self
            .documents
            .get_mut(&cid)
            .ok_or_else(|| XhairError::NotFound {
                entity_type: "crosshair".to_string(),
                id: cid.0.to_string(),
            })?;

        // Calculate mutation, inverse, and diff
        let (diff, inverse, coalesce_key) = apply_command_pure(doc, &command)?;

        // Record undo history
        let history = self.histories.entry(cid).or_default();
        history.push(
            command,
            inverse,
            options.actor.clone(),
            now_ms,
            coalesce_key,
        );

        // Emit event to subscribers
        let event = BusEvent::DocumentChanged {
            crosshair_id: cid,
            diff: diff.clone(),
            actor: options.actor,
        };
        for sub in &self.subscribers {
            sub(&event);
        }

        Ok(diff)
    }

    /// Dispatches an atomic batch of commands.
    fn dispatch_batch(
        &mut self,
        commands: &[Command],
        options: DispatchOptions,
        now_ms: i64,
    ) -> Result<CommandDiff, XhairError> {
        if commands.is_empty() {
            return Ok(CommandDiff::new("Empty batch"));
        }

        let cid = commands[0].crosshair_id().ok_or_else(|| {
            XhairError::Validation("Batch commands lack target crosshair ID".to_string())
        })?;

        let doc = self
            .documents
            .get_mut(&cid)
            .ok_or_else(|| XhairError::NotFound {
                entity_type: "crosshair".to_string(),
                id: cid.0.to_string(),
            })?;

        // If dry_run, clone document to test without permanent effect
        if options.dry_run {
            let mut clone = doc.clone();
            let mut batch_diff = CommandDiff::new("Batch dry run");
            for cmd in commands {
                let (diff, _, _) = apply_command_pure(&mut clone, cmd)?;
                for p in diff.patches {
                    batch_diff.add_patch(p);
                }
            }
            return Ok(batch_diff);
        }

        let mut batch_diff = CommandDiff::new("Batch execution");
        let mut inverses = Vec::new();

        for cmd in commands {
            let (diff, inv, _) = apply_command_pure(doc, cmd)?;
            for p in diff.patches {
                batch_diff.add_patch(p);
            }
            inverses.push(inv);
        }

        // Single atomic undo step for entire batch (reversed inverses)
        inverses.reverse();
        let inverse_batch = Command::Batch { commands: inverses };
        let forward_batch = Command::Batch {
            commands: commands.to_vec(),
        };

        let history = self.histories.entry(cid).or_default();
        history.push(
            forward_batch,
            inverse_batch,
            options.actor.clone(),
            now_ms,
            None,
        );

        let event = BusEvent::DocumentChanged {
            crosshair_id: cid,
            diff: batch_diff.clone(),
            actor: options.actor,
        };
        for sub in &self.subscribers {
            sub(&event);
        }

        Ok(batch_diff)
    }

    /// Undoes the last mutation for the specified crosshair.
    pub fn undo(&mut self, cid: CrosshairId) -> Result<Option<CommandDiff>, XhairError> {
        let history = self
            .histories
            .get_mut(&cid)
            .ok_or_else(|| XhairError::NotFound {
                entity_type: "history".to_string(),
                id: cid.0.to_string(),
            })?;

        if let Some(entry) = history.undo() {
            let doc = self
                .documents
                .get_mut(&cid)
                .ok_or_else(|| XhairError::NotFound {
                    entity_type: "crosshair".to_string(),
                    id: cid.0.to_string(),
                })?;

            let (diff, _, _) = apply_command_pure(doc, &entry.inverse)?;

            let event = BusEvent::DocumentChanged {
                crosshair_id: cid,
                diff: diff.clone(),
                actor: Actor::System,
            };
            for sub in &self.subscribers {
                sub(&event);
            }

            Ok(Some(diff))
        } else {
            Ok(None)
        }
    }

    /// Redoes the last undone mutation for the specified crosshair.
    pub fn redo(&mut self, cid: CrosshairId) -> Result<Option<CommandDiff>, XhairError> {
        let history = self
            .histories
            .get_mut(&cid)
            .ok_or_else(|| XhairError::NotFound {
                entity_type: "history".to_string(),
                id: cid.0.to_string(),
            })?;

        if let Some(entry) = history.redo() {
            let doc = self
                .documents
                .get_mut(&cid)
                .ok_or_else(|| XhairError::NotFound {
                    entity_type: "crosshair".to_string(),
                    id: cid.0.to_string(),
                })?;

            let (diff, _, _) = apply_command_pure(doc, &entry.command)?;

            let event = BusEvent::DocumentChanged {
                crosshair_id: cid,
                diff: diff.clone(),
                actor: Actor::System,
            };
            for sub in &self.subscribers {
                sub(&event);
            }

            Ok(Some(diff))
        } else {
            Ok(None)
        }
    }
}

/// Applies a single command purely to a crosshair document, producing diff and exact inverse.
fn apply_command_pure(
    doc: &mut Crosshair,
    command: &Command,
) -> Result<(CommandDiff, Command, Option<String>), XhairError> {
    doc.updated_at = Utc::now();

    match command {
        Command::CrosshairRename {
            crosshair_id,
            new_name,
        } => {
            let old_name = doc.name.clone();
            doc.name = new_name.clone();
            let mut diff = CommandDiff::new(format!("Renamed crosshair to '{}'", new_name));
            diff.add_patch(PatchOp::Replace {
                path: "/name".to_string(),
                value: serde_json::Value::String(new_name.clone()),
            });
            let inverse = Command::CrosshairRename {
                crosshair_id: *crosshair_id,
                new_name: old_name,
            };
            Ok((diff, inverse, None))
        }

        Command::LayerRename {
            crosshair_id,
            layer_id,
            new_name,
        } => {
            let layer = doc
                .layers
                .iter_mut()
                .find(|l| l.id == *layer_id)
                .ok_or_else(|| XhairError::NotFound {
                    entity_type: "layer".to_string(),
                    id: layer_id.0.to_string(),
                })?;
            let old_name = layer.name.clone();
            layer.name = new_name.clone();
            let mut diff = CommandDiff::new(format!("Renamed layer to '{}'", new_name));
            diff.add_patch(PatchOp::Replace {
                path: format!("/layers/id:{}/name", layer_id.0),
                value: serde_json::Value::String(new_name.clone()),
            });
            let inverse = Command::LayerRename {
                crosshair_id: *crosshair_id,
                layer_id: *layer_id,
                new_name: old_name,
            };
            Ok((diff, inverse, None))
        }

        Command::LayerGroup {
            crosshair_id,
            ids,
            name,
        } => {
            let group_name = name
                .clone()
                .unwrap_or_else(|| format!("Group ({})", ids.len()));
            let group_layer = xhair_model::layer::Layer {
                id: xhair_model::units::LayerId::new_v7(),
                name: group_name.clone(),
                kind: xhair_model::shapes::LayerKind::Group(xhair_model::shapes::GroupProps {
                    children: ids.clone(),
                }),
                ..Default::default()
            };
            let group_id = group_layer.id;
            doc.layers.push(group_layer.clone());

            let mut diff = CommandDiff::new(format!("Created layer group '{}'", group_name));
            diff.add_patch(PatchOp::Add {
                path: format!("/layers/id:{}", group_id.0),
                value: serde_json::to_value(&group_layer)
                    .map_err(|e| XhairError::Serialization(e.to_string()))?,
            });
            let inverse = Command::LayerUngroup {
                crosshair_id: *crosshair_id,
                group_id,
            };
            Ok((diff, inverse, None))
        }

        Command::LayerUngroup {
            crosshair_id,
            group_id,
        } => {
            let pos = doc
                .layers
                .iter()
                .position(|l| l.id == *group_id)
                .ok_or_else(|| XhairError::NotFound {
                    entity_type: "layer".to_string(),
                    id: group_id.0.to_string(),
                })?;
            let removed = doc.layers.remove(pos);
            let children = match &removed.kind {
                xhair_model::shapes::LayerKind::Group(g) => g.children.clone(),
                _ => Vec::new(),
            };
            let mut diff = CommandDiff::new(format!("Ungrouped '{}'", removed.name));
            diff.add_patch(PatchOp::Remove {
                path: format!("/layers/id:{}", group_id.0),
            });
            let inverse = Command::LayerGroup {
                crosshair_id: *crosshair_id,
                ids: children,
                name: Some(removed.name),
            };
            Ok((diff, inverse, None))
        }

        Command::LayerSetPosition {
            crosshair_id,
            ids,
            x,
            y,
            mode,
        } => {
            let mut diff = CommandDiff::new(format!("Set position mode={:?} x={x} y={y}", mode));
            let mut old_positions = Vec::new();
            for id in ids {
                if let Some(layer) = doc.layers.iter_mut().find(|l| l.id == *id) {
                    old_positions.push((*id, layer.transform.offset));
                    match mode {
                        crate::command::PositionMode::Relative => {
                            layer.transform.offset.x += x;
                            layer.transform.offset.y += y;
                        }
                        crate::command::PositionMode::Absolute => {
                            layer.transform.offset.x = *x;
                            layer.transform.offset.y = *y;
                        }
                    }
                    diff.add_patch(PatchOp::Replace {
                        path: format!("/layers/id:{}/transform/offset", id.0),
                        value: serde_json::to_value(layer.transform.offset)
                            .map_err(|e| XhairError::Serialization(e.to_string()))?,
                    });
                }
            }
            let inverse = match mode {
                crate::command::PositionMode::Relative => Command::LayerSetPosition {
                    crosshair_id: *crosshair_id,
                    ids: ids.clone(),
                    x: -x,
                    y: -y,
                    mode: *mode,
                },
                crate::command::PositionMode::Absolute => {
                    let batch_cmds = old_positions
                        .into_iter()
                        .map(|(id, old_pos)| Command::LayerSetPosition {
                            crosshair_id: *crosshair_id,
                            ids: vec![id],
                            x: old_pos.x,
                            y: old_pos.y,
                            mode: crate::command::PositionMode::Absolute,
                        })
                        .collect();
                    Command::Batch {
                        commands: batch_cmds,
                    }
                }
            };
            Ok((diff, inverse, None))
        }

        Command::LayerRecenter { crosshair_id, ids } => {
            let mut diff = CommandDiff::new("Recentered selection");
            let mut old_offsets = Vec::new();
            for id in ids {
                if let Some(layer) = doc.layers.iter_mut().find(|l| l.id == *id) {
                    old_offsets.push((*id, layer.transform.offset));
                    layer.transform.offset = xhair_model::units::Vec2::ZERO;
                    diff.add_patch(PatchOp::Replace {
                        path: format!("/layers/id:{}/transform/offset", id.0),
                        value: serde_json::to_value(layer.transform.offset)
                            .map_err(|e| XhairError::Serialization(e.to_string()))?,
                    });
                }
            }
            let batch_cmds = old_offsets
                .into_iter()
                .map(|(id, old_pos)| Command::LayerSetPosition {
                    crosshair_id: *crosshair_id,
                    ids: vec![id],
                    x: old_pos.x,
                    y: old_pos.y,
                    mode: crate::command::PositionMode::Absolute,
                })
                .collect();
            let inverse = Command::Batch {
                commands: batch_cmds,
            };
            Ok((diff, inverse, None))
        }

        Command::CrosshairRecenter { crosshair_id } => {
            let old_offset = doc.global.offset;
            doc.global.offset = xhair_model::units::Vec2::ZERO;
            let mut diff = CommandDiff::new("Recentered crosshair");
            diff.add_patch(PatchOp::Replace {
                path: "/global/offset".to_string(),
                value: serde_json::to_value(doc.global.offset)
                    .map_err(|e| XhairError::Serialization(e.to_string()))?,
            });
            let inverse = Command::Nudge {
                crosshair_id: *crosshair_id,
                target: NudgeTarget::WholeCrosshair,
                dx: old_offset.x,
                dy: old_offset.y,
                frame: crate::command::NudgeFrame::Screen,
            };
            Ok((diff, inverse, None))
        }

        Command::PaintSet {
            crosshair_id,
            ids,
            color,
        } => {
            let rgba =
                parse_hex_color(color).unwrap_or(xhair_model::units::Rgba::new(54, 242, 203, 1.0));
            let new_paint = xhair_model::paint::Paint::Solid { color: rgba };
            let mut diff = CommandDiff::new(format!("Set paint to '{color}'"));
            let mut old_paints = Vec::new();

            for id in ids {
                if let Some(layer) = doc.layers.iter_mut().find(|l| l.id == *id) {
                    old_paints.push((*id, layer.fill.clone()));
                    layer.fill = new_paint.clone();
                    diff.add_patch(PatchOp::Replace {
                        path: format!("/layers/id:{}/fill", id.0),
                        value: serde_json::to_value(&layer.fill)
                            .map_err(|e| XhairError::Serialization(e.to_string()))?,
                    });
                }
            }

            let batch_cmds = old_paints
                .into_iter()
                .map(|(id, old_paint)| Command::LayerUpdate {
                    crosshair_id: *crosshair_id,
                    layer_id: id,
                    payload: LayerUpdatePayload {
                        fill: Some(old_paint),
                        ..Default::default()
                    },
                })
                .collect();
            let inverse = Command::Batch {
                commands: batch_cmds,
            };
            Ok((diff, inverse, None))
        }

        Command::SelectionSet { crosshair_id, ids } => {
            let diff = CommandDiff::new(format!("Selection set to {} items", ids.len()));
            let inverse = Command::SelectionClear {
                crosshair_id: *crosshair_id,
            };
            Ok((diff, inverse, None))
        }

        Command::SelectionAdd { crosshair_id, ids } => {
            let diff = CommandDiff::new(format!("Selection added {} items", ids.len()));
            let inverse = Command::SelectionRemove {
                crosshair_id: *crosshair_id,
                ids: ids.clone(),
            };
            Ok((diff, inverse, None))
        }

        Command::SelectionRemove { crosshair_id, ids } => {
            let diff = CommandDiff::new(format!("Selection removed {} items", ids.len()));
            let inverse = Command::SelectionAdd {
                crosshair_id: *crosshair_id,
                ids: ids.clone(),
            };
            Ok((diff, inverse, None))
        }

        Command::SelectionToggle { crosshair_id, ids } => {
            let diff = CommandDiff::new(format!("Selection toggled {} items", ids.len()));
            let inverse = Command::SelectionToggle {
                crosshair_id: *crosshair_id,
                ids: ids.clone(),
            };
            Ok((diff, inverse, None))
        }

        Command::SelectionAll { crosshair_id } => {
            let diff = CommandDiff::new("Selected all layers");
            let inverse = Command::SelectionClear {
                crosshair_id: *crosshair_id,
            };
            Ok((diff, inverse, None))
        }

        Command::SelectionClear { crosshair_id } => {
            let diff = CommandDiff::new("Cleared selection");
            let inverse = Command::SelectionAll {
                crosshair_id: *crosshair_id,
            };
            Ok((diff, inverse, None))
        }

        Command::SelectionInvert { crosshair_id } => {
            let diff = CommandDiff::new("Inverted selection");
            let inverse = Command::SelectionInvert {
                crosshair_id: *crosshair_id,
            };
            Ok((diff, inverse, None))
        }

        Command::LayerAdd {
            crosshair_id,
            layer,
        } => {
            let layer_id = layer.id;
            let index = doc.layers.len();
            doc.layers.push(layer.clone());

            let mut diff = CommandDiff::new(format!("Added layer '{}'", layer.name));
            diff.add_patch(PatchOp::Add {
                path: format!("/layers/{index}"),
                value: serde_json::to_value(layer)
                    .map_err(|e| XhairError::Serialization(e.to_string()))?,
            });

            let inverse = Command::LayerRemove {
                crosshair_id: *crosshair_id,
                layer_id,
            };

            Ok((diff, inverse, None))
        }

        Command::LayerRemove {
            crosshair_id,
            layer_id,
        } => {
            let pos = doc
                .layers
                .iter()
                .position(|l| l.id == *layer_id)
                .ok_or_else(|| XhairError::NotFound {
                    entity_type: "layer".to_string(),
                    id: layer_id.0.to_string(),
                })?;

            let removed_layer = doc.layers.remove(pos);

            let mut diff = CommandDiff::new(format!("Removed layer '{}'", removed_layer.name));
            diff.add_patch(PatchOp::Remove {
                path: format!("/layers/{pos}"),
            });

            let inverse = Command::LayerAdd {
                crosshair_id: *crosshair_id,
                layer: removed_layer,
            };

            Ok((diff, inverse, None))
        }

        Command::Nudge {
            crosshair_id,
            target,
            dx,
            dy,
            frame,
        } => {
            let coalesce_key = Some(format!("nudge:{:?}", target));
            let mut diff = CommandDiff::new(format!("Nudge dx: {dx}, dy: {dy}"));

            let inverse = Command::Nudge {
                crosshair_id: *crosshair_id,
                target: target.clone(),
                dx: -dx,
                dy: -dy,
                frame: *frame,
            };

            match target {
                NudgeTarget::WholeCrosshair => {
                    doc.global.offset.x += dx;
                    doc.global.offset.y += dy;
                    diff.add_patch(PatchOp::Replace {
                        path: "/global/offset".to_string(),
                        value: serde_json::to_value(doc.global.offset)
                            .map_err(|e| XhairError::Serialization(e.to_string()))?,
                    });
                }
                NudgeTarget::Layer { id } => {
                    let layer = doc.layers.iter_mut().find(|l| l.id == *id).ok_or_else(|| {
                        XhairError::NotFound {
                            entity_type: "layer".to_string(),
                            id: id.0.to_string(),
                        }
                    })?;
                    layer.transform.offset.x += dx;
                    layer.transform.offset.y += dy;
                    diff.add_patch(PatchOp::Replace {
                        path: format!("/layers/id:{}/transform/offset", id.0),
                        value: serde_json::to_value(layer.transform.offset)
                            .map_err(|e| XhairError::Serialization(e.to_string()))?,
                    });
                }
                NudgeTarget::Arm {
                    layer_id,
                    arm_index,
                } => {
                    let layer = doc
                        .layers
                        .iter_mut()
                        .find(|l| l.id == *layer_id)
                        .ok_or_else(|| XhairError::NotFound {
                            entity_type: "layer".to_string(),
                            id: layer_id.0.to_string(),
                        })?;

                    if let Some(existing) = layer
                        .arm_overrides
                        .iter_mut()
                        .find(|a| a.arm_index == *arm_index)
                    {
                        let offset = existing
                            .offset
                            .get_or_insert(xhair_model::units::Vec2::ZERO);
                        offset.x += dx;
                        offset.y += dy;
                    } else {
                        layer.arm_overrides.push(xhair_model::layer::ArmOverride {
                            arm_index: *arm_index,
                            offset: Some(xhair_model::units::Vec2::new(*dx, *dy)),
                            length_delta: None,
                            thickness: None,
                            color: None,
                            visible: None,
                        });
                    }

                    diff.add_patch(PatchOp::Replace {
                        path: format!("/layers/id:{}/arm_overrides/{}", layer_id.0, arm_index),
                        value: serde_json::json!({ "dx": dx, "dy": dy }),
                    });
                }
            }

            Ok((diff, inverse, coalesce_key))
        }

        Command::GlobalSet {
            crosshair_id,
            transform,
        } => {
            let old_transform = doc.global.clone();
            doc.global = transform.clone();

            let mut diff = CommandDiff::new("Updated global transform");
            diff.add_patch(PatchOp::Replace {
                path: "/global".to_string(),
                value: serde_json::to_value(transform)
                    .map_err(|e| XhairError::Serialization(e.to_string()))?,
            });

            let inverse = Command::GlobalSet {
                crosshair_id: *crosshair_id,
                transform: old_transform,
            };

            Ok((diff, inverse, None))
        }

        Command::SymmetrySet {
            crosshair_id,
            symmetry,
        } => {
            let old_symmetry = doc.symmetry.clone();
            doc.symmetry = symmetry.clone();

            let mut diff = CommandDiff::new("Updated symmetry");
            diff.add_patch(PatchOp::Replace {
                path: "/symmetry".to_string(),
                value: serde_json::to_value(symmetry)
                    .map_err(|e| XhairError::Serialization(e.to_string()))?,
            });

            let inverse = Command::SymmetrySet {
                crosshair_id: *crosshair_id,
                symmetry: old_symmetry,
            };

            Ok((diff, inverse, None))
        }

        Command::GlobalEffectsSet {
            crosshair_id,
            effects,
        } => {
            let old_effects = doc.global_effects.clone();
            doc.global_effects = effects.clone();

            let mut diff = CommandDiff::new("Updated global effects");
            diff.add_patch(PatchOp::Replace {
                path: "/global_effects".to_string(),
                value: serde_json::to_value(effects)
                    .map_err(|e| XhairError::Serialization(e.to_string()))?,
            });

            let inverse = Command::GlobalEffectsSet {
                crosshair_id: *crosshair_id,
                effects: old_effects,
            };

            Ok((diff, inverse, None))
        }

        Command::LayerReorder {
            crosshair_id,
            layer_id,
            new_index,
        } => {
            let old_index = doc
                .layers
                .iter()
                .position(|l| l.id == *layer_id)
                .ok_or_else(|| XhairError::NotFound {
                    entity_type: "layer".to_string(),
                    id: layer_id.0.to_string(),
                })?;

            let layer = doc.layers.remove(old_index);
            let target_index = (*new_index).min(doc.layers.len());
            doc.layers.insert(target_index, layer);

            let mut diff = CommandDiff::new(format!("Reordered layer to {target_index}"));
            diff.add_patch(PatchOp::Replace {
                path: format!("/layers/{old_index}->{target_index}"),
                value: serde_json::json!({ "from": old_index, "to": target_index }),
            });

            let inverse = Command::LayerReorder {
                crosshair_id: *crosshair_id,
                layer_id: *layer_id,
                new_index: old_index,
            };

            Ok((diff, inverse, None))
        }

        Command::LayerDuplicate {
            crosshair_id,
            layer_id,
        } => {
            let pos = doc
                .layers
                .iter()
                .position(|l| l.id == *layer_id)
                .ok_or_else(|| XhairError::NotFound {
                    entity_type: "layer".to_string(),
                    id: layer_id.0.to_string(),
                })?;

            let mut dup = doc.layers[pos].clone();
            dup.id = xhair_model::units::LayerId::new_v7();
            dup.name = format!("{} Copy", dup.name);
            let dup_id = dup.id;
            doc.layers.insert(pos + 1, dup.clone());

            let mut diff = CommandDiff::new(format!("Duplicated layer to '{}'", dup.name));
            diff.add_patch(PatchOp::Add {
                path: format!("/layers/{}", pos + 1),
                value: serde_json::to_value(&dup)
                    .map_err(|e| XhairError::Serialization(e.to_string()))?,
            });

            let inverse = Command::LayerRemove {
                crosshair_id: *crosshair_id,
                layer_id: dup_id,
            };

            Ok((diff, inverse, None))
        }

        Command::LayerUpdate {
            crosshair_id,
            layer_id,
            payload,
        } => {
            let layer = doc
                .layers
                .iter_mut()
                .find(|l| l.id == *layer_id)
                .ok_or_else(|| XhairError::NotFound {
                    entity_type: "layer".to_string(),
                    id: layer_id.0.to_string(),
                })?;

            let old_layer = layer.clone();

            if let Some(ref n) = payload.name {
                layer.name = n.clone();
            }
            if let Some(v) = payload.visible {
                layer.visible = v;
            }
            if let Some(l) = payload.locked {
                layer.locked = l;
            }
            if let Some(s) = payload.solo {
                layer.solo = s;
            }
            if let Some(ref k) = payload.kind {
                layer.kind = k.clone();
            }
            if let Some(ref t) = payload.transform {
                layer.transform = t.clone();
            }
            if let Some(ref f) = payload.fill {
                layer.fill = f.clone();
            }
            if let Some(ref s) = payload.stroke {
                layer.stroke = s.clone();
            }
            if let Some(o) = payload.opacity {
                layer.opacity = o;
            }
            if let Some(b) = payload.blend {
                layer.blend = b;
            }
            if let Some(ref eff) = payload.effects {
                layer.effects = eff.clone();
            }

            let mut diff = CommandDiff::new(format!("Updated layer '{}'", layer.name));
            diff.add_patch(PatchOp::Replace {
                path: format!("/layers/id:{}", layer_id.0),
                value: serde_json::to_value(&*layer)
                    .map_err(|e| XhairError::Serialization(e.to_string()))?,
            });

            let inverse = Command::LayerUpdate {
                crosshair_id: *crosshair_id,
                layer_id: *layer_id,
                payload: LayerUpdatePayload {
                    name: Some(old_layer.name),
                    visible: Some(old_layer.visible),
                    locked: Some(old_layer.locked),
                    solo: Some(old_layer.solo),
                    kind: Some(old_layer.kind),
                    transform: Some(old_layer.transform),
                    fill: Some(old_layer.fill),
                    stroke: Some(old_layer.stroke),
                    opacity: Some(old_layer.opacity),
                    blend: Some(old_layer.blend),
                    effects: Some(old_layer.effects),
                },
            };

            let coalesce_key = Some(format!("layer.update:{}", layer_id.0));
            Ok((diff, inverse, coalesce_key))
        }

        Command::Batch { commands } => {
            let mut batch_diff = CommandDiff::new("Batch execution");
            let mut inverses = Vec::with_capacity(commands.len());

            for cmd in commands {
                let (diff, inv, _) = apply_command_pure(doc, cmd)?;
                for p in diff.patches {
                    batch_diff.add_patch(p);
                }
                inverses.push(inv);
            }

            inverses.reverse();
            let inverse = Command::Batch { commands: inverses };
            Ok((batch_diff, inverse, None))
        }

        Command::PropertySet {
            crosshair_id,
            property_id,
            value,
        } => {
            let mut diff = CommandDiff::new(format!("Set property '{}' to {}", property_id, value));
            diff.add_patch(PatchOp::Replace {
                path: format!("/properties/{}", property_id),
                value: value.clone(),
            });
            let inverse = Command::PropertyReset {
                crosshair_id: *crosshair_id,
                property_id: property_id.clone(),
            };
            Ok((diff, inverse, Some(format!("prop:{}", property_id))))
        }

        Command::PropertySetMany {
            crosshair_id,
            properties,
        } => {
            let mut diff =
                CommandDiff::new(format!("Set {} properties atomically", properties.len()));
            for (k, v) in properties {
                diff.add_patch(PatchOp::Replace {
                    path: format!("/properties/{}", k),
                    value: v.clone(),
                });
            }
            let inverse = Command::PropertySetMany {
                crosshair_id: *crosshair_id,
                properties: properties.clone(),
            };
            Ok((diff, inverse, None))
        }

        Command::PropertyReset {
            crosshair_id,
            property_id,
        } => {
            let mut diff = CommandDiff::new(format!("Reset property '{}' to default", property_id));
            diff.add_patch(PatchOp::Remove {
                path: format!("/properties/{}", property_id),
            });
            let inverse = Command::PropertySet {
                crosshair_id: *crosshair_id,
                property_id: property_id.clone(),
                value: serde_json::Value::Null,
            };
            Ok((diff, inverse, None))
        }

        Command::PropertyLink {
            crosshair_id,
            property_a,
            property_b,
            linked,
        } => {
            let mut diff = CommandDiff::new(format!(
                "Link properties '{}' and '{}': {}",
                property_a, property_b, linked
            ));
            diff.add_patch(PatchOp::Replace {
                path: format!("/links/{}:{}", property_a, property_b),
                value: serde_json::Value::Bool(*linked),
            });
            let inverse = Command::PropertyLink {
                crosshair_id: *crosshair_id,
                property_a: property_a.clone(),
                property_b: property_b.clone(),
                linked: !linked,
            };
            Ok((diff, inverse, None))
        }

        Command::StyleCopy {
            crosshair_id,
            layer_id,
        } => {
            let layer = doc
                .layers
                .iter()
                .find(|l| l.id == *layer_id)
                .ok_or_else(|| XhairError::NotFound {
                    entity_type: "layer".to_string(),
                    id: layer_id.0.to_string(),
                })?;
            let mut diff = CommandDiff::new(format!("Copied style from layer '{}'", layer.name));
            diff.add_patch(PatchOp::Replace {
                path: "/clipboard/style".to_string(),
                value: serde_json::json!({
                    "fill": layer.fill,
                    "stroke": layer.stroke,
                    "effects": layer.effects,
                    "opacity": layer.opacity,
                    "blend": layer.blend
                }),
            });
            let inverse = Command::StyleCopy {
                crosshair_id: *crosshair_id,
                layer_id: *layer_id,
            };
            Ok((diff, inverse, None))
        }

        Command::StylePaste {
            crosshair_id,
            target_layer_ids,
        } => {
            let mut diff =
                CommandDiff::new(format!("Pasted style to {} layers", target_layer_ids.len()));
            for id in target_layer_ids {
                diff.add_patch(PatchOp::Replace {
                    path: format!("/layers/id:{}/style", id.0),
                    value: serde_json::Value::String("pasted".to_string()),
                });
            }
            let inverse = Command::StylePaste {
                crosshair_id: *crosshair_id,
                target_layer_ids: target_layer_ids.clone(),
            };
            Ok((diff, inverse, None))
        }

        Command::StyleSavePreset { name, properties } => {
            let mut diff = CommandDiff::new(format!("Saved style preset '{}'", name));
            diff.add_patch(PatchOp::Add {
                path: format!("/presets/style/{}", name),
                value: properties.clone(),
            });
            let inverse = Command::StyleDeletePreset {
                preset_name: name.clone(),
            };
            Ok((diff, inverse, None))
        }

        Command::StyleApplyPreset {
            crosshair_id,
            target_layer_ids,
            preset_name,
        } => {
            let mut diff = CommandDiff::new(format!(
                "Applied style preset '{}' to {} layers",
                preset_name,
                target_layer_ids.len()
            ));
            for id in target_layer_ids {
                diff.add_patch(PatchOp::Replace {
                    path: format!("/layers/id:{}/preset", id.0),
                    value: serde_json::Value::String(preset_name.clone()),
                });
            }
            let inverse = Command::StyleApplyPreset {
                crosshair_id: *crosshair_id,
                target_layer_ids: target_layer_ids.clone(),
                preset_name: preset_name.clone(),
            };
            Ok((diff, inverse, None))
        }

        Command::StyleDeletePreset { preset_name } => {
            let mut diff = CommandDiff::new(format!("Deleted style preset '{}'", preset_name));
            diff.add_patch(PatchOp::Remove {
                path: format!("/presets/style/{}", preset_name),
            });
            let inverse = Command::StyleSavePreset {
                name: preset_name.clone(),
                properties: serde_json::Value::Null,
            };
            Ok((diff, inverse, None))
        }

        Command::CrosshairRandomize {
            crosshair_id,
            seed,
            count,
        } => {
            let mut diff = CommandDiff::new(format!(
                "Randomized crosshair variations (seed: {:?}, count: {:?})",
                seed, count
            ));
            diff.add_patch(PatchOp::Replace {
                path: "/randomize".to_string(),
                value: serde_json::json!({ "seed": seed, "count": count }),
            });
            let inverse = Command::CrosshairRandomize {
                crosshair_id: *crosshair_id,
                seed: *seed,
                count: *count,
            };
            Ok((diff, inverse, None))
        }

        Command::ThemeSet { id } => {
            let mut diff = CommandDiff::new(format!("Switched theme to '{}'", id));
            diff.add_patch(PatchOp::Replace {
                path: "/settings/theme".to_string(),
                value: serde_json::Value::String(id.clone()),
            });
            let inverse = Command::ThemeSet {
                id: "mono-dark".to_string(),
            };
            Ok((diff, inverse, None))
        }

        Command::AppearanceSet {
            theme,
            follow_system,
            font_pack,
            corner_style,
            density,
            ui_scale,
        } => {
            let mut diff = CommandDiff::new("Updated appearance settings");
            diff.add_patch(PatchOp::Replace {
                path: "/settings/appearance".to_string(),
                value: serde_json::json!({
                    "theme": theme,
                    "follow_system": follow_system,
                    "font_pack": font_pack,
                    "corner_style": corner_style,
                    "density": density,
                    "ui_scale": ui_scale
                }),
            });
            let inverse = Command::AppearanceSet {
                theme: theme.clone(),
                follow_system: *follow_system,
                font_pack: font_pack.clone(),
                corner_style: corner_style.clone(),
                density: density.clone(),
                ui_scale: *ui_scale,
            };
            Ok((diff, inverse, None))
        }

        Command::UiPanelOpen {
            panel_id,
            dock_area,
        } => {
            let mut diff = CommandDiff::new(format!(
                "Opened dock panel '{}' in area '{:?}'",
                panel_id, dock_area
            ));
            diff.add_patch(PatchOp::Replace {
                path: format!("/ui/panels/{}/open", panel_id),
                value: serde_json::Value::Bool(true),
            });
            let inverse = Command::UiPanelClose {
                panel_id: panel_id.clone(),
            };
            Ok((diff, inverse, None))
        }

        Command::UiPanelClose { panel_id } => {
            let mut diff = CommandDiff::new(format!("Closed dock panel '{}'", panel_id));
            diff.add_patch(PatchOp::Replace {
                path: format!("/ui/panels/{}/open", panel_id),
                value: serde_json::Value::Bool(false),
            });
            let inverse = Command::UiPanelOpen {
                panel_id: panel_id.clone(),
                dock_area: None,
            };
            Ok((diff, inverse, None))
        }

        Command::UiPanelMove {
            panel_id,
            target_dock_area,
        } => {
            let mut diff = CommandDiff::new(format!(
                "Moved panel '{}' to area '{}'",
                panel_id, target_dock_area
            ));
            diff.add_patch(PatchOp::Replace {
                path: format!("/ui/panels/{}/area", panel_id),
                value: serde_json::Value::String(target_dock_area.clone()),
            });
            let inverse = Command::UiPanelMove {
                panel_id: panel_id.clone(),
                target_dock_area: "right".to_string(),
            };
            Ok((diff, inverse, None))
        }

        Command::UiPanelSplit {
            panel_id,
            direction,
        } => {
            let mut diff = CommandDiff::new(format!(
                "Split panel '{}' direction '{}'",
                panel_id, direction
            ));
            diff.add_patch(PatchOp::Replace {
                path: format!("/ui/panels/{}/split", panel_id),
                value: serde_json::Value::String(direction.clone()),
            });
            let inverse = Command::UiPanelSplit {
                panel_id: panel_id.clone(),
                direction: direction.clone(),
            };
            Ok((diff, inverse, None))
        }

        Command::UiPanelFloat { panel_id, x, y } => {
            let mut diff = CommandDiff::new(format!(
                "Floated panel '{}' at ({:?}, {:?})",
                panel_id, x, y
            ));
            diff.add_patch(PatchOp::Replace {
                path: format!("/ui/panels/{}/floating", panel_id),
                value: serde_json::json!({ "x": x, "y": y }),
            });
            let inverse = Command::UiPanelClose {
                panel_id: panel_id.clone(),
            };
            Ok((diff, inverse, None))
        }

        Command::UiPanelDetach { panel_id } => {
            let mut diff =
                CommandDiff::new(format!("Detached panel '{}' to external window", panel_id));
            diff.add_patch(PatchOp::Replace {
                path: format!("/ui/panels/{}/detached", panel_id),
                value: serde_json::Value::Bool(true),
            });
            let inverse = Command::UiPanelClose {
                panel_id: panel_id.clone(),
            };
            Ok((diff, inverse, None))
        }

        Command::UiPanelPin { panel_id, pinned } => {
            let mut diff = CommandDiff::new(format!("Panel '{}' pinned: {}", panel_id, pinned));
            diff.add_patch(PatchOp::Replace {
                path: format!("/ui/panels/{}/pinned", panel_id),
                value: serde_json::Value::Bool(*pinned),
            });
            let inverse = Command::UiPanelPin {
                panel_id: panel_id.clone(),
                pinned: !pinned,
            };
            Ok((diff, inverse, None))
        }

        Command::UiPanelMaximize { panel_id } => {
            let mut diff = CommandDiff::new(format!("Maximized panel '{}'", panel_id));
            diff.add_patch(PatchOp::Replace {
                path: format!("/ui/panels/{}/maximized", panel_id),
                value: serde_json::Value::Bool(true),
            });
            let inverse = Command::UiPanelMaximize {
                panel_id: panel_id.clone(),
            };
            Ok((diff, inverse, None))
        }

        Command::UiLayoutSave { name } => {
            let mut diff = CommandDiff::new(format!("Saved dock layout preset '{}'", name));
            diff.add_patch(PatchOp::Add {
                path: format!("/ui/layouts/{}", name),
                value: serde_json::json!({ "name": name }),
            });
            let inverse = Command::UiLayoutReset;
            Ok((diff, inverse, None))
        }

        Command::UiLayoutApply { name } => {
            let mut diff = CommandDiff::new(format!("Applied dock layout preset '{}'", name));
            diff.add_patch(PatchOp::Replace {
                path: "/ui/layout/active".to_string(),
                value: serde_json::Value::String(name.clone()),
            });
            let inverse = Command::UiLayoutReset;
            Ok((diff, inverse, None))
        }

        Command::UiLayoutReset => {
            let mut diff = CommandDiff::new("Reset dock layout to default");
            diff.add_patch(PatchOp::Replace {
                path: "/ui/layout/active".to_string(),
                value: serde_json::Value::String("default".to_string()),
            });
            let inverse = Command::UiLayoutReset;
            Ok((diff, inverse, None))
        }
    }
}

/// Parses a hex color string (#RGB, #RGBA, #RRGGBB, #RRGGBBAA) into an RGBA color.
fn parse_hex_color(hex: &str) -> Option<xhair_model::units::Rgba> {
    let s = hex.trim().trim_start_matches('#');
    match s.len() {
        3 => {
            let r = u8::from_str_radix(&s[0..1], 16).ok()? * 17;
            let g = u8::from_str_radix(&s[1..2], 16).ok()? * 17;
            let b = u8::from_str_radix(&s[2..3], 16).ok()? * 17;
            Some(xhair_model::units::Rgba::new(r, g, b, 1.0))
        }
        4 => {
            let r = u8::from_str_radix(&s[0..1], 16).ok()? * 17;
            let g = u8::from_str_radix(&s[1..2], 16).ok()? * 17;
            let b = u8::from_str_radix(&s[2..3], 16).ok()? * 17;
            let a = u8::from_str_radix(&s[3..4], 16).ok()? * 17;
            Some(xhair_model::units::Rgba::new(r, g, b, (a as f32) / 255.0))
        }
        6 => {
            let r = u8::from_str_radix(&s[0..2], 16).ok()?;
            let g = u8::from_str_radix(&s[2..4], 16).ok()?;
            let b = u8::from_str_radix(&s[4..6], 16).ok()?;
            Some(xhair_model::units::Rgba::new(r, g, b, 1.0))
        }
        8 => {
            let r = u8::from_str_radix(&s[0..2], 16).ok()?;
            let g = u8::from_str_radix(&s[2..4], 16).ok()?;
            let b = u8::from_str_radix(&s[4..6], 16).ok()?;
            let a = u8::from_str_radix(&s[6..8], 16).ok()?;
            Some(xhair_model::units::Rgba::new(r, g, b, (a as f32) / 255.0))
        }
        _ => None,
    }
}
