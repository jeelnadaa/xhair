//! Display monitor enumeration and Per-Monitor-V2 DPI scaling (§6.5).

use crate::error::PlatformError;
use windows::Win32::Foundation::{BOOL, LPARAM, RECT};
use windows::Win32::Graphics::Gdi::{
    EnumDisplayMonitors, GetMonitorInfoW, HDC, HMONITOR, MONITORINFOEXW,
};
use windows::Win32::UI::HiDpi::{GetDpiForMonitor, MDT_EFFECTIVE_DPI};

/// High-level representation of an active physical display monitor.
#[derive(Debug, Clone, PartialEq)]
pub struct MonitorInfo {
    /// Unique monitor device name or adapter string (e.g. `\\.\DISPLAY1`).
    pub device_name: String,
    /// Virtual desktop bounds of this monitor in physical pixels.
    pub bounds: (i32, i32, i32, i32), // (left, top, right, bottom)
    /// Work area rectangle (excluding taskbar) in physical pixels.
    pub work_area: (i32, i32, i32, i32),
    /// Effective DPI (horizontal, vertical).
    pub dpi: (u32, u32),
    /// Scaling multiplier relative to standard 96 DPI (e.g. 1.25 for 120 DPI).
    pub scale_factor: f32,
    /// Whether this is the primary Windows display monitor.
    pub is_primary: bool,
}

impl MonitorInfo {
    /// Width of monitor in physical pixels.
    pub fn width(&self) -> u32 {
        (self.bounds.2 - self.bounds.0).max(0) as u32
    }

    /// Height of monitor in physical pixels.
    pub fn height(&self) -> u32 {
        (self.bounds.3 - self.bounds.1).max(0) as u32
    }
}

/// Enumerates all currently connected and active display monitors.
pub fn enumerate_monitors() -> Result<Vec<MonitorInfo>, PlatformError> {
    let mut monitors = Vec::new();
    let ptr = &mut monitors as *mut Vec<MonitorInfo> as isize;

    unsafe {
        let res = EnumDisplayMonitors(HDC::default(), None, Some(enum_monitor_proc), LPARAM(ptr));
        if !res.as_bool() {
            return Err(PlatformError::MonitorQuery(
                "EnumDisplayMonitors returned FALSE".to_string(),
            ));
        }
    }

    Ok(monitors)
}

/// Retrieves the primary display monitor, or the first available monitor if none flagged primary.
pub fn get_primary_monitor() -> Result<MonitorInfo, PlatformError> {
    let monitors = enumerate_monitors()?;
    monitors
        .iter()
        .find(|m| m.is_primary)
        .cloned()
        .or_else(|| monitors.first().cloned())
        .ok_or_else(|| PlatformError::MonitorQuery("No active display monitors found".to_string()))
}

/// Finds the monitor containing the specified screen point.
pub fn get_monitor_at_point(point: (i32, i32)) -> Result<MonitorInfo, PlatformError> {
    let monitors = enumerate_monitors()?;
    monitors
        .iter()
        .find(|m| {
            point.0 >= m.bounds.0
                && point.0 < m.bounds.2
                && point.1 >= m.bounds.1
                && point.1 < m.bounds.3
        })
        .cloned()
        .or_else(|| monitors.first().cloned())
        .ok_or_else(|| PlatformError::MonitorQuery("No monitor found for coordinate".to_string()))
}

unsafe extern "system" fn enum_monitor_proc(
    hmonitor: HMONITOR,
    _: HDC,
    _: *mut RECT,
    lparam: LPARAM,
) -> BOOL {
    unsafe {
        let list = &mut *(lparam.0 as *mut Vec<MonitorInfo>);

        let mut info = MONITORINFOEXW::default();
        info.monitorInfo.cbSize = std::mem::size_of::<MONITORINFOEXW>() as u32;

        if GetMonitorInfoW(hmonitor, &mut info as *mut _ as *mut _).as_bool() {
            let is_primary = (info.monitorInfo.dwFlags & 1) != 0; // MONITORINFOF_PRIMARY = 1

            let dev_name = String::from_utf16_lossy(
                &info.szDevice[..info.szDevice.iter().position(|&c| c == 0).unwrap_or(32)],
            );

            let mut dpi_x = 96u32;
            let mut dpi_y = 96u32;
            let _ = GetDpiForMonitor(hmonitor, MDT_EFFECTIVE_DPI, &mut dpi_x, &mut dpi_y);

            let scale = dpi_x as f32 / 96.0;

            let b = info.monitorInfo.rcMonitor;
            let w = info.monitorInfo.rcWork;

            list.push(MonitorInfo {
                device_name: dev_name,
                bounds: (b.left, b.top, b.right, b.bottom),
                work_area: (w.left, w.top, w.right, w.bottom),
                dpi: (dpi_x, dpi_y),
                scale_factor: scale,
                is_primary,
            });
        }

        BOOL(1) // Continue enumeration
    }
}
