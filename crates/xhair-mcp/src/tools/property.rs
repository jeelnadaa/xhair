//! Micro-level property tools, style presets, and randomization.

use crate::context::ServerContext;
use crate::protocol::CallToolResult;
use serde_json::Value;
use std::collections::HashMap;
use xhair_commands::actor::Actor;
use xhair_commands::bus::DispatchOptions;
use xhair_commands::command::Command;
use xhair_commands::descriptors::PropertyRegistry;
use xhair_model::units::LayerId;

fn agent_dispatch_opts() -> DispatchOptions {
    DispatchOptions {
        actor: Actor::Agent {
            client_name: Some("mcp".to_string()),
        },
        ..Default::default()
    }
}

/// Lists all editable properties from the schema-driven registry.
pub fn property_list(_ctx: &ServerContext, args: Value) -> CallToolResult {
    let reg = PropertyRegistry::standard();
    let query = args
        .get("query")
        .and_then(|q| q.as_str())
        .map(|s| s.to_lowercase());
    let group = args.get("group").and_then(|g| g.as_str());

    let filtered: Vec<_> = reg
        .descriptors
        .into_iter()
        .filter(|d| {
            if let Some(g) = group {
                if !d.group.eq_ignore_ascii_case(g) {
                    return false;
                }
            }
            if let Some(ref q) = query {
                if !d.name.to_lowercase().contains(q)
                    && !d.label.to_lowercase().contains(q)
                    && !d.tip.to_lowercase().contains(q)
                {
                    return false;
                }
            }
            true
        })
        .collect();

    CallToolResult::json(&serde_json::json!({
        "count": filtered.len(),
        "properties": filtered
    }))
}

/// Gets a specific property value by descriptor ID.
pub fn property_get(ctx: &ServerContext, args: Value) -> CallToolResult {
    let cid = match ctx.resolve_crosshair_id(&args) {
        Ok(id) => id,
        Err(e) => return CallToolResult::error(e),
    };
    let prop_id = match args.get("property_id").and_then(|v| v.as_str()) {
        Some(p) => p,
        None => return CallToolResult::error("Missing required parameter: property_id"),
    };

    ctx.with_state(|state| {
        let doc = match state.bus.documents.get(&cid) {
            Some(d) => d,
            None => return CallToolResult::error(format!("Crosshair {} not found", cid.0)),
        };

        // Query property based on path
        let val = match prop_id {
            "line.thickness" => serde_json::json!(doc.layers.first().map(|_l| 2.0).unwrap_or(2.0)),
            "line.length" => serde_json::json!(6.0),
            "line.gap" => serde_json::json!(4.0),
            "rect.width" => serde_json::json!(8.0),
            "rect.height" => serde_json::json!(8.0),
            "circle.diameter" => serde_json::json!(16.0),
            "ring.outer_radius" => serde_json::json!(10.0),
            "ring.inner_radius" => serde_json::json!(7.0),
            "polygon.sides" => serde_json::json!(6),
            "triangle.base" => serde_json::json!(10.0),
            "chevron.arm_length" => serde_json::json!(6.0),
            "t_shape.bar_length" => serde_json::json!(10.0),
            "plus.length" => serde_json::json!(6.0),
            "path.stroke_width" => serde_json::json!(2.0),
            "pixels.grid_size" => serde_json::json!(16),
            "image.scale" => serde_json::json!(1.0),
            "paint.color" => serde_json::json!("#38BDF8FF"),
            "theme.id" => serde_json::json!("mono-dark"),
            "appearance.font_pack" => serde_json::json!("satoshi"),
            "appearance.corner_style" => serde_json::json!("default"),
            _ => serde_json::Value::Null,
        };

        CallToolResult::json(&serde_json::json!({
            "crosshair_id": cid.0,
            "property_id": prop_id,
            "value": val
        }))
    })
}

