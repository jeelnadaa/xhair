use std::io::{Read, Write};
use std::net::TcpStream;
use tempfile::tempdir;
use xhair_library::LibraryEngine;
use xhair_mcp::context::ServerContext;
use xhair_mcp::http::{HttpConfig, HttpServer};
use xhair_mcp::protocol::JsonRpcRequest;
use xhair_mcp::proxy::{is_endpoint_alive, proxy_request, EndpointInfo};
use xhair_mcp::stdio::handle_request;

fn create_test_context() -> ServerContext {
    let tmp = tempdir().expect("Failed to create tempdir");
    let engine = LibraryEngine::new(tmp.path().to_path_buf()).expect("Failed to open test library");
    ServerContext::new(Some(engine))
}

#[test]
fn test_initialize_handshake() {
    let ctx = create_test_context();
    let req = JsonRpcRequest::call(
        1,
        "initialize",
        Some(serde_json::json!({
            "protocolVersion": "2024-11-05",
            "clientInfo": { "name": "test-client", "version": "1.0.0" }
        })),
    );

    let resp = handle_request(&ctx, req).expect("Expected response for initialize");
    assert!(resp.error.is_none());
    let res = resp.result.expect("Expected result");
    assert_eq!(res["serverInfo"]["name"], "xhair");
    assert!(res["capabilities"]["tools"].is_object());
    assert!(res["capabilities"]["resources"].is_object());
    assert!(res["capabilities"]["prompts"].is_object());
}

