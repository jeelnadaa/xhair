# Project Memory Graph Schema

The project memory graph records the architecture, invariants, requirements, decisions, crates, commands, and tools as a directed graph.

## Node Types
| Type | Description |
|---|---|
| `Requirement` | High-level non-negotiable principles (P1–P7) or spec constraints |
| `Milestone` | Project milestone (Phase -1, M0..M10) |
| `Task` | Individual actionable tasks from `TASKS.md` |
| `Decision` | Architectural Decision Record (ADR) from `DECISIONS/` |
| `Crate` | Rust workspace member crate |
| `Module` | Internal code module or subsystem |
| `Command` | Registered command in `xhair-commands` |
| `McpTool` | Exposed tool in `xhair-mcp` |
| `Property` | Declared property descriptor |
| `Screen` | UI screen from `design/` |
| `Component` | UI component from `design/` |
| `Shape` | Rasterizable shape kind in `xhair-render` |
| `Format` | Serialization format (`.xhair`, `.xhairpack`, share code, etc.) |
| `WinApi` | Specific allowed Windows OS API |
| `Test` | Integration, property, or golden-image test suite |
| `Doc` | Official documentation artifact |
| `Invariant` | Safety invariant that must never be broken |

## Edge Types
| Type | Meaning |
|---|---|
| `depends_on` | A depends on B |
| `implements` | A implements specification B |
| `exposes` | Command A exposes MCP tool B |
| `uses` | Screen or module A uses command B |
| `renders` | Screen A renders component B |
| `decided_by` | Component or behavior decided by ADR B |
| `supersedes` | ADR A supersedes ADR B |
| `tested_by` | Module or requirement A is verified by test B |
| `documents` | Doc A documents component B |
| `constrained_by` | A is constrained by invariant B |
| `belongs_to` | Task A belongs to milestone B |
| `calls_api` | Module A calls Windows API B |
