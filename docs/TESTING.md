# Testing Strategy & Verification Guide — XHAIR

This document outlines the testing matrix, continuous verification suites, and instructions for running all unit, integration, and parity tests (§15.2, §15.3).

---

## 1. Testing Matrix

| Layer / Crate | Test Scope | Verification Method |
|---|---|---|
| **`xhair-geometry`** | Center parity (even/odd), subpixel rounding, aspect ratio stretch, DPI 100%–500%, screenshot alignment | Property-based testing with `proptest`, deterministic unit tests |
| **`xhair-model` & `xhair-format`** | Canonical JSON schema, Brotli share codes, CRC32 verification, Valorant string parsing, version migrations | Serialization round-trips, golden fixture files, fuzz smoke tests |
| **`xhair-commands`** | Undo/redo inverse correctness, slider coalescing, atomic transaction rollback on failure | State-machine bus tests, history stack invariants |
| **`xhair-render`** | Native CPU vs. WebAssembly rasterization equivalence, bloom/glow filters, procedural animation | Bit-for-bit golden PNG comparison, criterion performance benchmarks |
| **`xhair-dynamics`** | Recoil accumulation, spread recovery, synthetic input state transitions, binding transfer curves | Deterministic simulation assertions, timeline verification |
| **`xhair-mcp`** | Tool schemas, JSON-RPC 2.0 over stdio & loopback HTTP, bearer authentication, DNS-rebinding protection | Integration test harnesses, automated agent session scripts |
| **`xhair-platform-win`** | Win32 layered window flags, click-through transparency, monitor enum, registry startup | Windows API integration tests, mocked monitor bounds |
| **`ui/` (Frontend)** | Svelte 5 state reactivity, layer ordering, canvas grid controls, export/import modals | `vitest` unit tests, Svelte component test runner |
| **Parity (`tests/parity.rs`)** | 1:1 parity between UI actions, commands, and MCP tools | Static AST & registry reflection checks in CI |
| **Memory & Performance** | Idle overlay $\le 20\text{ MB}$, zero CPU when static | Automated private working set memory gate in CI |

---

## 2. Running Test Suites

### 2.1 Workspace Rust Tests
Runs all unit and integration tests across all 11 workspace crates:
```bash
cargo test --workspace
```

### 2.2 UI Tests
Runs frontend store, state, and component tests:
```bash
pnpm -C ui test
```

### 2.3 Parity Verification Test
Enforces that every UI action has an equivalent command and MCP tool:
```bash
cargo test --test parity
```

### 2.4 Golden Image Parity Tests
Verifies that native `tiny-skia` and WebAssembly rendering produce identical golden bitmaps:
```bash
cargo test --package xhair-render --test golden_tests
```

### 2.5 Geometry Property Tests
Runs extensive random property tests exploring edge cases in coordinate math:
```bash
cargo test --package xhair-geometry --test proptest_geometry
```

### 2.6 Full Workspace Quality Suite
To execute the complete pre-merge CI validation pipeline:
```bash
# Code formatting
cargo fmt --check

# Strict linting (zero warnings permitted)
cargo clippy --workspace -- -D warnings
pnpm -C ui lint

# Agent continuity & task graph verification
cargo run --package xtask -- check-agent-state
cargo run --package xtask -- graph query --summary
```
