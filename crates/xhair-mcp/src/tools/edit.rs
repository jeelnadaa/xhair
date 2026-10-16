//! Editing tools: batch, layer mutations, symmetry, nudge, undo, redo.

use crate::context::ServerContext;
use crate::protocol::CallToolResult;
use serde_json::Value;
use xhair_commands::actor::Actor;
use xhair_commands::bus::DispatchOptions;
use xhair_commands::command::{Command, LayerUpdatePayload, NudgeFrame, NudgeTarget};
use xhair_model::crosshair::{GlobalTransform, Symmetry};
use xhair_model::effects::Effects;
use xhair_model::layer::Layer;
use xhair_model::units::{Alpha, LayerId};

fn agent_dispatch_opts() -> DispatchOptions {
    DispatchOptions {
        actor: Actor::Agent {
            client_name: Some("mcp".to_string()),
        },
        ..Default::default()
    }
}

/// Adds a layer to a crosshair.
pub fn layer_add(ctx: &ServerContext, args: Value) -> CallToolResult {
    let cid = match ctx.resolve_crosshair_id(&args) {
        Ok(id) => id,
        Err(e) => return CallToolResult::error(e),
    };

    let layer_val = match args.get("layer") {
        Some(v) => v,
        None => return CallToolResult::error("Missing required parameter: layer"),
    };

    let layer: Layer = match serde_json::from_value(layer_val.clone()) {
        Ok(l) => l,
        Err(e) => return CallToolResult::error(format!("Invalid layer payload: {}", e)),
    };

    let lid = layer.id;
    let cmd = Command::LayerAdd {
        crosshair_id: cid,
        layer,
    };

    ctx.with_state(
        |state| match state.bus.dispatch(cmd, agent_dispatch_opts()) {
            Ok(diff) => CallToolResult::json(&serde_json::json!({
                "status": "layer_added",
                "layer_id": lid,
                "diff": diff
            })),
            Err(e) => CallToolResult::error(e.to_string()),
        },
    )
}

/// Updates layer properties.
pub fn layer_update(ctx: &ServerContext, args: Value) -> CallToolResult {
    let cid = match ctx.resolve_crosshair_id(&args) {
        Ok(id) => id,
        Err(e) => return CallToolResult::error(e),
    };

    let lid_str = match args.get("layer_id").and_then(|v| v.as_str()) {
        Some(s) => s,
        None => return CallToolResult::error("Missing required parameter: layer_id"),
    };

    let lid = match lid_str.parse::<LayerId>() {
        Ok(id) => id,
        Err(_) => return CallToolResult::error(format!("Invalid layer_id: {}", lid_str)),
    };

    let payload: LayerUpdatePayload =
        match serde_json::from_value(args.get("payload").cloned().unwrap_or(Value::Null)) {
            Ok(p) => p,
            Err(e) => return CallToolResult::error(format!("Invalid update payload: {}", e)),
        };

    let cmd = Command::LayerUpdate {
        crosshair_id: cid,
        layer_id: lid,
        payload,
    };

    ctx.with_state(
        |state| match state.bus.dispatch(cmd, agent_dispatch_opts()) {
            Ok(diff) => CallToolResult::json(&serde_json::json!({
                "status": "layer_updated",
                "layer_id": lid,
                "diff": diff
            })),
            Err(e) => CallToolResult::error(e.to_string()),
        },
    )
}

/// Removes a layer from a crosshair.
pub fn layer_remove(ctx: &ServerContext, args: Value) -> CallToolResult {
    let cid = match ctx.resolve_crosshair_id(&args) {
        Ok(id) => id,
        Err(e) => return CallToolResult::error(e),
    };

    let lid_str = match args.get("layer_id").and_then(|v| v.as_str()) {
        Some(s) => s,
        None => return CallToolResult::error("Missing required parameter: layer_id"),
    };

    let lid = match lid_str.parse::<LayerId>() {
        Ok(id) => id,
        Err(_) => return CallToolResult::error(format!("Invalid layer_id: {}", lid_str)),
    };

    let cmd = Command::LayerRemove {
        crosshair_id: cid,
        layer_id: lid,
    };

    ctx.with_state(
        |state| match state.bus.dispatch(cmd, agent_dispatch_opts()) {
            Ok(diff) => CallToolResult::json(&serde_json::json!({
                "status": "layer_removed",
                "layer_id": lid,
                "diff": diff
            })),
            Err(e) => CallToolResult::error(e.to_string()),
        },
    )
}

