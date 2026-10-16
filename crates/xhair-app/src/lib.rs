//! Desktop application entry point, system tray daemon, and webview window management.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

/// UI crosshair JSON parser and bridge adapter.
pub mod bridge;
/// Local HTTP server and embedded asset delivery.
pub mod server;

use std::sync::Arc;
use xhair_geometry::ResolutionProfile;
use xhair_model::crosshair::Crosshair;
use xhair_platform_win::{
    spawn_overlay, OverlayCommand, OverlayConfig, OverlaySender, VisibilityMode,
};

/// Application default window title.
pub const APP_TITLE: &str = "XHAIR";

/// Bridge between MCP tools and the physical Win32 overlay window.
pub struct Win32OverlayController {
    sender: OverlaySender,
}

impl Win32OverlayController {
    /// Creates a new Win32OverlayController.
    pub fn new(sender: OverlaySender) -> Self {
        Self { sender }
    }
}

impl xhair_mcp::OverlayController for Win32OverlayController {
    fn set_enabled(&self, enabled: bool) -> Result<(), String> {
        self.sender
            .send(OverlayCommand::SetEnabled(enabled))
            .map_err(|e| e.to_string())
    }

    fn set_crosshair(&self, crosshair: Box<Crosshair>) -> Result<(), String> {
        self.sender
            .send(OverlayCommand::SetCrosshair(crosshair))
            .map_err(|e| e.to_string())
    }

    fn toggle_visibility(&self) -> Result<(), String> {
        self.sender
            .send(OverlayCommand::ToggleVisibility)
            .map_err(|e| e.to_string())
    }

    fn toggle_calibration_pattern(&self) -> Result<(), String> {
        self.sender
            .send(OverlayCommand::ToggleCalibrationPattern)
            .map_err(|e| e.to_string())
    }

    fn nudge(&self, delta: xhair_model::Vec2) -> Result<(), String> {
        self.sender
            .send(OverlayCommand::Nudge(delta))
            .map_err(|e| e.to_string())
    }

    fn list_monitors(&self) -> Result<Vec<serde_json::Value>, String> {
        let monitors = xhair_platform_win::enumerate_monitors().map_err(|e| e.to_string())?;
        let list = monitors
            .into_iter()
            .map(|m| {
                serde_json::json!({
                    "id": m.device_name,
                    "name": m.device_name,
                    "is_primary": m.is_primary,
                    "width": m.width(),
                    "height": m.height(),
                    "dpi_scale": m.scale_factor
                })
            })
            .collect();
        Ok(list)
    }
}

