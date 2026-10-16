//! Layered click-through overlay window implementation (§8.1, §8.2, §8.3).

use crate::calibration::create_calibration_crosshair;
use crate::error::PlatformError;
use crate::foreground::{get_foreground_window_info, is_game_foreground};
use crate::monitor::{get_primary_monitor, MonitorInfo};
use crate::tray::{TrayIcon, TrayStatus};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::{self, JoinHandle};
use tiny_skia::Pixmap;
use windows::core::w;
use windows::Win32::Foundation::{COLORREF, HINSTANCE, HWND, LPARAM, LRESULT, POINT, SIZE, WPARAM};
use windows::Win32::Graphics::Gdi::{
    CreateCompatibleDC, CreateDIBSection, DeleteDC, DeleteObject, GetDC, ReleaseDC, SelectObject,
    BITMAPINFO, BITMAPINFOHEADER, BI_RGB, BLENDFUNCTION, DIB_RGB_COLORS,
};
use windows::Win32::UI::HiDpi::{
    SetProcessDpiAwarenessContext, DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2,
};
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, DestroyWindow, DispatchMessageW, GetMessageW, KillTimer,
    PostMessageW, PostQuitMessage, RegisterClassW, SetTimer, SetWindowDisplayAffinity,
    SetWindowPos, ShowWindow, TranslateMessage, UpdateLayeredWindow, HWND_TOPMOST, MSG,
    SWP_NOACTIVATE, SWP_SHOWWINDOW, SW_HIDE, SW_SHOWNOACTIVATE, ULW_ALPHA, WM_TIMER, WNDCLASSW,
    WS_EX_LAYERED, WS_EX_NOACTIVATE, WS_EX_TOOLWINDOW, WS_EX_TOPMOST, WS_EX_TRANSPARENT, WS_POPUP,
};
use xhair_geometry::{GeometryMapping, ResolutionProfile};
use xhair_model::crosshair::Crosshair;
use xhair_model::units::Vec2;
use xhair_render::bounds::calculate_crosshair_bounds;
use xhair_render::context::{RenderContext, RenderQuality};
use xhair_render::render;

/// Custom Win32 message indicating an internal overlay state update.
pub const WM_APP_OVERLAY_UPDATE: u32 = 0x8002;

/// Visibility rules for overlay display (§8.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VisibilityMode {
    /// Always visible on top of all windows.
    AlwaysShow,
    /// Visible only when a configured game process has foreground focus.
    #[default]
    OnlyWhenGameForeground,
    /// Explicitly hidden.
    Hidden,
}

/// Operational configuration for the overlay window.
#[derive(Debug, Clone, PartialEq)]
pub struct OverlayConfig {
    /// Active visibility rule.
    pub visibility_mode: VisibilityMode,
    /// List of game executable image names for focus detection.
    pub target_game_images: Vec<String>,
    /// Whether to track and align center to the game window's client rectangle.
    pub follow_game_window: bool,
    /// Hide overlay from game streams and recordings via WDA_EXCLUDEFROMCAPTURE.
    pub exclude_from_capture: bool,
    /// Custom calibration nudge offset in game pixels.
    pub calibration_offset: Vec2,
    /// Whether the calibration test pattern is active instead of normal crosshair.
    pub calibration_mode: bool,
}

impl Default for OverlayConfig {
    fn default() -> Self {
        Self {
            visibility_mode: VisibilityMode::OnlyWhenGameForeground,
            target_game_images: vec!["VALORANT-Win64-Shipping.exe".to_string()],
            follow_game_window: true,
            exclude_from_capture: false,
            calibration_offset: Vec2::ZERO,
            calibration_mode: false,
        }
    }
}

/// Commands sent asynchronously to the overlay window message loop.
#[derive(Debug)]
pub enum OverlayCommand {
    /// Update the currently rendered crosshair document.
    SetCrosshair(Box<Crosshair>),
    /// Update active resolution profile.
    SetResolutionProfile(Box<ResolutionProfile>),
    /// Update configuration flags.
    SetConfig(Box<OverlayConfig>),
    /// Toggle overlay visibility.
    ToggleVisibility,
    /// Explicitly enable or disable overlay visibility.
    SetEnabled(bool),
    /// Toggle calibration test pattern overlay.
    ToggleCalibrationPattern,
    /// Nudge crosshair position by delta.
    Nudge(Vec2),
    /// Terminate overlay message loop.
    Quit,
}