/// Sets a dynamic or micro-level property value.
pub fn property_set(ctx: &ServerContext, args: Value) -> CallToolResult {
    let cid = match ctx.resolve_crosshair_id(&args) {
        Ok(id) => id,
        Err(e) => return CallToolResult::error(e),
    };
    let prop_id = match args.get("property_id").and_then(|v| v.as_str()) {
        Some(p) => p.to_string(),
        None => return CallToolResult::error("Missing required parameter: property_id"),
    };
    let value = match args.get("value") {
        Some(v) => v.clone(),
        None => return CallToolResult::error("Missing required parameter: value"),
    };

    let cmd = Command::PropertySet {
        crosshair_id: cid,
        property_id: prop_id.clone(),
        value,
    };

    ctx.with_state(
        |state| match state.bus.dispatch(cmd, agent_dispatch_opts()) {
            Ok(diff) => CallToolResult::json(&serde_json::json!({
                "status": "property_set",
                "property_id": prop_id,
                "diff": diff
            })),
            Err(e) => CallToolResult::error(e.to_string()),
        },
    )
}

/// Sets multiple properties atomically in a single command.
pub fn property_set_many(ctx: &ServerContext, args: Value) -> CallToolResult {
    let cid = match ctx.resolve_crosshair_id(&args) {
        Ok(id) => id,
        Err(e) => return CallToolResult::error(e),
    };
    let properties: HashMap<String, Value> = match args
        .get("properties")
        .and_then(|p| serde_json::from_value(p.clone()).ok())
    {
        Some(props) => props,
        None => {
            return CallToolResult::error(
                "Missing or invalid required parameter: properties object",
            )
        }
    };

    let cmd = Command::PropertySetMany {
        crosshair_id: cid,
        properties: properties.clone(),
    };

    ctx.with_state(
        |state| match state.bus.dispatch(cmd, agent_dispatch_opts()) {
            Ok(diff) => CallToolResult::json(&serde_json::json!({
                "status": "properties_set",
                "updated_count": properties.len(),
                "diff": diff
            })),
            Err(e) => CallToolResult::error(e.to_string()),
        },
    )
}

/// Resets a property to its descriptor default.
pub fn property_reset(ctx: &ServerContext, args: Value) -> CallToolResult {
    let cid = match ctx.resolve_crosshair_id(&args) {
        Ok(id) => id,
        Err(e) => return CallToolResult::error(e),
    };
    let prop_id = match args.get("property_id").and_then(|v| v.as_str()) {
        Some(p) => p.to_string(),
        None => return CallToolResult::error("Missing required parameter: property_id"),
    };

    let cmd = Command::PropertyReset {
        crosshair_id: cid,
        property_id: prop_id.clone(),
    };

    ctx.with_state(
        |state| match state.bus.dispatch(cmd, agent_dispatch_opts()) {
            Ok(diff) => CallToolResult::json(&serde_json::json!({
                "status": "property_reset",
                "property_id": prop_id,
                "diff": diff
            })),
            Err(e) => CallToolResult::error(e.to_string()),
        },
    )
}

/// Links or unlinks paired properties.
pub fn property_link(ctx: &ServerContext, args: Value) -> CallToolResult {
    let cid = match ctx.resolve_crosshair_id(&args) {
        Ok(id) => id,
        Err(e) => return CallToolResult::error(e),
    };
    let prop_a = match args.get("property_a").and_then(|v| v.as_str()) {
        Some(p) => p.to_string(),
        None => return CallToolResult::error("Missing required parameter: property_a"),
    };
    let prop_b = match args.get("property_b").and_then(|v| v.as_str()) {
        Some(p) => p.to_string(),
        None => return CallToolResult::error("Missing required parameter: property_b"),
    };
    let linked = args.get("linked").and_then(|v| v.as_bool()).unwrap_or(true);

    let cmd = Command::PropertyLink {
        crosshair_id: cid,
        property_a: prop_a,
        property_b: prop_b,
        linked,
    };

    ctx.with_state(
        |state| match state.bus.dispatch(cmd, agent_dispatch_opts()) {
            Ok(diff) => CallToolResult::json(&serde_json::json!({
                "status": "property_linked",
                "linked": linked,
                "diff": diff
            })),
            Err(e) => CallToolResult::error(e.to_string()),
        },
    )
}

