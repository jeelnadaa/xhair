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

/// Lists all crosshair groups.
pub fn group_list(ctx: &ServerContext, _args: Value) -> CallToolResult {
    ctx.with_state(|state| {
        if let Some(ref engine) = state.library {
            CallToolResult::json(&engine.index().folders)
        } else {
            CallToolResult::json(&state.in_memory_groups)
        }
    })
}

/// Creates a new crosshair group.
pub fn group_create(ctx: &ServerContext, args: Value) -> CallToolResult {
    let name = match args.get("name").and_then(|v| v.as_str()) {
        Some(n) => n,
        None => return CallToolResult::error("Missing required parameter: name"),
    };
    let parent_id = args
        .get("parent_id")
        .and_then(|v| v.as_str())
        .map(String::from);
    let icon = args.get("icon").and_then(|v| v.as_str()).map(String::from);
    let color = args.get("color").and_then(|v| v.as_str()).map(String::from);

    ctx.with_state(|state| {
        if let Some(ref mut engine) = state.library {
            match engine.create_group(name, parent_id, icon, color) {
                Ok(folder) => CallToolResult::json(
                    &serde_json::json!({ "status": "group_created", "group": folder }),
                ),
                Err(e) => CallToolResult::error(e.to_string()),
            }
        } else {
            let id = uuid::Uuid::now_v7().to_string();
            let folder = xhair_library::Folder {
                id: id.clone(),
                name: name.to_string(),
                parent_id,
                icon,
                color,
                collapsed: false,
                order: state.in_memory_groups.len(),
                game_binding: None,
                active_crosshair_id: None,
                default_resolution_profile: None,
            };
            state.in_memory_groups.push(folder.clone());
            CallToolResult::json(&serde_json::json!({ "status": "group_created", "group": folder }))
        }
    })
}

/// Renames a crosshair group.
pub fn group_rename(ctx: &ServerContext, args: Value) -> CallToolResult {
    let group_id = match args.get("group_id").and_then(|v| v.as_str()) {
        Some(id) => id,
        None => return CallToolResult::error("Missing required parameter: group_id"),
    };
    let new_name = match args.get("new_name").and_then(|v| v.as_str()) {
        Some(n) => n,
        None => return CallToolResult::error("Missing required parameter: new_name"),
    };

    ctx.with_state(|state| {
        if let Some(ref mut engine) = state.library {
            match engine.rename_folder(group_id, new_name) {
                Ok(_) => CallToolResult::json(&serde_json::json!({ "status": "group_renamed", "id": group_id, "name": new_name })),
                Err(e) => CallToolResult::error(e.to_string()),
            }
        } else if let Some(g) = state.in_memory_groups.iter_mut().find(|g| g.id == group_id) {
            g.name = new_name.to_string();
            CallToolResult::json(&serde_json::json!({ "status": "group_renamed", "id": group_id, "name": new_name }))
        } else {
            CallToolResult::error(format!("Group not found: {}", group_id))
        }
    })
}

/// Deletes a crosshair group.
pub fn group_delete(ctx: &ServerContext, args: Value) -> CallToolResult {
    let group_id = match args.get("group_id").and_then(|v| v.as_str()) {
        Some(id) => id,
        None => return CallToolResult::error("Missing required parameter: group_id"),
    };
    let delete_contents = args
        .get("delete_members")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    ctx.with_state(|state| {
        if let Some(ref mut engine) = state.library {
            match engine.delete_folder(group_id, delete_contents) {
                Ok(_) => CallToolResult::json(
                    &serde_json::json!({ "status": "group_deleted", "id": group_id }),
                ),
                Err(e) => CallToolResult::error(e.to_string()),
            }
        } else {
            state.in_memory_groups.retain(|g| g.id != group_id);
            CallToolResult::json(&serde_json::json!({ "status": "group_deleted", "id": group_id }))
        }
    })
}

/// Reorders groups.
pub fn group_reorder(ctx: &ServerContext, args: Value) -> CallToolResult {
    let ordered_ids: Vec<String> = args
        .get("group_ids")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|x| x.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();

    ctx.with_state(|state| {
        if let Some(ref mut engine) = state.library {
            match engine.reorder_folders(&ordered_ids) {
                Ok(_) => CallToolResult::json(&serde_json::json!({ "status": "groups_reordered" })),
                Err(e) => CallToolResult::error(e.to_string()),
            }
        } else {
            for (idx, id) in ordered_ids.iter().enumerate() {
                if let Some(g) = state.in_memory_groups.iter_mut().find(|g| g.id == *id) {
                    g.order = idx;
                }
            }
            state.in_memory_groups.sort_by_key(|g| g.order);
            CallToolResult::json(&serde_json::json!({ "status": "groups_reordered" }))
        }
    })
}

