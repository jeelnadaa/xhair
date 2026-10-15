//! Local crash reporting and panic handler (§16, M10).
//!
//! Important compliance invariant (P1, §2, §16):
//! Local reports only. Strictly zero network calls, zero analytics, zero external telemetry.

#![allow(deprecated)]

use crate::error::PlatformError;
use std::backtrace::Backtrace;
use std::fs;
use std::panic::{self, PanicInfo};
use std::path::PathBuf;

/// Locates or creates the local crashes directory in `%APPDATA%/xhair/crashes`.
pub fn get_crash_reports_dir() -> PathBuf {
    let base = if let Ok(appdata) = std::env::var("APPDATA") {
        PathBuf::from(appdata).join("xhair")
    } else {
        std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join(".xhair")
    };
    base.join("crashes")
}

/// Formats and writes a panic report to a local text file.
///
/// Returns the path to the created crash file, or None if write failed.
pub fn write_crash_report(info: &PanicInfo<'_>) -> Option<PathBuf> {
    let crashes_dir = get_crash_reports_dir();
    let _ = fs::create_dir_all(&crashes_dir);

    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S").to_string();
    let report_filename = format!("crash_{}.log", timestamp);
    let report_path = crashes_dir.join(report_filename);

    let payload = if let Some(s) = info.payload().downcast_ref::<&str>() {
        s.to_string()
    } else if let Some(s) = info.payload().downcast_ref::<String>() {
        s.clone()
    } else {
        "Unknown panic payload".to_string()
    };

    let location = if let Some(loc) = info.location() {
        format!("{}:{}:{}", loc.file(), loc.line(), loc.column())
    } else {
        "Unknown location".to_string()
    };

    let thread_name = std::thread::current()
        .name()
        .unwrap_or("<unnamed>")
        .to_string();

    let backtrace = Backtrace::capture();

    let content = format!(
        "================================================================================\n\
         XHAIR LOCAL CRASH REPORT (§16)\n\
         ================================================================================\n\
         Timestamp (UTC): {}\n\
         Version        : {}\n\
         Platform       : Windows x86_64\n\
         Thread         : {}\n\
         Location       : {}\n\
         Message        : {}\n\
         --------------------------------------------------------------------------------\n\
         BACKTRACE:\n\
         {}\n\
         ================================================================================\n\
         PRIVACY NOTICE (§2 Invariant P1):\n\
         This crash dump was generated purely on local storage.\n\
         Zero telemetry or diagnostic data was transmitted over the network.\n\
         You may inspect or share this report at https://github.com/jeelnadaa/xhair/issues\n\
         ================================================================================\n",
        chrono::Utc::now().to_rfc3339(),
        env!("CARGO_PKG_VERSION"),
        thread_name,
        location,
        payload,
        backtrace
    );

    if fs::write(&report_path, content).is_ok() {
        Some(report_path)
    } else {
        None
    }
}

/// Installs the global panic hook to output user-facing crash reports locally.
pub fn install_crash_handler() -> Result<PathBuf, PlatformError> {
    let dir = get_crash_reports_dir();
    fs::create_dir_all(&dir)
        .map_err(|e| PlatformError::Windows(format!("Failed to create crash directory: {}", e)))?;

    panic::set_hook(Box::new(|info| {
        if let Some(path) = write_crash_report(info) {
            eprintln!("\n[XHAIR] Unexpected fatal error occurred.");
            eprintln!("[XHAIR] A diagnostic report was written locally to:");
            eprintln!("        {}", path.display());
            eprintln!("[XHAIR] Zero telemetry was sent. To report this issue, visit:");
            eprintln!("        https://github.com/jeelnadaa/xhair/issues\n");
        }
    }));

    Ok(dir)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crash_reports_directory() {
        let dir = get_crash_reports_dir();
        assert!(dir.ends_with("crashes"));
    }
}
