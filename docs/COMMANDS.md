# Command Registry & Mutation Catalog — XHAIR

This document catalogs every typed command mutation handled by `crates/xhair-commands` (§5, §15.2).

---

## 1. Command Bus Architecture

In accordance with Principle P3 (One Source of Truth), no document, configuration, or overlay state is ever mutated directly. Every change is packaged into a typed `Command` enum and dispatched through the `CommandBus`:

```
DispatchOptions {
    record_undo: true,
    coalesce_key: Option<String>,
    dry_run: false
}
```

- **Undo / Redo:** Any command dispatched with `record_undo: true` computes its exact mathematical inverse and appends it to the document's undo stack.
- **Coalescing:** Rapid slider adjustments (e.g. continuous slider dragging in the inspector) share a `coalesce_key`, collapsing multiple micro-mutations into a single undoable step.
- **Atomic Batches:** Multi-step operations (`Command::Batch`) execute as a single atomic transaction; if any sub-command fails validation, all previous sub-commands roll back instantly.

---

## 2. Command Catalog

### Document Operations
| Command Variant | Scope | Undoable | Description |
|---|---|---|---|
| `CrosshairCreate { name, template }` | Library | Yes | Creates a new crosshair from a named template (`classic`, `dot`, `blank`). |
| `CrosshairDelete { id }` | Library | Yes | Moves crosshair to 30-day trash retention. |
| `CrosshairRename { id, new_name }` | Document | Yes | Renames the crosshair document. |
| `CrosshairSetMetadata { id, tags, author, favorite, description }` | Document | Yes | Updates descriptive and organizational metadata. |
| `CrosshairSetGlobal { id, scale, opacity, rotation_deg, offset_x, offset_y }` | Document | Yes | Updates master coordinate transform and opacity. |
| `CrosshairSetSymmetry { id, symmetry }` | Document | Yes | Updates radial count (2, 3, 4, 6, 8) and axis mirroring. |
| `CrosshairSetEffects { id, effects }` | Document | Yes | Configures bloom, glow radius/color, and drop shadow. |

### Layer Operations (v2 updates)
| Command Variant | Scope | Undoable | Description |
|---|---|---|---|
| `LayerAdd { crosshair_id, layer }` | Document | Yes | Appends a new shape layer (`Line`, `Dot`, `Ring`, `Polygon`, `TShaped`, `Chevron`) to the layer stack. Default placed at (0, 0) center. |
| `LayerUpdate { crosshair_id, layer_id, payload }` | Document | Yes | Mutates shape-specific properties, stroke width, color, or transforms. |
| `LayerRename { crosshair_id, layer_id, new_name }` | Document | Yes | Renames a specific layer in the layer stack. |
| `LayerGroup { crosshair_id, ids, name }` | Document | Yes | Groups selected layers into a named expandable group row. |
| `LayerUngroup { crosshair_id, group_id }` | Document | Yes | Dissolves a layer group container, retaining child layers. |
| `LayerSetPosition { crosshair_id, ids, x, y, mode }` | Document | Yes | Sets selection position relative to center (`Relative` or `Absolute`). |
| `LayerRecenter { crosshair_id, ids }` | Document | Yes | Resets offset of selected layers back to (0, 0). |
| `CrosshairRecenter { id }` | Document | Yes | Resets master crosshair coordinate offset back to (0, 0). |
| `PaintSet { crosshair_id, ids, color }` | Document | Yes | Sets hex color (`#RGB`, `#RRGGBB`, `#RRGGBBAA`) for target layers. |
| `LayerRemove { crosshair_id, layer_id }` | Document | Yes | Deletes a layer from the stack. Inverse restores the exact layer. |
| `LayerDuplicate { crosshair_id, layer_id }` | Document | Yes | Clones a layer and inserts it above the original. |
| `LayerReorder { crosshair_id, layer_id, new_index }` | Document | Yes | Changes the z-order position of a layer in the render stack. |

### Precision Positioning & Nudge (§5)
| Command Variant | Scope | Undoable | Description |
|---|---|---|---|
| `Nudge { crosshair_id, target, dx, dy, frame }` | Document | Yes | Performs subpixel or game-pixel fine displacement. Target can be `WholeCrosshair`, a specific `Layer`, or an individual `Arm`. |

