# MCP Architecture & Human/Agent Parity (P4)

**Project:** XHAIR — Model Context Protocol Architecture  
**Reference:** `docs/spec/CROSSHAIR_OVERLAY_SPEC.md` §13, Invariant P3 & P4

---

## 1. Architectural Gap: Static Parity vs Runtime Disconnect

In Milestone 6 and Spec v5, a test named `parity.rs` was created:
```rust
// tests/parity.rs
#[test]
fn test_ui_command_mcp_parity() {
    let ui_actions = serde_json::from_str(&actions_json);
    let tools = list_tools();
    // Asserts that every string in actions.json matches a tool in list_tools()
}
```
While this test verified that **schema string names match**, at **runtime**:
1. `xhair.exe` does not start the MCP HTTP server.
2. `xhair-cli mcp` runs a separate, empty instance of `ServerContext` in memory.
3. Mutations executed through MCP never reach the running desktop overlay or editor UI.
4. Overlay control tools (`overlay.set_enabled`, `calibration.show`, `calibration.set_offset`) only mutate in-memory struct fields without notifying the Win32 window thread.

---

## 2. Target Production Architecture

To make the application production-ready, the MCP server must be integrated into the running application runtime with a transparent proxy mechanism:

```
┌────────────────────────────────────────────────────────────────────────────────┐
│                         xhair.exe (Main Application)                           │
│                                                                                │
│  ┌───────────────────────┐   Shared State    ┌──────────────────────────────┐  │
│  │     CommandBus        │ ────────────────► │       OverlaySender          │  │
│  │   (Single Truth)      │                   │   (Win32 Message Loop)       │  │
│  └──────────┬────────────┘                   └──────────────────────────────┘  │
│             │                                               ▲                  │
│             │ Event Broadcast                               │ Direct Commands  │
│             ▼                                               │                  │
│  ┌───────────────────────┐                   ┌──────────────┴───────────────┐  │
│  │     WebView (UI)      │                   │        McpHttpServer         │  │
│  │  - Real-time updates  │                   │  - Loopback: 127.0.0.1:port  │  │
│  │  - "Agent is editing" │                   │  - Bearer Token Auth         │  │
│  │  - Live Activity Log  │                   │  - DNS Rebinding Guard       │  │
│  └───────────────────────┘                   └──────────────▲───────────────┘  │
│                                                             │                  │
│                                Writes on startup:           │                  │
│                                %TEMP%/xhair-endpoint.json   │                  │
└─────────────────────────────────────────────────────────────┼──────────────────┘
                                                              │
                                     Local HTTP JSON-RPC 2.0  │
                                                              │
┌─────────────────────────────────────────────────────────────┴──────────────────┐
│                            xhair-cli mcp (Stdio)                               │
│  - Connects to stdin / stdout of Claude Desktop, Cursor, or AI Agent          │
│  - Discovers running xhair-endpoint.json                                      │
│  - Proxies JSON-RPC requests via run_stdio_proxy()                            │
│  - Fallback: Headless offline engine if xhair.exe is not running              │
└────────────────────────────────────────────────────────────────────────────────┘
```

---

## 3. Core Requirements for Production MCP

### 3.1 Endpoint Discovery & Lifecycle (`xhair-endpoint.json`)
On startup of `xhair.exe`:
1. Generate a cryptographically secure 256-bit alphanumeric Bearer token.
2. Bind `HttpServer` to `127.0.0.1:0` (OS-assigned free ephemeral port).
3. Atomically write `%TEMP%/xhair-endpoint.json`:
   ```json
   {
     "port": 49215,
     "token": "xhair_sec_9f8a7b6c5d4e3f2a1...",
     "pid": 14280,
     "version": "1.0.0"
   }
   ```
4. On application shutdown (or panic hook), remove `%TEMP%/xhair-endpoint.json`.

### 3.2 Dual-Mode Stdio Proxy in `xhair-cli mcp`
When `xhair-cli mcp` is executed by an AI agent:
1. Call `read_endpoint_info()`.
2. If the endpoint file exists, test if it is responsive via `is_endpoint_alive(&info)`.
3. If responsive:
   - Call `run_stdio_proxy(info)`.
   - All tool invocations are transparently forwarded to the running `xhair.exe` over loopback HTTP with the Bearer token.
4. If not running:
   - Initialize headless `ServerContext::new(Some(LibraryEngine::new(...)))`.
   - All mutations are written to the disk library so they are preserved when `xhair.exe` next starts.

### 3.3 Wiring MCP Tools to the Win32 Overlay & Command Bus
Every MCP tool must execute against the live running state:

| Tool Group | MCP Tool Name | Required Production Behavior |
|---|---|---|
| **Overlay** | `overlay.set_enabled` | Send `OverlayCommand::SetEnabled(enabled)` to `OverlaySender`. |
| **Overlay** | `overlay.activate` | Load crosshair document, dispatch to bus, send `OverlayCommand::SetCrosshair` to `OverlaySender`. |
| **Overlay** | `overlay.next` / `overlay.previous` | Cycle library index, update active ID, send `OverlayCommand::SetCrosshair`. |
| **Calibration**| `calibration.show` | Send `OverlayCommand::ToggleCalibrationPattern` (set to `true`). |
| **Calibration**| `calibration.hide` | Send `OverlayCommand::ToggleCalibrationPattern` (set to `false`). |
| **Calibration**| `calibration.set_offset` | Send `OverlayCommand::Nudge(Vec2::new(dx, dy))`. |
| **Displays** | `display.list` | Call Win32 `EnumDisplayMonitors` + `GetDpiForMonitor` and return physical bounds & DPI factors. |
| **Edit** | `layer.add`, `layer.update`, `nudge`, etc. | Dispatch to `CommandBus`, convert resulting `Crosshair` and send `OverlayCommand::SetCrosshair` immediately. |

---

## 4. Live UI Agent Activity Streaming

When an AI agent invokes an MCP tool:
1. The server records an `ActivityEntry` with timestamp, tool name, parameter summary, and execution duration.
2. If `xhair.exe` has an open webview window, it pushes an event via `webview.evaluate_script(...)`:
   ```javascript
   window.dispatchEvent(new CustomEvent('xhair:mcp_activity', {
     detail: {
       tool: "property.set",
       summary: "Updated line.thickness to 2.5",
       timestamp: "14:10:45",
       status: "success"
     }
   }));
   ```
3. The Svelte 5 UI listens to this event:
   - Appends the entry to `editorState.mcpActivityLog`.
   - Sets `editorState.isAgentEditing = true` for 1500ms, displaying the neon pulsing "Agent is editing" indicator in the TopBar.
   - Updates the live canvas rendering with the modified properties without requiring a page reload.

---

## 5. Security & Anti-Cheat Compliance Guardrails

1. **Loopback Only (`127.0.0.1`):** Never bind to `0.0.0.0` or external network interfaces.
2. **DNS Rebinding Guard:** Validate that HTTP request headers `Host` and `Origin` strictly begin with `127.0.0.1` or `localhost`. Drop all requests with unrecognized external hosts.
3. **Bearer Token Authentication:** Reject any request lacking `Authorization: Bearer <token>` with `401 Unauthorized`.
4. **Zero Game Access:** MCP tools operate strictly on crosshair geometry and overlay parameters. Prohibit any MCP tool or prompt that mentions game memory injection or screen capture.