/// Thread-safe cloneable handle for sending commands to the overlay window.
#[derive(Clone)]
pub struct OverlaySender {
    sender: Sender<OverlayCommand>,
    hwnd_raw: isize,
}

impl OverlaySender {
    /// Creates a new OverlaySender handle.
    pub fn new(sender: Sender<OverlayCommand>, hwnd_raw: isize) -> Self {
        Self { sender, hwnd_raw }
    }

    /// Sends a command to the overlay window thread.
    pub fn send(&self, cmd: OverlayCommand) -> Result<(), PlatformError> {
        self.sender
            .send(cmd)
            .map_err(|e| PlatformError::ChannelError(e.to_string()))?;
        if self.hwnd_raw != 0 {
            unsafe {
                let hwnd = HWND(self.hwnd_raw as *mut _);
                let _ = PostMessageW(hwnd, WM_APP_OVERLAY_UPDATE, WPARAM(0), LPARAM(0));
            }
        }
        Ok(())
    }
}

/// Handle to the background overlay thread.
pub struct OverlayHandle {
    sender: Sender<OverlayCommand>,
    hwnd: HWND,
    thread_handle: Option<JoinHandle<()>>,
}

impl OverlayHandle {
    /// Obtains a cloneable sender for issuing overlay commands from other threads.
    pub fn get_sender(&self) -> OverlaySender {
        OverlaySender {
            sender: self.sender.clone(),
            hwnd_raw: self.hwnd.0 as isize,
        }
    }

    /// Sends a command to the overlay window thread.
    pub fn send(&self, cmd: OverlayCommand) -> Result<(), PlatformError> {
        self.get_sender().send(cmd)
    }

    /// Stops the overlay thread and waits for clean exit.
    pub fn stop(mut self) {
        let _ = self.send(OverlayCommand::Quit);
        if let Some(handle) = self.thread_handle.take() {
            let _ = handle.join();
        }
    }

    /// Waits for the overlay thread to exit.
    pub fn wait(mut self) {
        if let Some(handle) = self.thread_handle.take() {
            let _ = handle.join();
        }
    }
}

/// Spawns the overlay window on a dedicated message-loop thread (§8.1).
pub fn spawn_overlay(
    initial_crosshair: Crosshair,
    initial_profile: ResolutionProfile,
    config: OverlayConfig,
) -> Result<OverlayHandle, PlatformError> {
    let (cmd_tx, cmd_rx) = channel();
    let (ready_tx, ready_rx) = channel::<Result<isize, PlatformError>>();

    let thread_handle = thread::Builder::new()
        .name("xhair-overlay".to_string())
        .spawn(move || {
            run_overlay_thread(initial_crosshair, initial_profile, config, cmd_rx, ready_tx);
        })
        .map_err(|e| PlatformError::WindowCreation(e.to_string()))?;

    let hwnd_raw = ready_rx
        .recv()
        .map_err(|e| PlatformError::ChannelError(e.to_string()))??;
    let hwnd = HWND(hwnd_raw as *mut _);

    Ok(OverlayHandle {
        sender: cmd_tx,
        hwnd,
        thread_handle: Some(thread_handle),
    })
}

