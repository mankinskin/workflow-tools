# Problem

Cross-cutting design principles that recur across many specs (workspace identifiers, typed errors, JSON contracts, browser validation, ticket traceability link format, generated-file markers, `<x>-api` ownership, `viewer-ctl` lifecycle boundary, etc.) live only as prose duplicated in individual spec bodies. They are not addressable as canonical rule entries, so agents cannot find a single authoritative source, generated documents cannot reuse them, and feedback cannot be attached to them.

# Desired outcome

Each recurring principle exists as a canonical rule entry in the rule store of the workspace that owns it (context-engine root, memory-viewers/memory-api, or memory-viewers/viewer-api). Each owning workspace has one spec authored under the `a5fe4c58` pipeline whose `body.md` summarises the principles and whose `sections/<principle>.md` files carry the canonical prose; rule entries materialise from those sections via `rule scan`. A `rule-targets.yaml` target per workspace composes the entries into a generated document, and the spec's `generated.toml` rebuilds the body/sections from those targets so the round-trip is byte-stable.

# Proposed direction

- Use the migration pipeline introduced by `a5fe4c58 Adopt rule targets for generated spec artifacts` and proven by spec `1cf68c36 generated documents`.
- Author one principles spec per workspace (context-engine, memory-api, viewer-api).
- Materialise rule entries via `rule scan` against each spec body/sections.
- Wire a per-workspace `rule-targets/<n>-recurring-principles.yaml` target referencing the spec sources, and a spec-local `generated.toml` mapping `body.md` and each section to that target's outputs.
- Validate that `spec sync-generated` regenerates each spec byte-stably.

# Acceptance criteria

- A principles spec exists in each of context-engine root, memory-viewers/memory-api, and memory-viewers/viewer-api with `## ` sections for every principle owned by that workspace.
- `rule list --where slug~"recurring-principles"` (or equivalent) returns one rule entry per principle in each workspace.
- A `rule-targets/<n>-recurring-principles.yaml` exists per workspace with one target per spec artifact (`body` + each `sections/*.md`).
- Each spec has a `generated.toml` whose `body` and `sections` entries point at those targets.
- `spec sync-generated <id> --workspace-root <ws>` succeeds for all three specs and produces no diff on a second run.
- Tracker ticket is linked from each spec via `spec_refs` and from the related-tickets section of each spec body.