### Non-Destructive Selection Query & Control (§3)
| Command Variant | Scope | Undoable | Description |
|---|---|---|---|
| `SelectionSet { crosshair_id, ids }` | Editor | No | Sets active layer selection. |
| `SelectionAdd { crosshair_id, ids }` | Editor | No | Adds layers to active selection (Ctrl+Click / Shift range). |
| `SelectionRemove { crosshair_id, ids }` | Editor | No | Removes layers from active selection. |
| `SelectionToggle { crosshair_id, ids }` | Editor | No | Toggles selection state of specified layers. |
| `SelectionAll { crosshair_id }` | Editor | No | Selects all layers in the document (Ctrl+A). |
| `SelectionClear { crosshair_id }` | Editor | No | Clears layer selection (Esc). |
| `SelectionInvert { crosshair_id }` | Editor | No | Inverts selection across all document layers (Ctrl+Shift+I). |

### Crosshair Groups (Library Level) (§8)
| Command Variant | Scope | Undoable | Description |
|---|---|---|---|
| `GroupCreate { name, game_binding }` | Library | Yes | Creates a named collection of crosshairs with optional executable process binding. |
| `GroupRename { group_id, new_name }` | Library | Yes | Renames a crosshair group. |
| `GroupDelete { group_id, move_to_trash }` | Library | Yes | Deletes a crosshair group, moving children to Ungrouped or Trash. |
| `GroupReorder { group_ids }` | Library | Yes | Updates display sorting order of library groups. |
| `GroupSetIcon { group_id, icon, color }` | Library | Yes | Configures icon or color dot indicator. |
| `GroupAdd { group_id, crosshair_ids }` | Library | Yes | Assigns crosshairs to a group. |
| `GroupRemove { group_id, crosshair_ids }` | Library | Yes | Moves crosshairs back to Ungrouped. |
| `GroupSetGameBinding { group_id, process_name }` | Library | Yes | Binds group to focused game window executable process name. |
| `GroupSetResolutionProfile { group_id, profile_id }` | Library | Yes | Sets default resolution profile for the group. |
| `GroupSetActive { group_id, crosshair_id }` | Library | Yes | Remembers active crosshair slot within group. |
| `GroupNext` / `GroupPrevious` | Library | No | Cycles crosshairs within active group. |
| `GroupActivate { group_id }` | Library | No | Switches active group in library and overlay. |
| `ExportPack { group_id }` | Library | No | Exports entire group and metadata into a single `.xhairpack`. |
| `ImportPack { data_base64 }` | Library | Yes | Imports a `.xhairpack` bundle archive. |

### Overlay & Runtime Control
| Command Variant | Scope | Undoable | Description |
|---|---|---|---|
| `OverlaySetVisibility { visible }` | Overlay | No | Toggles transparent overlay visibility on the Desktop Window Manager (Ctrl+Shift+X). |
| `OverlaySetSlot { slot }` | Overlay | No | Selects active crosshair slot (1–9). |
| `OverlayNextSlot` / `OverlayPrevSlot` | Overlay | No | Cycles through active crosshair slots sequentially. |
| `CalibrationSetOffset { profile_id, dx, dy }` | Settings | Yes | Stores subpixel centering calibration offset for a screen profile. |
| `DynamicsSetEnabled { enabled, confirm_risk }` | Settings | No | Toggles dynamic reticle simulation with mandatory risk opt-in. |

### Batch Execution
| Command Variant | Scope | Undoable | Description |
|---|---|---|---|
| `Batch { commands, description }` | Composite | Yes | Atomically executes an array of commands as a single transaction with a single undo step. |

---

## 3. Patch Diff Generation

Whenever a command successfully executes, the `CommandBus` calculates a minimal `PatchDiff`:
```json
{
  "document_id": "0192e2b3-...",
  "sequence": 42,
  "patches": [
    { "op": "replace", "path": "/layers/0/kind/radius", "value": 2.5 }
  ]
}
```
This diff is emitted to the UI and WebSocket/stdio subscribers, allowing reactive components to re-render without reloading the entire document.