/// Duplicates a layer.
pub fn layer_duplicate(ctx: &ServerContext, args: Value) -> CallToolResult {
    let cid = match ctx.resolve_crosshair_id(&args) {
        Ok(id) => id,
        Err(e) => return CallToolResult::error(e),
    };

    let lid_str = match args.get("layer_id").and_then(|v| v.as_str()) {
        Some(s) => s,
        None => return CallToolResult::error("Missing required parameter: layer_id"),
    };

    let lid = match lid_str.parse::<LayerId>() {
        Ok(id) => id,
        Err(_) => return CallToolResult::error(format!("Invalid layer_id: {}", lid_str)),
    };

    let cmd = Command::LayerDuplicate {
        crosshair_id: cid,
        layer_id: lid,
    };

    ctx.with_state(
        |state| match state.bus.dispatch(cmd, agent_dispatch_opts()) {
            Ok(diff) => CallToolResult::json(&serde_json::json!({
                "status": "layer_duplicated",
                "diff": diff
            })),
            Err(e) => CallToolResult::error(e.to_string()),
        },
    )
}

/// Reorders a layer.
pub fn layer_reorder(ctx: &ServerContext, args: Value) -> CallToolResult {
    let cid = match ctx.resolve_crosshair_id(&args) {
        Ok(id) => id,
        Err(e) => return CallToolResult::error(e),
    };

    let lid_str = match args.get("layer_id").and_then(|v| v.as_str()) {
        Some(s) => s,
        None => return CallToolResult::error("Missing required parameter: layer_id"),
    };

    let lid = match lid_str.parse::<LayerId>() {
        Ok(id) => id,
        Err(_) => return CallToolResult::error(format!("Invalid layer_id: {}", lid_str)),
    };

    let new_index = match args.get("new_index").and_then(|v| v.as_u64()) {
        Some(idx) => idx as usize,
        None => return CallToolResult::error("Missing required parameter: new_index"),
    };

    let cmd = Command::LayerReorder {
        crosshair_id: cid,
        layer_id: lid,
        new_index,
    };

    ctx.with_state(
        |state| match state.bus.dispatch(cmd, agent_dispatch_opts()) {
            Ok(diff) => CallToolResult::json(&serde_json::json!({
                "status": "layer_reordered",
                "diff": diff
            })),
            Err(e) => CallToolResult::error(e.to_string()),
        },
    )
}

/// Sets global crosshair transform.
pub fn global_set(ctx: &ServerContext, args: Value) -> CallToolResult {
    let cid = match ctx.resolve_crosshair_id(&args) {
        Ok(id) => id,
        Err(e) => return CallToolResult::error(e),
    };

    let scale = args
        .get("scale")
        .and_then(|v| v.as_f64())
        .map(|s| s as f32)
        .unwrap_or(1.0);
    let opacity = args
        .get("opacity")
        .and_then(|v| v.as_f64())
        .map(|o| Alpha::from(o as f32))
        .unwrap_or_default();

    let transform = GlobalTransform {
        scale,
        opacity,
        ..Default::default()
    };

    let cmd = Command::GlobalSet {
        crosshair_id: cid,
        transform,
    };

    ctx.with_state(
        |state| match state.bus.dispatch(cmd, agent_dispatch_opts()) {
            Ok(diff) => CallToolResult::json(&serde_json::json!({
                "status": "global_set",
                "diff": diff
            })),
            Err(e) => CallToolResult::error(e.to_string()),
        },
    )
}

/// Sets crosshair symmetry configuration.
pub fn symmetry_set(ctx: &ServerContext, args: Value) -> CallToolResult {
    let cid = match ctx.resolve_crosshair_id(&args) {
        Ok(id) => id,
        Err(e) => return CallToolResult::error(e),
    };

    let symmetry: Symmetry =
        match serde_json::from_value(args.get("symmetry").cloned().unwrap_or(Value::Null)) {
            Ok(s) => s,
            Err(e) => return CallToolResult::error(format!("Invalid symmetry payload: {}", e)),
        };

    let cmd = Command::SymmetrySet {
        crosshair_id: cid,
        symmetry,
    };

    ctx.with_state(
        |state| match state.bus.dispatch(cmd, agent_dispatch_opts()) {
            Ok(diff) => CallToolResult::json(&serde_json::json!({
                "status": "symmetry_set",
                "diff": diff
            })),
            Err(e) => CallToolResult::error(e.to_string()),
        },
    )
}

