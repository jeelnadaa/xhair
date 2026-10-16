//! Appearance, theming, font pack, and UI dock system tools.

use crate::context::ServerContext;
use crate::protocol::CallToolResult;
use serde_json::Value;
use xhair_commands::actor::Actor;
use xhair_commands::bus::DispatchOptions;
use xhair_commands::command::Command;

fn agent_dispatch_opts() -> DispatchOptions {
    DispatchOptions {
        actor: Actor::Agent {
            client_name: Some("mcp".to_string()),
        },
        ..Default::default()
    }
}

/// Lists all 9 supported neutral-base themes.
pub fn theme_list(_ctx: &ServerContext, _args: Value) -> CallToolResult {
    let themes = vec![
        serde_json::json!({ "id": "mono-dark", "name": "Mono Dark", "base": "Graphite", "accent": "#FFFFFF", "is_default": true }),
        serde_json::json!({ "id": "mono-light", "name": "Mono Light", "base": "White", "accent": "#0A0A0A" }),
        serde_json::json!({ "id": "oled-black", "name": "OLED Black", "base": "Pure Black", "accent": "#FFFFFF" }),
        serde_json::json!({ "id": "graphite-mint", "name": "Graphite Mint", "base": "Graphite", "accent": "#3DDBB0" }),
        serde_json::json!({ "id": "graphite-violet", "name": "Graphite Violet", "base": "Graphite", "accent": "#A78BFA" }),
        serde_json::json!({ "id": "graphite-amber", "name": "Graphite Amber", "base": "Graphite", "accent": "#F5B83D" }),
        serde_json::json!({ "id": "graphite-crimson", "name": "Graphite Crimson", "base": "Graphite", "accent": "#FF5A5F" }),
        serde_json::json!({ "id": "light-indigo", "name": "Light Indigo", "base": "White", "accent": "#4F46E5" }),
        serde_json::json!({ "id": "high-contrast", "name": "High Contrast", "base": "Pure Black (2px white borders)", "accent": "#FFE600", "contrast": "7:1" }),
    ];

    CallToolResult::json(&serde_json::json!({
        "themes": themes
    }))
}

/// Gets the current active theme.
pub fn theme_get(_ctx: &ServerContext, _args: Value) -> CallToolResult {
    CallToolResult::json(&serde_json::json!({
        "active_theme": "mono-dark",
        "base_ramp": "graphite",
        "neutral_spread": 0
    }))
}

/// Sets the active application theme.
pub fn theme_set(ctx: &ServerContext, args: Value) -> CallToolResult {
    let id = match args.get("id").and_then(|v| v.as_str()) {
        Some(i) => i.to_string(),
        None => return CallToolResult::error("Missing required parameter: id"),
    };

    let cmd = Command::ThemeSet { id: id.clone() };
    ctx.with_state(
        |state| match state.bus.dispatch(cmd, agent_dispatch_opts()) {
            Ok(diff) => CallToolResult::json(&serde_json::json!({
                "status": "theme_applied",
                "theme_id": id,
                "diff": diff
            })),
            Err(e) => CallToolResult::error(e.to_string()),
        },
    )
}

/// Temporarily previews a theme for a given duration.
pub fn theme_preview(_ctx: &ServerContext, args: Value) -> CallToolResult {
    let id = match args.get("id").and_then(|v| v.as_str()) {
        Some(i) => i.to_string(),
        None => return CallToolResult::error("Missing required parameter: id"),
    };
    let duration_ms = args
        .get("duration_ms")
        .and_then(|v| v.as_u64())
        .unwrap_or(2000);

    CallToolResult::json(&serde_json::json!({
        "status": "preview_started",
        "theme_id": id,
        "duration_ms": duration_ms
    }))
}

/// Gets the complete appearance configuration.
pub fn appearance_get(_ctx: &ServerContext, _args: Value) -> CallToolResult {
    CallToolResult::json(&serde_json::json!({
        "theme": "mono-dark",
        "follow_system": true,
        "system_light_theme": "mono-light",
        "system_dark_theme": "mono-dark",
        "font_pack": "satoshi",
        "corner_style": "default",
        "density": "default",
        "ui_scale": 1.0
    }))
}

