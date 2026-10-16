# Model Context Protocol (MCP) Reference — XHAIR

The `xhair-mcp` crate implements the Model Context Protocol (JSON-RPC 2.0) interface for `xhair`, allowing autonomous AI agents to interact with every crosshair feature with complete human/agent parity (Principle P4).

## 1. Transports

### stdio Transport
Used in CLI and headless modes:
```bash
xhair-cli mcp
xhair.exe --headless --mcp-stdio
```
Reads newline-delimited JSON-RPC 2.0 messages from `stdin` and writes JSON-RPC 2.0 responses to `stdout`.

### Streamable HTTP Transport
Bound strictly to loopback (`127.0.0.1` only). Off by default; enabled in MCP settings.
- **Port:** Random available port (or configured port).
- **Authentication:** Bearer token (`Authorization: Bearer <token>`).
- **DNS-Rebinding Protection:** Strict `Host` (`127.0.0.1` / `localhost`) and `Origin` validation. Foreign headers receive `403 Forbidden`.

### Multi-Instance Proxy Mode
If an instance of `xhair` is already running with an active loopback endpoint, secondary stdio or CLI invocations automatically proxy their JSON-RPC calls over HTTP to the running instance via discovery of the endpoint descriptor (`xhair-endpoint.json`).

---

## 2. Tools Catalog

### Library Management
| Tool | Description | Key Arguments |
|---|---|---|
| `library.list` | List all crosshairs with metadata | `{}` |
| `library.search` | Search crosshairs by query or tag | `{"query": "...", "tag": "..."}` |
| `crosshair.get` | Retrieve full crosshair definition | `{"id": "..."}` |
| `crosshair.create` | Create a crosshair from template | `{"name": "...", "template": "classic"|"blank"}` |
| `crosshair.duplicate`| Clone a crosshair | `{"id": "..."}` |
| `crosshair.delete` | Move crosshair to 30-day trash | `{"id": "...", "dry_run": false}` |
| `crosshair.rename` | Rename crosshair | `{"id": "...", "new_name": "..."}` |
| `crosshair.set_metadata` | Update tags, author, favorite, description | `{"id": "...", "tags": [...], "favorite": true}` |

### Editing & Mutation (v2 updates)
| Tool | Description | Key Arguments |
|---|---|---|
| `edit.batch` | Atomic execution of multiple commands | `{"commands": [...]}` |
| `layer.add` | Add shape layer to crosshair (default `at: "center"`) | `{"crosshair_id": "...", "layer": {...}, "at": "center"}` |
| `layer.update` | Update properties of a layer | `{"crosshair_id": "...", "layer_id": "...", "payload": {...}}` |
| `layer.rename` | Rename a layer | `{"crosshair_id": "...", "layer_id": "...", "new_name": "..."}` |
| `layer.group` | Group selected layers into an expandable container | `{"crosshair_id": "...", "ids": [...], "name": "..."}` |
| `layer.ungroup` | Ungroup a layer container | `{"crosshair_id": "...", "group_id": "..."}` |
| `layer.set_position` | Set selection position relative to center | `{"crosshair_id": "...", "ids": [...], "x": 0.0, "y": 0.0, "mode": "relative"|"absolute"}` |
| `layer.recenter` | Reset selection offset to (0, 0) | `{"crosshair_id": "...", "ids": [...]}` |
| `crosshair.recenter` | Reset whole crosshair offset to (0, 0) | `{"crosshair_id": "..."}` |
| `paint.set` | Set hex color (`#RGB`, `#RRGGBB`, `#RRGGBBAA`) | `{"crosshair_id": "...", "ids": [...], "color": "#38BDF8FF"}` |
| `layer.remove` | Remove a layer | `{"crosshair_id": "...", "layer_id": "..."}` |
| `layer.duplicate` | Clone a layer in the stack | `{"crosshair_id": "...", "layer_id": "..."}` |
| `layer.reorder` | Change layer z-index stack position | `{"crosshair_id": "...", "layer_id": "...", "new_index": 0}` |
| `global.set` | Update master scale and opacity | `{"crosshair_id": "...", "scale": 1.0, "opacity": 1.0}` |
| `symmetry.set` | Configure crosshair symmetry rules | `{"crosshair_id": "...", "symmetry": {...}}` |
| `effects.set` | Configure glow, shadow, bloom, blur | `{"crosshair_id": "...", "effects": {...}}` |
| `nudge` | Fine subpixel/game-pixel nudge | `{"target": "crosshair"|"layer"|"arm", "dx": 1.0, "dy": 0.0, "step": 0.5, "frame": "screen"|"local"}` |
| `selection.set|add|remove|toggle|all|clear|invert` | Non-destructive selection query and mutation | `{"crosshair_id": "...", "ids": [...]}` |
| `undo` | Undo last mutation | `{"crosshair_id": "..."}` |
| `redo` | Redo undone mutation | `{"crosshair_id": "..."}` |

