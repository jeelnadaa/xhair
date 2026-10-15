//! Model Context Protocol (MCP) and JSON-RPC 2.0 type definitions.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// JSON-RPC 2.0 Request ID.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RequestId {
    /// Integer ID.
    Number(i64),
    /// String ID.
    String(String),
}

/// JSON-RPC 2.0 Request envelope.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    /// JSON-RPC version (must be "2.0").
    pub jsonrpc: String,
    /// Unique request ID (absent in notifications).
    pub id: Option<RequestId>,
    /// Method name.
    pub method: String,
    /// Optional method parameters.
    #[serde(default)]
    pub params: Value,
}

impl JsonRpcRequest {
    /// Constructs a request with integer ID.
    pub fn call(id: i64, method: impl Into<String>, params: Option<Value>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: Some(RequestId::Number(id)),
            method: method.into(),
            params: params.unwrap_or(Value::Null),
        }
    }

    /// Constructs a notification (no ID).
    pub fn notification(method: impl Into<String>, params: Option<Value>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: None,
            method: method.into(),
            params: params.unwrap_or(Value::Null),
        }
    }
}

/// JSON-RPC 2.0 Error object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcError {
    /// Error code.
    pub code: i32,
    /// Short human-readable error description.
    pub message: String,
    /// Optional structured data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

impl JsonRpcError {
    /// Parse error (-32700)
    pub fn parse_error(msg: impl Into<String>) -> Self {
        Self {
            code: -32700,
            message: msg.into(),
            data: None,
        }
    }

    /// Invalid request (-32600)
    pub fn invalid_request(msg: impl Into<String>) -> Self {
        Self {
            code: -32600,
            message: msg.into(),
            data: None,
        }
    }

    /// Method not found (-32601)
    pub fn method_not_found(method: impl Into<String>) -> Self {
        Self {
            code: -32601,
            message: format!("Method not found: {}", method.into()),
            data: None,
        }
    }

    /// Invalid params (-32602)
    pub fn invalid_params(msg: impl Into<String>) -> Self {
        Self {
            code: -32602,
            message: msg.into(),
            data: None,
        }
    }

    /// Internal error (-32603)
    pub fn internal_error(msg: impl Into<String>) -> Self {
        Self {
            code: -32603,
            message: msg.into(),
            data: None,
        }
    }
}

/// JSON-RPC 2.0 Response envelope.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    /// JSON-RPC version.
    pub jsonrpc: String,
    /// Request ID matching the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<RequestId>,
    /// Result payload on success.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    /// Error payload on failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

impl JsonRpcResponse {
    /// Constructs a success response.
    pub fn success(id: Option<RequestId>, result: Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(result),
            error: None,
        }
    }

    /// Constructs an error response.
    pub fn error(id: Option<RequestId>, error: JsonRpcError) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: None,
            error: Some(error),
        }
    }
}

/// MCP Tool descriptor.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ToolDefinition {
    /// Unique tool name.
    pub name: String,
    /// Description of the tool's purpose and usage.
    pub description: String,
    /// JSON Schema defining the input arguments.
    #[serde(rename = "inputSchema")]
    pub input_schema: Value,
}

/// Content element in a Tool call result.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ToolContent {
    /// Textual content.
    Text {
        /// Text string.
        text: String,
    },
    /// Embedded image content.
    Image {
        /// Base64-encoded image data.
        data: String,
        /// MIME type (e.g. "image/png").
        #[serde(rename = "mimeType")]
        mime_type: String,
    },
}

/// Result returned from executing a Tool.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CallToolResult {
    /// Array of content pieces (text, images, structured data).
    pub content: Vec<ToolContent>,
    /// True if the tool execution resulted in an error.
    #[serde(rename = "isError")]
    pub is_error: bool,
}

impl CallToolResult {
    /// Creates a success text result.
    pub fn text(text: impl Into<String>) -> Self {
        Self {
            content: vec![ToolContent::Text { text: text.into() }],
            is_error: false,
        }
    }

    /// Creates a success JSON result.
    pub fn json(val: &impl Serialize) -> Self {
        let text = serde_json::to_string_pretty(val).unwrap_or_else(|_| "{}".to_string());
        Self::text(text)
    }

    /// Creates an image result.
    pub fn image(data_base64: impl Into<String>, mime: impl Into<String>) -> Self {
        Self {
            content: vec![ToolContent::Image {
                data: data_base64.into(),
                mime_type: mime.into(),
            }],
            is_error: false,
        }
    }

    /// Creates an error result with message.
    pub fn error(msg: impl Into<String>) -> Self {
        Self {
            content: vec![ToolContent::Text { text: msg.into() }],
            is_error: true,
        }
    }
}

/// MCP Resource descriptor.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ResourceDefinition {
    /// URI of the resource (e.g. "xhair://library").
    pub uri: String,
    /// Human-readable name.
    pub name: String,
    /// Optional description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// MIME type.
    #[serde(rename = "mimeType")]
    pub mime_type: String,
}

/// Resource read content.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceContent {
    /// Resource URI.
    pub uri: String,
    /// MIME type.
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    /// Textual content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Binary base64 content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blob: Option<String>,
}

/// Argument definition for an MCP Prompt.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PromptArgument {
    /// Argument name.
    pub name: String,
    /// Argument description.
    pub description: String,
    /// Whether argument is mandatory.
    pub required: bool,
}

/// MCP Prompt definition.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PromptDefinition {
    /// Prompt name.
    pub name: String,
    /// Prompt description.
    pub description: String,
    /// Accepted arguments.
    #[serde(default)]
    pub arguments: Vec<PromptArgument>,
}

/// Prompt message content.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptMessage {
    /// Role ("user" or "assistant").
    pub role: String,
    /// Content piece.
    pub content: ToolContent,
}
