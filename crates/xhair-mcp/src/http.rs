//! Loopback HTTP transport with bearer token authentication (§13.2, §13.7).

use crate::context::ServerContext;
use crate::protocol::{JsonRpcError, JsonRpcRequest, JsonRpcResponse};
use crate::stdio::handle_request;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

/// Configuration options for the MCP HTTP server.
#[derive(Debug, Clone)]
pub struct HttpConfig {
    /// Port to listen on (e.g. 0 for random free port).
    pub port: u16,
    /// Bearer authentication token.
    pub bearer_token: String,
}

impl Default for HttpConfig {
    fn default() -> Self {
        Self {
            port: 0,
            bearer_token: "xhair-default-token-dev".to_string(),
        }
    }
}

/// Loopback HTTP server instance.
pub struct HttpServer {
    ctx: ServerContext,
    config: HttpConfig,
    running: Arc<AtomicBool>,
}

impl HttpServer {
    /// Creates a new HttpServer.
    pub fn new(ctx: ServerContext, config: HttpConfig) -> Self {
        Self {
            ctx,
            config,
            running: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Starts the HTTP server on a background thread and returns the bound socket address.
    pub fn start(&self) -> std::io::Result<SocketAddr> {
        // Enforce strictly 127.0.0.1 loopback binding (§13.2)
        let listener = TcpListener::bind(format!("127.0.0.1:{}", self.config.port))?;
        listener.set_nonblocking(true)?;
        let bound_addr = listener.local_addr()?;

        self.running.store(true, Ordering::SeqCst);
        let running_clone = Arc::clone(&self.running);
        let ctx_clone = self.ctx.clone();
        let config_clone = self.config.clone();

        std::thread::Builder::new()
            .name("xhair-mcp-http".to_string())
            .spawn(move || {
                tracing::info!(
                    "MCP HTTP loopback server listening on http://{}",
                    bound_addr
                );
                while running_clone.load(Ordering::SeqCst) {
                    match listener.accept() {
                        Ok((stream, _peer)) => {
                            let _ = handle_http_connection(stream, &ctx_clone, &config_clone);
                        }
                        Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                            std::thread::sleep(Duration::from_millis(10));
                        }
                        Err(e) => {
                            tracing::error!("TCP accept error: {}", e);
                            break;
                        }
                    }
                }
            })?;

        Ok(bound_addr)
    }

    /// Stops the running HTTP server.
    pub fn stop(&self) {
        self.running.store(false, Ordering::SeqCst);
    }
}

fn handle_http_connection(
    mut stream: TcpStream,
    ctx: &ServerContext,
    config: &HttpConfig,
) -> std::io::Result<()> {
    stream.set_read_timeout(Some(Duration::from_secs(5)))?;
    let mut buffer = [0u8; 16384];
    let bytes_read = stream.read(&mut buffer)?;
    if bytes_read == 0 {
        return Ok(());
    }

    let request_str = String::from_utf8_lossy(&buffer[..bytes_read]);

    // Split headers and body at \r\n\r\n or \n\n
    let (header_part, body_part) = if let Some(idx) = request_str.find("\r\n\r\n") {
        (&request_str[..idx], &request_str[idx + 4..])
    } else if let Some(idx) = request_str.find("\n\n") {
        (&request_str[..idx], &request_str[idx + 2..])
    } else {
        return send_bad_request(&mut stream);
    };

    let mut lines = header_part.lines();
    let request_line = lines.next().unwrap_or("");
    let is_post = request_line.starts_with("POST ");
    let is_options = request_line.starts_with("OPTIONS ");

    if is_options {
        // Pre-flight CORS
        let cors_resp = "HTTP/1.1 204 No Content\r\n\
            Access-Control-Allow-Origin: *\r\n\
            Access-Control-Allow-Methods: POST, OPTIONS\r\n\
            Access-Control-Allow-Headers: Authorization, Content-Type\r\n\
            Connection: close\r\n\r\n";
        stream.write_all(cors_resp.as_bytes())?;
        return stream.flush();
    }

    if !is_post {
        return send_bad_request(&mut stream);
    }

    let mut auth_token = None;
    for line in lines {
        if let Some((name, val)) = line.split_once(':') {
            let name_trim = name.trim();
            let val_trim = val.trim();

            // 1. Validate Origin / Host to protect against DNS-rebinding attacks (§13.7)
            if name_trim.eq_ignore_ascii_case("Host")
                && !val_trim.starts_with("127.0.0.1")
                && !val_trim.starts_with("localhost")
            {
                return send_forbidden(&mut stream, "Invalid Host header (loopback only)");
            }
            if name_trim.eq_ignore_ascii_case("Origin")
                && !val_trim.contains("127.0.0.1")
                && !val_trim.contains("localhost")
            {
                return send_forbidden(
                    &mut stream,
                    "Invalid Origin (DNS-rebinding protection)",
                );
            }

            // 2. Extract Bearer Token
            if name_trim.eq_ignore_ascii_case("Authorization") {
                if let Some(token) = val_trim.strip_prefix("Bearer ") {
                    auth_token = Some(token.trim().to_string());
                }
            }
        }
    }

    // Check Bearer Token authentication (§13.7)
    if auth_token.as_deref() != Some(&config.bearer_token) {
        return send_unauthorized(&mut stream);
    }

    // 3. Process JSON-RPC payload in body
    let json_req = match serde_json::from_str::<JsonRpcRequest>(body_part.trim()) {
        Ok(r) => r,
        Err(e) => {
            let err_resp = JsonRpcResponse::error(
                None,
                JsonRpcError::parse_error(format!("Malformed JSON-RPC: {}", e)),
            );
            return send_json(&mut stream, &err_resp);
        }
    };

    let response = handle_request(ctx, json_req);
    if let Some(resp) = response {
        send_json(&mut stream, &resp)?;
    } else {
        // Notification
        let empty_ok = "HTTP/1.1 204 No Content\r\n\r\n";
        stream.write_all(empty_ok.as_bytes())?;
    }

    Ok(())
}

fn send_json(stream: &mut TcpStream, resp: &JsonRpcResponse) -> std::io::Result<()> {
    let json = serde_json::to_string(resp).unwrap_or_else(|_| "{}".to_string());
    let response = format!(
        "HTTP/1.1 200 OK\r\n\
        Content-Type: application/json\r\n\
        Content-Length: {}\r\n\
        Access-Control-Allow-Origin: *\r\n\
        Connection: close\r\n\r\n\
        {}",
        json.len(),
        json
    );
    stream.write_all(response.as_bytes())?;
    stream.flush()
}

fn send_unauthorized(stream: &mut TcpStream) -> std::io::Result<()> {
    let body = r#"{"error":"Unauthorized: Missing or invalid Bearer token"}"#;
    let response = format!(
        "HTTP/1.1 401 Unauthorized\r\n\
        Content-Type: application/json\r\n\
        Content-Length: {}\r\n\
        WWW-Authenticate: Bearer realm=\"xhair\"\r\n\
        Connection: close\r\n\r\n\
        {}",
        body.len(),
        body
    );
    stream.write_all(response.as_bytes())?;
    stream.flush()
}

fn send_forbidden(stream: &mut TcpStream, msg: &str) -> std::io::Result<()> {
    let body = format!(r#"{{"error":"Forbidden: {}"}}"#, msg);
    let response = format!(
        "HTTP/1.1 403 Forbidden\r\n\
        Content-Type: application/json\r\n\
        Content-Length: {}\r\n\
        Connection: close\r\n\r\n\
        {}",
        body.len(),
        body
    );
    stream.write_all(response.as_bytes())?;
    stream.flush()
}

fn send_bad_request(stream: &mut TcpStream) -> std::io::Result<()> {
    let body = r#"{"error":"Bad Request"}"#;
    let response = format!(
        "HTTP/1.1 400 Bad Request\r\n\
        Content-Type: application/json\r\n\
        Content-Length: {}\r\n\
        Connection: close\r\n\r\n\
        {}",
        body.len(),
        body
    );
    stream.write_all(response.as_bytes())?;
    stream.flush()
}