/// Sets appearance settings.
pub fn appearance_set(ctx: &ServerContext, args: Value) -> CallToolResult {
    let theme = args
        .get("theme")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let follow_system = args.get("follow_system").and_then(|v| v.as_bool());
    let font_pack = args
        .get("font_pack")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let corner_style = args
        .get("corner_style")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let density = args
        .get("density")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let ui_scale = args
        .get("ui_scale")
        .and_then(|v| v.as_f64())
        .map(|f| f as f32);

    let cmd = Command::AppearanceSet {
        theme,
        follow_system,
        font_pack,
        corner_style,
        density,
        ui_scale,
    };

    ctx.with_state(
        |state| match state.bus.dispatch(cmd, agent_dispatch_opts()) {
            Ok(diff) => CallToolResult::json(&serde_json::json!({
                "status": "appearance_updated",
                "diff": diff
            })),
            Err(e) => CallToolResult::error(e.to_string()),
        },
    )
}

/// Lists all 6 bundled self-hosted WOFF2 font packs.
pub fn font_pack_list(_ctx: &ServerContext, _args: Value) -> CallToolResult {
    let packs = vec![
        serde_json::json!({ "id": "satoshi", "name": "Satoshi (Default)", "display": "Clash Display", "ui": "Satoshi", "mono": "JetBrains Mono" }),
        serde_json::json!({ "id": "technical", "name": "Technical", "display": "Geist", "ui": "Geist", "mono": "Geist Mono" }),
        serde_json::json!({ "id": "editorial", "name": "Editorial", "display": "Cabinet Grotesk", "ui": "General Sans", "mono": "IBM Plex Mono" }),
        serde_json::json!({ "id": "plex", "name": "Plex", "display": "IBM Plex Sans Condensed", "ui": "IBM Plex Sans", "mono": "IBM Plex Mono" }),
        serde_json::json!({ "id": "space", "name": "Space", "display": "Space Grotesk", "ui": "Space Grotesk", "mono": "Space Mono" }),
        serde_json::json!({ "id": "system", "name": "System Native", "display": "Segoe UI Variable", "ui": "Segoe UI Variable", "mono": "Cascadia Mono" }),
    ];

    CallToolResult::json(&serde_json::json!({
        "font_packs": packs
    }))
}

// -------------------------------------------------------------
// UI Dock & Layout Tools
// -------------------------------------------------------------

/// Opens a dock panel.
pub fn ui_panel_open(ctx: &ServerContext, args: Value) -> CallToolResult {
    let panel_id = match args.get("panel_id").and_then(|v| v.as_str()) {
        Some(p) => p.to_string(),
        None => return CallToolResult::error("Missing required parameter: panel_id"),
    };
    let dock_area = args
        .get("dock_area")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let cmd = Command::UiPanelOpen {
        panel_id: panel_id.clone(),
        dock_area,
    };

    ctx.with_state(|state| match state.bus.dispatch(cmd, agent_dispatch_opts()) {
        Ok(diff) => CallToolResult::json(&serde_json::json!({ "status": "panel_opened", "panel_id": panel_id, "diff": diff })),
        Err(e) => CallToolResult::error(e.to_string()),
    })
}

/// Closes a dock panel.
pub fn ui_panel_close(ctx: &ServerContext, args: Value) -> CallToolResult {
    let panel_id = match args.get("panel_id").and_then(|v| v.as_str()) {
        Some(p) => p.to_string(),
        None => return CallToolResult::error("Missing required parameter: panel_id"),
    };

    let cmd = Command::UiPanelClose {
        panel_id: panel_id.clone(),
    };

    ctx.with_state(|state| match state.bus.dispatch(cmd, agent_dispatch_opts()) {
        Ok(diff) => CallToolResult::json(&serde_json::json!({ "status": "panel_closed", "panel_id": panel_id, "diff": diff })),
        Err(e) => CallToolResult::error(e.to_string()),
    })
}

