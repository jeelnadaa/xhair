# Error Code Catalog — XHAIR

This catalog defines the standardized error codes (`XH-1xxx` to `XH-9xxx`) used across all XHAIR crates, user-facing error dialogs, logs, and MCP responses (§15.1, §15.2).

---

## Error Code Structure

Every error code carries:
1. **Code:** Unique identifier (`XH-xxxx`).
2. **Category:** Subsystem classification.
3. **User Hint:** Clear, non-technical instructions for remediation.

---

## 1. Geometry & Display (`XH-1xxx`)

| Code | Identifier | Description | User Remediation Hint |
|---|---|---|---|
| `XH-1001` | `MonitorNotFound` | Target display monitor handle is invalid or disconnected. | Re-plug display cable or select a primary monitor in Settings. |
| `XH-1002` | `InvalidDpiScale` | DPI scale factor reported by OS is outside supported range (0.5–5.0). | Verify Windows display scaling settings. |
| `XH-1003` | `CalibrationOutOfBounds` | Subpixel offset $(\Delta x, \Delta y)$ exceeds maximum $\pm 100\text{ px}$. | Re-capture screenshot closer to true screen center. |
| `XH-1004` | `ScreenshotAlignFailed` | Could not detect crosshair centroid in provided screenshot. | Ensure the in-game crosshair is clearly visible against a contrasting background. |

---

## 2. Model & Validation (`XH-2xxx`)

| Code | Identifier | Description | User Remediation Hint |
|---|---|---|---|
| `XH-2001` | `SchemaVersionMismatch` | Document schema version is newer than supported by this release. | Update XHAIR to the latest version to open this file. |
| `XH-2002` | `LayerLimitExceeded` | Crosshair exceeds maximum complexity limit (32 layers). | Remove unnecessary layers to maintain zero-CPU rendering performance. |
| `XH-2003` | `InvalidColorFormat` | Color string is not a valid 8-digit hex code (`#RRGGBBAA`). | Pick a valid color from the color picker palette. |
| `XH-2004` | `LayerNotFound` | Specified layer ID does not exist in the document. | Refresh document or undo the deletion. |

---

## 3. Format & Share Codes (`XH-3xxx`)

| Code | Identifier | Description | User Remediation Hint |
|---|---|---|---|
| `XH-3001` | `InvalidShareCode` | String does not begin with `XH1-` or contains illegal characters. | Verify that the complete share code was copied without trailing spaces. |
| `XH-3002` | `ChecksumMismatch` | 32-bit CRC32 checksum verification failed (corrupt code). | Request a fresh share code from the author; payload was truncated. |
| `XH-3003` | `DecompressionFailed` | Brotli stream decompressor failed to decode payload. | The share code payload is corrupted or malformed. |
| `XH-3004` | `ValorantParseError` | In-game crosshair code contains unrecognized tokens. | Export a fresh profile directly from the in-game settings menu. |

---

## 4. Command Bus & Transactions (`XH-4xxx`)

| Code | Identifier | Description | User Remediation Hint |
|---|---|---|---|
| `XH-4001` | `UndoStackEmpty` | No previous mutation exists in the undo history. | Mutation history is empty for this document. |
| `XH-4002` | `RedoStackEmpty` | No forward mutation exists in the redo history. | No mutations available to redo. |
| `XH-4003` | `BatchExecutionFailed` | Atomic transaction failed validation during multi-command run. | Entire batch was safely rolled back; check tool arguments. |

---

## 5. Rendering Engine (`XH-5xxx`)

| Code | Identifier | Description | User Remediation Hint |
|---|---|---|---|
| `XH-5001` | `PixmapAllocFailed` | Failed to allocate memory for software rasterization buffer. | System memory low; close background applications. |
| `XH-5002` | `WasmExportError` | WebAssembly canvas serialization failed. | Ensure WebGL2 or HTML5 Canvas2D is enabled in the browser. |

---

## 6. Windows Platform & Windowing (`XH-6xxx`)

| Code | Identifier | Description | User Remediation Hint |
|---|---|---|---|
| `XH-6001` | `WindowCreationFailed` | Win32 `CreateWindowExW` failed to create transparent overlay. | Verify Desktop Window Manager (`dwm.exe`) is running. |
| `XH-6002` | `HotkeyConflict` | Global shortcut key is already registered by another application. | Choose an alternative shortcut key combination in Settings. |
| `XH-6003` | `LayeredUpdateFailed` | Win32 `UpdateLayeredWindow` call returned OS error. | Restart XHAIR; graphics driver may have reset. |
| `XH-6004` | `StartupRegistryError` | Failed to modify `HKCU\...\Run` registry key. | Check Windows permissions or anti-virus restrictions on user registry. |

---

## 7. Reticle Dynamics & Simulation (`XH-7xxx`)

| Code | Identifier | Description | User Remediation Hint |
|---|---|---|---|
| `XH-7001` | `DynamicsOptInRequired` | Dynamic reticle features accessed without active user opt-in. | Accept the Dynamic Simulation safety advisory modal in Settings. |
| `XH-7002` | `GunProfileNotFound` | Named gun profile does not exist in the simulation database. | Select a valid template: `rifle_standard`, `smg_rapid`, `pistol_precision`. |

---

## 8. Model Context Protocol Server (`XH-8xxx`)

| Code | Identifier | Description | User Remediation Hint |
|---|---|---|---|
| `XH-8001` | `McpAuthFailed` | HTTP request missing or invalid Bearer token. | Pass `Authorization: Bearer <token>` matching `xhair-endpoint.json`. |
| `XH-8002` | `DnsRebindingBlocked` | Request contained non-loopback `Host` or suspicious `Origin`. | Requests must originate strictly from `localhost` or `127.0.0.1`. |
| `XH-8003` | `ToolNotFound` | Requested JSON-RPC tool name does not exist. | Query `tools/list` to inspect available MCP tools. |
| `XH-8004` | `ToolArgumentsInvalid` | Parameters failed JSON Schema validation for tool. | Inspect schema in `docs/MCP.md` or call `meta.schema`. |

---

## 9. Storage & System (`XH-9xxx`)

| Code | Identifier | Description | User Remediation Hint |
|---|---|---|---|
| `XH-9001` | `AtomicWriteFailed` | Failed to atomically commit `.xhair` file to disk. | Ensure write permissions in `%APPDATA%/xhair/`. |
| `XH-9002` | `LibraryCorrupt` | Library index JSON failed parsing. | Automatic backup restored; corrupted file moved to `.bak`. |