fn run_overlay_thread(
    mut crosshair: Crosshair,
    mut profile: ResolutionProfile,
    mut config: OverlayConfig,
    cmd_rx: Receiver<OverlayCommand>,
    ready_tx: Sender<Result<isize, PlatformError>>,
) {
    // 1. Enable Per-Monitor-V2 DPI awareness (§6.5)
    unsafe {
        let _ = SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2);
    }

    // 2. Register overlay window class
    let class_name = w!("XHAIR_OVERLAY_WINDOW_CLASS");
    let hinstance = HINSTANCE::default();

    let wnd_class = WNDCLASSW {
        lpfnWndProc: Some(overlay_wnd_proc),
        hInstance: hinstance,
        lpszClassName: class_name,
        ..Default::default()
    };

    unsafe {
        let atom = RegisterClassW(&wnd_class);
        if atom == 0 {
            let _ = ready_tx.send(Err(PlatformError::WindowCreation(
                "RegisterClassW failed".to_string(),
            )));
            return;
        }
    }

    // 3. Create layered click-through topmost popup window (§8.1)
    let ex_style =
        WS_EX_LAYERED | WS_EX_TRANSPARENT | WS_EX_TOPMOST | WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE;

    let hwnd = match unsafe {
        CreateWindowExW(
            ex_style,
            class_name,
            w!("XHAIR Overlay"),
            WS_POPUP,
            0,
            0,
            1,
            1,
            None,
            None,
            hinstance,
            None,
        )
    } {
        Ok(h) => h,
        Err(e) => {
            let _ = ready_tx.send(Err(PlatformError::WindowCreation(e.to_string())));
            return;
        }
    };

    // Configure capture affinity
    unsafe {
        let affinity = if config.exclude_from_capture {
            0x00000011
        } else {
            0
        }; // WDA_EXCLUDEFROMCAPTURE = 0x11
        let _ = SetWindowDisplayAffinity(
            hwnd,
            windows::Win32::UI::WindowsAndMessaging::WINDOW_DISPLAY_AFFINITY(affinity),
        );
    }

    // Initialize System Tray icon (§8.3)
    let mut tray_status = TrayStatus {
        overlay_enabled: config.visibility_mode != VisibilityMode::Hidden,
        crosshair_name: crosshair.name.clone(),
        dynamic_enabled: false,
        mcp_enabled: false,
    };
    let mut tray_icon = TrayIcon::new(hwnd, &tray_status).ok();

    // Signal ready
    if ready_tx.send(Ok(hwnd.0 as isize)).is_err() {
        return;
    }

    // Initial render and display
    render_and_update(hwnd, &crosshair, &profile, &config);

    // Register global hotkeys (§8.4): ID 1: Ctrl+Shift+X (Toggle Overlay), ID 2: Ctrl+Shift+C (Recenter)
    unsafe {
        use windows::Win32::UI::Input::KeyboardAndMouse::{
            RegisterHotKey, MOD_CONTROL, MOD_NOREPEAT, MOD_SHIFT,
        };
        let _ = RegisterHotKey(hwnd, 1, MOD_CONTROL | MOD_SHIFT | MOD_NOREPEAT, 0x58); // X
        let _ = RegisterHotKey(hwnd, 2, MOD_CONTROL | MOD_SHIFT | MOD_NOREPEAT, 0x43);
        // C
    }

    // Set a periodic 200ms timer to check foreground focus and refresh overlay positioning (§8.1)
    unsafe {
        let _ = SetTimer(hwnd, 1, 200, None);
    }

    // Win32 Message Loop
    let mut msg = MSG::default();
    loop {
        let res = unsafe { GetMessageW(&mut msg, None, 0, 0) };
        if !res.as_bool() || res.0 == -1 {
            break;
        }

        if msg.message == windows::Win32::UI::WindowsAndMessaging::WM_HOTKEY {
            let hotkey_id = msg.wParam.0 as i32;
            if hotkey_id == 1 {
                let is_hidden = config.visibility_mode == VisibilityMode::Hidden;
                config.visibility_mode = if is_hidden {
                    if config.follow_game_window {
                        VisibilityMode::OnlyWhenGameForeground
                    } else {
                        VisibilityMode::AlwaysShow
                    }
                } else {
                    VisibilityMode::Hidden
                };
                tray_status.overlay_enabled = config.visibility_mode != VisibilityMode::Hidden;
                if let Some(ref mut tray) = tray_icon {
                    let _ = tray.update_status(&tray_status);
                }
                render_and_update(hwnd, &crosshair, &profile, &config);
            } else if hotkey_id == 2 {
                config.calibration_offset = Vec2::ZERO;
                render_and_update(hwnd, &crosshair, &profile, &config);
            }
        } else if msg.message == WM_APP_OVERLAY_UPDATE || msg.message == WM_TIMER {
            while let Ok(cmd) = cmd_rx.try_recv() {
                match cmd {
                    OverlayCommand::SetCrosshair(new_xhair) => {
                        crosshair = *new_xhair;
                        tray_status.crosshair_name = crosshair.name.clone();
                        if let Some(ref mut tray) = tray_icon {
                            let _ = tray.update_status(&tray_status);
                        }
                    }
                    OverlayCommand::SetResolutionProfile(new_profile) => {
                        profile = *new_profile;
                    }
                    OverlayCommand::SetConfig(new_config) => {
                        config = *new_config;
                        tray_status.overlay_enabled =
                            config.visibility_mode != VisibilityMode::Hidden;
                        if let Some(ref mut tray) = tray_icon {
                            let _ = tray.update_status(&tray_status);
                        }
                    }
                    OverlayCommand::ToggleVisibility => {
                        let is_hidden = config.visibility_mode == VisibilityMode::Hidden;
                        config.visibility_mode = if is_hidden {
                            if config.follow_game_window {
                                VisibilityMode::OnlyWhenGameForeground
                            } else {
                                VisibilityMode::AlwaysShow
                            }
                        } else {
                            VisibilityMode::Hidden
                        };
                        tray_status.overlay_enabled =
                            config.visibility_mode != VisibilityMode::Hidden;
                        if let Some(ref mut tray) = tray_icon {
                            let _ = tray.update_status(&tray_status);
                        }
                    }
                    OverlayCommand::SetEnabled(enabled) => {
                        config.visibility_mode = if enabled {
                            if config.follow_game_window {
                                VisibilityMode::OnlyWhenGameForeground
                            } else {
                                VisibilityMode::AlwaysShow
                            }
                        } else {
                            VisibilityMode::Hidden
                        };
                        tray_status.overlay_enabled = enabled;
                        if let Some(ref mut tray) = tray_icon {
                            let _ = tray.update_status(&tray_status);
                        }
                    }
                    OverlayCommand::ToggleCalibrationPattern => {
                        config.calibration_mode = !config.calibration_mode;
                    }
                    OverlayCommand::Nudge(delta) => {
                        config.calibration_offset = config.calibration_offset + delta;
                    }
                    OverlayCommand::Quit => {
                        unsafe {
                            use windows::Win32::UI::Input::KeyboardAndMouse::UnregisterHotKey;
                            let _ = UnregisterHotKey(hwnd, 1);
                            let _ = UnregisterHotKey(hwnd, 2);
                            let _ = KillTimer(hwnd, 1);
                            let _ = DestroyWindow(hwnd);
                            PostQuitMessage(0);
                        }
                        return;
                    }
                }
            }
            render_and_update(hwnd, &crosshair, &profile, &config);
        } else {
            unsafe {
                let _ = TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }
    }
}

