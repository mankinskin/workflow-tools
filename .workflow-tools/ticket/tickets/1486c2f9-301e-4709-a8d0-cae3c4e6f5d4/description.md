# Implement mission-api durable store

## Objective

Implement the internal `mission-api` crate and the `.workflow-tools/mission/` UUID-per-mission store defined by [Mission Domain](../../workflow-tools/.workflow-tools/spec/specs/38a8ea3a-9a6a-4a0f-bc5a-97dc9ba670fc/body.md) (`ce://default/spec/38a8ea3a-9a6a-4a0f-bc5a-97dc9ba670fc`).

## Scope

- Implement mission manifests, persistence, read-back, mutation integrity, and focused tests.
- Preserve references to external domain entities as canonical identifiers or URNs.
- Reject malformed UUIDs, duplicate references, invalid URNs, unknown waypoint edges, and cyclic waypoint dependencies without corrupting the prior manifest.

## Acceptance Criteria

- `cargo test -p mission-api` passes.
- `cargo test -p mission-api -- store` passes.
- Store reads return actionable errors for malformed or incomplete manifests.

## Non-goals

CLI/MCP transports, session pin routing, legacy-dossier migration, guidance changes, HTTP, viewers, and scheduling.