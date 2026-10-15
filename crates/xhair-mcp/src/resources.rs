//! MCP Resources provider (§13.5).

use crate::context::ServerContext;
use crate::protocol::{ResourceContent, ResourceDefinition};
use schemars::schema_for;
use xhair_model::crosshair::Crosshair;
use xhair_model::units::CrosshairId;

/// Lists all resources provided by the MCP server.
pub fn list_resources(ctx: &ServerContext) -> Vec<ResourceDefinition> {
    let mut resources = vec![
        ResourceDefinition {
            uri: "xhair://library".to_string(),
            name: "Crosshair Library".to_string(),
            description: Some("Complete index of crosshairs, folders, and slots".to_string()),
            mime_type: "application/json".to_string(),
        },
        ResourceDefinition {
            uri: "xhair://schema/crosshair".to_string(),
            name: "Crosshair Schema".to_string(),
            description: Some("JSON Schema for xhair crosshair documents".to_string()),
            mime_type: "application/json".to_string(),
        },
        ResourceDefinition {
            uri: "xhair://settings".to_string(),
            name: "Application Settings".to_string(),
            description: Some("Current application settings and overlay state".to_string()),
            mime_type: "application/json".to_string(),
        },
        ResourceDefinition {
            uri: "xhair://resolution-profiles".to_string(),
            name: "Resolution Profiles".to_string(),
            description: Some(
                "Supported target resolution profiles and stretch ratios".to_string(),
            ),
            mime_type: "application/json".to_string(),
        },
    ];

    // Add entries for each active document
    ctx.with_state(|state| {
        for (id, ch) in &state.bus.documents {
            resources.push(ResourceDefinition {
                uri: format!("xhair://crosshair/{}", id),
                name: format!("Crosshair: {}", ch.name),
                description: Some(ch.description.clone()),
                mime_type: "application/json".to_string(),
            });
        }
    });

    resources
}

/// Reads the contents of a specified resource URI.
pub fn read_resource(ctx: &ServerContext, uri: &str) -> Result<ResourceContent, String> {
    if uri == "xhair://library" {
        return ctx.with_state(|state| {
            let list: Vec<_> = state
                .bus
                .documents
                .values()
                .map(|ch| {
                    serde_json::json!({
                        "id": ch.id,
                        "name": ch.name,
                        "tags": ch.tags,
                        "favorite": ch.favorite,
                        "layers": ch.layers.len()
                    })
                })
                .collect();
            let json = serde_json::to_string_pretty(&list).map_err(|e| e.to_string())?;
            Ok(ResourceContent {
                uri: uri.to_string(),
                mime_type: "application/json".to_string(),
                text: Some(json),
                blob: None,
            })
        });
    }

    if uri == "xhair://schema/crosshair" {
        let schema = schema_for!(Crosshair);
        let json = serde_json::to_string_pretty(&schema).map_err(|e| e.to_string())?;
        return Ok(ResourceContent {
            uri: uri.to_string(),
            mime_type: "application/json".to_string(),
            text: Some(json),
            blob: None,
        });
    }

    if uri == "xhair://settings" {
        return ctx.with_state(|state| {
            let settings = serde_json::json!({
                "overlay_enabled": state.overlay_enabled,
                "visibility_mode": state.visibility_mode,
                "active_slot": state.active_slot,
                "calibration_offset": state.calibration_offset
            });
            let json = serde_json::to_string_pretty(&settings).map_err(|e| e.to_string())?;
            Ok(ResourceContent {
                uri: uri.to_string(),
                mime_type: "application/json".to_string(),
                text: Some(json),
                blob: None,
            })
        });
    }

    if uri == "xhair://resolution-profiles" {
        let profiles = serde_json::json!([
            { "id": "1080p", "name": "1920×1080 Native", "width": 1920, "height": 1080, "is_stretched": false },
            { "id": "1440p", "name": "2560×1440 Native", "width": 2560, "height": 1440, "is_stretched": false },
            { "id": "4-3-stretch", "name": "1280×960 Stretched", "width": 1280, "height": 960, "is_stretched": true }
        ]);
        let json = serde_json::to_string_pretty(&profiles).map_err(|e| e.to_string())?;
        return Ok(ResourceContent {
            uri: uri.to_string(),
            mime_type: "application/json".to_string(),
            text: Some(json),
            blob: None,
        });
    }

    if let Some(id_part) = uri.strip_prefix("xhair://crosshair/") {
        let id = id_part
            .parse::<CrosshairId>()
            .map_err(|_| format!("Invalid crosshair ID in URI: {}", id_part))?;

        return ctx.with_state(|state| {
            if let Some(ch) = state.bus.documents.get(&id) {
                let json = serde_json::to_string_pretty(ch).map_err(|e| e.to_string())?;
                Ok(ResourceContent {
                    uri: uri.to_string(),
                    mime_type: "application/json".to_string(),
                    text: Some(json),
                    blob: None,
                })
            } else {
                Err(format!("Resource not found: {}", uri))
            }
        });
    }

    Err(format!("Unknown resource URI: {}", uri))
}
