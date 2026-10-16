//! Local HTTP server, overlay API, and embedded asset delivery.

use rust_embed::RustEmbed;
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::thread;
use xhair_platform_win::{OverlayCommand, OverlaySender};

/// Compile-time embedded UI assets from ui/dist.
#[derive(RustEmbed)]
#[folder = "../../ui/dist/"]
pub struct EmbeddedDist;

/// Finds the directory containing ui/dist assets on disk (optional override).
pub fn find_ui_dist_dir() -> Option<PathBuf> {
    // 1. Current working directory
    let cwd_dist = PathBuf::from("ui/dist");
    if cwd_dist.join("index.html").exists() {
        return Some(cwd_dist);
    }

    // 2. Relative to workspace root if running from a crate dir
    let crate_dist = PathBuf::from("../../ui/dist");
    if crate_dist.join("index.html").exists() {
        return Some(crate_dist);
    }

    // 3. Beside executable
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let exe_dist = exe_dir.join("ui").join("dist");
            if exe_dist.join("index.html").exists() {
                return Some(exe_dist);
            }
            let flat_dist = exe_dir.join("dist");
            if flat_dist.join("index.html").exists() {
                return Some(flat_dist);
            }
            // If running inside target/debug or target/release
            if let Some(target_dir) = exe_dir.parent() {
                if let Some(ws_dir) = target_dir.parent() {
                    let ws_dist = ws_dir.join("ui").join("dist");
                    if ws_dist.join("index.html").exists() {
                        return Some(ws_dist);
                    }
                }
            }
        }
    }

    // 4. APPDATA/xhair/ui
    if let Ok(appdata) = std::env::var("APPDATA") {
        let appdata_dist = PathBuf::from(appdata).join("xhair").join("ui");
        if appdata_dist.join("index.html").exists() {
            return Some(appdata_dist);
        }
    }

    None
}

/// Starts the local HTTP server in a background thread and returns the port.
pub fn start_ui_server(
    overlay_sender: Option<OverlaySender>,
    mcp_ctx: Option<xhair_mcp::ServerContext>,
) -> Option<u16> {
    // Try binding to port 4173 or an OS-assigned free port
    let listener = TcpListener::bind("127.0.0.1:4173")
        .or_else(|_| TcpListener::bind("127.0.0.1:0"))
        .ok()?;
    let port = listener.local_addr().ok()?.port();

    thread::Builder::new()
        .name("xhair-ui-server".to_string())
        .spawn(move || {
            for stream in listener.incoming().flatten() {
                let sender_clone = overlay_sender.clone();
                let mcp_clone = mcp_ctx.clone();
                thread::spawn(move || {
                    handle_connection(stream, sender_clone.as_ref(), mcp_clone.as_ref());
                });
            }
        })
        .ok()?;

    Some(port)
}

fn guess_mime(name: &str) -> &'static str {
    if name.ends_with(".js") {
        "application/javascript; charset=utf-8"
    } else if name.ends_with(".css") {
        "text/css; charset=utf-8"
    } else if name.ends_with(".html") {
        "text/html; charset=utf-8"
    } else if name.ends_with(".svg") {
        "image/svg+xml"
    } else if name.ends_with(".png") {
        "image/png"
    } else if name.ends_with(".woff2") {
        "font/woff2"
    } else if name.ends_with(".woff") {
        "font/woff"
    } else if name.ends_with(".ttf") {
        "font/ttf"
    } else if name.ends_with(".json") {
        "application/json"
    } else {
        "application/octet-stream"
    }
}

fn read_http_request(stream: &mut TcpStream) -> Option<(String, String, Vec<u8>)> {
    let mut buffer = Vec::with_capacity(4096);
    let mut chunk = [0u8; 2048];
    let mut content_length = None;
    let mut header_end = None;

    loop {
        let n = stream.read(&mut chunk).ok()?;
        if n == 0 {
            break;
        }
        buffer.extend_from_slice(&chunk[..n]);

        if header_end.is_none() {
            if let Some(pos) = buffer.windows(4).position(|w| w == b"\r\n\r\n") {
                header_end = Some(pos + 4);
                let header_str = String::from_utf8_lossy(&buffer[..pos]);
                for line in header_str.lines() {
                    if line.to_ascii_lowercase().starts_with("content-length:") {
                        if let Some(val_str) = line.split(':').nth(1) {
                            content_length = val_str.trim().parse::<usize>().ok();
                        }
                    }
                }
            }
        }

        if let Some(end) = header_end {
            let cl = content_length.unwrap_or(0);
            if buffer.len() >= end + cl {
                break;
            }
        }
    }

    let end = header_end.unwrap_or(buffer.len());
    let header_str = String::from_utf8_lossy(&buffer[..end]).to_string();
    let first_line = header_str.lines().next().unwrap_or("");
    let mut parts = first_line.split_whitespace();
    let method = parts.next().unwrap_or("").to_string();
    let raw_path = parts.next().unwrap_or("/").to_string();

    let body = if end < buffer.len() {
        buffer[end..].to_vec()
    } else {
        Vec::new()
    };

    Some((method, raw_path, body))
}