### Crosshair Groups (Library Level) (§8)
| Tool | Description | Key Arguments |
|---|---|---|
| `group.list` | List all library crosshair groups | `{}` |
| `group.create` | Create a crosshair group | `{"name": "Valorant", "game_binding": "VALORANT-Win64-Shipping.exe"}` |
| `group.rename` | Rename a crosshair group | `{"group_id": "...", "new_name": "..."}` |
| `group.delete` | Delete crosshair group | `{"group_id": "...", "move_to_trash": false}` |
| `group.reorder` | Reorder display priority of groups | `{"group_ids": [...]}` |
| `group.set_icon` | Set group color indicator or icon | `{"group_id": "...", "icon": "...", "color": "#10B981"}` |
| `group.add` | Add crosshairs to group | `{"group_id": "...", "crosshair_ids": [...]}` |
| `group.remove` | Remove crosshairs from group | `{"group_id": "...", "crosshair_ids": [...]}` |
| `group.set_game_binding` | Link group to executable process name | `{"group_id": "...", "process_name": "VALORANT-Win64-Shipping.exe"}` |
| `group.set_resolution_profile` | Assign default display resolution | `{"group_id": "...", "profile_id": "4-3-stretched"}` |
| `group.set_active` | Designate active crosshair in group | `{"group_id": "...", "crosshair_id": "..."}` |
| `group.next` / `previous` | Cycle crosshairs within active group | `{}` |
| `group.activate` | Make a group the active library group | `{"group_id": "..."}` |
| `export.pack` | Export entire group as single `.xhairpack` | `{"group_id": "..."}` |
| `import.pack` | Import `.xhairpack` archive bundle | `{"data_base64": "..."}` |

### Preview & Analysis
| Tool | Description | Key Output |
|---|---|---|
| `render.preview` | Render crosshair to PNG data URI | Returns `image/png` base64 data |
| `analyze.alignment` | Center parity and pixel boundary report | Diagnostic report (edge vs middle center) |
| `analyze.visibility` | WCAG contrast ratio on game backdrops | Contrast ratio (e.g., 7.5:1) |
| `validate` | Validate limits (complexity, assets) | Pass/fail status with details |

### Overlay Control
| Tool | Description | Key Arguments |
|---|---|---|
| `overlay.get_state` | Query overlay visibility and active slot | `{}` |
| `overlay.set_enabled` | Show or hide layered overlay window | `{"enabled": true}` |
| `overlay.activate` | Set active crosshair for overlay | `{"id": "..."}` or `{"slot": 1}` |
| `overlay.next` / `previous` | Cycle crosshair slots mid-game | `{}` |
| `display.list` | List displays, bounds, and DPI scales | `{}` |
| `calibration.show` / `hide` | Toggle calibration pattern overlay | `{}` |
| `calibration.set_offset` | Set subpixel calibration offset | `{"dx": 0.0, "dy": 0.0}` |

### Format & Sharing
| Tool | Description | Key Arguments |
|---|---|---|
| `export.code` | Export as compressed Brotli share code | `{"crosshair_id": "..."}` -> `XH1-...` |
| `import.code` | Import from compressed Brotli share code | `{"code": "XH1-..."}` |
| `import.valorant_code` | Import in-game Valorant crosshair string | `{"code": "0;P;...", "name": "..."}` |

### Metadata & Diagnostics
| Tool | Description |
|---|---|
| `meta.capabilities` | Server runtime mode, version, and active subsystem flags |
| `meta.schema` | Crosshair property descriptors and JSON Schema definitions |
| `meta.shortcuts` | Complete shortcut registry and command palette actions |
| `gun.list` | Gun profile templates for Dynamics simulation |
| `dynamics.set_enabled` | Opt-in dynamic reticle simulation (requires `confirm_risk: true`) |

---

