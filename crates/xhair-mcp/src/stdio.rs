//! stdio JSON-RPC transport loop (§13.2).

use crate::context::ServerContext;
use crate::prompts::{get_prompt, list_prompts};
use crate::protocol::{JsonRpcError, JsonRpcRequest, JsonRpcResponse};
use crate::resources::{list_resources, read_resource};
use crate::tools::{call_tool, list_tools};
use std::io::{BufRead, Write};

/// Handles a single incoming JSON-RPC 2.0 request.
pub fn handle_request(ctx: &ServerContext, req: JsonRpcRequest) -> Option<JsonRpcResponse> {
    let id = req.id.clone();

    match req.method.as_str() {
        // Initialization
        "initialize" => {
            let res = serde_json::json!({
                "protocolVersion": "2024-11-05",
                "capabilities": {
                    "tools": { "listChanged": false },
                    "resources": { "subscribe": false, "listChanged": false },
                    "prompts": { "listChanged": false }
                },
                "serverInfo": {
                    "name": "xhair",
                    "version": env!("CARGO_PKG_VERSION")
                }
            });
            Some(JsonRpcResponse::success(id, res))
        }

        "notifications/initialized" => None,

        "ping" => Some(JsonRpcResponse::success(id, serde_json::json!({}))),

        // Tools
        "tools/list" => {
            let tools = list_tools();
            Some(JsonRpcResponse::success(
                id,
                serde_json::json!({ "tools": tools }),
            ))
        }

        "tools/call" => {
            let name = match req.params.get("name").and_then(|v| v.as_str()) {
                Some(n) => n,
                None => {
                    return Some(JsonRpcResponse::error(
                        id,
                        JsonRpcError::invalid_params("Missing required 'name' parameter"),
                    ));
                }
            };
            let args = req
                .params
                .get("arguments")
                .cloned()
                .unwrap_or(serde_json::json!({}));
            let call_result = call_tool(ctx, name, args);
            Some(JsonRpcResponse::success(
                id,
                serde_json::to_value(call_result).unwrap(),
            ))
        }

        // Resources
        "resources/list" => {
            let resources = list_resources(ctx);
            Some(JsonRpcResponse::success(
                id,
                serde_json::json!({ "resources": resources }),
            ))
        }

        "resources/read" => {
            let uri = match req.params.get("uri").and_then(|v| v.as_str()) {
                Some(u) => u,
                None => {
                    return Some(JsonRpcResponse::error(
                        id,
                        JsonRpcError::invalid_params("Missing required 'uri' parameter"),
                    ));
                }
            };

            match read_resource(ctx, uri) {
                Ok(content) => Some(JsonRpcResponse::success(
                    id,
                    serde_json::json!({ "contents": [content] }),
                )),
                Err(e) => Some(JsonRpcResponse::error(id, JsonRpcError::internal_error(e))),
            }
        }

        // Prompts
        "prompts/list" => {
            let prompts = list_prompts();
            Some(JsonRpcResponse::success(
                id,
                serde_json::json!({ "prompts": prompts }),
            ))
        }

        "prompts/get" => {
            let name = match req.params.get("name").and_then(|v| v.as_str()) {
                Some(n) => n,
                None => {
                    return Some(JsonRpcResponse::error(
                        id,
                        JsonRpcError::invalid_params("Missing required 'name' parameter"),
                    ));
                }
            };
            let args = req
                .params
                .get("arguments")
                .cloned()
                .unwrap_or(serde_json::json!({}));

            match get_prompt(name, args) {
                Ok((desc, messages)) => Some(JsonRpcResponse::success(
                    id,
                    serde_json::json!({
                        "description": desc,
                        "messages": messages
                    }),
                )),
                Err(e) => Some(JsonRpcResponse::error(id, JsonRpcError::invalid_params(e))),
            }
        }

        unknown => Some(JsonRpcResponse::error(
            id,
            JsonRpcError::method_not_found(unknown),
        )),
    }
}

/// Runs the stdio JSON-RPC transport loop until EOF or error.
pub fn run_stdio_transport(ctx: ServerContext) -> std::io::Result<()> {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut reader = std::io::BufReader::new(stdin.lock());
    let mut writer = std::io::BufWriter::new(stdout.lock());

    let mut line = String::new();
    while reader.read_line(&mut line)? > 0 {
        let trimmed = line.trim();
        if !trimmed.is_empty() {
            match serde_json::from_str::<JsonRpcRequest>(trimmed) {
                Ok(req) => {
                    if let Some(resp) = handle_request(&ctx, req) {
                        let json = serde_json::to_string(&resp)?;
                        writer.write_all(json.as_bytes())?;
                        writer.write_all(b"\n")?;
                        writer.flush()?;
                    }
                }
                Err(e) => {
                    let err_resp = JsonRpcResponse::error(
                        None,
                        JsonRpcError::parse_error(format!("Failed to parse JSON: {}", e)),
                    );
                    let json = serde_json::to_string(&err_resp)?;
                    writer.write_all(json.as_bytes())?;
                    writer.write_all(b"\n")?;
                    writer.flush()?;
                }
            }
        }
        line.clear();
    }

    Ok(())
}
