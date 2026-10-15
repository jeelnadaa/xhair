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