## 3. Resources

- `xhair://library`: JSON summary of catalog crosshairs and slots.
- `xhair://crosshair/{id}`: Canonical JSON representation of a crosshair.
- `xhair://schema/crosshair`: JSON Schema for document validation.
- `xhair://settings`: Application settings snapshot.
- `xhair://resolution-profiles`: Supported display resolution and stretch profiles.

---

## 4. Prompt Templates

- `design_crosshair_from_description`: Guides the agent to convert human descriptions into concrete layer creation, preview rendering, and alignment verification steps.
- `tune_for_visibility`: Analyzes contrast ratio and recommends color, stroke, and glow adjustments.
- `convert_game_code`: Parses external strings into native layered models.
- `build_calibration_plan`: Generates a step-by-step calibration sequence for custom resolutions.
- `make_symmetric_variants`: Produces 2-way, 4-way, and radial rotational variants.

---

## 5. Parity Enforcement

Human/Agent parity is continuously enforced in CI by `crates/xhair-mcp/tests/parity.rs`:
1. Every UI action declared in `ui/src/lib/actions.ts` / `actions.json` must map to an executable command.
2. Every command in `xhair-commands` must map to an available MCP tool.
3. Every MCP tool must declare complete documentation, examples, and a valid JSON input schema.

---

## 6. Example Agent Session (§10 Walkthrough)

The following sequence illustrates an end-to-end autonomous agent workflow creating, transforming, grouping, and binding a crosshair using the v2 API:

```json
// 1. Create a clean blank crosshair starting with 0 layers at center
{"tool": "crosshair.create", "arguments": {"name": "Viper Tactical", "template": "blank"}}
// -> Returns {"id": "ch-viper-001", "name": "Viper Tactical", "layers": []}

// 2. Add Center Dot (lands at exact center 0, 0 as its own layer)
{"tool": "layer.add", "arguments": {"crosshair_id": "ch-viper-001", "layer": {"name": "Center Dot", "shape": {"kind": "dot", "diameter": 2.0}}, "at": "center"}}
// -> Returns {"layer_id": "layer-dot-1"}

// 3. Add 4 Arms (each lands at center 0, 0 as its own layer)
{"tool": "layer.add", "arguments": {"crosshair_id": "ch-viper-001", "layer": {"name": "Arm Top", "shape": {"kind": "line", "length": 6.0, "thickness": 2.0, "gap": 4.0}}, "at": "center"}}
{"tool": "layer.add", "arguments": {"crosshair_id": "ch-viper-001", "layer": {"name": "Arm Bottom", "shape": {"kind": "line", "length": 6.0, "thickness": 2.0, "gap": 4.0}}, "at": "center"}}
{"tool": "layer.add", "arguments": {"crosshair_id": "ch-viper-001", "layer": {"name": "Arm Left", "shape": {"kind": "line", "length": 6.0, "thickness": 2.0, "gap": 4.0}}, "at": "center"}}
{"tool": "layer.add", "arguments": {"crosshair_id": "ch-viper-001", "layer": {"name": "Arm Right", "shape": {"kind": "line", "length": 6.0, "thickness": 2.0, "gap": 4.0}}, "at": "center"}}

// 4. Select the top arm and nudge it by 0.5 px upward in screen frame
{"tool": "nudge", "arguments": {"target": "layer", "ids": ["layer-arm-top"], "dx": 0.0, "dy": -1.0, "step": 0.5, "frame": "screen"}}

// 5. Group two layers into an expandable container
{"tool": "layer.group", "arguments": {"crosshair_id": "ch-viper-001", "ids": ["layer-arm-top", "layer-arm-bottom"], "name": "Vertical Axis"}}

// 6. Recenter the grouped selection
{"tool": "layer.recenter", "arguments": {"crosshair_id": "ch-viper-001", "ids": ["grp-vert-axis"]}}

// 7. Paint the reticle with a precise hex color
{"tool": "paint.set", "arguments": {"crosshair_id": "ch-viper-001", "ids": ["grp-vert-axis", "layer-dot-1"], "color": "#38BDF8FF"}}

// 8. Add crosshair to the "Valorant" crosshair group
{"tool": "group.add", "arguments": {"group_id": "grp-val", "crosshair_ids": ["ch-viper-001"]}}

// 9. Activate the group
{"tool": "group.activate", "arguments": {"group_id": "grp-val"}}
```
