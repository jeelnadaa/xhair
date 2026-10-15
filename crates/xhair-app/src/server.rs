//! Embedded local HTTP server to serve the Svelte 5 editor UI.

use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::thread;

/// Finds the directory containing ui/dist assets.
pub fn find_ui_dist_dir() -> Option<PathBuf> {
    // 1. Current working directory
    let cwd_dist = PathBuf::from("ui/dist");
    if cwd_dist.join("index.html").exists() {
        return Some(cwd_dist);
    }

    // 2. Beside executable
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
        }
    }

    // 3. APPDATA/xhair/ui
    if let Ok(appdata) = std::env::var("APPDATA") {
        let appdata_dist = PathBuf::from(appdata).join("xhair").join("ui");
        if appdata_dist.join("index.html").exists() {
            return Some(appdata_dist);
        }
    }

    None
}

/// Starts the local HTTP server in a background thread and returns the port.
pub fn start_ui_server() -> Option<u16> {
    // Try binding to port 4173 or an OS-assigned free port
    let listener = TcpListener::bind("127.0.0.1:4173")
        .or_else(|_| TcpListener::bind("127.0.0.1:0"))
        .ok()?;
    let port = listener.local_addr().ok()?.port();

    thread::Builder::new()
        .name("xhair-ui-server".to_string())
        .spawn(move || {
            for stream in listener.incoming().flatten() {
                thread::spawn(move || {
                    handle_connection(stream);
                });
            }
        })
        .ok()?;

    Some(port)
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0u8; 2048];
    let n = match stream.read(&mut buffer) {
        Ok(n) if n > 0 => n,
        _ => return,
    };

    let req = String::from_utf8_lossy(&buffer[..n]);
    let first_line = req.lines().next().unwrap_or("");
    let mut parts = first_line.split_whitespace();
    let method = parts.next().unwrap_or("");
    let raw_path = parts.next().unwrap_or("/");

    if method != "GET" && method != "HEAD" {
        let response = "HTTP/1.1 405 Method Not Allowed\r\nContent-Length: 0\r\n\r\n";
        let _ = stream.write_all(response.as_bytes());
        return;
    }

    let dist_dir = find_ui_dist_dir();
    let path = raw_path.split('?').next().unwrap_or("/");

    let (content_type, body): (&str, Vec<u8>) = if let Some(ref dist) = dist_dir {
        let file_path = if path == "/" || path == "/index.html" {
            dist.join("index.html")
        } else {
            let trimmed = path.trim_start_matches('/');
            dist.join(trimmed)
        };

        if file_path.exists() && file_path.is_file() {
            let mime = if file_path.extension().and_then(|s| s.to_str()) == Some("js") {
                "application/javascript; charset=utf-8"
            } else if file_path.extension().and_then(|s| s.to_str()) == Some("css") {
                "text/css; charset=utf-8"
            } else if file_path.extension().and_then(|s| s.to_str()) == Some("svg") {
                "image/svg+xml"
            } else if file_path.extension().and_then(|s| s.to_str()) == Some("html") {
                "text/html; charset=utf-8"
            } else {
                "application/octet-stream"
            };
            let data = fs::read(&file_path).unwrap_or_default();
            (mime, data)
        } else {
            // SPA fallback: return index.html
            let index_path = dist.join("index.html");
            let data = fs::read(index_path).unwrap_or_default();
            ("text/html; charset=utf-8", data)
        }
    } else {
        // Fallback info page if ui/dist was not found
        let fallback = "<!DOCTYPE html><html><head><title>XHAIR Editor</title></head>\
             <body style='background:#0B0D10;color:#EEF2F6;font-family:sans-serif;text-align:center;padding:50px;'>\
             <h1>XHAIR Overlay is Active</h1>\
             <p>The overlay is currently rendering on your desktop.</p>\
             <p>To use the web editor, run <code>pnpm -C ui dev</code> and open <a href='http://localhost:5173' style='color:#36F2CB;'>http://localhost:5173</a></p>\
             <p>Hotkeys: <b>Ctrl+Shift+X</b> (Toggle Overlay), <b>Ctrl+Shift+1..9</b> (Switch Slot)</p>\
             </body></html>".to_string();
        ("text/html; charset=utf-8", fallback.into_bytes())
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
