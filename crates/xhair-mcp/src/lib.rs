//! Model Context Protocol (MCP) server generated from the xhair command registry.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

/// MCP protocol API version for xhair.
pub const MCP_API_VERSION: &str = "0.1.0";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mcp_api_version() {
        assert_eq!(MCP_API_VERSION, "0.1.0");
    }
}
