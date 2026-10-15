# XHAIR — Addendum: Releases, Versioning, Agent Continuity & Project Memory Graph

> **Companion to `CROSSHAIR_OVERLAY_SPEC.md`.** Copy both files into the repo at `docs/spec/` (read-only reference). This addendum adds **Phase −1 (Bootstrap)** which runs **before** Phase 0 (Design), and adds Sections 19–22 to the main spec.
> Where the two files conflict, this addendum wins for: repo bootstrap, versioning, CI/CD, agent workflow, and memory.

---

## 19. Phase −1 — Bootstrap (do this first, before the FigCraft design)

Create, commit, and push (to `main`, as the first commits):

1. Repo skeleton from Section 3.3 of the main spec (empty crates with `README.md` stubs are fine).
2. `docs/spec/CROSSHAIR_OVERLAY_SPEC.md` and `docs/spec/AGENT_CONTINUITY_AND_RELEASES.md` (verbatim copies, never edited by the agent; changes go through an ADR + human approval — see 20.4).
3. **Agent continuity files** (Section 20): `AGENTS.md`, thin pointer files, and the `docs/agent/` state files, pre-filled from the templates below.
4. **Release pipeline** (Section 21): version files, `release-please` config, CI + release workflows, PR-title lint, issue/PR templates.
5. **Project memory graph** skeleton (Section 22) and `cargo xtask` scaffolding with `graph`, `check-agent-state`, `package` subcommands (stubs that exit 0 and are filled in by later milestones).
6. `LICENSE` (ask the human which one — Appendix C question), `.gitignore`, `.editorconfig`, `rust-toolchain.toml` (pinned stable), `deny.toml`, `rustfmt.toml`, `clippy.toml`.

**Acceptance:** CI is green on an empty skeleton; a `feat:` commit merged to `main` produces a release PR; `docs/agent/PROJECT_STATE.md` says "Phase −1 complete, next: Phase 0 design". Then report and continue to Phase 0.

---

## 20. Agent Continuity System (works across new sessions, new machines, different agents)

**Problem:** every new session starts with an empty context. **Solution:** the repository itself is the memory. A new session must be able to resume correctly by reading a small, fixed set of files. Never rely on chat history.

### 20.1 File set
```
AGENTS.md                      # canonical rules + read-order (≤ 150 lines)
CLAUDE.md                      # one line: pointer to AGENTS.md (see 20.5)
.github/copilot-instructions.md# pointer to AGENTS.md
.cursor/rules/xhair.mdc        # pointer to AGENTS.md
docs/agent/
├─ PROJECT_STATE.md            # CURRENT truth: phase, milestone, done/next, blockers
├─ TASKS.md                    # checklist with stable IDs, status, owner-session
├─ SESSION_LOG.md              # append-only, one entry per session
├─ DECISIONS/                  # ADRs: 0001-title.md ... (immutable once accepted)
├─ OPEN_QUESTIONS.md           # questions for the human, with defaults if unanswered
├─ GLOSSARY.md                 # domain terms (game px, edge coords, arm, slot, ...)
├─ INVARIANTS.md               # rules that must never break (P1–P7 + extras)
└─ graph/                      # project memory graph (Section 22)
docs/spec/                     # the two spec files (read-only)
```

### 20.2 Session START ritual (mandatory, in `AGENTS.md`)
1. Read `AGENTS.md` (this tells you the rest).
2. Read `docs/agent/PROJECT_STATE.md` → know exactly where the project is.
3. Read the **last 3 entries** of `docs/agent/SESSION_LOG.md`.
4. Read `docs/agent/INVARIANTS.md` and `docs/agent/OPEN_QUESTIONS.md`.
5. Run `cargo xtask graph query --summary` (or read `docs/agent/graph/graph.md`) to see the architecture map; query it before touching an unfamiliar area.
6. Open the relevant section of `docs/spec/CROSSHAIR_OVERLAY_SPEC.md` for the current milestone only (do not re-read the whole spec every time).
7. Run `git status`, `git log -n 15 --oneline`, and the test suite to confirm the baseline is green. If red, **fix or document that first**.
8. Write a **"Session start"** line in `SESSION_LOG.md`: date, goal, planned tasks (by ID).

