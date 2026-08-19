---
theme: seriph
title: Workflow Tools
info: |
  ## Workflow Tools
  A tour of the domain-crate repositories nested under `workflow-tools`.
transition: fade
mdc: true
---

# Workflow Tools

The domain-crate suite behind ticket, spec, rule, session, and their sibling
workflow domains.

---

## One contract, every domain

Each domain repo follows the same shape:

- a `-api` library crate (domain-neutral logic)
- one facade crate with `cli` / `mcp` / `http` feature-gated binaries
- `memory-kernel`'s `transport-harness` owns transport-generic startup

See `contract-reference/` for the minimal reference implementation.

---

## Nested domain repositories

<div grid="~ cols-2 gap-4">
<div>

- `ticket` — ticket graph, board, workflow state machine
- `spec` — specification manifests and sections
- `rule` — rule manifests and target composition
- `session` — session worktree lifecycle, cost governance
- `audit` — repository quality audits

</div>
<div>

- `test` — validation specs and executions
- `doc` — documentation indexing
- `log` — tracing log analysis
- `feedback` — low-signal feedback ingestion
- `peek` — transcript/session inspection
- `interview` — structured interview workflows

</div>
</div>

---

## Shared substrate

`memory-kernel` nests under `workflow-tools/memory-kernel`: the filesystem-backed
entity store, indexing, search, workspace, and board primitives every domain
above builds on.

---

## contract-reference

The smallest possible domain: `example-api` → `example` (cli/mcp/http) →
`example-viewer` → `example-vscode`. Use it as the starting template for a new
domain repo.