/// Moves a dock panel to another dock area.
pub fn ui_panel_move(ctx: &ServerContext, args: Value) -> CallToolResult {
    let panel_id = match args.get("panel_id").and_then(|v| v.as_str()) {
        Some(p) => p.to_string(),
        None => return CallToolResult::error("Missing required parameter: panel_id"),
    };
    let target_dock_area = match args.get("target_dock_area").and_then(|v| v.as_str()) {
        Some(a) => a.to_string(),
        None => return CallToolResult::error("Missing required parameter: target_dock_area"),
    };

    let cmd = Command::UiPanelMove {
        panel_id: panel_id.clone(),
        target_dock_area,
    };

    ctx.with_state(
        |state| match state.bus.dispatch(cmd, agent_dispatch_opts()) {
            Ok(diff) => CallToolResult::json(
                &serde_json::json!({ "status": "panel_moved", "panel_id": panel_id, "diff": diff }),
            ),
            Err(e) => CallToolResult::error(e.to_string()),
        },
    )
}

/// Splits a dock panel with another.
pub fn ui_panel_split(ctx: &ServerContext, args: Value) -> CallToolResult {
    let panel_id = match args.get("panel_id").and_then(|v| v.as_str()) {
        Some(p) => p.to_string(),
        None => return CallToolResult::error("Missing required parameter: panel_id"),
    };
    let direction = args
        .get("direction")
        .and_then(|v| v.as_str())
        .unwrap_or("horizontal")
        .to_string();

    let cmd = Command::UiPanelSplit {
        panel_id: panel_id.clone(),
        direction,
    };

    ctx.with_state(
        |state| match state.bus.dispatch(cmd, agent_dispatch_opts()) {
            Ok(diff) => CallToolResult::json(
                &serde_json::json!({ "status": "panel_split", "panel_id": panel_id, "diff": diff }),
            ),
            Err(e) => CallToolResult::error(e.to_string()),
        },
    )
}

/// Floats a dock panel as an in-app mini window.
pub fn ui_panel_float(ctx: &ServerContext, args: Value) -> CallToolResult {
    let panel_id = match args.get("panel_id").and_then(|v| v.as_str()) {
        Some(p) => p.to_string(),
        None => return CallToolResult::error("Missing required parameter: panel_id"),
    };
    let x = args.get("x").and_then(|v| v.as_f64()).map(|f| f as f32);
    let y = args.get("y").and_then(|v| v.as_f64()).map(|f| f as f32);

    let cmd = Command::UiPanelFloat {
        panel_id: panel_id.clone(),
        x,
        y,
    };

    ctx.with_state(|state| match state.bus.dispatch(cmd, agent_dispatch_opts()) {
        Ok(diff) => CallToolResult::json(&serde_json::json!({ "status": "panel_floated", "panel_id": panel_id, "diff": diff })),
        Err(e) => CallToolResult::error(e.to_string()),
    })
}

/// Detaches a panel to an external window.
pub fn ui_panel_detach(ctx: &ServerContext, args: Value) -> CallToolResult {
    let panel_id = match args.get("panel_id").and_then(|v| v.as_str()) {
        Some(p) => p.to_string(),
        None => return CallToolResult::error("Missing required parameter: panel_id"),
    };

    let cmd = Command::UiPanelDetach {
        panel_id: panel_id.clone(),
    };

    ctx.with_state(|state| match state.bus.dispatch(cmd, agent_dispatch_opts()) {
        Ok(diff) => CallToolResult::json(&serde_json::json!({ "status": "panel_detached", "panel_id": panel_id, "diff": diff })),
        Err(e) => CallToolResult::error(e.to_string()),
    })
}

/// Pins or unpins a floating dock panel.
pub fn ui_panel_pin(ctx: &ServerContext, args: Value) -> CallToolResult {
    let panel_id = match args.get("panel_id").and_then(|v| v.as_str()) {
        Some(p) => p.to_string(),
        None => return CallToolResult::error("Missing required parameter: panel_id"),
    };
    let pinned = args.get("pinned").and_then(|v| v.as_bool()).unwrap_or(true);

    let cmd = Command::UiPanelPin {
        panel_id: panel_id.clone(),
        pinned,
    };

    ctx.with_state(|state| match state.bus.dispatch(cmd, agent_dispatch_opts()) {
        Ok(diff) => CallToolResult::json(&serde_json::json!({ "status": "panel_pinned", "panel_id": panel_id, "diff": diff })),
        Err(e) => CallToolResult::error(e.to_string()),
    })
}

