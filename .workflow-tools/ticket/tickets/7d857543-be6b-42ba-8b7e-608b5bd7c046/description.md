# Origin

Carved from the **original scope** of 5ad77aba. That parent session delivered and validated only the **session-mcp** `session_capabilities` slice; its REVIEWER NOTE flags the broader ticket/spec/rule catalog as NOT delivered. This ticket delivers that carved scope.

# Acceptance Criteria

- A command/tool lists canonical ticket/spec/rule workflows + required parameters.
- Catalog states nested-root/store support per workflow.
- MCP and CLI help agree on named workflows and targeting semantics.
- Rule-oriented workflows are discoverable from the same catalog.
- Parity gaps are explicit.

# Implementation Status — in-implementation (reopened 2026-07-25 after review)

Delivered the ticket/spec/rule capability catalog:

- New shared catalog `ticket_api::contracts::capability_catalog::capability_catalog()` describes ticket/spec/rule domains with read / mutate / board / next-and-why-not / validation workflows, required params, per-workflow `nested_roots_supported`, a rule-oriented `author-and-generate` workflow, and a documented `parity_gaps` section.
- CLI: new `ticket catalog` command — human-readable render + `--json`/`--toon` machine form.
- MCP: new `ticket_capabilities` tool returning the same shared catalog.
- CLI and MCP agree by construction (byte-equality test).

Validation: vt-ticket-spec-rule-catalog / exec-vt-ticket-spec-rule-catalog-20260725 (passed).

# REVIEW FINDINGS — 2026-07-25 (reviewer: send back to implementation)

Content and CLI/MCP parity are ACCEPTED, but the catalog is a **hand-maintained `json!` literal** and can silently drift from the real surfaces.

- **Required change:** generate the catalog **programmatically from the live capability shapes** — enumerate CLI workflows/params from the clap command tree and MCP tools/params from the registered tool set — with a drift check that fails when code and catalog disagree. Keep curated prose (purpose, nested-root notes, parity gaps) bound to the derived surface.
- Corrective work tracked in follow-up **c3e01552** (relates).

ACCEPTED and must NOT regress: ticket/spec/rule domain coverage, per-workflow nested-root flags, rule `author-and-generate` workflow, explicit parity_gaps, `ticket catalog` CLI, `ticket_capabilities` MCP, byte-equality parity.

Coupled doc fix: the catalog `mutate` note currently describes the rejected strict-hop behavior; that note update is folded into follow-up **16d8aed9** (auto-walk default).

# Likely Surfaces

- tools/ticket-cli/, tools/ticket-mcp/, tools/spec-cli/, tools/spec-mcp/, crates/rule-api/, README.md, .agents/instructions/