#[test]
fn test_library_and_editing_flow() {
    let ctx = create_test_context();

    // 1. crosshair.create
    let create_req = JsonRpcRequest::call(
        2,
        "tools/call",
        Some(serde_json::json!({
            "name": "crosshair.create",
            "arguments": { "name": "Agent Reticle", "template": "classic" }
        })),
    );
    let create_resp = handle_request(&ctx, create_req).expect("Create response");
    assert!(create_resp.error.is_none());
    let create_text = &create_resp.result.unwrap()["content"][0]["text"];
    let create_json: serde_json::Value =
        serde_json::from_str(create_text.as_str().unwrap()).unwrap();
    let ch_id = create_json["id"].as_str().unwrap().to_string();
    assert!(!ch_id.is_empty());

    // 2. crosshair.get
    let get_req = JsonRpcRequest::call(
        3,
        "tools/call",
        Some(serde_json::json!({
            "name": "crosshair.get",
            "arguments": { "id": ch_id }
        })),
    );
    let get_resp = handle_request(&ctx, get_req).expect("Get response");
    assert!(get_resp.error.is_none());

    // 3. layer.add
    let add_layer_req = JsonRpcRequest::call(
        4,
        "tools/call",
        Some(serde_json::json!({
            "name": "layer.add",
            "arguments": {
                "crosshair_id": ch_id,
                "layer": {
                    "name": "Center Dot",
                    "visible": true,
                    "kind": { "type": "dot", "size": 3.0, "style": "square" }
                }
            }
        })),
    );
    let add_layer_resp = handle_request(&ctx, add_layer_req).expect("Add layer response");
    assert!(add_layer_resp.error.is_none());

    // 4. nudge
    let nudge_req = JsonRpcRequest::call(
        5,
        "tools/call",
        Some(serde_json::json!({
            "name": "nudge",
            "arguments": {
                "crosshair_id": ch_id,
                "dx": 1.0,
                "dy": -1.0,
                "target": "crosshair"
            }
        })),
    );
    let nudge_resp = handle_request(&ctx, nudge_req).expect("Nudge response");
    assert!(nudge_resp.error.is_none());

    // 5. undo & redo
    let undo_req = JsonRpcRequest::call(
        6,
        "tools/call",
        Some(serde_json::json!({
            "name": "undo",
            "arguments": { "crosshair_id": ch_id }
        })),
    );
    let undo_resp = handle_request(&ctx, undo_req).expect("Undo response");
    assert!(undo_resp.error.is_none());

    let redo_req = JsonRpcRequest::call(
        7,
        "tools/call",
        Some(serde_json::json!({
            "name": "redo",
            "arguments": { "crosshair_id": ch_id }
        })),
    );
    let redo_resp = handle_request(&ctx, redo_req).expect("Redo response");
    assert!(redo_resp.error.is_none());

    // 6. render.preview
    let preview_req = JsonRpcRequest::call(
        8,
        "tools/call",
        Some(serde_json::json!({
            "name": "render.preview",
            "arguments": {
                "crosshair_id": ch_id,
                "size": 64,
                "scale": 1.0
            }
        })),
    );
    let preview_resp = handle_request(&ctx, preview_req).expect("Preview response");
    assert!(preview_resp.error.is_none());
    let preview_res = &preview_resp.result.unwrap()["content"][0];
    assert_eq!(preview_res["type"], "image");
    assert_eq!(preview_res["mimeType"], "image/png");
    assert!(!preview_res["data"].as_str().unwrap().is_empty());

    // 7. analyze.alignment
    let align_req = JsonRpcRequest::call(
        9,
        "tools/call",
        Some(serde_json::json!({
            "name": "analyze.alignment",
            "arguments": { "crosshair_id": ch_id, "width": 1920, "height": 1080 }
        })),
    );
    let align_resp = handle_request(&ctx, align_req).expect("Alignment response");
    assert!(align_resp.error.is_none());

    // 8. analyze.visibility
    let vis_req = JsonRpcRequest::call(
        10,
        "tools/call",
        Some(serde_json::json!({
            "name": "analyze.visibility",
            "arguments": { "crosshair_id": ch_id }
        })),
    );
    let vis_resp = handle_request(&ctx, vis_req).expect("Visibility response");
    assert!(vis_resp.error.is_none());

    // 9. export.code and import.code
    let export_req = JsonRpcRequest::call(
        11,
        "tools/call",
        Some(serde_json::json!({
            "name": "export.code",
            "arguments": { "crosshair_id": ch_id }
        })),
    );
    let export_resp = handle_request(&ctx, export_req).expect("Export response");
    assert!(export_resp.error.is_none());
    let export_text = &export_resp.result.unwrap()["content"][0]["text"];
    let export_json: serde_json::Value =
        serde_json::from_str(export_text.as_str().unwrap()).unwrap();
    let share_code = export_json["share_code"].as_str().unwrap();
    assert!(share_code.starts_with("XH1-"));

    let import_req = JsonRpcRequest::call(
        12,
        "tools/call",
        Some(serde_json::json!({
            "name": "import.code",
            "arguments": { "code": share_code }
        })),
    );
    let import_resp = handle_request(&ctx, import_req).expect("Import response");
    assert!(import_resp.error.is_none());

    // 10. import.valorant_code
    let val_code = "0;P;c;5;o;1;d;1;z;3;f;0;0t;1;0l;3;0a;1;0f;0;1b;0";
    let val_req = JsonRpcRequest::call(
        13,
        "tools/call",
        Some(serde_json::json!({
            "name": "import.valorant_code",
            "arguments": { "code": val_code, "name": "Val Import" }
        })),
    );
    let val_resp = handle_request(&ctx, val_req).expect("Valorant import response");
    assert!(val_resp.error.is_none());

    // 11. meta.capabilities
    let meta_req = JsonRpcRequest::call(
        14,
        "tools/call",
        Some(serde_json::json!({
            "name": "meta.capabilities",
            "arguments": {}
        })),
    );
    let meta_resp = handle_request(&ctx, meta_req).expect("Capabilities response");
    assert!(meta_resp.error.is_none());
}

