//! Desktop application entry point, system tray daemon, and webview window management.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

/// UI crosshair JSON parser and bridge adapter.
pub mod bridge;
/// Local HTTP server and embedded asset delivery.
pub mod server;

use xhair_geometry::ResolutionProfile;
use xhair_model::crosshair::Crosshair;
use xhair_platform_win::{spawn_overlay, OverlayCommand, OverlayConfig, VisibilityMode};

/// Application default window title.
pub const APP_TITLE: &str = "XHAIR";

/// Runs the main application event loop.
pub fn run() -> anyhow::Result<()> {
    // 1. Install local panic crash handler with zero telemetry (§16, M10)
    let crash_dir = xhair_platform_win::install_crash_handler()
        .map_err(|e| anyhow::anyhow!("Failed to initialize crash handler: {}", e))?;
    tracing::debug!("Crash reports directory: {:?}", crash_dir);

    let version = env!("CARGO_PKG_VERSION");
    let args: Vec<String> = std::env::args().collect();
    let is_minimized = args.iter().any(|a| a == "--minimized");

    tracing::info!(
        "Starting {} v{} (minimized={})",
        APP_TITLE,
        version,
        is_minimized
    );

    // 2. Load default crosshair ("Cyan Gap" with 4 arms + center dot) and resolution profile (§4, §6)
    let crosshair = Crosshair::new_default();
    let profile = ResolutionProfile::default_1080p();

    // 3. Configure overlay: default to visible on screen (§8.1)
    let config = OverlayConfig {
        visibility_mode: VisibilityMode::AlwaysShow,
        follow_game_window: false,
        ..Default::default()
    };

    // 4. Spawn the Win32 transparent click-through overlay window + tray icon + hotkeys (§8)
    let overlay_handle = spawn_overlay(crosshair, profile, config)
        .map_err(|e| anyhow::anyhow!("Failed to spawn overlay: {}", e))?;
    let overlay_sender = overlay_handle.get_sender();

    // 5. If not launched minimized, start the local UI server and launch native desktop window
    if !is_minimized {
        if let Some(port) = server::start_ui_server(Some(overlay_sender.clone())) {
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
            let webview = wry::WebViewBuilder::new()
                .with_url(&url)
                .with_devtools(true)
                .with_ipc_handler(move |msg| {
                    if let Ok(ch) = bridge::parse_ui_crosshair(msg.body()) {
                        let _ = ipc_sender.send(OverlayCommand::SetCrosshair(Box::new(ch)));
                    }
                })
                .build(&window)
                .map_err(|e| anyhow::anyhow!("Failed to create webview: {}", e))?;

            event_loop.run(move |event, _, control_flow| {
                let _ = &webview;
                *control_flow = tao::event_loop::ControlFlow::Wait;
                if let tao::event::Event::WindowEvent {
                    event: tao::event::WindowEvent::CloseRequested,
                    ..
                } = event
                {
                    *control_flow = tao::event_loop::ControlFlow::Exit;
                }
            });
        }
    }

    // 6. Keep daemon running until exit (blocks on overlay thread message loop / tray menu)
    overlay_handle.wait();

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
        let port = server::start_ui_server(None).expect("failed to start server");
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
}
