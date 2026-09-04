---
description: "Use when reading workspace files or conducting structural exploration. Covers bounded reads, peek CLI usage, repo_map.toon orientation, and avoiding full-file pulls."
applyTo: "**/*.md,**/*.toon"
---

## Default Agent Tool Suite

Reading files is the **read** category of the default agent tool suite
(read / execute / edit / filesystem / search). Its implementation is the `peek`
family, and it is the default path for **every** agent, including delegated
sub-agents that inherit none of the orchestrator's context:

- MCP (preferred): `peek-mcp` — `peek_read`, `peek_grep`, `peek_count`,
  `peek_skeleton`. Reachable through the `'peek-mcp/*'` wildcard in
  `.agents/agents/*.agent.md`.
- CLI fallback: `peek` (`workflow-tools/peek/crates/peek-cli`) when MCP is unavailable.
- Behavior owner: `workflow-tools/peek/crates/peek-api`.

Use this suite instead of unbounded built-in file reads. The sibling **execute**
category is covered in
[tool-output.instructions.md](tool-output.instructions.md).

## Structural Awareness Before Exploration

Before running exploratory searches or broad file reads, consult compact structural sources first:

1. **`repo_map.toon`** — compact workspace map at the repository root. Read this first for directory/crate layout.
  Refresh with `cargo run -p peek-cli -- . --repo-map --output repo_map.toon` when crates or agent files change.
2. **Interface skeletons** — stripped function/type signatures without bodies (when available).
3. **Crate `README.md`** and `HIGH_LEVEL_GUIDE.md`** — API patterns, common gotchas, and design context.

Only fall back to broad `semantic_search` or exploratory file listing when the compact sources are insufficient.

## Bounded File Inspection

Never pull an entire file when only a targeted slice is needed.

Preferred pattern:
1. Use `repo_map.toon` (`repo_map.toon` at the repository root) for structural orientation before opening any file.
2. Use an interface skeleton view before reading full source when available.
3. Open a bounded line window with explicit start/end coordinates.
4. Only escalate to a full-file read when the bounded window is genuinely insufficient.

Use the `peek` CLI tool (`workflow-tools/peek/crates/peek-cli`) for bounded reads from the terminal:

```bash
# Step 1: learn file size
peek path/to/file.rs --count

# Step 2: locate the target (returns matching line numbers)
peek path/to/file.rs --grep "fn my_function"

# Step 3: read a tight window
peek path/to/file.rs --start 42 --end 80

# Step 4: show context around a pattern match
peek path/to/file.rs --grep "fn my_function" --window 15

# Escape hatch (explicit, token-expensive)
peek path/to/file.rs --all
```

When the required line coordinates are unknown, use `grep_search`, `semantic_search`, or `peek --grep` to locate the target region first, then read a bounded window around it.

Full-file reads should become the exception. The `--all` flag is intentionally named to make the cost visible in command history.

## Known Suite Gaps

Two categories of the suite have no owned tool yet, so bounded discipline must be
applied manually until they land:

- **Filesystem operations** (bounded list / stat / move) — raw shell is the
  current fallback. Cap depth and entry counts explicitly.
- **Repo-wide search** — `peek_grep` is single-file only. Repo-wide scans fall
  back to `grep_search` / `rg`; always pass a narrowing path filter and prefer a
  counts-first pass before requesting content.

Tracked by epic `.ticket/tickets/e342cc4c-a7a4-42de-81fc-572d0497d12b`.