fn send_json_response(stream: &mut TcpStream, status: &str, body: &str) {
    let header = format!(
        "HTTP/1.1 {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET, POST, DELETE, OPTIONS\r\nAccess-Control-Allow-Headers: *\r\nConnection: close\r\n\r\n",
        status,
        body.len()
    );
    let _ = stream.write_all(header.as_bytes());
    let _ = stream.write_all(body.as_bytes());
}

fn handle_connection(
    mut stream: TcpStream,
    overlay_sender: Option<&OverlaySender>,
    mcp_ctx: Option<&xhair_mcp::ServerContext>,
) {
    let (method, raw_path, body) = match read_http_request(&mut stream) {
        Some(req) => req,
        None => return,
    };

    if method == "OPTIONS" {
        let response = "HTTP/1.1 200 OK\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET, POST, DELETE, OPTIONS\r\nAccess-Control-Allow-Headers: *\r\nContent-Length: 0\r\n\r\n";
        let _ = stream.write_all(response.as_bytes());
        return;
    }

    let path = raw_path.split('?').next().unwrap_or("/");

    if method == "POST" {
        if path == "/api/overlay/crosshair" {
            let body_str = String::from_utf8_lossy(&body);
            if let Some(sender) = overlay_sender {
                match crate::bridge::parse_ui_crosshair(&body_str) {
                    Ok(ch) => {
                        let _ = sender.send(OverlayCommand::SetCrosshair(Box::new(ch.clone())));
                        if let Some(ctx) = mcp_ctx {
                            ctx.with_state(|state| {
                                state.active_crosshair_id = Some(ch.id);
                                state.bus.register_document(ch.clone());
                                if let Some(ref mut lib) = state.library {
                                    let _ = lib.save_crosshair(&ch, None);
                                }
                            });
                        }
                        send_json_response(&mut stream, "200 OK", "{\"status\":\"ok\"}");
                    }
                    Err(e) => {
                        let err_msg = format!("{{\"error\":\"{}\"}}", e);
                        send_json_response(&mut stream, "400 Bad Request", &err_msg);
                    }
                }
            } else {
                send_json_response(&mut stream, "200 OK", "{\"status\":\"no_overlay\"}");
            }
            return;
        } else if path == "/api/overlay/toggle" {
            if let Some(sender) = overlay_sender {
                let _ = sender.send(OverlayCommand::ToggleVisibility);
            }
            send_json_response(&mut stream, "200 OK", "{\"status\":\"ok\"}");
            return;
        } else if path == "/api/library/save" {
            let body_str = String::from_utf8_lossy(&body);
            match serde_json::from_str::<xhair_model::crosshair::Crosshair>(&body_str)
                .or_else(|_| crate::bridge::parse_ui_crosshair(&body_str))
            {
                Ok(ch) => {
                    let ch_id = ch.id;
                    if let Some(sender) = overlay_sender {
                        let _ = sender.send(OverlayCommand::SetCrosshair(Box::new(ch.clone())));
                    }
                    if let Some(ctx) = mcp_ctx {
                        ctx.with_state(|state| {
                            state.active_crosshair_id = Some(ch_id);
                            state.bus.register_document(ch.clone());
                            if let Some(ref mut lib) = state.library {
                                let _ = lib.save_crosshair(&ch, None);
                            }
                        });
                    }
                    send_json_response(
                        &mut stream,
                        "200 OK",
                        &format!("{{\"status\":\"ok\",\"id\":\"{}\"}}", ch_id),
                    );
                }
                Err(e) => {
                    send_json_response(
                        &mut stream,
                        "400 Bad Request",
                        &format!("{{\"error\":\"{}\"}}", e),
                    );
                }
            }
            return;
        } else if path == "/api/library/delete" {
            let body_str = String::from_utf8_lossy(&body);
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(&body_str) {
                if let Some(id_str) = val.get("id").and_then(|v| v.as_str()) {
                    if let Ok(id) = id_str.parse::<xhair_model::units::CrosshairId>() {
                        if let Some(ctx) = mcp_ctx {
                            ctx.with_state(|state| {
                                if let Some(ref mut lib) = state.library {
                                    let _ = lib.delete_crosshair(id);
                                }
                                state.bus.documents.remove(&id);
                            });
                        }
                        send_json_response(&mut stream, "200 OK", "{\"status\":\"ok\"}");
                        return;
                    }
                }
            }
            send_json_response(&mut stream, "400 Bad Request", "{\"error\":\"Invalid id\"}");
            return;
        }

        send_json_response(&mut stream, "404 Not Found", "{\"error\":\"Not found\"}");
        return;
    }

    if method == "GET" {
        if path == "/api/library" {
            if let Some(ctx) = mcp_ctx {
                let json = ctx.with_state(|state| {
                    let (crosshairs, folders) = if let Some(ref lib) = state.library {
                        (lib.index().crosshairs.clone(), lib.index().folders.clone())
                    } else {
                        (vec![], vec![])
                    };
                    serde_json::json!({
                        "crosshairs": crosshairs,
                        "folders": folders,
                        "active_id": state.active_crosshair_id,
                    })
                });
                send_json_response(&mut stream, "200 OK", &json.to_string());
                return;
            } else {
                send_json_response(
                    &mut stream,
                    "200 OK",
                    "{\"crosshairs\":[],\"folders\":[],\"active_id\":null}",
                );
                return;
            }
        } else if path == "/api/library/get" {
            let id_param = raw_path.split('?').nth(1).and_then(|q| {
                q.split('&')
                    .find(|pair| pair.starts_with("id="))
                    .map(|pair| pair.trim_start_matches("id="))
            });

            if let Some(id_str) = id_param {
                if let Ok(id) = id_str.parse::<xhair_model::units::CrosshairId>() {
                    if let Some(ctx) = mcp_ctx {
                        let doc = ctx.with_state(|state| {
                            if let Some(ref lib) = state.library {
                                lib.load_crosshair(id).ok()
                            } else {
                                state.bus.documents.get(&id).cloned()
                            }
                        });
                        if let Some(ch) = doc {
                            let json = serde_json::to_string(&ch).unwrap_or_default();
                            send_json_response(&mut stream, "200 OK", &json);
                            return;
                        }
                    }
                }
            }
            send_json_response(&mut stream, "404 Not Found", "{\"error\":\"Not found\"}");
            return;
        } else if path == "/api/mcp/activity" {
            if let Some(ctx) = mcp_ctx {
                let log = ctx.with_state(|state| state.activity_log.clone());
                let json = serde_json::to_string(&log).unwrap_or_else(|_| "[]".to_string());
                send_json_response(&mut stream, "200 OK", &json);
                return;
            } else {
                send_json_response(&mut stream, "200 OK", "[]");
                return;
            }
        }
    }

    if method != "GET" && method != "HEAD" {
        send_json_response(
            &mut stream,
            "405 Method Not Allowed",
            "{\"error\":\"Method not allowed\"}",
        );
        return;
    }

    let dist_dir = find_ui_dist_dir();
    let path = raw_path.split('?').next().unwrap_or("/");
    let trimmed = path.trim_start_matches('/');
    let target_file = if trimmed.is_empty() || trimmed == "index.html" {
        "index.html"
    } else {
        trimmed
    };

    let (content_type, body): (&str, Vec<u8>) = if let Some(ref dist) = dist_dir {
        let file_path = dist.join(target_file);
        if file_path.exists() && file_path.is_file() {
            let mime = guess_mime(target_file);
            let data = fs::read(&file_path).unwrap_or_default();
            (mime, data)
        } else if let Some(file) = EmbeddedDist::get(target_file) {
            let mime = guess_mime(target_file);
            (mime, file.data.to_vec())
        } else {
            // SPA fallback: return index.html
            let index_data = fs::read(dist.join("index.html"))
                .ok()
                .or_else(|| EmbeddedDist::get("index.html").map(|f| f.data.to_vec()))
                .unwrap_or_default();
            ("text/html; charset=utf-8", index_data)
        }
    } else if let Some(file) = EmbeddedDist::get(target_file) {
        let mime = guess_mime(target_file);
        (mime, file.data.to_vec())
    } else if let Some(index) = EmbeddedDist::get("index.html") {
        ("text/html; charset=utf-8", index.data.to_vec())
    } else {
        (
            "text/html; charset=utf-8",
            b"<!DOCTYPE html><html><body><h1>XHAIR UI</h1></body></html>".to_vec(),
        )
    };

    let header = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\nAccess-Control-Allow-Origin: *\r\n\r\n",
        content_type,
        body.len()
    );

    let _ = stream.write_all(header.as_bytes());
    if method == "GET" {
        let _ = stream.write_all(&body);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embedded_dist_has_index() {
        let index = EmbeddedDist::get("index.html").expect("EmbeddedDist must contain index.html");
        let content = String::from_utf8_lossy(&index.data);
        assert!(content.contains("XHAIR"));
    }

    #[test]
    fn test_guess_mime() {
        assert_eq!(guess_mime("index.html"), "text/html; charset=utf-8");
        assert_eq!(
            guess_mime("app.js"),
            "application/javascript; charset=utf-8"
        );
        assert_eq!(guess_mime("style.css"), "text/css; charset=utf-8");
        assert_eq!(guess_mime("icon.svg"), "image/svg+xml");
    }
}