/// Sets global composite effects.
pub fn effects_set(ctx: &ServerContext, args: Value) -> CallToolResult {
    let cid = match ctx.resolve_crosshair_id(&args) {
        Ok(id) => id,
        Err(e) => return CallToolResult::error(e),
    };

    let effects: Effects =
        match serde_json::from_value(args.get("effects").cloned().unwrap_or(Value::Null)) {
            Ok(e) => e,
            Err(e) => return CallToolResult::error(format!("Invalid effects payload: {}", e)),
        };

    let cmd = Command::GlobalEffectsSet {
        crosshair_id: cid,
        effects,
    };

    ctx.with_state(
        |state| match state.bus.dispatch(cmd, agent_dispatch_opts()) {
            Ok(diff) => CallToolResult::json(&serde_json::json!({
                "status": "effects_set",
                "diff": diff
            })),
            Err(e) => CallToolResult::error(e.to_string()),
        },
    )
}

/// Fine nudge in game pixels (§9.4, §13.3).
pub fn nudge(ctx: &ServerContext, args: Value) -> CallToolResult {
    let cid = match ctx.resolve_crosshair_id(&args) {
        Ok(id) => id,
        Err(e) => return CallToolResult::error(e),
    };

    let target_str = args
        .get("target")
        .and_then(|v| v.as_str())
        .unwrap_or("crosshair");
    let dx = args.get("dx").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
    let dy = args.get("dy").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
    let frame_str = args
        .get("frame")
        .and_then(|v| v.as_str())
        .unwrap_or("screen");

    let target = match target_str {
        "layer" => {
            let lid_str = match args.get("layer_id").and_then(|v| v.as_str()) {
                Some(s) => s,
                None => return CallToolResult::error("Missing layer_id for target=layer"),
            };
            let lid = match lid_str.parse::<LayerId>() {
                Ok(id) => id,
                Err(_) => return CallToolResult::error(format!("Invalid layer_id: {}", lid_str)),
            };
            NudgeTarget::Layer { id: lid }
        }
        "arm" => {
            let lid_str = match args.get("layer_id").and_then(|v| v.as_str()) {
                Some(s) => s,
                None => return CallToolResult::error("Missing layer_id for target=arm"),
            };
            let lid = match lid_str.parse::<LayerId>() {
                Ok(id) => id,
                Err(_) => return CallToolResult::error(format!("Invalid layer_id: {}", lid_str)),
            };
            let arm_index = args.get("arm_index").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
            NudgeTarget::Arm {
                layer_id: lid,
                arm_index,
            }
        }
        _ => NudgeTarget::WholeCrosshair,
    };

    let frame = match frame_str {
        "layer" => NudgeFrame::Layer,
        _ => NudgeFrame::Screen,
    };

    let cmd = Command::Nudge {
        crosshair_id: cid,
        target,
        dx,
        dy,
        frame,
    };

    ctx.with_state(
        |state| match state.bus.dispatch(cmd, agent_dispatch_opts()) {
            Ok(diff) => CallToolResult::json(&serde_json::json!({
                "status": "nudged",
                "dx": dx,
                "dy": dy,
                "diff": diff
            })),
            Err(e) => CallToolResult::error(e.to_string()),
        },
    )
}

/// Executes an atomic batch of commands.
pub fn edit_batch(ctx: &ServerContext, args: Value) -> CallToolResult {
    let commands_val = match args.get("commands").and_then(|v| v.as_array()) {
        Some(arr) => arr,
        None => return CallToolResult::error("Missing required parameter: commands"),
    };

    let mut commands = Vec::new();
    for val in commands_val {
        match serde_json::from_value::<Command>(val.clone()) {
            Ok(cmd) => commands.push(cmd),
            Err(e) => {
                return CallToolResult::error(format!("Failed to parse command in batch: {}", e))
            }
        }
    }

    let batch = Command::Batch { commands };
    ctx.with_state(
        |state| match state.bus.dispatch(batch, agent_dispatch_opts()) {
            Ok(diff) => CallToolResult::json(&serde_json::json!({
                "status": "batch_executed",
                "diff": diff
            })),
            Err(e) => CallToolResult::error(e.to_string()),
        },
    )
}

/// Undoes the last mutation.
pub fn undo(ctx: &ServerContext, args: Value) -> CallToolResult {
    let cid = match ctx.resolve_crosshair_id(&args) {
        Ok(id) => id,
        Err(e) => return CallToolResult::error(e),
    };

    ctx.with_state(|state| match state.bus.undo(cid) {
        Ok(Some(diff)) => CallToolResult::json(&serde_json::json!({
            "status": "undone",
            "diff": diff
        })),
        Ok(None) => CallToolResult::error("Nothing to undo"),
        Err(e) => CallToolResult::error(e.to_string()),
    })
}

