//! MCP Tools registry and dispatcher.

pub mod appearance;
pub mod edit;
pub mod format;
pub mod library;
pub mod meta;
pub mod overlay;
pub mod preview;
pub mod property;

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
        ToolDefinition {
            name: "layer.rename".to_string(),
            description: "Renames a layer within a crosshair.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["layer_id", "new_name"],
                "properties": {
                    "crosshair_id": { "type": "string" },
                    "layer_id": { "type": "string" },
                    "new_name": { "type": "string" }
                }
            }),
        },
        ToolDefinition {
            name: "layer.group".to_string(),
            description: "Groups selected layers into a named layer group.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["ids"],
                "properties": {
                    "crosshair_id": { "type": "string" },
                    "ids": { "type": "array", "items": { "type": "string" } },
                    "name": { "type": "string" }
                }
            }),
        },
        ToolDefinition {
            name: "layer.ungroup".to_string(),
            description: "Dissolves a layer group back to individual layers.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["group_id"],
                "properties": {
                    "crosshair_id": { "type": "string" },
                    "group_id": { "type": "string" }
                }
            }),
        },
        ToolDefinition {
            name: "layer.set_position".to_string(),
            description: "Sets explicit layer position in game pixels (relative or absolute)."
                .to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["ids", "x", "y"],
                "properties": {
                    "crosshair_id": { "type": "string" },
                    "ids": { "type": "array", "items": { "type": "string" } },
                    "x": { "type": "number" },
                    "y": { "type": "number" },
                    "mode": { "type": "string", "enum": ["relative", "absolute"] }
                }
            }),
        },
        ToolDefinition {
            name: "layer.recenter".to_string(),
            description: "Recenters selected layers back to (0, 0).".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["ids"],
                "properties": {
                    "crosshair_id": { "type": "string" },
                    "ids": { "type": "array", "items": { "type": "string" } }
                }
            }),
        },
        ToolDefinition {
            name: "crosshair.recenter".to_string(),
            description: "Resets the whole crosshair global offset back to (0, 0).".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "crosshair_id": { "type": "string" }
                }
            }),
        },
        ToolDefinition {
            name: "paint.set".to_string(),
            description: "Sets reticle fill color in hex format (#RGB, #RRGGBB, #RRGGBBAA)."
                .to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["ids", "color"],
                "properties": {
                    "crosshair_id": { "type": "string" },
                    "ids": { "type": "array", "items": { "type": "string" } },
                    "color": { "type": "string" }
                }
            }),
        },
        ToolDefinition {
            name: "selection.set".to_string(),
            description: "Sets layer selection to the specified layer IDs.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["ids"],
                "properties": {
                    "crosshair_id": { "type": "string" },
                    "ids": { "type": "array", "items": { "type": "string" } }
                }
            }),
        },
        ToolDefinition {
            name: "selection.add".to_string(),
            description: "Adds layer IDs to current selection.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["ids"],
                "properties": {
                    "crosshair_id": { "type": "string" },
                    "ids": { "type": "array", "items": { "type": "string" } }
                }
            }),
        },
        ToolDefinition {
            name: "selection.remove".to_string(),
            description: "Removes layer IDs from current selection.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["ids"],
                "properties": {
                    "crosshair_id": { "type": "string" },
                    "ids": { "type": "array", "items": { "type": "string" } }
                }
            }),
        },
        ToolDefinition {
            name: "selection.toggle".to_string(),
            description: "Toggles selection status of specified layer IDs.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["ids"],
                "properties": {
                    "crosshair_id": { "type": "string" },
                    "ids": { "type": "array", "items": { "type": "string" } }
                }
            }),
        },
        ToolDefinition {
            name: "selection.all".to_string(),
            description: "Selects all layers in the active crosshair.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "crosshair_id": { "type": "string" }
                }
            }),
        },
        ToolDefinition {
            name: "selection.clear".to_string(),
            description: "Clears layer selection.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "crosshair_id": { "type": "string" }
                }
            }),
        },
        ToolDefinition {
            name: "selection.invert".to_string(),
            description: "Inverts layer selection.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "crosshair_id": { "type": "string" }
                }
            }),
        },
        ToolDefinition {
            name: "group.list".to_string(),
            description: "Lists all library crosshair groups with counts, bindings, and status."
                .to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {}
            }),
        },
        ToolDefinition {
            name: "group.create".to_string(),
            description: "Creates a named crosshair group collection.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["name"],
                "properties": {
                    "name": { "type": "string" },
                    "parent_id": { "type": "string" },
                    "icon": { "type": "string" },
                    "color": { "type": "string" }
                }
            }),
        },
        ToolDefinition {
            name: "group.rename".to_string(),
            description: "Renames a crosshair group.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["group_id", "new_name"],
                "properties": {
                    "group_id": { "type": "string" },
                    "new_name": { "type": "string" }
                }
            }),
        },
        ToolDefinition {
            name: "group.delete".to_string(),
            description: "Deletes a crosshair group.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["group_id"],
                "properties": {
                    "group_id": { "type": "string" },
                    "delete_members": { "type": "boolean" }
                }
            }),
        },
        ToolDefinition {
            name: "group.reorder".to_string(),
            description: "Reorders crosshair groups.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["group_ids"],
                "properties": {
                    "group_ids": { "type": "array", "items": { "type": "string" } }
                }
            }),
        },
        ToolDefinition {
            name: "group.set_icon".to_string(),
            description: "Sets the icon or color dot of a crosshair group.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["group_id"],
                "properties": {
                    "group_id": { "type": "string" },
                    "icon": { "type": "string" },
                    "color": { "type": "string" }
                }
            }),
        },
        ToolDefinition {
            name: "group.add".to_string(),
            description: "Adds crosshairs to a group.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["group_id", "crosshair_ids"],
                "properties": {
                    "group_id": { "type": "string" },
                    "crosshair_ids": { "type": "array", "items": { "type": "string" } }
                }
            }),
        },
        ToolDefinition {
            name: "group.remove".to_string(),
            description: "Removes crosshairs from a group (unassigns to Ungrouped).".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["group_id", "crosshair_ids"],
                "properties": {
                    "group_id": { "type": "string" },
                    "crosshair_ids": { "type": "array", "items": { "type": "string" } }
                }
            }),
        },
        ToolDefinition {
            name: "group.set_game_binding".to_string(),
            description: "Binds a crosshair group to a target executable process name.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["group_id"],
                "properties": {
                    "group_id": { "type": "string" },
                    "process_name": { "type": "string" }
                }
            }),
        },
        ToolDefinition {
            name: "group.set_resolution_profile".to_string(),
            description: "Sets the default resolution profile for a crosshair group.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["group_id"],
                "properties": {
                    "group_id": { "type": "string" },
                    "profile": { "type": "string" }
                }
            }),
        },
        ToolDefinition {
            name: "group.set_active".to_string(),
            description: "Sets the active crosshair within a group.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["group_id"],
                "properties": {
                    "group_id": { "type": "string" },
                    "crosshair_id": { "type": "string" }
                }
            }),
        },
        ToolDefinition {
            name: "group.next".to_string(),
            description: "Cycles to the next crosshair in the active group.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["group_id"],
                "properties": {
                    "group_id": { "type": "string" }
                }
            }),
        },
        ToolDefinition {
            name: "group.previous".to_string(),
            description: "Cycles to the previous crosshair in the active group.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["group_id"],
                "properties": {
                    "group_id": { "type": "string" }
                }
            }),
        },
        ToolDefinition {
            name: "group.activate".to_string(),
            description: "Activates a group's active crosshair onto the desktop overlay."
                .to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["group_id"],
                "properties": {
                    "group_id": { "type": "string" }
                }
            }),
        },
        ToolDefinition {
            name: "export.pack".to_string(),
            description: "Exports a crosshair group as a .xhairpack bundle archive.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["group_id"],
                "properties": {
                    "group_id": { "type": "string" }
                }
            }),
        },
        ToolDefinition {
            name: "import.pack".to_string(),
            description: "Imports a .xhairpack bundle archive into the library.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["data"],
                "properties": {
                    "data": { "type": "string", "description": "Base64-encoded zip archive" }
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
        // Properties & Styles
        ToolDefinition {
            name: "property.list".to_string(),
            description: "Lists all editable properties from the schema-driven registry."
                .to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "query": { "type": "string" },
                    "group": { "type": "string" },
                    "modified_only": { "type": "boolean" }
                }
            }),
        },
        ToolDefinition {
            name: "property.get".to_string(),
            description: "Gets the value of a property by descriptor ID.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["property_id"],
                "properties": {
                    "crosshair_id": { "type": "string" },
                    "property_id": { "type": "string" }
                }
            }),
        },
        ToolDefinition {
            name: "property.set".to_string(),
            description: "Sets a single property value by descriptor ID.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["property_id", "value"],
                "properties": {
                    "crosshair_id": { "type": "string" },
                    "property_id": { "type": "string" },
                    "value": {}
                }
            }),
        },
        ToolDefinition {
            name: "property.set_many".to_string(),
            description: "Sets multiple property values atomically.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["properties"],
                "properties": {
                    "crosshair_id": { "type": "string" },
                    "properties": { "type": "object" }
                }
            }),
        },
        ToolDefinition {
            name: "property.reset".to_string(),
            description: "Resets a property to its descriptor default value.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["property_id"],
                "properties": {
                    "crosshair_id": { "type": "string" },
                    "property_id": { "type": "string" }
                }
            }),
        },
        ToolDefinition {
            name: "property.link".to_string(),
            description: "Links or unlinks paired properties (e.g. width/height, x/y).".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["property_a", "property_b"],
                "properties": {
                    "crosshair_id": { "type": "string" },
                    "property_a": { "type": "string" },
                    "property_b": { "type": "string" },
                    "linked": { "type": "boolean" }
                }
            }),
        },
        ToolDefinition {
            name: "property.search".to_string(),
            description: "Searches property registry by keywords.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["query"],
                "properties": {
                    "query": { "type": "string" }
                }
            }),
        },
        ToolDefinition {
            name: "style.copy".to_string(),
            description: "Copies visual style from a source layer.".to_string(),
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
            name: "style.paste".to_string(),
            description: "Pastes copied visual style to target layers.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["target_layer_ids"],
                "properties": {
                    "crosshair_id": { "type": "string" },
                    "target_layer_ids": { "type": "array", "items": { "type": "string" } }
                }
            }),
        },
        ToolDefinition {
            name: "style.save_preset".to_string(),
            description: "Saves a style preset by name.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["name"],
                "properties": {
                    "name": { "type": "string" },
                    "properties": { "type": "object" }
                }
            }),
        },
        ToolDefinition {
            name: "style.apply_preset".to_string(),
            description: "Applies a style preset to target layers.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["preset_name", "target_layer_ids"],
                "properties": {
                    "crosshair_id": { "type": "string" },
                    "preset_name": { "type": "string" },
                    "target_layer_ids": { "type": "array", "items": { "type": "string" } }
                }
            }),
        },
        ToolDefinition {
            name: "style.list_presets".to_string(),
            description: "Lists all saved style presets.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {}
            }),
        },
        ToolDefinition {
            name: "style.delete_preset".to_string(),
            description: "Deletes a saved style preset.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["preset_name"],
                "properties": {
                    "preset_name": { "type": "string" }
                }
            }),
        },
        ToolDefinition {
            name: "crosshair.randomize".to_string(),
            description: "Generates bounded random variations of the crosshair.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "crosshair_id": { "type": "string" },
                    "seed": { "type": "integer" },
                    "count": { "type": "integer" }
                }
            }),
        },
        // Appearance & Themes (App-Only)
        ToolDefinition {
            name: "theme.list".to_string(),
            description: "Lists all 9 neutral-base supported themes.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {}
            }),
        },
        ToolDefinition {
            name: "theme.get".to_string(),
            description: "Gets the active application theme.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {}
            }),
        },
        ToolDefinition {
            name: "theme.set".to_string(),
            description: "Sets the active application theme.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["id"],
                "properties": {
                    "id": { "type": "string" }
                }
            }),
        },
        ToolDefinition {
            name: "theme.preview".to_string(),
            description: "Temporarily previews a theme without persisting.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["id"],
                "properties": {
                    "id": { "type": "string" },
                    "duration_ms": { "type": "integer" }
                }
            }),
        },
        ToolDefinition {
            name: "appearance.get".to_string(),
            description: "Gets full appearance configuration (theme, font, corner style)."
                .to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {}
            }),
        },
        ToolDefinition {
            name: "appearance.set".to_string(),
            description: "Sets appearance settings (theme, font pack, corner style, density)."
                .to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "theme": { "type": "string" },
                    "follow_system": { "type": "boolean" },
                    "font_pack": { "type": "string" },
                    "corner_style": { "type": "string", "enum": ["sharp", "default", "soft"] },
                    "density": { "type": "string" },
                    "ui_scale": { "type": "number" }
                }
            }),
        },
        ToolDefinition {
            name: "font_pack.list".to_string(),
            description: "Lists all 6 bundled self-hosted typography packs.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {}
            }),
        },
        // UI Dock & Layouts (App-Only)
        ToolDefinition {
            name: "ui.panel.open".to_string(),
            description: "Opens a dock panel in specified dock area.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["panel_id"],
                "properties": {
                    "panel_id": { "type": "string" },
                    "dock_area": { "type": "string" }
                }
            }),
        },
        ToolDefinition {
            name: "ui.panel.close".to_string(),
            description: "Closes a dock panel.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["panel_id"],
                "properties": {
                    "panel_id": { "type": "string" }
                }
            }),
        },
        ToolDefinition {
            name: "ui.panel.move".to_string(),
            description: "Moves a dock panel to another dock area.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["panel_id", "target_dock_area"],
                "properties": {
                    "panel_id": { "type": "string" },
                    "target_dock_area": { "type": "string" }
                }
            }),
        },
        ToolDefinition {
            name: "ui.panel.split".to_string(),
            description: "Splits a dock panel with another panel.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["panel_id"],
                "properties": {
                    "panel_id": { "type": "string" },
                    "direction": { "type": "string" }
                }
            }),
        },
        ToolDefinition {
            name: "ui.panel.float".to_string(),
            description: "Floats a dock panel as an in-app mini window.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["panel_id"],
                "properties": {
                    "panel_id": { "type": "string" },
                    "x": { "type": "number" },
                    "y": { "type": "number" }
                }
            }),
        },
        ToolDefinition {
            name: "ui.panel.detach".to_string(),
            description: "Detaches a dock panel to a separate OS window.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["panel_id"],
                "properties": {
                    "panel_id": { "type": "string" }
                }
            }),
        },
        ToolDefinition {
            name: "ui.panel.pin".to_string(),
            description: "Pins or unpins a floating dock panel.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["panel_id"],
                "properties": {
                    "panel_id": { "type": "string" },
                    "pinned": { "type": "boolean" }
                }
            }),
        },
        ToolDefinition {
            name: "ui.panel.maximize".to_string(),
            description: "Maximizes a panel within its dock area.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["panel_id"],
                "properties": {
                    "panel_id": { "type": "string" }
                }
            }),
        },
        ToolDefinition {
            name: "ui.layout.list".to_string(),
            description: "Lists all saved dock layout presets.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {}
            }),
        },
        ToolDefinition {
            name: "ui.layout.save".to_string(),
            description: "Saves current dock layout as a preset.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["name"],
                "properties": {
                    "name": { "type": "string" }
                }
            }),
        },
        ToolDefinition {
            name: "ui.layout.apply".to_string(),
            description: "Applies a saved dock layout preset.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["name"],
                "properties": {
                    "name": { "type": "string" }
                }
            }),
        },
        ToolDefinition {
            name: "ui.layout.reset".to_string(),
            description: "Resets dock layout to default configuration.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {}
            }),
        },
        ToolDefinition {
            name: "ui.layout.export".to_string(),
            description: "Exports current dock layout as JSON.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "name": { "type": "string" }
                }
            }),
        },
        ToolDefinition {
            name: "ui.layout.import".to_string(),
            description: "Imports a dock layout configuration JSON.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["data"],
                "properties": {
                    "data": { "type": "object" }
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

        // Groups & Packs
        "group.list" => library::group_list(ctx, args),
        "group.create" => library::group_create(ctx, args),
        "group.rename" => library::group_rename(ctx, args),
        "group.delete" => library::group_delete(ctx, args),
        "group.reorder" => library::group_reorder(ctx, args),
        "group.set_icon" => library::group_set_icon(ctx, args),
        "group.add" => library::group_add(ctx, args),
        "group.remove" => library::group_remove(ctx, args),
        "group.set_game_binding" => library::group_set_game_binding(ctx, args),
        "group.set_resolution_profile" => library::group_set_resolution_profile(ctx, args),
        "group.set_active" => library::group_set_active(ctx, args),
        "group.next" => library::group_next(ctx, args),
        "group.previous" => library::group_previous(ctx, args),
        "group.activate" => library::group_activate(ctx, args),
        "export.pack" => library::export_pack_tool(ctx, args),
        "import.pack" => library::import_pack_tool(ctx, args),

        // Edit
        "edit.batch" => edit::edit_batch(ctx, args),
        "layer.add" => edit::layer_add(ctx, args),
        "layer.update" => edit::layer_update(ctx, args),
        "layer.remove" => edit::layer_remove(ctx, args),
        "layer.duplicate" => edit::layer_duplicate(ctx, args),
        "layer.reorder" => edit::layer_reorder(ctx, args),
        "layer.rename" => edit::layer_rename(ctx, args),
        "layer.group" => edit::layer_group(ctx, args),
        "layer.ungroup" => edit::layer_ungroup(ctx, args),
        "layer.set_position" => edit::layer_set_position(ctx, args),
        "layer.recenter" => edit::layer_recenter(ctx, args),
        "crosshair.recenter" => edit::crosshair_recenter(ctx, args),
        "paint.set" => edit::paint_set(ctx, args),
        "selection.set" => edit::selection_set(ctx, args),
        "selection.add" => edit::selection_add(ctx, args),
        "selection.remove" => edit::selection_remove(ctx, args),
        "selection.toggle" => edit::selection_toggle(ctx, args),
        "selection.all" => edit::selection_all(ctx, args),
        "selection.clear" => edit::selection_clear(ctx, args),
        "selection.invert" => edit::selection_invert(ctx, args),
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

        // Properties, Styles & Presets
        "property.list" => property::property_list(ctx, args),
        "property.get" => property::property_get(ctx, args),
        "property.set" => property::property_set(ctx, args),
        "property.set_many" => property::property_set_many(ctx, args),
        "property.reset" => property::property_reset(ctx, args),
        "property.link" => property::property_link(ctx, args),
        "property.search" => property::property_search(ctx, args),
        "style.copy" => property::style_copy(ctx, args),
        "style.paste" => property::style_paste(ctx, args),
        "style.save_preset" => property::style_save_preset(ctx, args),
        "style.apply_preset" => property::style_apply_preset(ctx, args),
        "style.list_presets" => property::style_list_presets(ctx, args),
        "style.delete_preset" => property::style_delete_preset(ctx, args),
        "crosshair.randomize" => property::crosshair_randomize(ctx, args),

        // Appearance, Themes & Font Packs
        "theme.list" => appearance::theme_list(ctx, args),
        "theme.get" => appearance::theme_get(ctx, args),
        "theme.set" => appearance::theme_set(ctx, args),
        "theme.preview" => appearance::theme_preview(ctx, args),
        "appearance.get" => appearance::appearance_get(ctx, args),
        "appearance.set" => appearance::appearance_set(ctx, args),
        "font_pack.list" => appearance::font_pack_list(ctx, args),

        // UI Dock & Layout
        "ui.panel.open" => appearance::ui_panel_open(ctx, args),
        "ui.panel.close" => appearance::ui_panel_close(ctx, args),
        "ui.panel.move" => appearance::ui_panel_move(ctx, args),
        "ui.panel.split" => appearance::ui_panel_split(ctx, args),
        "ui.panel.float" => appearance::ui_panel_float(ctx, args),
        "ui.panel.detach" => appearance::ui_panel_detach(ctx, args),
        "ui.panel.pin" => appearance::ui_panel_pin(ctx, args),
        "ui.panel.maximize" => appearance::ui_panel_maximize(ctx, args),
        "ui.layout.list" => appearance::ui_layout_list(ctx, args),
        "ui.layout.save" => appearance::ui_layout_save(ctx, args),
        "ui.layout.apply" => appearance::ui_layout_apply(ctx, args),
        "ui.layout.reset" => appearance::ui_layout_reset(ctx, args),
        "ui.layout.export" => appearance::ui_layout_export(ctx, args),
        "ui.layout.import" => appearance::ui_layout_import(ctx, args),

        _ => CallToolResult::error(format!("Unknown tool: {}", name)),
    };

    let status = if result.is_error { "error" } else { "success" };
    ctx.log_activity(name, &summary, status);
    result
}
