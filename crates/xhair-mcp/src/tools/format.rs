//! Import and export format tools: share code, Valorant code (§13.3).

use crate::context::ServerContext;
use crate::protocol::CallToolResult;
use serde_json::Value;
use xhair_format::share_code::{decode_share_code, encode_share_code};
use xhair_format::valorant::import_valorant_code as parse_val_code;

/// Exports a crosshair as a compressed Brotli share code (`XH1-...`).
pub fn export_code(ctx: &ServerContext, args: Value) -> CallToolResult {
    let cid = match ctx.resolve_crosshair_id(&args) {
        Ok(id) => id,
        Err(e) => return CallToolResult::error(e),
    };

    let crosshair = match ctx.with_state(|s| s.bus.documents.get(&cid).cloned()) {
        Some(c) => c,
        None => return CallToolResult::error(format!("Crosshair not found: {}", cid)),
    };

    match encode_share_code(&crosshair) {
        Ok(code) => CallToolResult::json(&serde_json::json!({
            "status": "exported",
            "crosshair_id": cid,
            "share_code": code
        })),
        Err(e) => CallToolResult::error(format!("Share code export failed: {}", e)),
    }
}

/// Imports a crosshair from a compressed Brotli share code.
pub fn import_code(ctx: &ServerContext, args: Value) -> CallToolResult {
    let code = match args.get("code").and_then(|v| v.as_str()) {
        Some(c) => c,
        None => return CallToolResult::error("Missing required parameter: code"),
    };

    match decode_share_code(code) {
        Ok(ch) => {
            let id = ch.id;
            let name = ch.name.clone();
            ctx.with_state(|state| {
                state.bus.register_document(ch);
                state.active_crosshair_id = Some(id);
            });
            CallToolResult::json(&serde_json::json!({
                "status": "imported",
                "id": id,
                "name": name
            }))
        }
        Err(e) => CallToolResult::error(format!("Invalid share code: {}", e)),
    }
}

/// Imports an in-game Valorant crosshair string (e.g. `0;P;c;5;o;1;...`).
pub fn import_valorant_code(ctx: &ServerContext, args: Value) -> CallToolResult {
    let code = match args.get("code").and_then(|v| v.as_str()) {
        Some(c) => c,
        None => return CallToolResult::error("Missing required parameter: code"),
    };

    let name = args
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("Valorant Import");

    match parse_val_code(code) {
        Ok(import_res) => {
            let mut ch = import_res.crosshair;
            ch.name = name.to_string();
            let id = ch.id;
            ctx.with_state(|state| {
                state.bus.register_document(ch);
                state.active_crosshair_id = Some(id);
            });
            CallToolResult::json(&serde_json::json!({
                "status": "valorant_imported",
                "id": id,
                "name": name,
                "warnings": import_res.warnings
            }))
        }
        Err(e) => CallToolResult::error(format!("Valorant parse error: {}", e)),
    }
}
