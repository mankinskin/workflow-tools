# Status: ON HOLD (deferred behind epic 69eb4118)

Deferred until the workflow-tools architectural migration lands. Direction change: rather than binding a hand-maintained catalog to live CLI/MCP shapes via a drift check within the current per-transport crate split, each **domain** will be consolidated into a single domain crate that can **generate a full catalog** directly. Each upstream dependency will publish its own catalog as documentation, which downstream catalogs/documentation consume. Implement this ticket only after the per-transport crates are merged into per-domain crates under epic 69eb4118.

`depends_on` -> 69eb4118 (epic, under specification). Do not start until that structural work provides the single-domain-crate surface to generate from.

Note: the coupled catalog `mutate`-note doc fix was already folded into 16d8aed9 and landed; this ticket is only the generation/architecture work.

---

# Origin

Review finding on 7d857543 (FUP-CAT). The delivered capability catalog is correct in content and passes CLI+MCP byte-equality, but it is a **hand-maintained `json!` blob** (crates/ticket-api/src/contracts/capability_catalog.rs). The reviewer requires the catalog to be **generated programmatically from the live capability shapes** so it cannot silently drift from the real surfaces.

# Problem

A hardcoded catalog duplicates truth that already exists in the code:
- CLI: the actual clap command/subcommand/argument definitions.
- MCP: the actual registered tool names + input schemas.
- Domains/workflows: enumerated by hand instead of derived.

Any new command/tool or renamed parameter can leave the catalog stale with no compile-time signal.

# Revised approach (post architectural migration)

- Consolidate each domain's per-transport crates (api/cli/mcp/http) into one domain crate that owns the canonical command/tool registry and generates its catalog from it.
- Each upstream dependency emits its own catalog as documentation; downstream domains consume upstream catalogs to compose full documentation.
- Generation replaces the hand-maintained literal; a drift check still guards that the generated catalog matches the live surface.

# Scope (unchanged intent, re-based on domain crates)

1. Derive the catalog from live capability shapes instead of a static literal:
   - Enumerate CLI workflows/params from the clap command tree (or a shared command registry).
   - Enumerate MCP tool names/params from the registered tool set / input schemas.
2. Keep the curated, human-authored layer (purpose text, nested-root notes, parity gaps) but bind it to the derived surface so a missing/renamed command fails a check.
3. Add a drift check (test or `catalog --check`) that fails when the derived surface and the rendered catalog disagree.
4. Preserve CLI+MCP byte-equality parity and the existing catalog content shape.

# Acceptance Criteria

- The catalog's command/tool/param lists are derived from the live CLI/MCP definitions, not hand-listed.
- A check fails when a command/tool exists in code but is absent from the catalog (or vice versa).
- Curated prose (purpose, notes, parity gaps) remains, bound to the derived entries.
- CLI and MCP still agree by construction.

# Kept from FUP-CAT (do not regress)

- ticket/spec/rule domain coverage, per-workflow nested-root flags, rule `author-and-generate` workflow, explicit parity_gaps, CLI `ticket catalog`, MCP `ticket_capabilities`, byte-equality parity test.