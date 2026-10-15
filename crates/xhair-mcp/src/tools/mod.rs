//! MCP Tools registry and dispatcher.

pub mod edit;
pub mod format;
pub mod library;
pub mod meta;
pub mod overlay;
pub mod preview;

use crate::context::ServerContext;
use crate::protocol::{CallToolResult, ToolDefinition};
use serde_json::Value;

/// Returns the complete list of available MCP tool definitions.
pub fn list_tools() -> Vec<ToolDefinition> {
    vec![
        // Library
        ToolDefinition {
            name: "library.list".to_string(),
            description: "Lists all crosshairs in the library with metadata.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {}
            }),
        },
        ToolDefinition {
            name: "library.search".to_string(),
            description: "Searches crosshairs by name, tag, or author.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "query": { "type": "string", "description": "Text query" },
                    "tag": { "type": "string", "description": "Filter by tag" }
                }
            }),
        },
        ToolDefinition {
            name: "crosshair.get".to_string(),
            description: "Gets the complete JSON definition of a crosshair by ID.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["id"],
                "properties": {
                    "id": { "type": "string", "description": "Crosshair UUID" }
                }
            }),
        },
        ToolDefinition {
            name: "crosshair.create".to_string(),
            description: "Creates a new crosshair from a template (blank, classic).".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "name": { "type": "string", "description": "Crosshair name" },
                    "template": { "type": "string", "enum": ["blank", "classic"], "description": "Template to start from" }
                }
            }),
        },
        ToolDefinition {
            name: "crosshair.duplicate".to_string(),
            description: "Duplicates an existing crosshair by ID.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["id"],
                "properties": {
                    "id": { "type": "string", "description": "Crosshair UUID" }
                }
            }),
        },
        ToolDefinition {
            name: "crosshair.delete".to_string(),
            description: "Deletes a crosshair by ID (moves to trash).".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["id"],
                "properties": {
                    "id": { "type": "string", "description": "Crosshair UUID" },
                    "dry_run": { "type": "boolean", "description": "Preview without mutating" }
                }
            }),
        },
        ToolDefinition {
            name: "crosshair.rename".to_string(),
            description: "Renames a crosshair.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["id", "new_name"],
                "properties": {
                    "id": { "type": "string", "description": "Crosshair UUID" },
                    "new_name": { "type": "string", "description": "New name" }
                }
            }),
        },
        ToolDefinition {
            name: "crosshair.set_metadata".to_string(),
            description: "Updates crosshair tags, author, description, or favorite flag."
                .to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["id"],
                "properties": {
                    "id": { "type": "string", "description": "Crosshair UUID" },
                    "description": { "type": "string" },
                    "author": { "type": "string" },
                    "favorite": { "type": "boolean" },
                    "tags": { "type": "array", "items": { "type": "string" } }
                }
            }),
        },
        // Edit
        ToolDefinition {
            name: "edit.batch".to_string(),
            description: "Executes an atomic list of commands in a single undo step.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["commands"],
                "properties": {
                    "commands": { "type": "array", "description": "List of commands" }
                }
            }),
        },
        ToolDefinition {
            name: "layer.add".to_string(),
            description: "Adds a new layer to a crosshair.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["layer"],
                "properties": {
                    "crosshair_id": { "type": "string" },
                    "layer": { "type": "object", "description": "Layer definition" }
                }
            }),
        },
        ToolDefinition {
            name: "layer.update".to_string(),
            description: "Updates properties of an existing layer.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["layer_id", "payload"],
                "properties": {
                    "crosshair_id": { "type": "string" },
                    "layer_id": { "type": "string" },
                    "payload": { "type": "object" }
                }
            }),
        },
        ToolDefinition {
            name: "layer.remove".to_string(),
            description: "Removes a layer from a crosshair.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["layer_id"],
                "properties": {
                    "crosshair_id": { "type": "string" },
                    "layer_id": { "type": "string" }
                }
            }),
        },
        ToolDefinition {
            name: "layer.duplicate".to_string(),
            description: "Duplicates a layer in the crosshair.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["layer_id"],
                "properties": {
                    "crosshair_id": { "type": "string" },
                    "layer_id": { "type": "string" }
                }
            }),
        },
        ToolDefinition {
            name: "layer.reorder".to_string(),
            description: "Changes the index of a layer in the stack.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["layer_id", "new_index"],
                "properties": {
                    "crosshair_id": { "type": "string" },
                    "layer_id": { "type": "string" },
                    "new_index": { "type": "integer" }
                }
            }),
        },
        ToolDefinition {
            name: "global.set".to_string(),
            description: "Updates global master transform (scale, opacity, rotation).".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "crosshair_id": { "type": "string" },
                    "scale": { "type": "number" },
                    "opacity": { "type": "number" }
                }
            }),
        },
        ToolDefinition {
            name: "symmetry.set".to_string(),
            description: "Configures crosshair symmetry (mode, rotational folds, mirror)."
                .to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["symmetry"],
                "properties": {
                    "crosshair_id": { "type": "string" },
                    "symmetry": { "type": "object" }
                }
            }),
        },
        ToolDefinition {
            name: "effects.set".to_string(),
            description: "Sets global composite effects (glow, shadow, blur).".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["effects"],
                "properties": {
                    "crosshair_id": { "type": "string" },
                    "effects": { "type": "object" }
                }
            }),
        },
        ToolDefinition {
            name: "nudge".to_string(),
            description: "Fine nudges crosshair, layer, or arm in game pixels.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["dx", "dy"],
                "properties": {
                    "crosshair_id": { "type": "string" },
                    "target": { "type": "string", "enum": ["crosshair", "layer", "arm"] },
                    "layer_id": { "type": "string" },
                    "arm_index": { "type": "integer" },
                    "dx": { "type": "number", "description": "Horizontal offset delta in px" },
                    "dy": { "type": "number", "description": "Vertical offset delta in px" },
                    "frame": { "type": "string", "enum": ["screen", "layer"] }
                }
            }),
        },
        ToolDefinition {
            name: "undo".to_string(),
            description: "Undoes the last mutation on the active crosshair.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "crosshair_id": { "type": "string" }
                }
            }),
        },
        ToolDefinition {
            name: "redo".to_string(),
            description: "Redoes the previously undone mutation.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "crosshair_id": { "type": "string" }
                }
            }),
        },
        // Preview & Analysis
        ToolDefinition {
            name: "render.preview".to_string(),
            description: "Renders the crosshair into a PNG image data URI.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "crosshair_id": { "type": "string" },
                    "size": { "type": "integer", "description": "Output square dimension in px" },
                    "scale": { "type": "number", "description": "Magnification scale" },
                    "dark_background": { "type": "boolean" }
                }
            }),
        },
        ToolDefinition {
            name: "analyze.alignment".to_string(),
            description: "Computes pixel alignment, center parity, and edge-vs-center report."
                .to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "crosshair_id": { "type": "string" },
                    "width": { "type": "integer", "description": "Screen width in px" },
                    "height": { "type": "integer", "description": "Screen height in px" }
                }
            }),
        },
        ToolDefinition {
            name: "analyze.visibility".to_string(),
            description: "Calculates WCAG contrast ratio against dark and light gaming backdrops."
                .to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "crosshair_id": { "type": "string" }
                }
            }),
        },
        ToolDefinition {
            name: "validate".to_string(),
            description: "Validates a crosshair against schema and complexity limits.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "crosshair": { "type": "object" },
                    "crosshair_id": { "type": "string" }
                }
            }),
        },
        // Overlay
        ToolDefinition {
            name: "overlay.get_state".to_string(),
            description: "Gets the desktop overlay visibility, slot, and calibration state."
                .to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {}
            }),
        },
        ToolDefinition {
            name: "overlay.set_enabled".to_string(),
            description: "Enables or disables desktop overlay visibility.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["enabled"],
                "properties": {
                    "enabled": { "type": "boolean" }
                }
            }),
        },
        ToolDefinition {
            name: "overlay.activate".to_string(),
            description: "Activates a crosshair for the overlay by crosshair ID or slot number."
                .to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "id": { "type": "string" },
                    "slot": { "type": "integer", "minimum": 1, "maximum": 20 }
                }
            }),
        },
        ToolDefinition {
            name: "overlay.next".to_string(),
            description: "Cycles to the next crosshair in the library.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {}
            }),
        },
        ToolDefinition {
            name: "overlay.previous".to_string(),
            description: "Cycles to the previous crosshair in the library.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {}
            }),
        },
        ToolDefinition {
            name: "display.list".to_string(),
            description: "Lists connected displays, bounds, and DPI scaling factors.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {}
            }),
        },
        ToolDefinition {
            name: "calibration.show".to_string(),
            description: "Shows the full-screen calibration alignment pattern overlay.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {}
            }),
        },
        ToolDefinition {
            name: "calibration.hide".to_string(),
            description: "Hides the calibration alignment pattern overlay.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {}
            }),
        },
        ToolDefinition {
            name: "calibration.set_offset".to_string(),
            description: "Sets the physical pixel calibration offset.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["dx", "dy"],
                "properties": {
                    "dx": { "type": "number" },
                    "dy": { "type": "number" }
                }
            }),
        },
        // Format
        ToolDefinition {
            name: "export.code".to_string(),
            description: "Exports a crosshair as a compressed Brotli share code.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "crosshair_id": { "type": "string" }
                }
            }),
        },
        ToolDefinition {
            name: "import.code".to_string(),
            description: "Imports a crosshair from a compressed Brotli share code.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["code"],
                "properties": {
                    "code": { "type": "string" }
                }
            }),
        },
        ToolDefinition {
            name: "import.valorant_code".to_string(),
            description: "Imports an in-game Valorant crosshair string.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["code"],
                "properties": {
                    "code": { "type": "string" },
                    "name": { "type": "string" }
                }
            }),
        },
        // Meta & Dynamics
        ToolDefinition {
            name: "meta.capabilities".to_string(),
            description: "Reports application capabilities, mode, and transport.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {}
            }),
        },
        ToolDefinition {
            name: "meta.schema".to_string(),
            description: "Returns property descriptors and JSON schema for crosshair models."
                .to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {}
            }),
        },
        ToolDefinition {
            name: "meta.shortcuts".to_string(),
            description: "Lists all keyboard shortcuts and command palette actions.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {}
            }),
        },
        ToolDefinition {
            name: "gun.list".to_string(),
            description: "Lists gun profile templates for dynamics simulation.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {}
            }),
        },
        ToolDefinition {
            name: "dynamics.set_enabled".to_string(),
            description: "Toggles dynamic reticle simulation with mandatory confirmation."
                .to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["confirm_risk"],
                "properties": {
                    "confirm_risk": { "type": "boolean" }
                }
            }),
        },
        ToolDefinition {
            name: "dynamics.simulate".to_string(),
            description:
                "Simulates continuous spray burst and calculates shot offsets and spread radii."
                    .to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "profile": { "type": "string", "enum": ["rifle", "smg", "pistol"] },
                    "shots": { "type": "integer", "minimum": 1 }
                }
            }),
        },
    ]
}

