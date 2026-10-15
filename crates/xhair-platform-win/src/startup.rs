//! Windows startup registry configuration (§14, M10).
//!
//! Manages `HKCU\Software\Microsoft\Windows\CurrentVersion\Run` key to enable or disable
//! starting xhair on user login without elevated administrator privileges.

use crate::error::PlatformError;
use std::env;
use std::path::PathBuf;
use windows::core::{HSTRING, PCWSTR};
use windows::Win32::Foundation::{ERROR_FILE_NOT_FOUND, ERROR_SUCCESS};
use windows::Win32::System::Registry::{
    RegCloseKey, RegDeleteValueW, RegOpenKeyExW, RegQueryValueExW, RegSetValueExW, HKEY,
    HKEY_CURRENT_USER, KEY_QUERY_VALUE, KEY_SET_VALUE, REG_SZ,
};

const RUN_KEY_PATH: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Run";
const RUN_VALUE_NAME: &str = "xhair";

/// Retrieves the path to the current executable.
fn get_current_exe_path() -> Result<PathBuf, PlatformError> {
    env::current_exe()
        .map_err(|e| PlatformError::Windows(format!("Failed to get current_exe: {}", e)))
}

/// Checks whether xhair is currently configured to run at Windows login.
pub fn is_run_at_startup_enabled() -> Result<bool, PlatformError> {
    let subkey = HSTRING::from(RUN_KEY_PATH);
    let mut hkey = HKEY::default();

    // SAFETY: Opening HKCU Run key with read-only query permissions.
    let status = unsafe {
        RegOpenKeyExW(
            HKEY_CURRENT_USER,
            PCWSTR(subkey.as_ptr()),
            0,
            KEY_QUERY_VALUE,
            &mut hkey,
        )
    };

    if status != ERROR_SUCCESS {
        return Ok(false);
    }

    let val_name = HSTRING::from(RUN_VALUE_NAME);
    let mut data_len = 0u32;

    // SAFETY: Querying size of registry value for "xhair".
    let query_status = unsafe {
        RegQueryValueExW(
            hkey,
            PCWSTR(val_name.as_ptr()),
            None,
            None,
            None,
            Some(&mut data_len),
        )
    };

    // SAFETY: Closing open registry key handle.
    unsafe {
        let _ = RegCloseKey(hkey);
    }

    if query_status == ERROR_SUCCESS {
        Ok(true)
    } else if query_status == ERROR_FILE_NOT_FOUND {
        Ok(false)
    } else {
        Err(PlatformError::Windows(format!(
            "Failed to query Run registry key: error code {}",
            query_status.0
        )))
    }
}

/// Configures xhair to start at Windows user login.
///
/// If `enabled` is true, registers the current executable path in `HKCU\...\Run`.
/// If `enabled` is false, removes the registry entry if it exists.
pub fn set_run_at_startup(enabled: bool, extra_args: Option<&str>) -> Result<(), PlatformError> {
    let subkey = HSTRING::from(RUN_KEY_PATH);
    let mut hkey = HKEY::default();

    // SAFETY: Opening HKCU Run key with write permissions.
    let status = unsafe {
        RegOpenKeyExW(
            HKEY_CURRENT_USER,
            PCWSTR(subkey.as_ptr()),
            0,
            KEY_SET_VALUE,
            &mut hkey,
        )
    };

    if status != ERROR_SUCCESS {
        return Err(PlatformError::Windows(format!(
            "Failed to open Run registry key: error code {}",
            status.0
        )));
    }

    let val_name = HSTRING::from(RUN_VALUE_NAME);

    let result = if enabled {
        let exe_path = get_current_exe_path()?;
        let mut cmd = format!("\"{}\"", exe_path.to_string_lossy());
        if let Some(args) = extra_args {
            cmd.push(' ');
            cmd.push_str(args);
        }

        let wide_cmd: Vec<u16> = cmd.encode_utf16().chain(std::iter::once(0)).collect();

        // SAFETY: Writing REG_SZ command string to HKCU Run key.
        let set_status = unsafe {
            RegSetValueExW(
                hkey,
                PCWSTR(val_name.as_ptr()),
                0,
                REG_SZ,
                Some(std::slice::from_raw_parts(
                    wide_cmd.as_ptr() as *const u8,
                    wide_cmd.len() * 2,
                )),
            )
        };

        if set_status == ERROR_SUCCESS {
            Ok(())
        } else {
            Err(PlatformError::Windows(format!(
                "Failed to set Run registry value: error code {}",
                set_status.0
            )))
        }
    } else {
        // SAFETY: Deleting value if disabling startup.
        let del_status = unsafe { RegDeleteValueW(hkey, PCWSTR(val_name.as_ptr())) };
        if del_status == ERROR_SUCCESS || del_status == ERROR_FILE_NOT_FOUND {
            Ok(())
        } else {
            Err(PlatformError::Windows(format!(
                "Failed to delete Run registry value: error code {}",
                del_status.0
            )))
        }
    };

    // SAFETY: Closing registry key handle.
    unsafe {
        let _ = RegCloseKey(hkey);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current_exe_path() {
        let path = get_current_exe_path().unwrap();
        assert!(path.exists() || path.extension().map_or(false, |e| e == "exe"));
    }

    #[test]
    fn test_startup_status_query() {
        // Querying current status should succeed without panicking
        let status = is_run_at_startup_enabled();
        assert!(status.is_ok());
    }
}
