# Project Memory Graph Queries

Examples of using `cargo xtask graph query` to inspect architecture, dependencies, and blast radius before modifying code.

## Quick Summary
```bash
cargo xtask graph query --summary
```
Prints total node & edge counts, current milestone, open ADRs, and high-level crate structure.

## Inspecting a Specific Node
```bash
cargo xtask graph query --node crate:xhair-commands
cargo xtask graph query --node req:P1
```
Outputs the node details, its outgoing edges, and its incoming edges (1–2 hops).

## Impact Analysis (Blast Radius)
```bash
cargo xtask graph query --impact crates/xhair-commands
cargo xtask graph query --impact cmd:layer.update
```
Shows everything that depends on this component: dependent crates, screens that use it, MCP tools that expose it, and tests that verify it.

## Shortest Path
```bash
cargo xtask graph query --path screen:editor tool:layer.update
```
Finds the relationship sequence between two entities.