### 20.3 Session END ritual (mandatory)
1. All tests, `clippy`, `fmt`, `eslint` pass (or failures are documented in `PROJECT_STATE.md` under *Blockers*).
2. Update `TASKS.md` (status), `PROJECT_STATE.md` (current phase/milestone, what changed, what's next, known issues), and append the **"Session end"** entry to `SESSION_LOG.md` (what was done, files touched, decisions, surprises, exact next step).
3. Run `cargo xtask graph` to regenerate the code/graph facts, then update hand-curated graph notes if concepts/decisions changed.
4. Commit state files **in the same PR/commit series as the code** they describe, using Conventional Commits (Section 21).
5. If the session is ending mid-task, leave a `WIP:` section in `PROJECT_STATE.md` with the exact file, function, and next command to run.

### 20.4 Rules for changing the plan
- The spec files are **authoritative and read-only**. If reality contradicts the spec, write an ADR in `docs/agent/DECISIONS/` (context, options, decision, consequences, status `proposed`), add it to `OPEN_QUESTIONS.md`, and **ask the human**. Only after approval set the ADR to `accepted` and proceed.
- ADRs are immutable once accepted; supersede with a new ADR (`supersedes: 00xx`).
- Never silently drop a requirement. If something is deferred, record it in `TASKS.md` with status `deferred` and a reason.

### 20.5 Pointer files (exact content)
`CLAUDE.md`:
```
@AGENTS.md
```
`.github/copilot-instructions.md` and `.cursor/rules/xhair.mdc`:
```
Follow AGENTS.md at the repository root. It is the single source of agent instructions.
```
(Verify each tool's current instruction-file conventions and adjust filenames if they changed. Keep **one** canonical file: `AGENTS.md`.)

### 20.6 `AGENTS.md` template (fill with real commands as they come to exist)
```markdown
# AGENTS.md — XHAIR

## What this is
Windows crosshair overlay + editor + MCP server. Overlay only; never touches the game process.
Full spec: docs/spec/CROSSHAIR_OVERLAY_SPEC.md (read-only). Release/agent rules: docs/spec/AGENT_CONTINUITY_AND_RELEASES.md.

## Read order at session start
1. docs/agent/PROJECT_STATE.md  2. last 3 entries of docs/agent/SESSION_LOG.md
3. docs/agent/INVARIANTS.md  4. docs/agent/OPEN_QUESTIONS.md
5. cargo xtask graph query --summary  6. the spec section for the current milestone

## Hard rules (never break)
- No game process access, no screen capture, no low-level hooks (spec §2).
- UI follows design/ exactly. Missing design => update design via FigCraft, tell the human.
- One source of truth: all mutations go through the command bus; UI/MCP/CLI are thin clients.
- Every UI action has a command and an MCP tool (parity test must pass).
- Every control has a tooltip; every property is declared once in a descriptor.
- Low RAM: idle overlay <= 20 MB; no render loop when static.
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
```

### 20.7 `PROJECT_STATE.md` template
```markdown
# Project State  (update every session; this file is the truth)
- Updated: <ISO date>   Session: <n>
- Phase / Milestone: <e.g. M4 Overlay daemon — in progress>
- Version released: <x.y.z>  | Unreleased on main: <summary>
## Done (milestone-level)
- [x] Phase -1 Bootstrap  - [x] M0 Design (approved <date>)  - [ ] M1 ...
## In progress
- <task IDs + 1-line status>
## Next exact step
- <file/function/command>
## WIP notes (if mid-task)
## Known issues / blockers
## Measured numbers (latest)
- Idle RAM: <MB>  Render time: <ms>  Editor-open RAM: <MB>
## Human decisions pending
- see OPEN_QUESTIONS.md
```

### 20.8 `TASKS.md` / `SESSION_LOG.md` formats
- `TASKS.md`: table with `ID | Milestone | Title | Status (todo/doing/blocked/done/deferred) | Spec ref | Notes`. IDs are stable (`M4-012`). Generate the initial list by splitting each milestone's acceptance criteria into tasks.
- `SESSION_LOG.md` entry:
```
## 2026-10-16 — Session 7
Start: goal, planned tasks (IDs)
End: done, files touched, decisions (ADR ids), surprises, tests status, exact next step
```

### 20.9 CI enforcement of continuity
`cargo xtask check-agent-state` (runs in CI on PRs):
- Fails if code under `crates/` or `ui/` changed but none of `PROJECT_STATE.md`, `TASKS.md`, `SESSION_LOG.md` changed in the PR (escape hatch: PR label `no-state-change` for trivial fixes).
- Fails if a `docs/agent/DECISIONS/*` file with status `accepted` was modified after acceptance.
- Fails if `docs/spec/*` changed without the label `spec-change-approved`.
- Fails if `docs/agent/graph/generated/*` is stale (graph regeneration produces a diff).

---

## 21. Versioning & GitHub Releases (automatic)

### 21.1 Rules
- **SemVer** (`MAJOR.MINOR.PATCH`). Start at `0.1.0`. While `< 1.0.0`, breaking changes bump **minor** (`bump-minor-pre-major: true`).
- **Single version source:** `[workspace.package] version` in the root `Cargo.toml`. These are kept in sync automatically: `ui/package.json`, `crates/xhair-app/tauri.conf.json`, and the installer metadata. The app displays this version in *About*, the tray tooltip, `meta.capabilities` (MCP), and `xhair-cli --version`, injected at build time from `CARGO_PKG_VERSION` plus short git SHA.
- **Separate version numbers** (documented in `docs/FORMAT.md`): app version (`x.y.z`), `.xhair` `format_version` (integer, with migrations), and **MCP tool API version** (`mcp_api_version`, bumped when a tool name/args change incompatibly).
- **What counts as breaking** (must use `!` or `BREAKING CHANGE:`): removing/renaming an MCP tool or arg, changing share-code/`.xhair` semantics without a migration, changing settings file shape without a migration, changing CLI flags.
- **Conventional Commits** are mandatory; PR titles are linted (squash-merge uses the PR title as the commit message).

### 21.2 Mechanism: release-please (verify current docs/versions before wiring)
Flow: merge to `main` → bot opens/updates a **Release PR** (version bump + `CHANGELOG.md`) → human merges it → tag `vX.Y.Z` + GitHub Release created → Windows build job attaches artifacts to that release.

**Gotcha:** tags/releases created with the default `GITHUB_TOKEN` do **not** trigger other workflows. Therefore build and upload artifacts **in the same workflow run**, gated on the release-please output (as below), instead of listening for `release: published`.

`release-please-config.json`:
```json
{
  "$schema": "https://raw.githubusercontent.com/googleapis/release-please/main/schemas/config.json",
  "packages": {
    ".": {
      "release-type": "simple",
      "package-name": "xhair",
      "bump-minor-pre-major": true,
      "bump-patch-for-minor-pre-major": false,
      "changelog-path": "CHANGELOG.md",
      "extra-files": [
        { "type": "toml", "path": "Cargo.toml", "jsonpath": "$.workspace.package.version" },
        { "type": "json", "path": "ui/package.json", "jsonpath": "$.version" },
        { "type": "json", "path": "crates/xhair-app/tauri.conf.json", "jsonpath": "$.version" }
      ]
    }
  }
}
```
`.release-please-manifest.json`: `{ ".": "0.1.0" }`

**Cargo.lock:** the lockfile must be refreshed after the bump. Add a step on the release PR branch that runs `cargo update --workspace` (or `cargo check --workspace --locked` after regenerating) and pushes the lockfile update; verify this works with the chosen release-please version, otherwise use `release-plz` instead. If neither handles the workspace cleanly, write `cargo xtask bump` and document why.

`.github/workflows/release.yml` (skeleton — agent must verify action versions and pin them to full commit SHAs):
```yaml
name: release
on:
  push:
    branches: [main]
permissions:
  contents: write
  pull-requests: write
concurrency:
  group: release
  cancel-in-progress: false
jobs:
  release-please:
    runs-on: ubuntu-latest
    outputs:
      release_created: ${{ steps.rp.outputs.release_created }}
      tag_name: ${{ steps.rp.outputs.tag_name }}
      version: ${{ steps.rp.outputs.version }}
    steps:
      - id: rp
        uses: googleapis/release-please-action@v4   # pin to SHA
        with:
          config-file: release-please-config.json
          manifest-file: .release-please-manifest.json

  build-windows:
    needs: release-please
    if: ${{ needs.release-please.outputs.release_created == 'true' }}
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v4                    # pin to SHA
        with: { ref: "${{ needs.release-please.outputs.tag_name }}" }
      - uses: dtolnay/rust-toolchain@stable          # or the pinned toolchain file
      - uses: Swatinem/rust-cache@v2
      - uses: pnpm/action-setup@v4
      - uses: actions/setup-node@v4
        with: { node-version: 22, cache: pnpm }
      - run: pnpm -C ui install --frozen-lockfile
      - run: cargo xtask package --version ${{ needs.release-please.outputs.version }}
      # produces dist/: installer (NSIS), portable zip, SHA256SUMS.txt, sbom.json
      - name: Upload to release
        env: { GH_TOKEN: "${{ secrets.GITHUB_TOKEN }}" }
        run: gh release upload "${{ needs.release-please.outputs.tag_name }}" dist/* --clobber
      - uses: actions/attest-build-provenance@v2     # optional: build provenance (verify availability)
        with: { subject-path: "dist/*" }
```

### 21.3 Artifacts per release
`xhair-<version>-x64-setup.exe` (NSIS installer), `xhair-<version>-x64-portable.zip` (contains `xhair.portable` marker), `xhair-cli-<version>-x64.zip`, `SHA256SUMS.txt`, SBOM (`cargo cyclonedx` or equivalent). Release notes = the generated changelog section. Mention in the README that unsigned builds trigger Windows SmartScreen; add code signing later via secrets (`SIGNING_CERT`, never in repo) if the human provides a certificate.

### 21.4 CI workflows (`.github/workflows/`)
- `ci.yml` (PRs + main): fmt, clippy, tests, wasm build + golden parity, fuzz smoke, UI lint/test, docs-generation freshness, memory check, `cargo deny`/`audit`, `cargo xtask check-agent-state`.
- `pr-title.yml`: Conventional Commit PR-title lint (e.g., `amannn/action-semantic-pull-request`; verify).
- `release.yml`: above.
- `nightly.yml` (optional): 10-minute fuzzing + performance benchmarks, files an issue on regression.
- Branch protection (document in `CONTRIBUTING.md`; ask the human to enable): require PR, require CI, require linear history, squash merges only.
- Dependabot/Renovate for Cargo, npm, and GitHub Actions (actions pinned by SHA).

### 21.5 Pre-releases (optional, ask the human)
Support `-beta.N` via a `prerelease` config or a `release/beta` branch; GitHub Releases marked "pre-release". Default: off.

### 21.6 Release checklist (put in `docs/RELEASING.md`)
Green CI → Release PR reviewed (changelog sane, version correct) → merge → verify the release has all artifacts + checksums → smoke-test the installer on a clean Windows VM → update `PROJECT_STATE.md` ("Version released").

---

## 22. Project Memory Graph (so any new session can read the architecture as a graph)

### 22.1 Honest answer to "is there a skill for this?"
There isn't one standard, universally adopted "skill" for this. What exists are **MCP memory servers** that store entities/relations (a knowledge graph) outside the conversation. Examples found at the time of writing — **verify maturity, license, and maintenance before adopting**:
- The **reference MCP "memory" server** from the MCP project (knowledge graph of entities, relations, observations) — verify current package name in the official MCP servers repo.
- **Graph-Mem MCP** — per-project knowledge graph stored in `.graphmem/graph.db` inside the project (github.com/Sathvik-1007/GraphMem-MCP).
- **Anchored** — cross-tool persistent memory MCP server with a SQLite-backed knowledge graph (Go single binary).

**Design decision (ADR-0001 to write in Phase −1): the repository files are the source of truth; MCP memory is an optional accelerator.**
Reasons: (1) an MCP memory DB lives outside git and won't follow a new machine, new agent, or CI; (2) files are reviewable by the human; (3) any agent (even without MCP) can read them. So: build the graph **in-repo**, and optionally mirror it into an MCP memory server for convenience.

### 22.2 In-repo graph (required)
```
docs/agent/graph/
├─ schema.md                # node & edge types (below)
├─ curated/
│  ├─ nodes.jsonl           # hand/agent-maintained: concepts, decisions, requirements, rationale
│  └─ edges.jsonl
├─ generated/               # produced by `cargo xtask graph` — NEVER hand-edit
│  ├─ nodes.jsonl
│  ├─ edges.jsonl
│  └─ graph.md              # Mermaid overview + per-crate summaries (human + agent readable)
└─ queries.md               # example queries
```
**Why JSONL:** diff-friendly, streamable, tiny, no database needed in the repo.

**Node types:** `Requirement` (P1–P7, spec sections), `Milestone`, `Task`, `Decision` (ADR), `Crate`, `Module`, `Command` (from registry), `McpTool`, `Property` (descriptor), `Screen` (design), `Component` (design), `Shape`, `Format` (xhair/pack/code), `WinApi` (OS API used), `Test`, `Doc`, `Invariant`.
**Edge types:** `depends_on`, `implements`, `exposes` (Command→McpTool), `uses` (Screen→Command), `renders` (Screen→Component), `decided_by` (X→ADR), `supersedes`, `tested_by`, `documents`, `constrained_by` (X→Invariant/Requirement), `belongs_to` (Task→Milestone), `calls_api` (Module→WinApi).
**Node/edge record:** `{"id":"cmd:layer.update","type":"Command","name":"layer.update","path":"crates/xhair-commands/src/layer.rs","summary":"...","tags":["document"],"updated":"<sha>"}` / `{"from":"cmd:layer.update","to":"tool:layer.update","type":"exposes"}`.

**Generated facts** (`cargo xtask graph`, deterministic, sorted output): crate dependency graph from `cargo metadata`; commands and MCP tools from the registry export; property descriptors; screens→routes→commands from `design/README.md`; `WinApi` usage by scanning `xhair-platform-win` for the allow-listed API names (and **failing** if a forbidden API from spec §2 appears anywhere — doubles as a compliance guard); tests→covered modules via naming convention; milestone/task nodes from `TASKS.md`; ADR nodes from `DECISIONS/`.
**Curated facts** (agent-maintained): concepts ("center alignment rule"), rationale ("why CPU rasterizer"), cross-cutting links ("Dynamic Reticle ← flagged by compliance ADR"), known pitfalls. Every curated node must link to at least one file/ADR/spec section (`xhair-graph lint` rejects orphans and dangling edges).

### 22.3 Query interface for agents
`cargo xtask graph query`:
- `--summary` (top-level map, counts, current milestone, open ADRs)
- `--node <id|name>` (node + neighbors, 1–2 hops)
- `--impact <path|node>` (what depends on this; what tests cover it; which docs mention it)
- `--path <a> <b>` (shortest relationship path)
- `--type <Type> [--tag t]`
- `--json` for machine output
Implement with a tiny in-memory graph loaded from the JSONL (no external DB; < 1 MB). Document in `docs/agent/graph/queries.md` with examples like *"what must I update if I add a shape?"* (→ model, descriptor, renderer, tool, tests, docs nodes) and *"which screens use command `nudge`?"*.
**Rule in `AGENTS.md`:** before changing a module, run `--impact`; after adding a command/shape/screen, run `cargo xtask graph` and commit the diff.

### 22.4 Optional MCP mirror (ask the human whether to enable)
If the human wants it, configure one of the MCP memory servers above for the agent and add `cargo xtask graph export --mcp` that emits entities/relations in that server's import format (verify format). Rules:
- The in-repo graph is canonical; the MCP memory is **rebuilt from it** at session start if missing/stale.
- The agent may store *ephemeral working notes* in MCP memory, but anything durable **must be written to the repo** before the session ends.
- Record which MCP memory server is used and its version in an ADR.

### 22.5 Acceptance (Phase −1 and ongoing)
- `cargo xtask graph` runs deterministically (two runs → identical output).
- `cargo xtask graph query --summary` works on the skeleton and lists crates, milestones, requirements P1–P7.
- CI fails on stale generated graph, orphan curated nodes, or forbidden-API detection.
- A fresh agent session, given only the repo, can answer "what is the current milestone, what's next, and what depends on the command bus?" using only `AGENTS.md`, `PROJECT_STATE.md`, and the graph (verify this once manually and note it in `SESSION_LOG.md`).

---

## 23. Updated Execution Order

1. **Phase −1 Bootstrap** (Section 19) → commit, push, report.
2. **Phase 0 Design with FigCraft** (main spec §17) → **stop for human approval**.
3. **M1…M10** (main spec §16) — every milestone merges via PR with Conventional Commits, updated continuity files, regenerated graph, and a release PR produced automatically.
4. Update Definition of Done (main spec §18) with:
   - [ ] A tagged GitHub Release exists with installer, portable zip, CLI zip, checksums, SBOM.
   - [ ] `AGENTS.md`, `docs/agent/*`, and the graph are current and pass `check-agent-state`.
   - [ ] A cold-start test: a brand-new agent session resumes from the repo alone and continues the next task correctly.

## 24. Additional questions to ask the human once, in Phase −1
1. GitHub repo owner/name and visibility (public/private)? Is Actions enabled and may the bot open PRs (needs "Allow GitHub Actions to create and approve pull requests")?
2. License choice.
3. Enable the optional MCP memory mirror? Which server?
4. Pre-release channel wanted?
5. Code-signing certificate available now or later?
