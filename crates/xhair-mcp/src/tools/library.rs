//! Library management tools: list, search, get, create, duplicate, delete, rename, metadata, trash.

use crate::context::ServerContext;
use crate::protocol::CallToolResult;
use serde_json::Value;
use xhair_model::crosshair::Crosshair;
use xhair_model::units::CrosshairId;

/// Lists all crosshairs in the library.
pub fn library_list(ctx: &ServerContext, _args: Value) -> CallToolResult {
    ctx.with_state(|state| {
        if let Some(ref engine) = state.library {
            let index = engine.index();
            CallToolResult::json(&index.crosshairs)
        } else {
            // Memory-only fallback using registered documents
            let list: Vec<_> = state
                .bus
                .documents
                .values()
                .map(|ch| {
                    serde_json::json!({
                        "id": ch.id,
                        "name": ch.name,
                        "author": ch.author,
                        "tags": ch.tags,
                        "favorite": ch.favorite,
                        "layers_count": ch.layers.len()
                    })
                })
                .collect();
            CallToolResult::json(&list)
        }
    })
}

/// Searches the library by text query or tag.
pub fn library_search(ctx: &ServerContext, args: Value) -> CallToolResult {
    let query = args.get("query").and_then(|v| v.as_str()).unwrap_or("");
    let tag = args.get("tag").and_then(|v| v.as_str());

    ctx.with_state(|state| {
        let matches: Vec<_> = state
            .bus
            .documents
            .values()
            .filter(|ch| {
                let matches_q = query.is_empty()
                    || ch.name.to_lowercase().contains(&query.to_lowercase())
                    || ch
                        .description
                        .to_lowercase()
                        .contains(&query.to_lowercase());
                let matches_tag = tag.map_or(true, |t| {
                    ch.tags.iter().any(|ct| ct.eq_ignore_ascii_case(t))
                });
                matches_q && matches_tag
            })
            .map(|ch| {
                serde_json::json!({
                    "id": ch.id,
                    "name": ch.name,
                    "author": ch.author,
                    "tags": ch.tags,
                    "favorite": ch.favorite,
                    "layers_count": ch.layers.len()
                })
            })
            .collect();

        CallToolResult::json(&matches)
    })
}

/// Gets the full crosshair JSON by ID.
pub fn crosshair_get(ctx: &ServerContext, args: Value) -> CallToolResult {
    let id_str = match args.get("id").and_then(|v| v.as_str()) {
        Some(s) => s,
        None => return CallToolResult::error("Missing required parameter: id"),
    };

    let id = match id_str.parse::<CrosshairId>() {
        Ok(id) => id,
        Err(_) => return CallToolResult::error(format!("Invalid UUID format: {}", id_str)),
    };

    ctx.with_state(|state| {
        if let Some(ch) = state.bus.documents.get(&id) {
            CallToolResult::json(ch)
        } else {
            CallToolResult::error(format!("Crosshair with id {} not found", id))
        }
    })
}

/// Creates a new crosshair from template or blank.
pub fn crosshair_create(ctx: &ServerContext, args: Value) -> CallToolResult {
    let name = args
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("New Crosshair");
    let template = args
        .get("template")
        .and_then(|v| v.as_str())
        .unwrap_or("blank");

    let ch = match template {
        "classic" => Crosshair::new_classic(name),
        _ => Crosshair::new_blank(name),
    };

    let id = ch.id;
    ctx.with_state(|state| {
        state.bus.register_document(ch);
        state.active_crosshair_id = Some(id);
    });

    CallToolResult::json(&serde_json::json!({
        "status": "created",
        "id": id,
        "name": name,
        "template": template
    }))
}