/// Computes physical window placement and dimensions symmetrically around the screen center,
/// ensuring that calibration offsets, layer nudges, and asymmetric arms never clip (§6.6, §8.1).
pub fn compute_overlay_window_rect(
    active_xhair: &Crosshair,
    mapping: &GeometryMapping,
    config: &OverlayConfig,
) -> (i32, i32, u32, u32) {
    let bbox = calculate_crosshair_bounds(active_xhair);
    let scale_x = mapping.sx as f32;
    let scale_y = mapping.sy as f32;
    let max_half_w = (bbox.min_x * scale_x)
        .abs()
        .max((bbox.max_x * scale_x).abs());
    let max_half_h = (bbox.min_y * scale_y)
        .abs()
        .max((bbox.max_y * scale_y).abs());
    let win_w = ((max_half_w * 2.0).ceil() as u32 + 16).clamp(32, 2048);
    let win_h = ((max_half_h * 2.0).ceil() as u32 + 16).clamp(32, 2048);

    let screen_center_x = mapping.out_center_x as f32 + config.calibration_offset.x;
    let screen_center_y = mapping.out_center_y as f32 + config.calibration_offset.y;
    let win_x = (screen_center_x - (win_w as f32 / 2.0)).round() as i32;
    let win_y = (screen_center_y - (win_h as f32 / 2.0)).round() as i32;

    (win_x, win_y, win_w, win_h)
}

