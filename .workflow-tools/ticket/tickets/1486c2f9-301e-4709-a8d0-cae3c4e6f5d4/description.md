# Implement mission-api durable store

## Objective

Implement the internal `mission-api` crate and the `.workflow-tools/mission/` store defined by [Mission Domain](../../../spec/specs/38a8ea3a-9a6a-4a0f-bc5a-97dc9ba670fc/body.md) (`ce://default/spec/38a8ea3a-9a6a-4a0f-bc5a-97dc9ba670fc`). Start only after W3 has been reviewed and the contract and serial ticket prerequisites have been read back.

## Scope

- Implement strict versioned mission/waypoint bundle models, parsing and safe manifest-relative fragment collection.
- Persist stable mission and waypoint IDs, declared total execution order, revision history, and revision-qualified historical display-label mappings.
- Validate exact order coverage, unique IDs, known prerequisites, prerequisite precedence, immediate-predecessor serial dependencies, acyclicity, requirements, canonical references, and ticket-backed status ownership.
- Emit stable diagnostics with error code, source file, JSON Pointer, waypoint ID where relevant, and actionable explanation.
- Provide a pure deterministic renderer for contiguous current-revision W1..Wn labels, ROADMAP.md, and Part navigation; rendering must not mutate the store or execute gates.
- Preserve the accepted manifest and prior revision when parsing, validation, stale-revision checks, or persistence fail.

## Acceptance Criteria

- `cargo test -p mission-api` passes.
- `cargo test -p mission-api --test roadmap_bundle` executes the required named bundle/order/revision cases from the specification and passes.
- `cargo test -p mission-api --test roadmap_render` executes the required named deterministic-output/numbering/link cases from the specification and passes.
- Both integration-test targets contain nonzero tests; CI fails if a required target or named test is absent.
- Tests read back persisted manifests, historical label mappings, and generated Markdown/Part bytes.

## Non-goals

CLI/MCP publication, session pin routing, legacy-dossier migration, guidance changes, HTTP, viewers, and agent scheduling.