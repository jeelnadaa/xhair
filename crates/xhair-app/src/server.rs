use rust_embed::RustEmbed;
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::thread;

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