/// Redoes the previously undone mutation.
pub fn redo(ctx: &ServerContext, args: Value) -> CallToolResult {
    let cid = match ctx.resolve_crosshair_id(&args) {
        Ok(id) => id,
        Err(e) => return CallToolResult::error(e),
    };

    ctx.with_state(|state| match state.bus.redo(cid) {
        Ok(Some(diff)) => CallToolResult::json(&serde_json::json!({
            "status": "redone",
            "diff": diff
        })),
        Ok(None) => CallToolResult::error("Nothing to redo"),
        Err(e) => CallToolResult::error(e.to_string()),
    })
}

/// Renames a layer.
pub fn layer_rename(ctx: &ServerContext, args: Value) -> CallToolResult {
    let cid = match ctx.resolve_crosshair_id(&args) {
        Ok(id) => id,
        Err(e) => return CallToolResult::error(e),
    };
    let lid_str = match args.get("layer_id").and_then(|v| v.as_str()) {
        Some(s) => s,
        None => return CallToolResult::error("Missing required parameter: layer_id"),
    };
    let lid = match lid_str.parse::<LayerId>() {
        Ok(id) => id,
        Err(_) => return CallToolResult::error(format!("Invalid layer_id: {}", lid_str)),
    };
    let new_name = match args.get("new_name").and_then(|v| v.as_str()) {
        Some(s) => s.to_string(),
        None => return CallToolResult::error("Missing required parameter: new_name"),
    };

    let cmd = Command::LayerRename {
        crosshair_id: cid,
        layer_id: lid,
        new_name,
    };
    ctx.with_state(
        |state| match state.bus.dispatch(cmd, agent_dispatch_opts()) {
            Ok(diff) => CallToolResult::json(
                &serde_json::json!({ "status": "layer_renamed", "diff": diff }),
            ),
            Err(e) => CallToolResult::error(e.to_string()),
        },
    )
}

/// Groups selected layers into a named layer group.
pub fn layer_group(ctx: &ServerContext, args: Value) -> CallToolResult {
    let cid = match ctx.resolve_crosshair_id(&args) {
        Ok(id) => id,
        Err(e) => return CallToolResult::error(e),
    };
    let ids: Vec<LayerId> = args
        .get("ids")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|x| x.as_str().and_then(|s| s.parse().ok()))
                .collect()
        })
        .unwrap_or_default();
    let name = args.get("name").and_then(|v| v.as_str()).map(String::from);

    let cmd = Command::LayerGroup {
        crosshair_id: cid,
        ids,
        name,
    };
    ctx.with_state(
        |state| match state.bus.dispatch(cmd, agent_dispatch_opts()) {
            Ok(diff) => CallToolResult::json(
                &serde_json::json!({ "status": "layer_grouped", "diff": diff }),
            ),
            Err(e) => CallToolResult::error(e.to_string()),
        },
    )
}

/// Ungroups a layer group.
pub fn layer_ungroup(ctx: &ServerContext, args: Value) -> CallToolResult {
    let cid = match ctx.resolve_crosshair_id(&args) {
        Ok(id) => id,
        Err(e) => return CallToolResult::error(e),
    };
    let gid_str = match args.get("group_id").and_then(|v| v.as_str()) {
        Some(s) => s,
        None => return CallToolResult::error("Missing required parameter: group_id"),
    };
    let group_id = match gid_str.parse::<LayerId>() {
        Ok(id) => id,
        Err(_) => return CallToolResult::error(format!("Invalid group_id: {}", gid_str)),
    };

    let cmd = Command::LayerUngroup {
        crosshair_id: cid,
        group_id,
    };
    ctx.with_state(
        |state| match state.bus.dispatch(cmd, agent_dispatch_opts()) {
            Ok(diff) => CallToolResult::json(
                &serde_json::json!({ "status": "layer_ungrouped", "diff": diff }),
            ),
            Err(e) => CallToolResult::error(e.to_string()),
        },
    )
}