/// Duplicates an existing crosshair.
pub fn crosshair_duplicate(ctx: &ServerContext, args: Value) -> CallToolResult {
    let id_str = match args.get("id").and_then(|v| v.as_str()) {
        Some(s) => s,
        None => return CallToolResult::error("Missing required parameter: id"),
    };

    let id = match id_str.parse::<CrosshairId>() {
        Ok(id) => id,
        Err(_) => return CallToolResult::error(format!("Invalid UUID format: {}", id_str)),
    };

    ctx.with_state(|state| {
        let orig = match state.bus.documents.get(&id) {
            Some(c) => c.clone(),
            None => return CallToolResult::error(format!("Crosshair not found: {}", id)),
        };

        let mut cloned = orig;
        cloned.id = CrosshairId::new_v7();
        cloned.name = format!("{} (Copy)", cloned.name);
        let new_id = cloned.id;

        state.bus.register_document(cloned);
        CallToolResult::json(&serde_json::json!({
            "status": "duplicated",
            "original_id": id,
            "new_id": new_id
        }))
    })
}

/// Deletes a crosshair by ID.
pub fn crosshair_delete(ctx: &ServerContext, args: Value) -> CallToolResult {
    let id_str = match args.get("id").and_then(|v| v.as_str()) {
        Some(s) => s,
        None => return CallToolResult::error("Missing required parameter: id"),
    };

    let id = match id_str.parse::<CrosshairId>() {
        Ok(id) => id,
        Err(_) => return CallToolResult::error(format!("Invalid UUID format: {}", id_str)),
    };

    let dry_run = args
        .get("dry_run")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    ctx.with_state(|state| {
        if !state.bus.documents.contains_key(&id) {
            return CallToolResult::error(format!("Crosshair not found: {}", id));
        }

        if !dry_run {
            state.bus.documents.remove(&id);
            if state.active_crosshair_id == Some(id) {
                state.active_crosshair_id = state.bus.documents.keys().next().copied();
            }
        }

        CallToolResult::json(&serde_json::json!({
            "status": if dry_run { "would_delete" } else { "deleted" },
            "id": id
        }))
    })
}

/// Renames a crosshair.
pub fn crosshair_rename(ctx: &ServerContext, args: Value) -> CallToolResult {
    let id_str = match args.get("id").and_then(|v| v.as_str()) {
        Some(s) => s,
        None => return CallToolResult::error("Missing required parameter: id"),
    };
    let new_name = match args.get("new_name").and_then(|v| v.as_str()) {
        Some(n) => n,
        None => return CallToolResult::error("Missing required parameter: new_name"),
    };

    let id = match id_str.parse::<CrosshairId>() {
        Ok(id) => id,
        Err(_) => return CallToolResult::error(format!("Invalid UUID format: {}", id_str)),
    };

    ctx.with_state(|state| {
        if let Some(ch) = state.bus.documents.get_mut(&id) {
            ch.name = new_name.to_string();
            CallToolResult::json(&serde_json::json!({
                "status": "renamed",
                "id": id,
                "name": new_name
            }))
        } else {
            CallToolResult::error(format!("Crosshair not found: {}", id))
        }
    })
}

/// Updates crosshair metadata.
pub fn crosshair_set_metadata(ctx: &ServerContext, args: Value) -> CallToolResult {
    let id_str = match args.get("id").and_then(|v| v.as_str()) {
        Some(s) => s,
        None => return CallToolResult::error("Missing required parameter: id"),
    };

    let id = match id_str.parse::<CrosshairId>() {
        Ok(id) => id,
        Err(_) => return CallToolResult::error(format!("Invalid UUID format: {}", id_str)),
    };

    ctx.with_state(|state| {
        if let Some(ch) = state.bus.documents.get_mut(&id) {
            if let Some(desc) = args.get("description").and_then(|v| v.as_str()) {
                ch.description = desc.to_string();
            }
            if let Some(author) = args.get("author").and_then(|v| v.as_str()) {
                ch.author = author.to_string();
            }
            if let Some(fav) = args.get("favorite").and_then(|v| v.as_bool()) {
                ch.favorite = fav;
            }
            if let Some(tags_val) = args.get("tags").and_then(|v| v.as_array()) {
                ch.tags = tags_val
                    .iter()
                    .filter_map(|t| t.as_str().map(|s| s.to_string()))
                    .collect();
            }

            CallToolResult::json(&serde_json::json!({
                "status": "metadata_updated",
                "id": id
            }))
        } else {
            CallToolResult::error(format!("Crosshair not found: {}", id))
        }
    })
}
