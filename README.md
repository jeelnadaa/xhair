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

XHAIR includes a full Model Context Protocol (MCP) server (35 tools, 25 commands) that lets AI agents (Claude, Cursor, Antigravity) design, calibrate, and live-update crosshairs in real time.

### Connect via AI Assistants (Claude Desktop / Cursor / Antigravity)

Add this entry to your MCP configuration file:
- **Claude Desktop:** `%APPDATA%\Claude\claude_desktop_config.json`
- **Cursor:** `.cursor\mcp.json`
- **Antigravity IDE:** `%APPDATA%\.gemini\antigravity-ide\mcp\xhair\mcp_config.json`

```json
{
  "mcpServers": {
    "xhair": {
      "command": "D:\\cross-aim\\target\\release\\xhair-cli.exe",
      "args": ["mcp"]
    }
  }
}
```
*(Replace `D:\\cross-aim` with your actual repository or installation path.)*

### How It Works
- **When `xhair.exe` is running:** `xhair-cli mcp` automatically connects to the live app via loopback HTTP (`127.0.0.1`) using token discovery in `%TEMP%\xhair-endpoint.json`. AI mutations update your on-screen crosshair in real time!
- **When `xhair.exe` is closed:** `xhair-cli mcp` seamlessly runs in headless mode, managing crosshairs directly from disk storage (`%APPDATA%\xhair\`).