/// Sets layer position explicitly.
pub fn layer_set_position(ctx: &ServerContext, args: Value) -> CallToolResult {
    let cid = match ctx.resolve_crosshair_id(&args) {
        Ok(id) => id,
        Err(e) => return CallToolResult::error(e),
    };
    let ids: Vec<LayerId> = args
        .get("ids")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|x| x.as_str().and_then(|s| s.parse().ok()))
                .collect()
        })
        .unwrap_or_default();
    let x = args.get("x").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
    let y = args.get("y").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
    let mode = match args.get("mode").and_then(|v| v.as_str()) {
        Some("absolute") => xhair_commands::command::PositionMode::Absolute,
        _ => xhair_commands::command::PositionMode::Relative,
    };

    let cmd = Command::LayerSetPosition {
        crosshair_id: cid,
        ids,
        x,
        y,
        mode,
    };
    ctx.with_state(
        |state| match state.bus.dispatch(cmd, agent_dispatch_opts()) {
            Ok(diff) => {
                CallToolResult::json(&serde_json::json!({ "status": "position_set", "diff": diff }))
            }
            Err(e) => CallToolResult::error(e.to_string()),
        },
    )
}

/// Recenters specified layers to (0, 0).
pub fn layer_recenter(ctx: &ServerContext, args: Value) -> CallToolResult {
    let cid = match ctx.resolve_crosshair_id(&args) {
        Ok(id) => id,
        Err(e) => return CallToolResult::error(e),
    };
    let ids: Vec<LayerId> = args
        .get("ids")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|x| x.as_str().and_then(|s| s.parse().ok()))
                .collect()
        })
        .unwrap_or_default();

    let cmd = Command::LayerRecenter {
        crosshair_id: cid,
        ids,
    };
    ctx.with_state(
        |state| match state.bus.dispatch(cmd, agent_dispatch_opts()) {
            Ok(diff) => CallToolResult::json(
                &serde_json::json!({ "status": "layer_recentered", "diff": diff }),
            ),
            Err(e) => CallToolResult::error(e.to_string()),
        },
    )
}

/// Recenters entire crosshair global offset to (0, 0).
pub fn crosshair_recenter(ctx: &ServerContext, args: Value) -> CallToolResult {
    let cid = match ctx.resolve_crosshair_id(&args) {
        Ok(id) => id,
        Err(e) => return CallToolResult::error(e),
    };

    let cmd = Command::CrosshairRecenter { crosshair_id: cid };
    ctx.with_state(
        |state| match state.bus.dispatch(cmd, agent_dispatch_opts()) {
            Ok(diff) => CallToolResult::json(
                &serde_json::json!({ "status": "crosshair_recentered", "diff": diff }),
            ),
            Err(e) => CallToolResult::error(e.to_string()),
        },
    )
}

/// Sets reticle fill color in hex.
pub fn paint_set(ctx: &ServerContext, args: Value) -> CallToolResult {
    let cid = match ctx.resolve_crosshair_id(&args) {
        Ok(id) => id,
        Err(e) => return CallToolResult::error(e),
    };
    let ids: Vec<LayerId> = args
        .get("ids")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|x| x.as_str().and_then(|s| s.parse().ok()))
                .collect()
        })
        .unwrap_or_default();
    let color = match args.get("color").and_then(|v| v.as_str()) {
        Some(s) => s.to_string(),
        None => return CallToolResult::error("Missing required parameter: color"),
    };

    let cmd = Command::PaintSet {
        crosshair_id: cid,
        ids,
        color,
    };
    ctx.with_state(
        |state| match state.bus.dispatch(cmd, agent_dispatch_opts()) {
            Ok(diff) => {
                CallToolResult::json(&serde_json::json!({ "status": "paint_set", "diff": diff }))
            }
            Err(e) => CallToolResult::error(e.to_string()),
        },
    )
}

/// Sets selection to specified layers.
pub fn selection_set(ctx: &ServerContext, args: Value) -> CallToolResult {
    let cid = match ctx.resolve_crosshair_id(&args) {
        Ok(id) => id,
        Err(e) => return CallToolResult::error(e),
    };
    let ids: Vec<LayerId> = args
        .get("ids")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|x| x.as_str().and_then(|s| s.parse().ok()))
                .collect()
        })
        .unwrap_or_default();

    let cmd = Command::SelectionSet {
        crosshair_id: cid,
        ids,
    };
    ctx.with_state(
        |state| match state.bus.dispatch(cmd, agent_dispatch_opts()) {
            Ok(diff) => CallToolResult::json(
                &serde_json::json!({ "status": "selection_set", "diff": diff }),
            ),
            Err(e) => CallToolResult::error(e.to_string()),
        },
    )
}

