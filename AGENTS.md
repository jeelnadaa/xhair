# AGENTS.md — XHAIR

## What this is
Windows crosshair overlay + editor + MCP server. Overlay only; never touches the game process.
Full spec: docs/spec/CROSSHAIR_OVERLAY_SPEC.md (read-only). Release/agent rules: docs/spec/AGENT_CONTINUITY_AND_RELEASES.md.

## Read order at session start
1. docs/agent/PROJECT_STATE.md
2. Last 3 entries of docs/agent/SESSION_LOG.md
3. docs/agent/INVARIANTS.md
4. docs/agent/OPEN_QUESTIONS.md
5. cargo xtask graph query --summary
6. The spec section for the current milestone (in docs/spec/)

## Hard rules (never break)
- No game process access, no screen capture, no low-level hooks (spec §2).
- UI follows design/ exactly. Missing design => update design via FigCraft, tell the human.
- One source of truth: all mutations go through the command bus; UI/MCP/CLI are thin clients.
- Every UI action has a command and an MCP tool (parity test must pass).
- Every control has a tooltip; every property is declared once in a descriptor.
- Low RAM: idle overlay <= 20 MB; zero CPU render loop when static.
- No network calls except loopback MCP.
- Do not edit docs/spec/*. Propose changes via ADR and ask the human.

## Commands
- Build:     cargo build --workspace
- Test:      cargo test --workspace && pnpm -C ui test
- Lint:      cargo fmt --check && cargo clippy --workspace -- -D warnings && pnpm -C ui lint
- Graph:     cargo xtask graph [query ...]
- State chk: cargo xtask check-agent-state
- Package:   cargo xtask package

## Commit & release rules
- Conventional Commits (feat:, fix:, perf:, refactor:, docs:, test:, build:, ci:, chore:).
- Breaking change => "feat!:" or "BREAKING CHANGE:" footer. Releases are automated (release-please). Never bump versions by hand.
- One logical change per commit; code + docs + state files together.

## Session end checklist
Tests green -> update TASKS.md, PROJECT_STATE.md, SESSION_LOG.md -> cargo xtask graph -> commit.

## When unsure
Ask the human. Record the question in docs/agent/OPEN_QUESTIONS.md with a proposed default.