#[test]
fn test_resources_and_prompts() {
    let ctx = create_test_context();

    // Resources list
    let res_list_req = JsonRpcRequest::call(20, "resources/list", None);
    let res_list_resp = handle_request(&ctx, res_list_req).expect("Resources list response");
    assert!(res_list_resp.error.is_none());
    let resources = res_list_resp.result.unwrap()["resources"]
        .as_array()
        .unwrap()
        .clone();
    assert!(resources.iter().any(|r| r["uri"] == "xhair://library"));
    assert!(resources
        .iter()
        .any(|r| r["uri"] == "xhair://schema/crosshair"));

    // Resource read schema
    let read_req = JsonRpcRequest::call(
        21,
        "resources/read",
        Some(serde_json::json!({ "uri": "xhair://schema/crosshair" })),
    );
    let read_resp = handle_request(&ctx, read_req).expect("Resource read response");
    assert!(read_resp.error.is_none());
    let contents = &read_resp.result.unwrap()["contents"][0];
    assert_eq!(contents["mimeType"], "application/json");

    // Prompts list
    let prompt_list_req = JsonRpcRequest::call(22, "prompts/list", None);
    let prompt_list_resp = handle_request(&ctx, prompt_list_req).expect("Prompt list response");
    assert!(prompt_list_resp.error.is_none());
    let prompts = prompt_list_resp.result.unwrap()["prompts"]
        .as_array()
        .unwrap()
        .clone();
    assert!(prompts
        .iter()
        .any(|p| p["name"] == "design_crosshair_from_description"));

    // Prompt get
    let prompt_get_req = JsonRpcRequest::call(
        23,
        "prompts/get",
        Some(serde_json::json!({
            "name": "design_crosshair_from_description",
            "arguments": { "description": "Cyan dot with thin black outline" }
        })),
    );
    let prompt_get_resp = handle_request(&ctx, prompt_get_req).expect("Prompt get response");
    assert!(prompt_get_resp.error.is_none());
    let messages = &prompt_get_resp.result.unwrap()["messages"];
    assert!(messages.as_array().unwrap().len() >= 1);
}

#[test]
fn test_loopback_http_transport_and_security() {
    let ctx = create_test_context();
    let config = HttpConfig {
        port: 0,
        bearer_token: "test-secret-token-12345".to_string(),
    };
    let server = HttpServer::new(ctx, config.clone());
    let bound_addr = server.start().expect("Failed to start HTTP server");

    let endpoint_info = EndpointInfo {
        port: bound_addr.port(),
        token: config.bearer_token.clone(),
        pid: std::process::id(),
    };

    assert!(is_endpoint_alive(&endpoint_info));

    // 1. Test 401 Unauthorized when Authorization header is missing
    {
        let mut stream = TcpStream::connect(bound_addr).expect("Connect failed");
        let raw_req = format!(
            "POST / HTTP/1.1\r\nHost: {}\r\nContent-Type: application/json\r\nContent-Length: 2\r\nConnection: close\r\n\r\n{{}}",
            bound_addr
        );
        stream.write_all(raw_req.as_bytes()).unwrap();
        let mut resp = String::new();
        stream.read_to_string(&mut resp).unwrap();
        assert!(resp.starts_with("HTTP/1.1 401 Unauthorized"));
    }

    // 2. Test 403 Forbidden with untrusted DNS-rebinding Origin
    {
        let mut stream = TcpStream::connect(bound_addr).expect("Connect failed");
        let raw_req = format!(
            "POST / HTTP/1.1\r\nHost: {}\r\nOrigin: http://malicious-site.com\r\nAuthorization: Bearer {}\r\nContent-Type: application/json\r\nContent-Length: 2\r\nConnection: close\r\n\r\n{{}}",
            bound_addr, config.bearer_token
        );
        stream.write_all(raw_req.as_bytes()).unwrap();
        let mut resp = String::new();
        stream.read_to_string(&mut resp).unwrap();
        assert!(resp.starts_with("HTTP/1.1 403 Forbidden"));
    }

    // 3. Test Proxy Request with valid Bearer Token
    {
        let req = JsonRpcRequest::call(99, "tools/list", None);
        let resp = proxy_request(&endpoint_info, &req).expect("Proxy request failed");
        assert!(resp.error.is_none());
        let tools = resp.result.unwrap()["tools"].as_array().unwrap().clone();
        assert!(tools.len() >= 20);
    }

    server.stop();
}