/// Adds layers to selection.
pub fn selection_add(ctx: &ServerContext, args: Value) -> CallToolResult {
    let cid = match ctx.resolve_crosshair_id(&args) {
        Ok(id) => id,
        Err(e) => return CallToolResult::error(e),
    };
    let ids: Vec<LayerId> = args
        .get("ids")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|x| x.as_str().and_then(|s| s.parse().ok()))
                .collect()
        })
        .unwrap_or_default();

    let cmd = Command::SelectionAdd {
        crosshair_id: cid,
        ids,
    };
    ctx.with_state(
        |state| match state.bus.dispatch(cmd, agent_dispatch_opts()) {
            Ok(diff) => CallToolResult::json(
                &serde_json::json!({ "status": "selection_added", "diff": diff }),
            ),
            Err(e) => CallToolResult::error(e.to_string()),
        },
    )
}

/// Removes layers from selection.
pub fn selection_remove(ctx: &ServerContext, args: Value) -> CallToolResult {
    let cid = match ctx.resolve_crosshair_id(&args) {
        Ok(id) => id,
        Err(e) => return CallToolResult::error(e),
    };
    let ids: Vec<LayerId> = args
        .get("ids")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|x| x.as_str().and_then(|s| s.parse().ok()))
                .collect()
        })
        .unwrap_or_default();

    let cmd = Command::SelectionRemove {
        crosshair_id: cid,
        ids,
    };
    ctx.with_state(
        |state| match state.bus.dispatch(cmd, agent_dispatch_opts()) {
            Ok(diff) => CallToolResult::json(
                &serde_json::json!({ "status": "selection_removed", "diff": diff }),
            ),
            Err(e) => CallToolResult::error(e.to_string()),
        },
    )
}

/// Toggles layer selection state.
pub fn selection_toggle(ctx: &ServerContext, args: Value) -> CallToolResult {
    let cid = match ctx.resolve_crosshair_id(&args) {
        Ok(id) => id,
        Err(e) => return CallToolResult::error(e),
    };
    let ids: Vec<LayerId> = args
        .get("ids")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|x| x.as_str().and_then(|s| s.parse().ok()))
                .collect()
        })
        .unwrap_or_default();

    let cmd = Command::SelectionToggle {
        crosshair_id: cid,
        ids,
    };
    ctx.with_state(
        |state| match state.bus.dispatch(cmd, agent_dispatch_opts()) {
            Ok(diff) => CallToolResult::json(
                &serde_json::json!({ "status": "selection_toggled", "diff": diff }),
            ),
            Err(e) => CallToolResult::error(e.to_string()),
        },
    )
}

/// Selects all layers.
pub fn selection_all(ctx: &ServerContext, args: Value) -> CallToolResult {
    let cid = match ctx.resolve_crosshair_id(&args) {
        Ok(id) => id,
        Err(e) => return CallToolResult::error(e),
    };

    let cmd = Command::SelectionAll { crosshair_id: cid };
    ctx.with_state(
        |state| match state.bus.dispatch(cmd, agent_dispatch_opts()) {
            Ok(diff) => {
                CallToolResult::json(&serde_json::json!({ "status": "selected_all", "diff": diff }))
            }
            Err(e) => CallToolResult::error(e.to_string()),
        },
    )
}

/// Clears selection.
pub fn selection_clear(ctx: &ServerContext, args: Value) -> CallToolResult {
    let cid = match ctx.resolve_crosshair_id(&args) {
        Ok(id) => id,
        Err(e) => return CallToolResult::error(e),
    };

    let cmd = Command::SelectionClear { crosshair_id: cid };
    ctx.with_state(
        |state| match state.bus.dispatch(cmd, agent_dispatch_opts()) {
            Ok(diff) => CallToolResult::json(
                &serde_json::json!({ "status": "selection_cleared", "diff": diff }),
            ),
            Err(e) => CallToolResult::error(e.to_string()),
        },
    )
}

/// Inverts selection.
pub fn selection_invert(ctx: &ServerContext, args: Value) -> CallToolResult {
    let cid = match ctx.resolve_crosshair_id(&args) {
        Ok(id) => id,
        Err(e) => return CallToolResult::error(e),
    };

    let cmd = Command::SelectionInvert { crosshair_id: cid };
    ctx.with_state(
        |state| match state.bus.dispatch(cmd, agent_dispatch_opts()) {
            Ok(diff) => CallToolResult::json(
                &serde_json::json!({ "status": "selection_inverted", "diff": diff }),
            ),
            Err(e) => CallToolResult::error(e.to_string()),
        },
    )
}
