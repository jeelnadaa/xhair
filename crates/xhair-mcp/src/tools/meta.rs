//! Metadata, capabilities, and dynamics tools (§13.3).

use crate::context::ServerContext;
use crate::protocol::CallToolResult;
use serde_json::Value;
use xhair_commands::descriptors::PropertyRegistry;

/// Reports system capabilities, runtime mode, and available tools.
pub fn meta_capabilities(ctx: &ServerContext, _args: Value) -> CallToolResult {
    ctx.with_state(|state| {
        CallToolResult::json(&serde_json::json!({
            "version": env!("CARGO_PKG_VERSION"),
            "runtime_mode": "app",
            "overlay_available": true,
            "active_crosshair_id": state.active_crosshair_id,
            "active_slot": state.active_slot,
            "transport": "stdio/http",
            "capabilities": {
                "tools": true,
                "resources": true,
                "prompts": true,
                "image_preview": true
            }
        }))
    })
}

/// Returns property descriptors and JSON schema for crosshair properties.
pub fn meta_schema(_ctx: &ServerContext, _args: Value) -> CallToolResult {
    let registry = PropertyRegistry::standard();
    CallToolResult::json(&serde_json::json!({
        "descriptors": registry.descriptors
    }))
}

/// Lists all keyboard shortcuts and command palette actions.
pub fn meta_shortcuts(_ctx: &ServerContext, _args: Value) -> CallToolResult {
    CallToolResult::json(&serde_json::json!([
        { "id": "edit.undo", "shortcut": "Ctrl+Z", "description": "Undo last action" },
        { "id": "edit.redo", "shortcut": "Ctrl+Y", "description": "Redo undone action" },
        { "id": "nudge.up", "shortcut": "ArrowUp", "description": "Fine nudge up" },
        { "id": "nudge.down", "shortcut": "ArrowDown", "description": "Fine nudge down" },
        { "id": "nudge.left", "shortcut": "ArrowLeft", "description": "Fine nudge left" },
        { "id": "nudge.right", "shortcut": "ArrowRight", "description": "Fine nudge right" },
        { "id": "view.grid", "shortcut": "G", "description": "Toggle pixel grid" },
        { "id": "overlay.toggle", "shortcut": "F10", "description": "Toggle desktop overlay" }
    ]))
}

/// Lists gun profiles (neutral templates).
pub fn gun_list(_ctx: &ServerContext, _args: Value) -> CallToolResult {
    let profiles = vec![
        xhair_dynamics::gun::GunProfile::template_rifle(),
        xhair_dynamics::gun::GunProfile::template_smg(),
        xhair_dynamics::gun::GunProfile::template_pistol(),
    ];
    CallToolResult::json(&profiles)
}

/// Simulates a spray burst or synthetic input sequence (§12.6, §13.3).
pub fn dynamics_simulate(_ctx: &ServerContext, args: Value) -> CallToolResult {
    let profile_type = args
        .get("profile")
        .and_then(|v| v.as_str())
        .unwrap_or("rifle");

    let profile = match profile_type {
        "smg" => xhair_dynamics::gun::GunProfile::template_smg(),
        "pistol" => xhair_dynamics::gun::GunProfile::template_pistol(),
        _ => xhair_dynamics::gun::GunProfile::template_rifle(),
    };

    let max_shots = args.get("shots").and_then(|v| v.as_u64()).map(|n| n as u32);
    let result = xhair_dynamics::simulator::simulate_spray(&profile, max_shots);
    CallToolResult::json(&result)
}

/// Toggles Dynamic Reticle simulation with mandatory risk confirmation dialog (§2, §12.2).
pub fn dynamics_set_enabled(ctx: &ServerContext, args: Value) -> CallToolResult {
    let confirm_risk = args
        .get("confirm_risk")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    if !confirm_risk {
        return CallToolResult::error(
            "Dynamic Reticle requires explicit user confirmation: confirm_risk: true",
        );
    }

    ctx.with_state(|state| {
        state.dynamics_confirmed = true;
        CallToolResult::json(&serde_json::json!({
            "status": "dynamics_enabled",
            "confirmed_risk": true
        }))
    })
}