/// Sets icon and color dot on a group.
pub fn group_set_icon(ctx: &ServerContext, args: Value) -> CallToolResult {
    let group_id = match args.get("group_id").and_then(|v| v.as_str()) {
        Some(id) => id,
        None => return CallToolResult::error("Missing required parameter: group_id"),
    };
    let icon = args.get("icon").and_then(|v| v.as_str()).map(String::from);
    let color = args.get("color").and_then(|v| v.as_str()).map(String::from);

    ctx.with_state(|state| {
        if let Some(ref mut engine) = state.library {
            match engine.set_folder_icon(group_id, icon, color) {
                Ok(_) => CallToolResult::json(
                    &serde_json::json!({ "status": "icon_set", "id": group_id }),
                ),
                Err(e) => CallToolResult::error(e.to_string()),
            }
        } else if let Some(g) = state.in_memory_groups.iter_mut().find(|g| g.id == group_id) {
            g.icon = icon;
            g.color = color;
            CallToolResult::json(&serde_json::json!({ "status": "icon_set", "id": group_id }))
        } else {
            CallToolResult::error(format!("Group not found: {}", group_id))
        }
    })
}

/// Adds crosshairs to a group.
pub fn group_add(ctx: &ServerContext, args: Value) -> CallToolResult {
    let group_id = match args.get("group_id").and_then(|v| v.as_str()) {
        Some(id) => id,
        None => return CallToolResult::error("Missing required parameter: group_id"),
    };
    let ch_ids: Vec<CrosshairId> = args
        .get("crosshair_ids")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|x| x.as_str().and_then(|s| s.parse().ok()))
                .collect()
        })
        .unwrap_or_default();

    ctx.with_state(|state| {
        if let Some(ref mut engine) = state.library {
            match engine.add_to_folder(group_id, &ch_ids) {
                Ok(_) => CallToolResult::json(
                    &serde_json::json!({ "status": "added_to_group", "count": ch_ids.len() }),
                ),
                Err(e) => CallToolResult::error(e.to_string()),
            }
        } else {
            CallToolResult::json(
                &serde_json::json!({ "status": "added_to_group", "count": ch_ids.len() }),
            )
        }
    })
}

/// Removes crosshairs from a group.
pub fn group_remove(ctx: &ServerContext, args: Value) -> CallToolResult {
    let group_id = match args.get("group_id").and_then(|v| v.as_str()) {
        Some(id) => id,
        None => return CallToolResult::error("Missing required parameter: group_id"),
    };
    let ch_ids: Vec<CrosshairId> = args
        .get("crosshair_ids")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|x| x.as_str().and_then(|s| s.parse().ok()))
                .collect()
        })
        .unwrap_or_default();

    ctx.with_state(|state| {
        if let Some(ref mut engine) = state.library {
            match engine.remove_from_folder(group_id, &ch_ids) {
                Ok(_) => CallToolResult::json(
                    &serde_json::json!({ "status": "removed_from_group", "count": ch_ids.len() }),
                ),
                Err(e) => CallToolResult::error(e.to_string()),
            }
        } else {
            CallToolResult::json(
                &serde_json::json!({ "status": "removed_from_group", "count": ch_ids.len() }),
            )
        }
    })
}

/// Sets target game executable process binding for a group.
pub fn group_set_game_binding(ctx: &ServerContext, args: Value) -> CallToolResult {
    let group_id = match args.get("group_id").and_then(|v| v.as_str()) {
        Some(id) => id,
        None => return CallToolResult::error("Missing required parameter: group_id"),
    };
    let binding = args
        .get("process_name")
        .and_then(|v| v.as_str())
        .map(String::from);

    ctx.with_state(|state| {
        if let Some(ref mut engine) = state.library {
            match engine.set_folder_game_binding(group_id, binding.clone()) {
                Ok(_) => CallToolResult::json(
                    &serde_json::json!({ "status": "game_binding_set", "binding": binding }),
                ),
                Err(e) => CallToolResult::error(e.to_string()),
            }
        } else if let Some(g) = state.in_memory_groups.iter_mut().find(|g| g.id == group_id) {
            g.game_binding = binding.clone();
            CallToolResult::json(
                &serde_json::json!({ "status": "game_binding_set", "binding": binding }),
            )
        } else {
            CallToolResult::error(format!("Group not found: {}", group_id))
        }
    })
}

