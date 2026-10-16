//! Model Context Protocol (MCP) server generated from the xhair command registry.
//!
//! Provides stdio and loopback Streamable HTTP JSON-RPC 2.0 transports, tool generation
//! from domain commands, resources, prompt templates, and security sandboxing.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

pub mod context;
pub mod http;
pub mod prompts;
pub mod protocol;
pub mod proxy;
pub mod resources;
pub mod stdio;
pub mod tools;

pub use context::{ActivityEntry, OverlayController, ServerContext, ServerState};
pub use http::{HttpConfig, HttpServer};
pub use prompts::{get_prompt, list_prompts};
pub use protocol::{CallToolResult, JsonRpcRequest, JsonRpcResponse, ToolDefinition};
pub use proxy::{
    clear_endpoint_info, is_endpoint_alive, proxy_request, read_endpoint_info, run_stdio_proxy,
    write_endpoint_info, EndpointInfo,
};
pub use resources::{list_resources, read_resource};
pub use stdio::{handle_request, run_stdio_transport};
pub use tools::{call_tool, list_tools};

/// MCP protocol API version for xhair.
pub const MCP_API_VERSION: &str = "0.1.0";

/// Main MCP Server coordinator.
pub struct McpServer {
    ctx: ServerContext,
    http_server: Option<HttpServer>,
}

impl Default for McpServer {
    fn default() -> Self {
        Self::new(ServerContext::default())
    }
}

impl McpServer {
    /// Creates a new McpServer with context.
    pub fn new(ctx: ServerContext) -> Self {
        Self {
            ctx,
            http_server: None,
        }
    }

    /// Access to server context.
    pub fn context(&self) -> &ServerContext {
        &self.ctx
    }

    /// Runs the stdio transport on the current thread.
    pub fn run_stdio(&self) -> std::io::Result<()> {
        run_stdio_transport(self.ctx.clone())
    }

    /// Starts the loopback HTTP transport with specified config.
    pub fn start_http(&mut self, config: HttpConfig) -> std::io::Result<std::net::SocketAddr> {
        let server = HttpServer::new(self.ctx.clone(), config);
        let addr = server.start()?;
        self.http_server = Some(server);
        Ok(addr)
    }

    /// Stops the HTTP transport if active.
    pub fn stop_http(&mut self) {
        if let Some(ref server) = self.http_server {
            server.stop();
        }
        self.http_server = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mcp_api_version() {
        assert_eq!(MCP_API_VERSION, "0.1.0");
    }

    #[test]
    fn test_tools_listing() {
        let tools = list_tools();
        assert!(tools.len() >= 20);
        let names: Vec<_> = tools.iter().map(|t| t.name.as_str()).collect();
        assert!(names.contains(&"library.list"));
        assert!(names.contains(&"crosshair.get"));
        assert!(names.contains(&"layer.add"));
        assert!(names.contains(&"nudge"));
        assert!(names.contains(&"render.preview"));
        assert!(names.contains(&"overlay.get_state"));
        assert!(names.contains(&"meta.capabilities"));
    }
}