/// Dispatches an MCP tool call by name.
pub fn call_tool(ctx: &ServerContext, name: &str, args: Value) -> CallToolResult {
    let summary = format!("{:?}", args);
    let result = match name {
        // Library
        "library.list" => library::library_list(ctx, args),
        "library.search" => library::library_search(ctx, args),
        "crosshair.get" => library::crosshair_get(ctx, args),
        "crosshair.create" => library::crosshair_create(ctx, args),
        "crosshair.duplicate" => library::crosshair_duplicate(ctx, args),
        "crosshair.delete" => library::crosshair_delete(ctx, args),
        "crosshair.rename" => library::crosshair_rename(ctx, args),
        "crosshair.set_metadata" => library::crosshair_set_metadata(ctx, args),

        // Edit
        "edit.batch" => edit::edit_batch(ctx, args),
        "layer.add" => edit::layer_add(ctx, args),
        "layer.update" => edit::layer_update(ctx, args),
        "layer.remove" => edit::layer_remove(ctx, args),
        "layer.duplicate" => edit::layer_duplicate(ctx, args),
        "layer.reorder" => edit::layer_reorder(ctx, args),
        "global.set" => edit::global_set(ctx, args),
        "symmetry.set" => edit::symmetry_set(ctx, args),
        "effects.set" => edit::effects_set(ctx, args),
        "nudge" => edit::nudge(ctx, args),
        "undo" => edit::undo(ctx, args),
        "redo" => edit::redo(ctx, args),

        // Preview & Analysis
        "render.preview" => preview::render_preview(ctx, args),
        "analyze.alignment" => preview::analyze_alignment_tool(ctx, args),
        "analyze.visibility" => preview::analyze_visibility(ctx, args),
        "validate" => preview::validate(ctx, args),

        // Overlay
        "overlay.get_state" => overlay::overlay_get_state(ctx, args),
        "overlay.set_enabled" => overlay::overlay_set_enabled(ctx, args),
        "overlay.activate" => overlay::overlay_activate(ctx, args),
        "overlay.next" => overlay::overlay_next(ctx, args),
        "overlay.previous" => overlay::overlay_previous(ctx, args),
        "display.list" => overlay::display_list(ctx, args),
        "calibration.show" => overlay::calibration_show(ctx, args),
        "calibration.hide" => overlay::calibration_hide(ctx, args),
        "calibration.set_offset" => overlay::calibration_set_offset(ctx, args),

        // Format
        "export.code" => format::export_code(ctx, args),
        "import.code" => format::import_code(ctx, args),
        "import.valorant_code" => format::import_valorant_code(ctx, args),

        // Meta & Dynamics
        "meta.capabilities" => meta::meta_capabilities(ctx, args),
        "meta.schema" => meta::meta_schema(ctx, args),
        "meta.shortcuts" => meta::meta_shortcuts(ctx, args),
        "gun.list" => meta::gun_list(ctx, args),
        "dynamics.set_enabled" => meta::dynamics_set_enabled(ctx, args),
        "dynamics.simulate" => meta::dynamics_simulate(ctx, args),

        _ => CallToolResult::error(format!("Unknown tool: {}", name)),
    };

    let status = if result.is_error { "error" } else { "success" };
    ctx.log_activity(name, &summary, status);
    result
}
