//! System tray notification icon and tooltip state management (§8.3).

use crate::error::PlatformError;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::Shell::{
    Shell_NotifyIconW, NIF_ICON, NIF_MESSAGE, NIF_TIP, NIM_ADD, NIM_DELETE, NIM_MODIFY,
    NOTIFYICONDATAW,
};
use windows::Win32::UI::WindowsAndMessaging::{CreateIcon, DestroyIcon, HICON};

/// Custom window message identifier for tray icon interactions.
pub const WM_TRAY_CALLBACK: u32 = 0x8001; // WM_APP + 1

/// Status flags displayed in tray tooltip.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrayStatus {
    /// Whether the overlay is currently visible.
    pub overlay_enabled: bool,
    /// Currently active crosshair display name.
    pub crosshair_name: String,
    /// Whether Dynamic Reticle is active.
    pub dynamic_enabled: bool,
    /// Whether MCP server is running.
    pub mcp_enabled: bool,
}

impl Default for TrayStatus {
    fn default() -> Self {
        Self {
            overlay_enabled: true,
            crosshair_name: "Default".to_string(),
            dynamic_enabled: false,
            mcp_enabled: false,
        }
    }
}

impl TrayStatus {
    /// Formats tooltip text per spec §8.3: "xhair | Overlay ON | Reticle: Name | ..."
    pub fn format_tooltip(&self) -> String {
        format!(
            "xhair\nOverlay: {}\nReticle: {}\nDynamic: {}\nMCP: {}",
            if self.overlay_enabled { "ON" } else { "OFF" },
            self.crosshair_name,
            if self.dynamic_enabled { "ON" } else { "OFF" },
            if self.mcp_enabled { "ON" } else { "OFF" }
        )
    }
}

/// Encapsulates the Windows system tray icon lifecycle.
pub struct TrayIcon {
    hwnd: HWND,
    icon: HICON,
    is_added: bool,
}

impl TrayIcon {
    /// Creates and registers a tray icon attached to the given window.
    pub fn new(hwnd: HWND, initial_status: &TrayStatus) -> Result<Self, PlatformError> {
        let icon = create_default_tray_icon();
        let mut tray = Self {
            hwnd,
            icon,
            is_added: false,
        };

        tray.add_icon(initial_status)?;
        Ok(tray)
    }

    /// Adds the notification icon into the Windows shell tray.
    fn add_icon(&mut self, status: &TrayStatus) -> Result<(), PlatformError> {
        let nid = self.create_notify_data(status);
        unsafe {
            if Shell_NotifyIconW(NIM_ADD, &nid).as_bool() {
                self.is_added = true;
                Ok(())
            } else {
                Err(PlatformError::TrayError(
                    "Shell_NotifyIconW NIM_ADD failed".to_string(),
                ))
            }
        }
    }

    /// Updates the tooltip string and icon status flags in the system tray.
    pub fn update_status(&mut self, status: &TrayStatus) -> Result<(), PlatformError> {
        if !self.is_added {
            return self.add_icon(status);
        }

        let nid = self.create_notify_data(status);
        unsafe {
            if Shell_NotifyIconW(NIM_MODIFY, &nid).as_bool() {
                Ok(())
            } else {
                Err(PlatformError::TrayError(
                    "Shell_NotifyIconW NIM_MODIFY failed".to_string(),
                ))
            }
        }
    }

    /// Removes the tray icon from the Windows shell.
    pub fn remove(&mut self) {
        if self.is_added {
            let nid = NOTIFYICONDATAW {
                cbSize: std::mem::size_of::<NOTIFYICONDATAW>() as u32,
                hWnd: self.hwnd,
                uID: 1,
                ..Default::default()
            };
            unsafe {
                let _ = Shell_NotifyIconW(NIM_DELETE, &nid);
            }
            self.is_added = false;
        }
    }

    fn create_notify_data(&self, status: &TrayStatus) -> NOTIFYICONDATAW {
        let mut nid = NOTIFYICONDATAW {
            cbSize: std::mem::size_of::<NOTIFYICONDATAW>() as u32,
            hWnd: self.hwnd,
            uID: 1,
            uFlags: NIF_ICON | NIF_MESSAGE | NIF_TIP,
            uCallbackMessage: WM_TRAY_CALLBACK,
            hIcon: self.icon,
            ..Default::default()
        };

        let tooltip = status.format_tooltip();
        let utf16: Vec<u16> = tooltip.encode_utf16().take(127).collect();
        for (i, &ch) in utf16.iter().enumerate() {
            nid.szTip[i] = ch;
        }
        nid.szTip[utf16.len().min(127)] = 0;

        nid
    }
}

impl Drop for TrayIcon {
    fn drop(&mut self) {
        self.remove();
        if !self.icon.0.is_null() {
            unsafe {
                let _ = DestroyIcon(self.icon);
            }
        }
    }
}

/// Creates a minimalist 16x16 pixel reticle icon for the system tray.
fn create_default_tray_icon() -> HICON {
    // 16x16 monochrome reticle bitmap
    let and_mask = [0xFFu8; 32]; // 16 rows * 2 bytes
    let mut xor_mask = [0x00u8; 32];

    // Draw simple crosshair in XOR mask
    // Row 7 & 8: horizontal line
    xor_mask[7 * 2] = 0xFE;
    xor_mask[7 * 2 + 1] = 0x7F;
    xor_mask[8 * 2] = 0xFE;
    xor_mask[8 * 2 + 1] = 0x7F;
    // Columns 7 & 8: vertical line
    for r in 0..16 {
        xor_mask[r * 2] |= 0x01;
        xor_mask[r * 2 + 1] |= 0x80;
    }

    unsafe {
        CreateIcon(None, 16, 16, 1, 1, and_mask.as_ptr(), xor_mask.as_ptr())
            .unwrap_or(HICON::default())
    }
}
