# ADR-0001: Repository Files as Canonical Project Memory Source of Truth

- **Status:** accepted
- **Date:** 2026-10-15
- **Deciders:** Human user, Antigravity Agent
- **Consulted:** `AGENT_CONTINUITY_AND_RELEASES.md` Section 22.1

## Context
AI agents need persistent memory across sessions, machines, and tools. Various external MCP memory servers exist (e.g. SQLite-backed or GraphMem), but external databases live outside git, cannot be reviewed in pull requests, do not travel with the code on checkout, and are inaccessible in automated CI runners.

## Decision
The git repository itself is the single, canonical source of truth for all project memory, continuity state, architecture graphs, and decisions:
1. `docs/agent/graph/curated/` and `docs/agent/graph/generated/` store nodes and edges as JSON Lines (`.jsonl`).
2. Generated facts are produced deterministically by `cargo xtask graph` from code metadata and registries.
3. External MCP memory servers, if ever configured, are strictly read-only mirrors rebuilt from git files.

## Consequences
- **Positive:** Project memory is 100% versioned, reproducible, inspectable, and diffable in git.
- **Positive:** Zero external runtime database dependencies for builds, CI, or new developer checkouts.
- **Negative:** Graph regeneration must be kept fast and deterministic in `cargo xtask graph`.