/// Renders the active crosshair and commits via UpdateLayeredWindow (§6.6, §8.1).
pub fn render_and_update(
    hwnd: HWND,
    crosshair: &Crosshair,
    profile: &ResolutionProfile,
    config: &OverlayConfig,
) {
    // Check visibility conditions
    let should_show = match config.visibility_mode {
        VisibilityMode::AlwaysShow => true,
        VisibilityMode::Hidden => false,
        VisibilityMode::OnlyWhenGameForeground => is_game_foreground(&config.target_game_images),
    };

    if !should_show {
        unsafe {
            let _ = ShowWindow(hwnd, SW_HIDE);
        }
        return;
    }

    // Determine target render geometry and center point
    let fg_info = if config.follow_game_window && is_game_foreground(&config.target_game_images) {
        get_foreground_window_info().ok().flatten()
    } else {
        None
    };

    let (out_origin, out_w, out_h) = if let Some(ref info) = fg_info {
        (
            (info.client_rect.0, info.client_rect.1),
            info.client_width(),
            info.client_height(),
        )
    } else {
        let monitor = get_primary_monitor().unwrap_or(MonitorInfo {
            device_name: "Default".to_string(),
            bounds: (0, 0, 1920, 1080),
            work_area: (0, 0, 1920, 1080),
            dpi: (96, 96),
            scale_factor: 1.0,
            is_primary: true,
        });
        (
            (monitor.bounds.0, monitor.bounds.1),
            monitor.width(),
            monitor.height(),
        )
    };

    let mapping = GeometryMapping::compute(
        profile,
        (out_origin.0 as f64, out_origin.1 as f64),
        (out_w as f64, out_h as f64),
    );
    let center_frac_x = (mapping.out_center_x.fract() + 1.0).fract() as f32;
    let center_frac_y = (mapping.out_center_y.fract() + 1.0).fract() as f32;
    let center_frac = Vec2::new(center_frac_x, center_frac_y);

    // Prepare active crosshair (either normal or calibration pattern)
    let active_xhair = if config.calibration_mode {
        create_calibration_crosshair(
            profile.game_resolution.width,
            profile.game_resolution.height,
        )
    } else {
        crosshair.clone()
    };

    // Calculate conservative bounding box
    let render_ctx = RenderContext {
        scale: Vec2::new(mapping.sx as f32, mapping.sy as f32),
        center_frac,
        aa: active_xhair.render_hints.aa_mode,
        time: 0.0,
        dynamics: None,
        quality: RenderQuality::Standard,
        static_only: false,
    };

    let (win_x, win_y, win_w, win_h) = compute_overlay_window_rect(&active_xhair, &mapping, config);

    // Allocate raster surface
    let mut pixmap = match Pixmap::new(win_w, win_h) {
        Some(p) => p,
        None => return,
    };

    // Deterministic tiny-skia render
    if render(&active_xhair, &render_ctx, &mut pixmap).is_err() {
        return;
    }

    // Swap Red and Blue channels in-place for Win32 32-bit premultiplied BGRA DIB
    let data = pixmap.data_mut();
    for chunk in data.chunks_exact_mut(4) {
        chunk.swap(0, 2);
    }

    unsafe {
        // Commit premultiplied BGRA pixels via UpdateLayeredWindow
        let hdc_screen = GetDC(HWND::default());
        let hdc_mem = CreateCompatibleDC(hdc_screen);

        let bmi = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: win_w as i32,
                biHeight: -(win_h as i32), // Top-down DIB
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB.0,
                ..Default::default()
            },
            ..Default::default()
        };

        let mut p_bits = std::ptr::null_mut();
        let hbitmap = CreateDIBSection(hdc_mem, &bmi, DIB_RGB_COLORS, &mut p_bits, None, 0);

        if let Ok(hbitmap) = hbitmap {
            if !p_bits.is_null() {
                std::ptr::copy_nonoverlapping(
                    pixmap.data().as_ptr(),
                    p_bits as *mut u8,
                    pixmap.data().len(),
                );

                let old_bitmap = SelectObject(hdc_mem, hbitmap);

                let pt_dst = POINT { x: win_x, y: win_y };
                let size = SIZE {
                    cx: win_w as i32,
                    cy: win_h as i32,
                };
                let pt_src = POINT { x: 0, y: 0 };

                let blend = BLENDFUNCTION {
                    BlendOp: 0, // AC_SRC_OVER
                    BlendFlags: 0,
                    SourceConstantAlpha: 255,
                    AlphaFormat: 1, // AC_SRC_ALPHA
                };

                let _ = UpdateLayeredWindow(
                    hwnd,
                    hdc_screen,
                    Some(&pt_dst),
                    Some(&size),
                    hdc_mem,
                    Some(&pt_src),
                    COLORREF(0),
                    Some(&blend),
                    ULW_ALPHA,
                );

                let _ = SelectObject(hdc_mem, old_bitmap);
            }
            let _ = DeleteObject(hbitmap);
        }

        let _ = DeleteDC(hdc_mem);
        let _ = ReleaseDC(HWND::default(), hdc_screen);

        // Ensure topmost layered click-through display
        let _ = SetWindowPos(
            hwnd,
            HWND_TOPMOST,
            win_x,
            win_y,
            win_w as i32,
            win_h as i32,
            SWP_NOACTIVATE | SWP_SHOWWINDOW,
        );
        let _ = ShowWindow(hwnd, SW_SHOWNOACTIVATE);
    }
}

unsafe extern "system" fn overlay_wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
}
