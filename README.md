# XHAIR

High-precision Windows crosshair overlay studio with hardware-accelerated rendering, subpixel centering, and full Model Context Protocol (MCP) server automation.

---

## How to Build

### Prerequisites
- **Rust:** Stable toolchain (`rustup default stable`)
- **Node.js & pnpm:** Node 18+ and `pnpm` (`npm install -g pnpm`)

### 1. Build Frontend UI
```powershell
pnpm -C ui install
pnpm -C ui build
```

### 2. Build Release Binaries
```powershell
cargo build --release --workspace
```

### 3. (Optional) Build Production Installer & Distribution Packages
```powershell
cargo run -p xtask -- package --version 1.0.0
```

---

## Where to Find

After building, all compiled binaries and packages are located in:

| Artifact | Location | Description |
|---|---|---|
| **Studio App** | `target\release\xhair.exe` | Main desktop application (Win32 overlay + Svelte 5 editor + embedded loopback MCP). |
| **CLI & MCP Tool** | `target\release\xhair-cli.exe` | Standalone CLI for offline rendering, format conversion, and MCP stdio proxying. |
| **Portable Zip** | `dist\xhair-1.0.0-x64-portable.zip` | Zero-install standalone archive with `xhair.exe`. |
| **CLI Zip** | `dist\xhair-cli-1.0.0-x64.zip` | Standalone archive with `xhair-cli.exe`. |
| **Installer** | `dist\installer.nsi` | NSIS installer script for creating Windows setup exe. |
| **Integrity Checksums** | `dist\SHA256SUMS.txt` | Cryptographic SHA-256 hashes of all release packages. |

---

## How to Connect MCP

XHAIR provides two MCP server modes allowing AI assistants (Claude, Cursor, Antigravity) to design, tune, calibrate, and manage crosshairs with 100% feature parity:

1. **`xhair` (Attached Mode):** Connects to the running desktop app (`xhair.exe`) over loopback. Tool calls update your active screen overlay reticle live in real time.
2. **`xhair-headless` (Headless Mode):** Operates offline without the GUI or overlay window, reading and writing crosshair documents directly to disk (`%APPDATA%\xhair\`).

### Configuration (Claude Desktop / Cursor / Antigravity)

Add this configuration block to your MCP settings file:
- **Claude Desktop:** `%APPDATA%\Claude\claude_desktop_config.json`
- **Cursor:** `.cursor\mcp.json`
- **Antigravity IDE:** `%APPDATA%\.gemini\antigravity-ide\mcp\mcp_config.json` (or `.agents\mcp_config.json`)

```json
{
  "mcpServers": {
    "xhair": {
      "command": "D:\\cross-aim\\target\\release\\xhair-cli.exe",
      "args": ["mcp", "--attached"]
    },
    "xhair-headless": {
      "command": "D:\\cross-aim\\target\\release\\xhair-cli.exe",
      "args": ["mcp", "--headless"]
    }
  }
}
```
*(Replace `D:\\cross-aim` with your actual repository or installation path.)*

### CLI Commands
```powershell
# Run in attached mode (controls active overlay window; requires xhair.exe running)
.\target\release\xhair-cli.exe mcp --attached

# Run in headless mode (standalone offline engine; no window required)
.\target\release\xhair-cli.exe mcp --headless

# Auto mode (connects to running xhair.exe if active; otherwise falls back to headless)
.\target\release\xhair-cli.exe mcp
```
