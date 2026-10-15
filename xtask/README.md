# xtask

Developer automation tools for `xhair`.

## Subcommands
- `cargo xtask graph query --summary`: Print summary of architecture graph, node counts, and invariant links.
- `cargo xtask graph query --node <id>`: Inspect relationships and metadata for a specific entity.
- `cargo xtask check-agent-state`: Verify all agent continuity files and invariant checks.
- `cargo xtask package --version <v>`: Package release artifacts (NSIS installer, portable zip, checksums, SBOM).
