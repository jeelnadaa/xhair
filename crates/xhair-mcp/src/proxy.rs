//! Proxy IPC mechanism to forward commands to an already-running app instance (§1.4, §13.2).

use crate::protocol::{JsonRpcRequest, JsonRpcResponse};
use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::path::PathBuf;
use std::time::Duration;

/// Information about a running xhair MCP HTTP endpoint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EndpointInfo {
    /// Port of the loopback HTTP server.
    pub port: u16,
    /// Bearer authentication token.
    pub token: String,
    /// Process ID owning the server.
    pub pid: u32,
}

/// Returns the standard path for the endpoint info file.
pub fn endpoint_file_path() -> PathBuf {
    std::env::temp_dir().join("xhair-endpoint.json")
}

/// Writes endpoint information to disk for discovery by secondary processes.
pub fn write_endpoint_info(info: &EndpointInfo) -> std::io::Result<()> {
    let path = endpoint_file_path();
    let data = serde_json::to_string_pretty(info)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    std::fs::write(path, data)
}

/// Removes the endpoint file on shutdown.
pub fn clear_endpoint_info() {
    let path = endpoint_file_path();
    let _ = std::fs::remove_file(path);
}

/// Reads existing endpoint info if present.
pub fn read_endpoint_info() -> Option<EndpointInfo> {
    let path = endpoint_file_path();
    if !path.exists() {
        return None;
    }
    let data = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(&data).ok()
}

/// Checks if the running endpoint is alive and responsive.
pub fn is_endpoint_alive(info: &EndpointInfo) -> bool {
    // Attempt TCP connection with 500ms timeout
    let addr = format!("127.0.0.1:{}", info.port);
    let socket_addr = match addr.parse::<std::net::SocketAddr>() {
        Ok(sa) => sa,
        Err(_) => return false,
    };

    match TcpStream::connect_timeout(&socket_addr, Duration::from_millis(500)) {
        Ok(mut stream) => {
            let req = format!(
                "OPTIONS / HTTP/1.1\r\nHost: 127.0.0.1:{}\r\nOrigin: http://127.0.0.1:{}\r\nConnection: close\r\n\r\n",
                info.port, info.port
            );
            if stream.write_all(req.as_bytes()).is_err() {
                return false;
            }
            let mut buf = [0u8; 128];
            stream
                .set_read_timeout(Some(Duration::from_millis(500)))
                .ok();
            match stream.read(&mut buf) {
                Ok(n) if n > 0 => {
                    let s = String::from_utf8_lossy(&buf[..n]);
                    s.starts_with("HTTP/1.1 204") || s.starts_with("HTTP/1.1 200")
                }
                _ => false,
            }
        }
        Err(_) => false,
    }
}

/// Sends a JSON-RPC request to the running instance and receives the JSON-RPC response.
pub fn proxy_request(info: &EndpointInfo, req: &JsonRpcRequest) -> Result<JsonRpcResponse, String> {
    let body = serde_json::to_string(req).map_err(|e| format!("Serialization error: {}", e))?;
    let addr = format!("127.0.0.1:{}", info.port);

    let mut stream = TcpStream::connect(&addr)
        .map_err(|e| format!("Failed to connect to proxy target: {}", e))?;
    stream
        .set_read_timeout(Some(Duration::from_secs(10)))
        .map_err(|e| format!("Failed to set timeout: {}", e))?;

    let http_req = format!(
        "POST / HTTP/1.1\r\n\
        Host: 127.0.0.1:{}\r\n\
        Origin: http://127.0.0.1:{}\r\n\
        Authorization: Bearer {}\r\n\
        Content-Type: application/json\r\n\
        Content-Length: {}\r\n\
        Connection: close\r\n\r\n\
        {}",
        info.port,
        info.port,
        info.token,
        body.len(),
        body
    );

    stream
        .write_all(http_req.as_bytes())
        .map_err(|e| format!("Write failed: {}", e))?;
    stream.flush().map_err(|e| format!("Flush failed: {}", e))?;

    let mut resp_bytes = Vec::new();
    stream
        .read_to_end(&mut resp_bytes)
        .map_err(|e| format!("Read failed: {}", e))?;

    let resp_str = String::from_utf8_lossy(&resp_bytes);
    let body_str = if let Some(idx) = resp_str.find("\r\n\r\n") {
        &resp_str[idx + 4..]
    } else if let Some(idx) = resp_str.find("\n\n") {
        &resp_str[idx + 2..]
    } else {
        return Err("Invalid HTTP response from proxy target".to_string());
    };

    serde_json::from_str::<JsonRpcResponse>(body_str.trim())
        .map_err(|e| format!("Failed to parse response JSON: {} (body: {})", e, body_str))
}

/// Runs a stdio proxy loop forwarding stdin requests to the running endpoint and writing stdout responses.
pub fn run_stdio_proxy(info: EndpointInfo) -> std::io::Result<()> {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let reader = BufReader::new(stdin.lock());

    for line_result in reader.lines() {
        let line = line_result?;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        match serde_json::from_str::<JsonRpcRequest>(trimmed) {
            Ok(req) => match proxy_request(&info, &req) {
                Ok(resp) => {
                    let out = serde_json::to_string(&resp).unwrap_or_else(|_| "{}".to_string());
                    stdout.write_all(out.as_bytes())?;
                    stdout.write_all(b"\n")?;
                    stdout.flush()?;
                }
                Err(err) => {
                    let err_resp = crate::protocol::JsonRpcResponse::error(
                        req.id,
                        crate::protocol::JsonRpcError::internal_error(format!(
                            "Proxy error: {}",
                            err
                        )),
                    );
                    let out = serde_json::to_string(&err_resp).unwrap_or_else(|_| "{}".to_string());
                    stdout.write_all(out.as_bytes())?;
                    stdout.write_all(b"\n")?;
                    stdout.flush()?;
                }
            },
            Err(e) => {
                let err_resp = crate::protocol::JsonRpcResponse::error(
                    None,
                    crate::protocol::JsonRpcError::parse_error(format!(
                        "Malformed JSON-RPC: {}",
                        e
                    )),
                );
                let out = serde_json::to_string(&err_resp).unwrap_or_else(|_| "{}".to_string());
                stdout.write_all(out.as_bytes())?;
                stdout.write_all(b"\n")?;
                stdout.flush()?;
            }
        }
    }

    Ok(())
}