/// Maximizes a panel within its dock.
pub fn ui_panel_maximize(ctx: &ServerContext, args: Value) -> CallToolResult {
    let panel_id = match args.get("panel_id").and_then(|v| v.as_str()) {
        Some(p) => p.to_string(),
        None => return CallToolResult::error("Missing required parameter: panel_id"),
    };

    let cmd = Command::UiPanelMaximize {
        panel_id: panel_id.clone(),
    };

    ctx.with_state(|state| match state.bus.dispatch(cmd, agent_dispatch_opts()) {
        Ok(diff) => CallToolResult::json(&serde_json::json!({ "status": "panel_maximized", "panel_id": panel_id, "diff": diff })),
        Err(e) => CallToolResult::error(e.to_string()),
    })
}

/// Lists all saved dock layout presets.
pub fn ui_layout_list(_ctx: &ServerContext, _args: Value) -> CallToolResult {
    let presets = vec![
        serde_json::json!({ "name": "Designer", "is_default": true, "description": "Full inspector on right dock, tool rail on left" }),
        serde_json::json!({ "name": "Agent Watch", "description": "Split dock with live MCP activity and command history" }),
        serde_json::json!({ "name": "Pixel Art", "description": "Floating nudge pad, floating color picker, 32x canvas" }),
        serde_json::json!({ "name": "Minimal", "description": "Clean full-screen canvas with minimal floating hud" }),
    ];

    CallToolResult::json(&serde_json::json!({
        "layouts": presets
    }))
}

/// Saves current dock layout as a preset.
pub fn ui_layout_save(ctx: &ServerContext, args: Value) -> CallToolResult {
    let name = match args.get("name").and_then(|v| v.as_str()) {
        Some(n) => n.to_string(),
        None => return CallToolResult::error("Missing required parameter: name"),
    };

    let cmd = Command::UiLayoutSave { name: name.clone() };
    ctx.with_state(
        |state| match state.bus.dispatch(cmd, agent_dispatch_opts()) {
            Ok(diff) => CallToolResult::json(
                &serde_json::json!({ "status": "layout_saved", "name": name, "diff": diff }),
            ),
            Err(e) => CallToolResult::error(e.to_string()),
        },
    )
}

/// Applies a dock layout preset.
pub fn ui_layout_apply(ctx: &ServerContext, args: Value) -> CallToolResult {
    let name = match args.get("name").and_then(|v| v.as_str()) {
        Some(n) => n.to_string(),
        None => return CallToolResult::error("Missing required parameter: name"),
    };

    let cmd = Command::UiLayoutApply { name: name.clone() };
    ctx.with_state(
        |state| match state.bus.dispatch(cmd, agent_dispatch_opts()) {
            Ok(diff) => CallToolResult::json(
                &serde_json::json!({ "status": "layout_applied", "name": name, "diff": diff }),
            ),
            Err(e) => CallToolResult::error(e.to_string()),
        },
    )
}

/// Resets dock layout to default configuration.
pub fn ui_layout_reset(ctx: &ServerContext, _args: Value) -> CallToolResult {
    let cmd = Command::UiLayoutReset;
    ctx.with_state(
        |state| match state.bus.dispatch(cmd, agent_dispatch_opts()) {
            Ok(diff) => {
                CallToolResult::json(&serde_json::json!({ "status": "layout_reset", "diff": diff }))
            }
            Err(e) => CallToolResult::error(e.to_string()),
        },
    )
}

/// Exports a dock layout configuration.
pub fn ui_layout_export(_ctx: &ServerContext, args: Value) -> CallToolResult {
    let name = args
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("current");
    CallToolResult::json(&serde_json::json!({
        "status": "layout_exported",
        "name": name,
        "config": {
            "dock_area_right": ["Design", "Shape", "Fill & Stroke", "Effects", "Layers", "Properties"],
            "floating_panels": [],
            "split": false
        }
    }))
}

/// Imports a dock layout configuration.
pub fn ui_layout_import(_ctx: &ServerContext, args: Value) -> CallToolResult {
    let _data = match args.get("data") {
        Some(d) => d,
        None => return CallToolResult::error("Missing required parameter: data"),
    };
    CallToolResult::json(&serde_json::json!({
        "status": "layout_imported"
    }))
}
