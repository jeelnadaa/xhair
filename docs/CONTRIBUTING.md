# Contributing to XHAIR

Thank you for contributing to XHAIR! This document outlines coding standards, commit rules, and architectural patterns (§15.2).

---

## 1. Development Prerequisites

- **OS:** Windows 10 / 11 (x64)
- **Rust:** Stable toolchain $\ge 1.80$ (`rustup default stable`)
- **Node.js & pnpm:** Node $\ge 20$, `pnpm` $\ge 9$ (`npm install -g pnpm`)
- **Editor:** Antigravity IDE, VSCode, or Cursor with Rust Analyzer and Svelte extensions.

---

## 2. Commit Message Conventions

XHAIR enforces [Conventional Commits](https://www.conventionalcommits.org/):
```
<type>(<scope>): <short description in present tense>

[optional body explaining rationale and invariants]

[optional footer(s)]
```

### Allowed Types:
- `feat`: New feature or user-facing capability.
- `fix`: Bug fix.
- `perf`: Performance improvement or memory reduction.
- `refactor`: Code change that neither fixes a bug nor adds a feature.
- `docs`: Documentation updates.
- `test`: Adding or updating tests.
- `build`: Build system or dependency updates.
- `ci`: CI workflows and xtask automation.
- `chore`: Maintenance tasks.

*Note:* Releases are managed automatically via `release-please`. Never manually bump versions in `Cargo.toml`.

---

## 3. Worked Example: "Add a New Shape in 5 Steps"

To prove that XHAIR's architecture remains lightweight, modular, and maintainable, adding a completely new geometric shape (e.g., `Diamond`) takes exactly 5 focused steps:

### Step 1: Model (`crates/xhair-model/src/shapes.rs`)
Define the shape struct and add it to `LayerKind`:
```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct DiamondProps {
    pub radius_x: f32,
    pub radius_y: f32,
    pub fill: Option<Paint>,
    pub stroke: Option<Stroke>,
}

// Add to enum LayerKind:
pub enum LayerKind {
    // ...
    Diamond(DiamondProps),
}
```

### Step 2: Descriptors (`crates/xhair-commands/src/descriptors.rs`)
Register UI labels, units, and slider ranges so the inspector and tooltips update automatically:
```rust
registry.add(PropertyDescriptor {
    name: "diamond.radius_x".into(),
    label: "Radius X".into(),
    group: "Geometry".into(),
    unit: Some("px".into()),
    min: Some(0.5),
    max: Some(50.0),
    default_value: "4.0".into(),
    tip: "Horizontal half-width of the diamond shape".into(),
});
```

### Step 3: Renderer (`crates/xhair-render/src/shapes/diamond.rs`)
Implement the tiny-skia path rendering:
```rust
pub fn render_diamond(pixmap: &mut Pixmap, props: &DiamondProps, center: Point) {
    let mut pb = PathBuilder::new();
    pb.move_to(center.x, center.y - props.radius_y);
    pb.line_to(center.x + props.radius_x, center.y);
    pb.line_to(center.x, center.y + props.radius_y);
    pb.line_to(center.x - props.radius_x, center.y);
    pb.close();
    if let Some(path) = pb.finish() {
        // Apply fill and stroke using standard paint pipeline
    }
}
```

### Step 4: Tool / UI (`crates/xhair-mcp/src/tools/` & `ui/`)
1. Add `diamond` variant to `layer.add` MCP tool schema.
2. In `ui/src/lib/components/LayerInspector.svelte`, the properties auto-populate from `ui/src/generated/properties.json`.

### Step 5: Test & Regenerate Documentation
1. Add a golden image test in `crates/xhair-render/tests/golden_tests.rs`.
2. Run `cargo xtask schema` to refresh `docs/PROPERTIES.md` and `ui/src/generated/properties.json`.
3. Verify test suite passes:
```bash
cargo test --workspace
```
That's it! In less than 100 lines of code, the new shape is fully supported in the core data model, file formats, undo/redo command bus, native renderer, WebAssembly preview, UI inspector, and MCP agent interface.
