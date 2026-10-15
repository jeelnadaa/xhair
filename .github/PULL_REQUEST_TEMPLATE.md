## Summary
<!-- Brief summary of changes. Follows Conventional Commits. -->

## Associated Milestone / Task
- Task ID: `P-1-xxx` or `Mxx-xxx`
- Milestone: 

## Invariants & Guardrails Self-Check
- [ ] P1: No game process memory access, injection, hooks, or live screen capture.
- [ ] P2: Low RAM & zero CPU when static.
- [ ] P3: State mutations flow through command bus.
- [ ] P4: UI actions mapped 1:1 to commands and MCP tools.
- [ ] P5: Properties declared in descriptors.
- [ ] P6: Pixel accuracy and center verified.
- [ ] P7: Clean code, no unwrap in production, strict rustdoc.

## Agent Continuity
- [ ] `PROJECT_STATE.md` updated.
- [ ] `TASKS.md` updated.
- [ ] `SESSION_LOG.md` entry appended.
- [ ] `cargo xtask check-agent-state` passed.
