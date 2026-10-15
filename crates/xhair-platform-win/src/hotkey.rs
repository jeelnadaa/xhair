//! Windows global hotkey registration and conflict detection (§8.4).

use crate::error::PlatformError;
use std::collections::HashMap;
use windows::Win32::Foundation::{GetLastError, HWND};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    RegisterHotKey, UnregisterHotKey, HOT_KEY_MODIFIERS, MOD_ALT, MOD_CONTROL, MOD_NOREPEAT,
    MOD_SHIFT, MOD_WIN,
};

/// Hotkey modifier keys flag bitmask.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct HotkeyModifiers {
    /// Control key.
    pub ctrl: bool,
    /// Alt key.
    pub alt: bool,
    /// Shift key.
    pub shift: bool,
    /// Windows logo key.
    pub win: bool,
}

impl HotkeyModifiers {
    /// Converts to Win32 HOT_KEY_MODIFIERS flags.
    pub fn to_win32_flags(self) -> HOT_KEY_MODIFIERS {
        let mut flags = MOD_NOREPEAT.0; // Prevent key repeat events
        if self.ctrl {
            flags |= MOD_CONTROL.0;
        }
        if self.alt {
            flags |= MOD_ALT.0;
        }
        if self.shift {
            flags |= MOD_SHIFT.0;
        }
        if self.win {
            flags |= MOD_WIN.0;
        }
        HOT_KEY_MODIFIERS(flags)
    }
}

/// Logical hotkey actions (§8.4).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HotkeyAction {
    /// Toggle overlay visibility on/off.
    ToggleOverlay,
    /// Switch to next crosshair in folder/favorites.
    NextCrosshair,
    /// Switch to previous crosshair.
    PrevCrosshair,
    /// Jump to quick-access slot 1 to 20 (1-indexed).
    JumpSlot(u8),
    /// Cycle through configured resolution profiles.
    CycleResolutionProfile,
    /// Toggle calibration test pattern overlay.
    ToggleCalibrationPattern,
    /// Nudge crosshair live in game (dx, dy in steps).
    LiveNudge {
        /// Horizontal offset delta.
        dx: i8,
        /// Vertical offset delta.
        dy: i8,
    },
}

/// Descriptor of a user-configured hotkey binding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HotkeyBinding {
    /// Unique hotkey identifier (integer passed to RegisterHotKey).
    pub id: i32,
    /// Bound logical action.
    pub action: HotkeyAction,
    /// Modifier keys.
    pub modifiers: HotkeyModifiers,
    /// Virtual key code (e.g. VK_F1, VK_OEM_3, etc.).
    pub vk: u32,
}

/// Manages registration and dispatching of Windows global hotkeys.
#[derive(Debug, Default)]
pub struct HotkeyManager {
    bindings: HashMap<i32, HotkeyBinding>,
}

impl HotkeyManager {
    /// Creates a new hotkey manager.
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }

    /// Tests whether a key combination is already reserved by the OS or another app.
    pub fn test_conflict(modifiers: HotkeyModifiers, vk: u32) -> Result<bool, PlatformError> {
        unsafe {
            let test_id = 0x7FFF;
            let res = RegisterHotKey(HWND::default(), test_id, modifiers.to_win32_flags(), vk);

            if res.is_ok() {
                let _ = UnregisterHotKey(HWND::default(), test_id);
                Ok(false) // No conflict, successfully registered and unregistered
            } else {
                let err = GetLastError();
                // ERROR_HOTKEY_ALREADY_REGISTERED = 1409
                if err.0 == 1409 {
                    Ok(true) // Conflicted
                } else {
                    Err(PlatformError::HotkeyConflict {
                        id: test_id,
                        vk,
                        message: format!("Win32 error code {:#x}", err.0),
                    })
                }
            }
        }
    }

    /// Registers a hotkey binding to the specified window handle.
    pub fn register(&mut self, hwnd: HWND, binding: HotkeyBinding) -> Result<(), PlatformError> {
        unsafe {
            // Unregister prior registration if present
            let _ = UnregisterHotKey(hwnd, binding.id);

            let res = RegisterHotKey(
                hwnd,
                binding.id,
                binding.modifiers.to_win32_flags(),
                binding.vk,
            );

            if let Err(e) = res {
                return Err(PlatformError::HotkeyConflict {
                    id: binding.id,
                    vk: binding.vk,
                    message: format!("Hotkey ID {} could not be registered: {e}", binding.id),
                });
            }

            self.bindings.insert(binding.id, binding);
            Ok(())
        }
    }

    /// Unregisters a single hotkey by ID.
    pub fn unregister(&mut self, hwnd: HWND, id: i32) -> Result<(), PlatformError> {
        unsafe {
            let res = UnregisterHotKey(hwnd, id);
            self.bindings.remove(&id);
            if let Err(e) = res {
                return Err(PlatformError::HotkeyConflict {
                    id,
                    vk: 0,
                    message: format!("UnregisterHotKey failed: {e}"),
                });
            }
            Ok(())
        }
    }

    /// Unregisters all registered hotkeys for cleanup.
    pub fn unregister_all(&mut self, hwnd: HWND) {
        for &id in self.bindings.keys() {
            unsafe {
                let _ = UnregisterHotKey(hwnd, id);
            }
        }
        self.bindings.clear();
    }

    /// Finds the action associated with a received `WM_HOTKEY` identifier.
    pub fn get_action(&self, id: i32) -> Option<HotkeyAction> {
        self.bindings.get(&id).map(|b| b.action)
    }
}
