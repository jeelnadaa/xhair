# File Formats & Share Code Specification — XHAIR

This document defines the storage and interchange specifications implemented in `crates/xhair-format` (§7).

---

## 1. Single Crosshair Document (`.xhair`)

A `.xhair` file is a UTF-8 JSON document adhering to canonical JSON Schema (`xhair://schema/crosshair`).

### 1.1 Schema Example
```json
{
  "schema_version": 1,
  "id": "0192e2b3-7a91-7212-9844-0b8d5a1b3c99",
  "name": "Cyan Dot Crisp",
  "description": "Ultra-sharp 1px cyan dot with 1px black outline",
  "tags": ["dot", "valorant", "1080p"],
  "author": "jeelnadaa",
  "created_at": "2026-10-15T12:00:00Z",
  "modified_at": "2026-10-15T12:30:00Z",
  "global": {
    "scale": 1.0,
    "opacity": 1.0,
    "rotation_deg": 0.0,
    "offset_x": 0.0,
    "offset_y": 0.0
  },
  "symmetry": {
    "radial_count": 4,
    "mirror_x": true,
    "mirror_y": true
  },
  "effects": {
    "bloom": false,
    "glow_radius": 0.0,
    "glow_color": "#00000000"
  },
  "layers": [
    {
      "id": "0192e2b3-7a91-7212-9844-0b8d5a1b3c9a",
      "name": "Center Dot",
      "visible": true,
      "locked": false,
      "kind": {
        "type": "dot",
        "radius": 1.0,
        "fill": { "color": "#00ffffff" },
        "stroke": { "color": "#000000ff", "width": 1.0 }
      }
    }
  ]
}
```

---

## 2. Multi-Crosshair Pack (`.xhairpack`)

A `.xhairpack` is a JSON archive containing a collection of crosshairs, category folders, and pack metadata:
```json
{
  "schema_version": 1,
  "pack_name": "Pro Esports Collection 2026",
  "author": "jeelnadaa",
  "crosshairs": [
    { /* Full .xhair object */ },
    { /* Full .xhair object */ }
  ]
}
```

---

## 3. Compact Share Code (`XH1-...`)

To facilitate sharing over Discord, Twitch chats, or social media, XHAIR defines a compact share code format:

### 3.1 Binary Structure
```
┌──────────┬─────────────────────────────┬──────────────────┐
│ Magic/Ver│ Compressed Payload (Brotli) │ CRC32 Checksum   │
│ (1 byte) │ (Variable length)           │ (4 bytes big-end)│
└──────────┴─────────────────────────────┴──────────────────┘
```
1. **Magic Header:** `0x01` (`XH1`).
2. **Compression:** Canonical JSON representation compressed using **Brotli** at compression level 9.
3. **Integrity:** 32-bit CRC32 checksum computed across the uncompressed payload.
4. **Encoding:** Standard URL-safe Base64 (`RFC 4648`), prefixed with `XH1-`.

### 3.2 Verification & Unpacking
When importing an `XH1-` code:
1. Validate prefix `XH1-`.
2. Decode URL-safe Base64 payload.
3. Verify minimum length (at least 5 bytes: 1 byte header + 4 byte CRC).
4. Decompress Brotli stream.
5. Verify CRC32 matches decompressed stream.
6. Validate JSON against `schema_version`.

---

## 4. In-Game Format Converters

### 4.1 Valorant Crosshair Import
Valorant crosshairs are encoded as semicolon-delimited key-value strings (e.g. `0;P;c;5;o;1;d;1;z;3;f;0;0t;1;0l;4;0o;2;0a;1;0f;0;1b;0`).

| Valorant Key | Meaning | Mapped Layer / Property |
|---|---|---|
| `c` | Primary Color (0=White, 1=Green, 2=YellowGreen, 3=GreenYellow, 4=Yellow, 5=Cyan, 6=Pink, 7=Red) | Layer Stroke/Fill color |
| `o` | Outline Enabled (0/1) | Stroke width & opacity |
| `d` | Center Dot Enabled (0/1) | Dot layer visibility |
| `z` | Center Dot Size (px) | Dot layer radius |
| `0b` / `0t` | Inner Lines Enabled / Thickness | Line layer thickness |
| `0l` | Inner Lines Length | Line layer length |
| `0o` | Inner Lines Offset (Gap) | Line layer center gap |
| `0a` | Inner Lines Opacity | Line layer alpha |
| `1b` / `1t` / `1l` / `1o` | Outer Lines | Second Line layer stack |

---

## 5. Schema Migration & Forward Compatibility

1. **Strict Unknown Field Preservation:** Deserializers preserve unknown fields where possible.
2. **Deterministic Version Upgrades:** Version migrations (`V1 -> V2`) are handled in `xhair_format::migration` via discrete transformation pipelines with automated unit test assertions.
3. **Atomic File Writes:** All disk saves write to a temporary file (`.xhair.tmp`) in `%APPDATA%/xhair/` and atomic-rename to target to prevent corruption on crash or system power loss.