/// Sets default resolution profile for a group.
pub fn group_set_resolution_profile(ctx: &ServerContext, args: Value) -> CallToolResult {
    let group_id = match args.get("group_id").and_then(|v| v.as_str()) {
        Some(id) => id,
        None => return CallToolResult::error("Missing required parameter: group_id"),
    };
    let profile = args
        .get("profile")
        .and_then(|v| v.as_str())
        .map(String::from);

    ctx.with_state(|state| {
        if let Some(ref mut engine) = state.library {
            match engine.set_folder_resolution_profile(group_id, profile.clone()) {
                Ok(_) => CallToolResult::json(
                    &serde_json::json!({ "status": "resolution_profile_set", "profile": profile }),
                ),
                Err(e) => CallToolResult::error(e.to_string()),
            }
        } else if let Some(g) = state.in_memory_groups.iter_mut().find(|g| g.id == group_id) {
            g.default_resolution_profile = profile.clone();
            CallToolResult::json(
                &serde_json::json!({ "status": "resolution_profile_set", "profile": profile }),
            )
        } else {
            CallToolResult::error(format!("Group not found: {}", group_id))
        }
    })
}

/// Sets the active crosshair within a group.
pub fn group_set_active(ctx: &ServerContext, args: Value) -> CallToolResult {
    let group_id = match args.get("group_id").and_then(|v| v.as_str()) {
        Some(id) => id,
        None => return CallToolResult::error("Missing required parameter: group_id"),
    };
    let ch_id = args
        .get("crosshair_id")
        .and_then(|v| v.as_str())
        .and_then(|s| s.parse::<CrosshairId>().ok());

    ctx.with_state(|state| {
        if let Some(ref mut engine) = state.library {
            match engine.set_folder_active(group_id, ch_id) {
                Ok(_) => CallToolResult::json(
                    &serde_json::json!({ "status": "group_active_set", "crosshair_id": ch_id }),
                ),
                Err(e) => CallToolResult::error(e.to_string()),
            }
        } else if let Some(g) = state.in_memory_groups.iter_mut().find(|g| g.id == group_id) {
            g.active_crosshair_id = ch_id;
            CallToolResult::json(
                &serde_json::json!({ "status": "group_active_set", "crosshair_id": ch_id }),
            )
        } else {
            CallToolResult::error(format!("Group not found: {}", group_id))
        }
    })
}

/// Cycles to next crosshair in the group.
pub fn group_next(ctx: &ServerContext, args: Value) -> CallToolResult {
    let group_id = match args.get("group_id").and_then(|v| v.as_str()) {
        Some(id) => id,
        None => return CallToolResult::error("Missing required parameter: group_id"),
    };

    ctx.with_state(|state| {
        if let Some(ref mut engine) = state.library {
            match engine.next_folder_crosshair(group_id) {
                Ok(Some(next_id)) => {
                    state.active_crosshair_id = Some(next_id);
                    CallToolResult::json(
                        &serde_json::json!({ "status": "cycled_next", "active_id": next_id }),
                    )
                }
                Ok(None) => CallToolResult::error("Group has no members to cycle"),
                Err(e) => CallToolResult::error(e.to_string()),
            }
        } else {
            CallToolResult::json(&serde_json::json!({ "status": "cycled_next" }))
        }
    })
}

/// Cycles to previous crosshair in the group.
pub fn group_previous(ctx: &ServerContext, args: Value) -> CallToolResult {
    let group_id = match args.get("group_id").and_then(|v| v.as_str()) {
        Some(id) => id,
        None => return CallToolResult::error("Missing required parameter: group_id"),
    };

    ctx.with_state(|state| {
        if let Some(ref mut engine) = state.library {
            match engine.previous_folder_crosshair(group_id) {
                Ok(Some(prev_id)) => {
                    state.active_crosshair_id = Some(prev_id);
                    CallToolResult::json(
                        &serde_json::json!({ "status": "cycled_previous", "active_id": prev_id }),
                    )
                }
                Ok(None) => CallToolResult::error("Group has no members to cycle"),
                Err(e) => CallToolResult::error(e.to_string()),
            }
        } else {
            CallToolResult::json(&serde_json::json!({ "status": "cycled_previous" }))
        }
    })
}

