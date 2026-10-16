# Core Desktop Application & Storage Engine Architecture

**Project:** XHAIR — Core Application, Win32 Overlay & Storage Wiring  
**Target:** Robust, crash-safe, multi-monitor Windows application

---

## 1. Storage Architecture: Unifying `xhair-library` with the Application

### 1.1 The Current Flaw
Currently, `xhair-library` contains atomic file writes, checksum validation, and folder/slot indexing, but sits completely idle. The Svelte 5 frontend writes JSON dumps to `localStorage` in the WebView2 user data folder.

### 1.2 Target Disk-Backed Pipeline
`xhair-app` must initialize `LibraryEngine` upon application startup and provide direct local API or Wry IPC endpoints to the UI:

```
┌────────────────────────────────────────────────────────┐
│                   Svelte 5 Editor UI                   │
│   - Fetches library on mount via GET /api/library      │
│   - Auto-saves profile on change via POST /api/save    │
│   - Zero reliance on browser localStorage              │
└───────────────────────────┬────────────────────────────┘
                            │
               Wry IPC or Local HTTP Bridge
                            │
┌───────────────────────────▼────────────────────────────┐
│              xhair-app (Desktop Daemon)                │
│                                                        │
│  ┌──────────────────────────────────────────────────┐  │
│  │     LibraryEngine::new(resolve_data_dir())       │  │
│  │                                                  │  │
│  │  - Atomic file write with fsync                  │  │
│  │  - .bak backup preservation                      │  │
│  │  - Folder hierarchy & quick-select slots 1..=20  │  │
│  │  - 30-day trash lifecycle                        │  │
│  └────────────────────────┬─────────────────────────┘  │
└───────────────────────────┼────────────────────────────┘
                            │
                 Local File System Write
                            │
┌───────────────────────────▼────────────────────────────┐
│      %APPDATA%/xhair/  or  ./data/ (Portable Mode)     │
│  ├── crosshairs/        (Atomic .xhair JSON files)     │
│  ├── library_index.json (Catalog index & folders)      │
│  ├── profiles/          (Resolution presets)           │
│  └── trash/             (Safely deleted items)         │
└────────────────────────────────────────────────────────┘
```

### 1.3 Data Directory Resolution
- **Portable Mode:** If a file named `xhair.portable` exists beside `xhair.exe`, all data is stored locally in `./data/` beside the executable.
- **Installed Mode:** Default storage is in Windows standard `%APPDATA%/xhair/`.

---

## 2. Win32 Overlay Engine Precision Fixes

### 2.1 Applying Calibration Offsets
In `crates/xhair-platform-win/src/overlay.rs`:
The `render_and_update()` function must incorporate `config.calibration_offset` into the screen center calculations:
```rust
// Correct calculation incorporating user calibration offset:
let screen_center_x = mapping.out_center_x as f32 + config.calibration_offset.x;
let screen_center_y = mapping.out_center_y as f32 + config.calibration_offset.y;
```

### 2.2 Fixing Bounding Box Centering & Offset Clipping
To guarantee that offset crosshairs or asymmetric shapes never clip:
1. Symmetrize the bounding box around the reticle center:
   ```rust
   let bbox = calculate_crosshair_bounds(&active_xhair);
   let max_extent_x = bbox.min_x.abs().max(bbox.max_x.abs());
   let max_extent_y = bbox.min_y.abs().max(bbox.max_y.abs());
   let win_w = ((max_extent_x * 2.0).ceil() as u32).clamp(32, 2048);
   let win_h = ((max_extent_y * 2.0).ceil() as u32).clamp(32, 2048);
   ```
2. By sizing `win_w` and `win_h` symmetrically around `(0, 0)`, the center of the pixmap `(win_w / 2, win_h / 2)` coincides exactly with the reticle coordinate center.
3. The layered window is placed on screen at:
   ```rust
   let win_x = (screen_center_x - (win_w as f32 / 2.0)).round() as i32;
   let win_y = (screen_center_y - (win_h as f32 / 2.0)).round() as i32;
   ```
This guarantees that **any translation, arm offset, or asymmetry is fully contained within the buffer** without clipping.

### 2.3 Multi-Monitor Selection & DPI Handling
- **Monitor Enumeration:** Use `EnumDisplayMonitors` to capture device names, physical pixel rectangles, and Per-Monitor-V2 DPI scaling factors.
- **Configurable Target Monitor:** In the application settings, allow the user to select which display should host the overlay (e.g. `Primary Display`, `Display 2 (2560×1440 @ 144Hz)`).
- **Monitor DPI Adaptation:** When the overlay is placed on a secondary monitor with a different scaling factor (e.g. 150% DPI), the overlay recalculates its `GeometryMapping` using that monitor's exact DPI.

---

## 3. High-Performance Foreground Game Following

- **Fast WinEventHook:** Replace simple 200ms timer polling with an out-of-context Win32 event hook:
  ```rust
  SetWinEventHook(
      EVENT_SYSTEM_FOREGROUND,
      EVENT_SYSTEM_FOREGROUND,
      HINSTANCE::default(),
      Some(foreground_event_proc),
      0,
      0,
      WINEVENT_OUTOFCONTEXT | WINEVENT_SKIPOWNPROCESS,
  );
  ```
  This guarantees instant, zero-latency hiding/showing when the user alt-tabs into or out of a game.
- **Safe Limited-Query Process Checking:** Continues using `OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION)` to read the image filename without opening or reading game memory.
