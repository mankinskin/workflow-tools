# Waypoint 6: Spec v2 model, storage, and explicit migration

## Governing specifications

- `.spec/specs/a608f774-9f50-4de1-90e2-ffeefb0198b1/body.md`
- `.spec/specs/fdb7645d-eac5-4b82-88eb-94cb22f1b0b2/body.md`
- `.spec/specs/55d8f2eb-70f1-4b90-8c8f-e50d5e311d48/body.md`

## Owned implementation surface

- `workflow-tools/spec/crates/spec-api/src/manifest.rs`
- `workflow-tools/spec/crates/spec-api/src/store.rs` and existing store children
- `workflow-tools/spec/crates/spec-api/schemas/specification.toml`
- `workflow-tools/spec/src/cli/` migration command surface and matching spec MCP operations
- reviewed migration-map fixtures under the spec workspace

This ticket implements the v2 typed model and persistence boundary only. It consumes the shared operation-journal contract from ticket `73b2cd22`; it must not create a second domain journal or implement relationships/templates, query projection, health policy, annotations, ticket gating, or the worktree pilot.

## Acceptance criteria

1. V2 manifests require explicit `format_version = 2`, immutable globally unique `component_id`, retained root fields, canonical typed tables, and no inferred v2/component identity.
2. The store round-trips component identity, criteria/evidence, provider-owned edge and template-binding records, observations, and typed targets while preserving sections, hierarchy, and `TicketRef`; retired fields are rejected or removed according to the governing contracts.
3. `spec migrate` and matching MCP operations use a reviewed explicit UUID-to-`component_id` map, reject missing/duplicate/generated mappings before writes, are idempotent, and expose dry-run, collision, resume, and rollback through the shared journal.
4. Multi-file spec writes use the shared journal and report recoverable state rather than silently repairing interruptions.

## Focused validation

- `cargo test -p spec-api`
- migration dry-run, interruption, resume, rollback, mapping-rejection, manifest/store round-trip, and hierarchy renderer tests
- `./target/debug/spec.exe --workspace . get f1b8f01a-c7da-4a71-97c5-39519a7d7f38 --json`

## Done condition

The v2 core can persist and explicitly migrate reviewed component specs using the shared journal, providing the sole base model for downstream tickets.