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

### Layer Operations
| Command Variant | Scope | Undoable | Description |
|---|---|---|---|
| `LayerAdd { crosshair_id, layer }` | Document | Yes | Appends a new shape layer (`Line`, `Dot`, `Ring`, `Polygon`, `TShaped`, `Chevron`) to the layer stack. |
| `LayerUpdate { crosshair_id, layer_id, payload }` | Document | Yes | Mutates shape-specific properties, stroke width, color, or transforms. |
| `LayerRemove { crosshair_id, layer_id }` | Document | Yes | Deletes a layer from the stack. Inverse restores the exact layer. |
| `LayerDuplicate { crosshair_id, layer_id }` | Document | Yes | Clones a layer and inserts it above the original. |
| `LayerReorder { crosshair_id, layer_id, new_index }` | Document | Yes | Changes the z-order position of a layer in the render stack. |

### Precision Positioning
| Command Variant | Scope | Undoable | Description |
|---|---|---|---|
| `Nudge { crosshair_id, target, dx, dy, frame }` | Document | Yes | Performs subpixel or game-pixel fine displacement. Target can be `WholeCrosshair`, a specific `Layer`, or an individual `Arm`. |

### Overlay & Runtime Control
| Command Variant | Scope | Undoable | Description |
|---|---|---|---|
| `OverlaySetVisibility { visible }` | Overlay | No | Toggles transparent overlay visibility on the Desktop Window Manager. |
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