/// Activates a group's active crosshair onto the overlay.
pub fn group_activate(ctx: &ServerContext, args: Value) -> CallToolResult {
    let group_id = match args.get("group_id").and_then(|v| v.as_str()) {
        Some(id) => id,
        None => return CallToolResult::error("Missing required parameter: group_id"),
    };

    ctx.with_state(|state| {
        let active_id = if let Some(ref engine) = state.library {
            engine
                .index()
                .folders
                .iter()
                .find(|f| f.id == group_id)
                .and_then(|f| f.active_crosshair_id)
        } else {
            state
                .in_memory_groups
                .iter()
                .find(|g| g.id == group_id)
                .and_then(|g| g.active_crosshair_id)
        };

        if let Some(id) = active_id {
            state.active_crosshair_id = Some(id);
            CallToolResult::json(
                &serde_json::json!({ "status": "group_activated", "active_crosshair_id": id }),
            )
        } else {
            CallToolResult::error(format!("Group '{}' has no active crosshair", group_id))
        }
    })
}

/// Exports a crosshair group into a `.xhairpack` bundle archive.
pub fn export_pack_tool(ctx: &ServerContext, args: Value) -> CallToolResult {
    let group_id = match args.get("group_id").and_then(|v| v.as_str()) {
        Some(id) => id,
        None => return CallToolResult::error("Missing required parameter: group_id"),
    };

    ctx.with_state(|state| {
        let (group_name, binding, profile, members) = if let Some(ref engine) = state.library {
            let folder = match engine.index().folders.iter().find(|f| f.id == group_id) {
                Some(f) => f.clone(),
                None => return CallToolResult::error(format!("Group not found: {}", group_id)),
            };
            let mut xhairs = Vec::new();
            for sum in &engine.index().crosshairs {
                if sum.folder_id.as_deref() == Some(group_id) {
                    if let Ok(ch) = engine.load_crosshair(sum.id) {
                        xhairs.push(ch);
                    }
                }
            }
            (
                folder.name,
                folder.game_binding,
                folder.default_resolution_profile,
                xhairs,
            )
        } else {
            let folder = match state.in_memory_groups.iter().find(|g| g.id == group_id) {
                Some(g) => g.clone(),
                None => return CallToolResult::error(format!("Group not found: {}", group_id)),
            };
            let xhairs: Vec<_> = state.bus.documents.values().cloned().collect();
            (
                folder.name,
                folder.game_binding,
                folder.default_resolution_profile,
                xhairs,
            )
        };

        match xhair_format::bundle::export_group_pack(
            &members,
            &group_name,
            "Exported crosshair pack",
            "XHAIR User",
            binding,
            profile,
            None,
        ) {
            Ok(bytes) => {
                use base64::Engine;
                let b64 = base64::engine::general_purpose::STANDARD.encode(&bytes);
                CallToolResult::json(&serde_json::json!({
                    "status": "pack_exported",
                    "group_name": group_name,
                    "crosshair_count": members.len(),
                    "data_base64": b64
                }))
            }
            Err(e) => CallToolResult::error(e.to_string()),
        }
    })
}

/// Imports a `.xhairpack` bundle archive and registers its members.
pub fn import_pack_tool(ctx: &ServerContext, args: Value) -> CallToolResult {
    let data_b64 = match args.get("data").and_then(|v| v.as_str()) {
        Some(d) => d,
        None => return CallToolResult::error("Missing required parameter: data (base64 string)"),
    };

    use base64::Engine;
    let bytes = match base64::engine::general_purpose::STANDARD.decode(data_b64) {
        Ok(b) => b,
        Err(e) => return CallToolResult::error(format!("Invalid base64 payload: {}", e)),
    };

    let (manifest, xhairs) = match xhair_format::bundle::import_pack(&bytes) {
        Ok(pair) => pair,
        Err(e) => return CallToolResult::error(format!("Failed to unpack archive: {}", e)),
    };

    ctx.with_state(|state| {
        let group_id = uuid::Uuid::now_v7().to_string();
        let folder = xhair_library::Folder {
            id: group_id.clone(),
            name: manifest.title.clone(),
            parent_id: None,
            icon: None,
            color: None,
            collapsed: false,
            order: state.in_memory_groups.len(),
            game_binding: manifest.game_binding.clone(),
            active_crosshair_id: xhairs.first().map(|c| c.id),
            default_resolution_profile: manifest.resolution_profile.clone(),
        };

        if let Some(ref mut engine) = state.library {
            let _ = engine.create_group(&folder.name, None, None, None);
        } else {
            state.in_memory_groups.push(folder);
        }

        for ch in &xhairs {
            state.bus.register_document(ch.clone());
        }

        CallToolResult::json(&serde_json::json!({
            "status": "pack_imported",
            "group_id": group_id,
            "title": manifest.title,
            "count": xhairs.len()
        }))
    })
}