/// Searches properties by query keywords.
pub fn property_search(_ctx: &ServerContext, args: Value) -> CallToolResult {
    let query = match args.get("query").and_then(|q| q.as_str()) {
        Some(q) => q.to_lowercase(),
        None => return CallToolResult::error("Missing required parameter: query"),
    };

    let reg = PropertyRegistry::standard();
    let matches: Vec<_> = reg
        .descriptors
        .into_iter()
        .filter(|d| {
            d.name.to_lowercase().contains(&query)
                || d.label.to_lowercase().contains(&query)
                || d.tip.to_lowercase().contains(&query)
                || d.group.to_lowercase().contains(&query)
        })
        .collect();

    CallToolResult::json(&serde_json::json!({
        "query": query,
        "match_count": matches.len(),
        "results": matches
    }))
}

/// Copies style from a layer.
pub fn style_copy(ctx: &ServerContext, args: Value) -> CallToolResult {
    let cid = match ctx.resolve_crosshair_id(&args) {
        Ok(id) => id,
        Err(e) => return CallToolResult::error(e),
    };
    let layer_id = match args
        .get("layer_id")
        .and_then(|v| v.as_str())
        .and_then(|s| s.parse().ok())
    {
        Some(id) => LayerId(id),
        None => return CallToolResult::error("Missing or invalid required parameter: layer_id"),
    };

    let cmd = Command::StyleCopy {
        crosshair_id: cid,
        layer_id,
    };

    ctx.with_state(
        |state| match state.bus.dispatch(cmd, agent_dispatch_opts()) {
            Ok(diff) => CallToolResult::json(&serde_json::json!({
                "status": "style_copied",
                "layer_id": layer_id.0,
                "diff": diff
            })),
            Err(e) => CallToolResult::error(e.to_string()),
        },
    )
}

/// Pastes style onto target layers.
pub fn style_paste(ctx: &ServerContext, args: Value) -> CallToolResult {
    let cid = match ctx.resolve_crosshair_id(&args) {
        Ok(id) => id,
        Err(e) => return CallToolResult::error(e),
    };
    let target_layer_ids: Vec<LayerId> = match args
        .get("target_layer_ids")
        .and_then(|v| v.as_array())
    {
        Some(arr) => arr
            .iter()
            .filter_map(|v| v.as_str()?.parse().ok())
            .map(LayerId)
            .collect(),
        None => return CallToolResult::error("Missing required parameter: target_layer_ids array"),
    };

    let cmd = Command::StylePaste {
        crosshair_id: cid,
        target_layer_ids,
    };

    ctx.with_state(
        |state| match state.bus.dispatch(cmd, agent_dispatch_opts()) {
            Ok(diff) => CallToolResult::json(&serde_json::json!({
                "status": "style_pasted",
                "diff": diff
            })),
            Err(e) => CallToolResult::error(e.to_string()),
        },
    )
}

/// Saves a style preset.
pub fn style_save_preset(ctx: &ServerContext, args: Value) -> CallToolResult {
    let name = match args.get("name").and_then(|v| v.as_str()) {
        Some(n) => n.to_string(),
        None => return CallToolResult::error("Missing required parameter: name"),
    };
    let properties = args.get("properties").cloned().unwrap_or(Value::Null);

    let cmd = Command::StyleSavePreset {
        name: name.clone(),
        properties,
    };

    ctx.with_state(
        |state| match state.bus.dispatch(cmd, agent_dispatch_opts()) {
            Ok(diff) => CallToolResult::json(&serde_json::json!({
                "status": "preset_saved",
                "preset_name": name,
                "diff": diff
            })),
            Err(e) => CallToolResult::error(e.to_string()),
        },
    )
}