/// Runs the main application event loop.
pub fn run() -> anyhow::Result<()> {
    // 1. Install local panic crash handler with zero telemetry (§16, M10)
    let crash_dir = xhair_platform_win::install_crash_handler()
        .map_err(|e| anyhow::anyhow!("Failed to initialize crash handler: {}", e))?;
    tracing::debug!("Crash reports directory: {:?}", crash_dir);

    let version = env!("CARGO_PKG_VERSION");
    let args: Vec<String> = std::env::args().collect();
    let is_minimized = args.iter().any(|a| a == "--minimized");
    let should_reset = args
        .iter()
        .any(|a| a == "--reset-storage" || a == "reset-storage");

    tracing::info!(
        "Starting {} v{} (minimized={}, reset_storage={})",
        APP_TITLE,
        version,
        is_minimized,
        should_reset
    );

    // 2. Initialize disk storage engine (§10)
    let data_dir = xhair_library::LibraryEngine::resolve_data_dir();
    if should_reset {
        let _ = std::fs::remove_dir_all(&data_dir);
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                let _ = std::fs::remove_dir_all(exe_dir.join("xhair.exe.WebView2"));
            }
        }
        tracing::info!("Storage reset completed successfully.");
    }

    let library = match xhair_library::LibraryEngine::new(&data_dir) {
        Ok(mut lib) => {
            tracing::info!("Initialized library engine at {:?}", data_dir);
            // Auto-seed default crosshair if catalog is empty
            if lib.index().crosshairs.is_empty() {
                let default_ch = Crosshair::new_default();
                let _ = lib.save_crosshair(&default_ch, None);
            }
            Some(lib)
        }
        Err(e) => {
            tracing::warn!("Failed to initialize library engine: {}", e);
            None
        }
    };

    // 3. Load default or active crosshair and resolution profile (§4, §6)
    let crosshair = if let Some(ref lib) = library {
        if let Some(first) = lib.index().crosshairs.first() {
            lib.load_crosshair(first.id)
                .unwrap_or_else(|_| Crosshair::new_default())
        } else {
            Crosshair::new_default()
        }
    } else {
        Crosshair::new_default()
    };
    let profile = ResolutionProfile::default_1080p();

    // 4. Configure overlay: default to visible on screen (§8.1)
    let config = OverlayConfig {
        visibility_mode: VisibilityMode::AlwaysShow,
        follow_game_window: false,
        ..Default::default()
    };

    // 5. Spawn the Win32 transparent click-through overlay window + tray icon + hotkeys (§8)
    let overlay_handle = spawn_overlay(crosshair, profile, config)
        .map_err(|e| anyhow::anyhow!("Failed to spawn overlay: {}", e))?;
    let overlay_sender = overlay_handle.get_sender();

    // 6. Initialize MCP Server context and wire Win32 overlay controller (§13)
    let mcp_ctx = xhair_mcp::ServerContext::new(library);
    mcp_ctx.set_overlay_controller(Arc::new(Win32OverlayController::new(
        overlay_sender.clone(),
    )));

    // 7. Start loopback MCP HTTP server and write %TEMP%/xhair-endpoint.json for discovery
    let token = format!("xhair_sec_{}", uuid::Uuid::new_v4().simple());
    let mut mcp_server = xhair_mcp::McpServer::new(mcp_ctx.clone());
    let http_config = xhair_mcp::HttpConfig {
        port: 0,
        bearer_token: token.clone(),
    };

    match mcp_server.start_http(http_config) {
        Ok(socket_addr) => {
            tracing::info!(
                "Started embedded MCP HTTP server on 127.0.0.1:{}",
                socket_addr.port()
            );
            let endpoint_info = xhair_mcp::EndpointInfo {
                port: socket_addr.port(),
                token,
                pid: std::process::id(),
            };
            if let Err(e) = xhair_mcp::write_endpoint_info(&endpoint_info) {
                tracing::warn!("Failed to write endpoint info file: {}", e);
            }
        }
        Err(e) => {
            tracing::warn!("Failed to start embedded MCP HTTP server: {}", e);
        }
    }

    // 8. If not launched minimized, start the local UI server and launch native desktop window
    if !is_minimized {
        if let Some(port) =
            server::start_ui_server(Some(overlay_sender.clone()), Some(mcp_ctx.clone()))
        {
            let url = format!("http://127.0.0.1:{}", port);
            tracing::info!("XHAIR Editor UI serving at {}", url);

            let event_loop = tao::event_loop::EventLoop::new();
            let window = tao::window::WindowBuilder::new()
                .with_title(APP_TITLE)
                .with_inner_size(tao::dpi::LogicalSize::new(1366.0, 820.0))
                .with_min_inner_size(tao::dpi::LogicalSize::new(1024.0, 640.0))
                .build(&event_loop)
                .map_err(|e| anyhow::anyhow!("Failed to create window: {}", e))?;

            let ipc_sender = overlay_sender.clone();
            let ipc_ctx = mcp_ctx.clone();
            let webview = wry::WebViewBuilder::new()
                .with_url(&url)
                .with_devtools(true)
                .with_ipc_handler(move |msg| {
                    let text = msg.body();
                    if text == "toggle_overlay" {
                        let _ = ipc_sender.send(OverlayCommand::ToggleVisibility);
                    } else if let Ok(val) = serde_json::from_str::<serde_json::Value>(text) {
                        if let Some(action) = val.get("action").and_then(|a| a.as_str()) {
                            match action {
                                "toggle_overlay" => {
                                    let _ = ipc_sender.send(OverlayCommand::ToggleVisibility);
                                }
                                "set_overlay_enabled" => {
                                    let enabled = val
                                        .get("enabled")
                                        .and_then(|e| e.as_bool())
                                        .unwrap_or(true);
                                    let _ = ipc_sender.send(OverlayCommand::SetEnabled(enabled));
                                }
                                "recenter" => {
                                    let _ = ipc_sender
                                        .send(OverlayCommand::Nudge(xhair_model::Vec2::ZERO));
                                }
                                _ => {}
                            }
                        } else if let Ok(ch) = bridge::parse_ui_crosshair(text) {
                            let _ =
                                ipc_sender.send(OverlayCommand::SetCrosshair(Box::new(ch.clone())));
                            ipc_ctx.with_state(|state| {
                                state.active_crosshair_id = Some(ch.id);
                                state.bus.register_document(ch.clone());
                                if let Some(ref mut lib) = state.library {
                                    let _ = lib.save_crosshair(&ch, None);
                                }
                            });
                        }
                    } else if let Ok(ch) = bridge::parse_ui_crosshair(text) {
                        let _ = ipc_sender.send(OverlayCommand::SetCrosshair(Box::new(ch.clone())));
                        ipc_ctx.with_state(|state| {
                            state.active_crosshair_id = Some(ch.id);
                            state.bus.register_document(ch.clone());
                            if let Some(ref mut lib) = state.library {
                                let _ = lib.save_crosshair(&ch, None);
                            }
                        });
                    }
                })
                .build(&window)
                .map_err(|e| anyhow::anyhow!("Failed to create webview: {}", e))?;

            let initial_size = window.inner_size();
            let _ = webview.set_bounds(wry::Rect {
                position: wry::dpi::Position::Logical(wry::dpi::LogicalPosition::new(0.0, 0.0)),
                size: wry::dpi::Size::Physical(wry::dpi::PhysicalSize::new(
                    initial_size.width,
                    initial_size.height,
                )),
            });

            event_loop.run(move |event, _, control_flow| {
                *control_flow = tao::event_loop::ControlFlow::Wait;
                match event {
                    tao::event::Event::WindowEvent {
                        event: tao::event::WindowEvent::Resized(size),
                        ..
                    } => {
                        let _ = webview.set_bounds(wry::Rect {
                            position: wry::dpi::Position::Logical(wry::dpi::LogicalPosition::new(
                                0.0, 0.0,
                            )),
                            size: wry::dpi::Size::Physical(wry::dpi::PhysicalSize::new(
                                size.width,
                                size.height,
                            )),
                        });
                    }
                    tao::event::Event::WindowEvent {
                        event: tao::event::WindowEvent::CloseRequested,
                        ..
                    } => {
                        xhair_mcp::clear_endpoint_info();
                        *control_flow = tao::event_loop::ControlFlow::Exit;
                    }
                    _ => {}
                }
            });
        }
    }

    // 9. Keep daemon running until exit (blocks on overlay thread message loop / tray menu)
    overlay_handle.wait();
    xhair_mcp::clear_endpoint_info();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_title() {
        assert_eq!(APP_TITLE, "XHAIR");
    }

    #[test]
    fn test_ui_server_and_dist() {
        let dist = server::find_ui_dist_dir();
        println!("find_ui_dist_dir returned: {:?}", dist);
        let port = server::start_ui_server(None, None).expect("failed to start server");
        println!("start_ui_server returned: {}", port);

        use std::io::{Read, Write};
        use std::net::TcpStream;

        let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port)).expect("connect failed");
        stream
            .write_all(b"GET / HTTP/1.1\r\nHost: 127.0.0.1\r\n\r\n")
            .unwrap();
        let mut resp = Vec::new();
        stream.read_to_end(&mut resp).unwrap();
        let resp_str = String::from_utf8_lossy(&resp);
        assert!(resp_str.contains("HTTP/1.1 200 OK"));
        assert!(resp_str.contains("XHAIR — Crosshair Overlay & Editor"));
        assert!(resp_str.contains("<div id=\"app\"></div>"));

        // Test fetching JS asset dynamically from embedded assets
        let js_file = crate::server::EmbeddedDist::iter()
            .find(|f| f.ends_with(".js"))
            .map(|f| f.into_owned())
            .expect("should find a .js asset in EmbeddedDist");

        let mut stream_js =
            TcpStream::connect(format!("127.0.0.1:{}", port)).expect("connect js failed");
        stream_js
            .write_all(format!("GET /{} HTTP/1.1\r\nHost: 127.0.0.1\r\n\r\n", js_file).as_bytes())
            .unwrap();
        let mut resp_js = Vec::new();
        stream_js.read_to_end(&mut resp_js).unwrap();
        let resp_js_str = String::from_utf8_lossy(&resp_js);
        assert!(resp_js_str.contains("HTTP/1.1 200 OK"));
        assert!(resp_js_str.contains("Content-Type: application/javascript"));
    }

    #[test]
    fn test_server_library_api_lifecycle() {
        use std::io::{Read, Write};
        use std::net::TcpStream;

        let temp_dir = tempfile::tempdir().expect("failed to create tempdir");
        let library =
            xhair_library::LibraryEngine::new(temp_dir.path()).expect("failed to create engine");
        let mcp_ctx = xhair_mcp::ServerContext::new(Some(library));
        let port =
            server::start_ui_server(None, Some(mcp_ctx.clone())).expect("failed to start server");

        // 1. GET /api/library
        let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port)).expect("connect failed");
        stream
            .write_all(b"GET /api/library HTTP/1.1\r\nHost: 127.0.0.1\r\n\r\n")
            .unwrap();
        let mut resp = Vec::new();
        stream.read_to_end(&mut resp).unwrap();
        let resp_str = String::from_utf8_lossy(&resp);
        assert!(resp_str.contains("HTTP/1.1 200 OK"));
        assert!(resp_str.contains("\"crosshairs\":"));

        // 2. POST /api/library/save
        let new_ch = Crosshair::new_classic("REST Reticle");
        let body = serde_json::to_string(&new_ch).unwrap();
        let req = format!(
            "POST /api/library/save HTTP/1.1\r\nHost: 127.0.0.1\r\nContent-Length: {}\r\nContent-Type: application/json\r\n\r\n{}",
            body.len(),
            body
        );
        let mut stream_save =
            TcpStream::connect(format!("127.0.0.1:{}", port)).expect("connect failed");
        stream_save.write_all(req.as_bytes()).unwrap();
        let mut resp_save = Vec::new();
        stream_save.read_to_end(&mut resp_save).unwrap();
        let resp_save_str = String::from_utf8_lossy(&resp_save);
        assert!(resp_save_str.contains("HTTP/1.1 200 OK"));
        assert!(resp_save_str.contains("\"status\":\"ok\""));

        // 3. GET /api/library/get?id=...
        let get_req = format!(
            "GET /api/library/get?id={} HTTP/1.1\r\nHost: 127.0.0.1\r\n\r\n",
            new_ch.id
        );
        let mut stream_get =
            TcpStream::connect(format!("127.0.0.1:{}", port)).expect("connect failed");
        stream_get.write_all(get_req.as_bytes()).unwrap();
        let mut resp_get = Vec::new();
        stream_get.read_to_end(&mut resp_get).unwrap();
        let resp_get_str = String::from_utf8_lossy(&resp_get);
        assert!(resp_get_str.contains("HTTP/1.1 200 OK"));
        assert!(resp_get_str.contains("REST Reticle"));

        // 4. GET /api/mcp/activity
        mcp_ctx.log_activity("test.action", "args summary", "success");
        let mut stream_act =
            TcpStream::connect(format!("127.0.0.1:{}", port)).expect("connect failed");
        stream_act
            .write_all(b"GET /api/mcp/activity HTTP/1.1\r\nHost: 127.0.0.1\r\n\r\n")
            .unwrap();
        let mut resp_act = Vec::new();
        stream_act.read_to_end(&mut resp_act).unwrap();
        let resp_act_str = String::from_utf8_lossy(&resp_act);
        assert!(resp_act_str.contains("HTTP/1.1 200 OK"));
        assert!(resp_act_str.contains("test.action"));
    }

    #[test]
    fn test_embedded_mcp_and_endpoint_info() {
        let mcp_ctx = xhair_mcp::ServerContext::default();
        let mut mcp_server = xhair_mcp::McpServer::new(mcp_ctx);
        let token = "test_token_secret_12345".to_string();
        let config = xhair_mcp::HttpConfig {
            port: 0,
            bearer_token: token.clone(),
        };

        let socket_addr = mcp_server
            .start_http(config)
            .expect("failed to start MCP HTTP server");
        assert!(socket_addr.port() > 0);

        let endpoint_info = xhair_mcp::EndpointInfo {
            port: socket_addr.port(),
            token: token.clone(),
            pid: std::process::id(),
        };

        xhair_mcp::write_endpoint_info(&endpoint_info).expect("write endpoint failed");
        let read_info = xhair_mcp::read_endpoint_info().expect("read endpoint failed");
        assert_eq!(read_info.port, socket_addr.port());
        assert_eq!(read_info.token, token);
        assert!(xhair_mcp::is_endpoint_alive(&read_info));

        xhair_mcp::clear_endpoint_info();
        assert!(xhair_mcp::read_endpoint_info().is_none());
        mcp_server.stop_http();
    }

    #[test]
    fn test_e2e_mcp_mutates_crosshair_library_and_overlay() {
        let temp_dir = tempfile::tempdir().expect("tempdir failed");
        let library = xhair_library::LibraryEngine::new(temp_dir.path()).expect("engine failed");
        let mcp_ctx = xhair_mcp::ServerContext::new(Some(library));

        let (tx, rx) = std::sync::mpsc::channel::<OverlayCommand>();
        let overlay_sender = OverlaySender::new(tx, 0);
        mcp_ctx.set_overlay_controller(Arc::new(Win32OverlayController::new(overlay_sender)));

        let initial_ch = Crosshair::new_classic("E2E Test Reticle");
        let ch_id = initial_ch.id;
        mcp_ctx.with_state(|state| {
            if let Some(ref mut lib) = state.library {
                lib.save_crosshair(&initial_ch, None).unwrap();
            }
            state.active_crosshair_id = Some(ch_id);
            state.bus.register_document(initial_ch.clone());
        });

        // Mutate crosshair using MCP layer.add tool
        let new_layer =
            xhair_model::layer::Layer::new("Center Dot", xhair_model::shapes::LayerKind::default());
        let args = serde_json::json!({
            "layer": new_layer,
        });
        let result = xhair_mcp::tools::call_tool(&mcp_ctx, "layer.add", args);
        assert!(
            !result.is_error,
            "layer.add tool should succeed: {:?}",
            result
        );

        // 1. Verify in-memory state has the added layer (classic 1 layer + 1 added = 2)
        let mutated_ch =
            mcp_ctx.with_state(|state| state.bus.documents.get(&ch_id).unwrap().clone());
        assert_eq!(mutated_ch.layers.len(), 2);

        // 2. Verify disk library has the updated crosshair
        mcp_ctx.with_state(|state| {
            if let Some(ref mut lib) = state.library {
                lib.save_crosshair(&mutated_ch, None).unwrap();
                let loaded = lib.load_crosshair(ch_id).unwrap();
                assert_eq!(loaded.layers.len(), 2);
            }
        });

        // 3. Verify overlay receiver received the updated crosshair command
        let cmd = rx.try_recv().expect("overlay should have received command");
        match cmd {
            OverlayCommand::SetCrosshair(ch) => {
                assert_eq!(ch.id, ch_id);
                assert_eq!(ch.layers.len(), 2);
            }
            other => panic!("expected SetCrosshair command, got {:?}", other),
        }
    }

    #[test]
    fn test_e2e_ui_saves_profile_to_disk() {
        use std::io::{Read, Write};
        use std::net::TcpStream;

        let temp_dir = tempfile::tempdir().expect("tempdir failed");
        let library = xhair_library::LibraryEngine::new(temp_dir.path()).expect("engine failed");
        let mcp_ctx = xhair_mcp::ServerContext::new(Some(library));

        let (tx, rx) = std::sync::mpsc::channel::<OverlayCommand>();
        let overlay_sender = OverlaySender::new(tx, 0);
        let port = server::start_ui_server(Some(overlay_sender), Some(mcp_ctx.clone()))
            .expect("start server");

        let ch = Crosshair::new_classic("Disk Saved Reticle");
        let ch_id = ch.id;
        let body = serde_json::to_string(&ch).unwrap();
        let req = format!(
            "POST /api/library/save HTTP/1.1\r\nHost: 127.0.0.1\r\nContent-Length: {}\r\nContent-Type: application/json\r\n\r\n{}",
            body.len(),
            body
        );

        let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port)).expect("connect failed");
        stream.write_all(req.as_bytes()).unwrap();
        let mut resp = Vec::new();
        stream.read_to_end(&mut resp).unwrap();
        let resp_str = String::from_utf8_lossy(&resp);
        assert!(resp_str.contains("HTTP/1.1 200 OK"));
        assert!(resp_str.contains("\"status\":\"ok\""));

        // 1. Verify file exists on disk in the library data dir (library/crosshairs/<id>.xhair)
        let expected_file = temp_dir
            .path()
            .join("library")
            .join("crosshairs")
            .join(format!("{}.xhair", ch_id));
        assert!(
            expected_file.exists(),
            "crosshair file should exist at {:?}",
            expected_file
        );
        let disk_content = std::fs::read_to_string(expected_file).unwrap();
        assert!(disk_content.contains("Disk Saved Reticle"));

        // 2. Verify overlay receiver received the updated crosshair command
        let cmd = rx.try_recv().expect("overlay should have received command");
        match cmd {
            OverlayCommand::SetCrosshair(received_ch) => {
                assert_eq!(received_ch.id, ch_id);
                assert_eq!(received_ch.name, "Disk Saved Reticle");
            }
            other => panic!("expected SetCrosshair command, got {:?}", other),
        }
    }
}
