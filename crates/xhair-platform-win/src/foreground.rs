//! Safe foreground window and game process detection (§2, §8.1, §8.2).

use crate::error::PlatformError;
use std::path::Path;
use windows::core::PWSTR;
use windows::Win32::Foundation::{CloseHandle, HWND, POINT, RECT};
use windows::Win32::Graphics::Dwm::{DwmGetWindowAttribute, DWMWA_EXTENDED_FRAME_BOUNDS};
use windows::Win32::Graphics::Gdi::ClientToScreen;
use windows::Win32::System::Threading::{
    OpenProcess, QueryFullProcessImageNameW, PROCESS_NAME_WIN32, PROCESS_QUERY_LIMITED_INFORMATION,
};
use windows::Win32::UI::WindowsAndMessaging::{
    GetClientRect, GetForegroundWindow, GetWindowRect, GetWindowThreadProcessId, IsWindowVisible,
};

/// High-level details of the currently focused foreground window.
#[derive(Debug, Clone, PartialEq)]
pub struct ForegroundWindowInfo {
    /// Native window handle.
    pub hwnd: HWND,
    /// Base executable file name (e.g. `VALORANT-Win64-Shipping.exe`).
    pub image_name: String,
    /// Full path to process executable (if query succeeded).
    pub image_path: Option<String>,
    /// Outer window frame bounds in screen physical pixels.
    pub window_rect: (i32, i32, i32, i32), // (left, top, right, bottom)
    /// Inner client area bounds in screen physical pixels.
    pub client_rect: (i32, i32, i32, i32),
    /// Whether the process is likely elevated higher than this app.
    pub is_elevated_access_limited: bool,
}

impl ForegroundWindowInfo {
    /// Width of the client rendering area.
    pub fn client_width(&self) -> u32 {
        (self.client_rect.2 - self.client_rect.0).max(0) as u32
    }

    /// Height of the client rendering area.
    pub fn client_height(&self) -> u32 {
        (self.client_rect.3 - self.client_rect.1).max(0) as u32
    }
}

/// Queries details of the currently focused foreground window.
pub fn get_foreground_window_info() -> Result<Option<ForegroundWindowInfo>, PlatformError> {
    unsafe {
        let hwnd = GetForegroundWindow();
        if hwnd.0.is_null() || !IsWindowVisible(hwnd).as_bool() {
            return Ok(None);
        }

        let mut pid = 0u32;
        GetWindowThreadProcessId(hwnd, Some(&mut pid));
        if pid == 0 {
            return Ok(None);
        }

        // Query process executable image name using limited-query rights (OS compliance §2)
        let process_handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid);
        let mut image_name = String::new();
        let mut image_path = None;
        let mut is_elevated_access_limited = false;

        match process_handle {
            Ok(handle) => {
                let mut buffer = [0u16; 1024];
                let mut size = buffer.len() as u32;

                if QueryFullProcessImageNameW(
                    handle,
                    PROCESS_NAME_WIN32,
                    PWSTR(buffer.as_mut_ptr()),
                    &mut size,
                )
                .is_ok()
                {
                    let path_str = String::from_utf16_lossy(&buffer[..size as usize]);
                    let base_name = Path::new(&path_str)
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("")
                        .to_string();

                    image_name = base_name;
                    image_path = Some(path_str);
                } else {
                    is_elevated_access_limited = true;
                }
                let _ = CloseHandle(handle);
            }
            Err(_) => {
                is_elevated_access_limited = true;
            }
        }

        // Query window rectangle (prefer DWM extended frame bounds for accurate borderless rect)
        let mut win_rect = RECT::default();
        let dwm_res = DwmGetWindowAttribute(
            hwnd,
            DWMWA_EXTENDED_FRAME_BOUNDS,
            &mut win_rect as *mut _ as *mut _,
            std::mem::size_of::<RECT>() as u32,
        );

        if dwm_res.is_err() {
            let _ = GetWindowRect(hwnd, &mut win_rect);
        }

        // Query client rectangle and map to screen coordinates
        let mut cl_rect = RECT::default();
        let _ = GetClientRect(hwnd, &mut cl_rect);

        let mut origin = POINT {
            x: cl_rect.left,
            y: cl_rect.top,
        };
        let _ = ClientToScreen(hwnd, &mut origin);

        let client_screen_rect = (
            origin.x,
            origin.y,
            origin.x + (cl_rect.right - cl_rect.left),
            origin.y + (cl_rect.bottom - cl_rect.top),
        );

        Ok(Some(ForegroundWindowInfo {
            hwnd,
            image_name,
            image_path,
            window_rect: (win_rect.left, win_rect.top, win_rect.right, win_rect.bottom),
            client_rect: client_screen_rect,
            is_elevated_access_limited,
        }))
    }
}

/// Checks whether the current foreground window matches any of the specified target image names.
pub fn is_game_foreground(target_image_names: &[String]) -> bool {
    if let Ok(Some(info)) = get_foreground_window_info() {
        if info.image_name.is_empty() {
            return false;
        }
        target_image_names
            .iter()
            .any(|target| target.eq_ignore_ascii_case(&info.image_name))
    } else {
        false
    }
}