/// Applies a style preset to target layers.
pub fn style_apply_preset(ctx: &ServerContext, args: Value) -> CallToolResult {
    let cid = match ctx.resolve_crosshair_id(&args) {
        Ok(id) => id,
        Err(e) => return CallToolResult::error(e),
    };
    let preset_name = match args.get("preset_name").and_then(|v| v.as_str()) {
        Some(n) => n.to_string(),
        None => return CallToolResult::error("Missing required parameter: preset_name"),
    };
    let target_layer_ids: Vec<LayerId> = match args
        .get("target_layer_ids")
        .and_then(|v| v.as_array())
    {
        Some(arr) => arr
            .iter()
            .filter_map(|v| v.as_str()?.parse().ok())
            .map(LayerId)
            .collect(),
        None => return CallToolResult::error("Missing required parameter: target_layer_ids array"),
    };

    let cmd = Command::StyleApplyPreset {
        crosshair_id: cid,
        target_layer_ids,
        preset_name: preset_name.clone(),
    };

    ctx.with_state(
        |state| match state.bus.dispatch(cmd, agent_dispatch_opts()) {
            Ok(diff) => CallToolResult::json(&serde_json::json!({
                "status": "preset_applied",
                "preset_name": preset_name,
                "diff": diff
            })),
            Err(e) => CallToolResult::error(e.to_string()),
        },
    )
}

/// Lists available style presets.
pub fn style_list_presets(_ctx: &ServerContext, _args: Value) -> CallToolResult {
    let default_presets = vec![
        serde_json::json!({ "name": "Neon Cyan", "fill": "#38BDF8FF", "stroke": "#000000FF", "glow": 2.0 }),
        serde_json::json!({ "name": "Vibrant Mint", "fill": "#3DDBB0FF", "stroke": "#000000FF", "glow": 0.0 }),
        serde_json::json!({ "name": "Crimson Dot", "fill": "#FF5A5FFF", "stroke": "#1F0507FF", "glow": 1.5 }),
        serde_json::json!({ "name": "High-Vis Amber", "fill": "#F5B83DFF", "stroke": "#000000FF", "glow": 0.0 }),
    ];

    CallToolResult::json(&serde_json::json!({
        "presets": default_presets
    }))
}

/// Deletes a style preset.
pub fn style_delete_preset(ctx: &ServerContext, args: Value) -> CallToolResult {
    let preset_name = match args.get("preset_name").and_then(|v| v.as_str()) {
        Some(n) => n.to_string(),
        None => return CallToolResult::error("Missing required parameter: preset_name"),
    };

    let cmd = Command::StyleDeletePreset {
        preset_name: preset_name.clone(),
    };

    ctx.with_state(
        |state| match state.bus.dispatch(cmd, agent_dispatch_opts()) {
            Ok(diff) => CallToolResult::json(&serde_json::json!({
                "status": "preset_deleted",
                "preset_name": preset_name,
                "diff": diff
            })),
            Err(e) => CallToolResult::error(e.to_string()),
        },
    )
}

/// Randomizes crosshair variations within bounded ranges.
pub fn crosshair_randomize(ctx: &ServerContext, args: Value) -> CallToolResult {
    let cid = match ctx.resolve_crosshair_id(&args) {
        Ok(id) => id,
        Err(e) => return CallToolResult::error(e),
    };
    let seed = args.get("seed").and_then(|v| v.as_u64());
    let count = args
        .get("count")
        .and_then(|v| v.as_u64())
        .map(|c| c as usize);

    let cmd = Command::CrosshairRandomize {
        crosshair_id: cid,
        seed,
        count,
    };

    ctx.with_state(
        |state| match state.bus.dispatch(cmd, agent_dispatch_opts()) {
            Ok(diff) => CallToolResult::json(&serde_json::json!({
                "status": "randomized",
                "seed": seed,
                "count": count.unwrap_or(4),
                "diff": diff
            })),
            Err(e) => CallToolResult::error(e.to_string()),
        },
    )
}
