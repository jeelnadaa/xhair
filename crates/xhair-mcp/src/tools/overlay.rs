//! Overlay and display management tools (§13.3).

use crate::context::ServerContext;
use crate::protocol::CallToolResult;
use serde_json::Value;
use xhair_model::units::CrosshairId;

/// Gets current overlay window state.
pub fn overlay_get_state(ctx: &ServerContext, _args: Value) -> CallToolResult {
    ctx.with_state(|state| {
        CallToolResult::json(&serde_json::json!({
            "overlay_enabled": state.overlay_enabled,
            "visibility_mode": state.visibility_mode,
            "active_crosshair_id": state.active_crosshair_id,
            "active_slot": state.active_slot,
            "calibration_active": state.calibration_active,
            "calibration_offset": state.calibration_offset
        }))
    })
}

/// Sets overlay window visibility enabled/disabled.
pub fn overlay_set_enabled(ctx: &ServerContext, args: Value) -> CallToolResult {
    let enabled = match args.get("enabled").and_then(|v| v.as_bool()) {
        Some(b) => b,
        None => return CallToolResult::error("Missing required parameter: enabled"),
    };

    ctx.with_state(|state| {
        state.overlay_enabled = enabled;
        CallToolResult::json(&serde_json::json!({
            "status": "overlay_updated",
            "overlay_enabled": enabled
        }))
    })
}

/// Activates a crosshair by ID or slot number.
pub fn overlay_activate(ctx: &ServerContext, args: Value) -> CallToolResult {
    if let Some(slot) = args.get("slot").and_then(|v| v.as_u64()) {
        let slot_u8 = slot as u8;
        return ctx.with_state(|state| {
            state.active_slot = Some(slot_u8);
            CallToolResult::json(&serde_json::json!({
                "status": "slot_activated",
                "slot": slot_u8
            }))
        });
    }

    if let Some(id_str) = args.get("id").and_then(|v| v.as_str()) {
        let id = match id_str.parse::<CrosshairId>() {
            Ok(id) => id,
            Err(_) => return CallToolResult::error(format!("Invalid UUID format: {}", id_str)),
        };
        return ctx.with_state(|state| {
            if !state.bus.documents.contains_key(&id) {
                return CallToolResult::error(format!("Crosshair not found: {}", id));
            }
            state.active_crosshair_id = Some(id);
            CallToolResult::json(&serde_json::json!({
                "status": "crosshair_activated",
                "id": id
            }))
        });
    }

    CallToolResult::error("Must provide either 'id' or 'slot'")
}

/// Cycles to the next crosshair.
pub fn overlay_next(ctx: &ServerContext, _args: Value) -> CallToolResult {
    ctx.with_state(|state| {
        let ids: Vec<_> = state.bus.documents.keys().copied().collect();
        if ids.is_empty() {
            return CallToolResult::error("No crosshairs available");
        }
        let current_idx = state
            .active_crosshair_id
            .and_then(|cid| ids.iter().position(|&id| id == cid))
            .unwrap_or(0);
        let next_idx = (current_idx + 1) % ids.len();
        state.active_crosshair_id = Some(ids[next_idx]);

        CallToolResult::json(&serde_json::json!({
            "status": "cycled_next",
            "active_crosshair_id": ids[next_idx]
        }))
    })
}

/// Cycles to the previous crosshair.
pub fn overlay_previous(ctx: &ServerContext, _args: Value) -> CallToolResult {
    ctx.with_state(|state| {
        let ids: Vec<_> = state.bus.documents.keys().copied().collect();
        if ids.is_empty() {
            return CallToolResult::error("No crosshairs available");
        }
        let current_idx = state
            .active_crosshair_id
            .and_then(|cid| ids.iter().position(|&id| id == cid))
            .unwrap_or(0);
        let prev_idx = (current_idx + ids.len() - 1) % ids.len();
        state.active_crosshair_id = Some(ids[prev_idx]);

        CallToolResult::json(&serde_json::json!({
            "status": "cycled_previous",
            "active_crosshair_id": ids[prev_idx]
        }))
    })
}

/// Lists available monitors and displays.
pub fn display_list(_ctx: &ServerContext, _args: Value) -> CallToolResult {
    // Return primary display info
    CallToolResult::json(&serde_json::json!([
        {
            "id": "\\\\.\\DISPLAY1",
            "name": "Primary Display",
            "is_primary": true,
            "width": 1920,
            "height": 1080,
            "dpi_scale": 1.0
        }
    ]))
}

/// Shows the calibration test pattern.
pub fn calibration_show(ctx: &ServerContext, _args: Value) -> CallToolResult {
    ctx.with_state(|state| {
        state.calibration_active = true;
        CallToolResult::json(&serde_json::json!({
            "status": "calibration_pattern_visible",
            "offset": state.calibration_offset
        }))
    })
}

/// Hides the calibration test pattern.
pub fn calibration_hide(ctx: &ServerContext, _args: Value) -> CallToolResult {
    ctx.with_state(|state| {
        state.calibration_active = false;
        CallToolResult::json(&serde_json::json!({
            "status": "calibration_pattern_hidden"
        }))
    })
}

/// Sets the physical calibration offset in pixels.
pub fn calibration_set_offset(ctx: &ServerContext, args: Value) -> CallToolResult {
    let dx = args.get("dx").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
    let dy = args.get("dy").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;

    ctx.with_state(|state| {
        state.calibration_offset = (dx, dy);
        CallToolResult::json(&serde_json::json!({
            "status": "offset_updated",
            "dx": dx,
            "dy": dy
        }))
    })
}
