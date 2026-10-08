# Migrate legacy dossiers to missions

## Objective

Implement the repeatable legacy-dossier migration defined by [Mission Domain](../../../spec/specs/38a8ea3a-9a6a-4a0f-bc5a-97dc9ba670fc/body.md) (`ce://default/spec/38a8ea3a-9a6a-4a0f-bc5a-97dc9ba670fc`). This ticket follows the session-attachment ticket in the roadmap's required serial execution order.

## Scope

- Inventory current legacy `transcripts/` dossier folders; the previously observed count of 69 is a research snapshot, not a fixed limit.
- Create or map mission entities idempotently, preserving stable waypoint IDs, source provenance, historical revision-qualified W-label mappings, and references.
- Normalize each accepted current roadmap to contiguous W1..Wn labels and one explicit serial execution order. Report ambiguity or inconsistent dependency data as blocked; never guess.
- Provide dry-run and recovery behavior without deleting or rewriting source dossiers.
- Migrate `transcripts/01-10-2026_roadmap-state-integration/` first and bind its mission identifier as `$MISSION_ID`.

## Acceptance Criteria

- `cargo test -p mission --test dossier_migration -- --exact migration_is_idempotent` executes one test and passes.
- `cargo test -p mission --test dossier_migration -- --exact migration_preserves_source_and_label_mapping` executes one test and passes.
- `cargo test -p mission --test dossier_migration -- --exact ambiguous_source_is_reported_blocked` executes one test and passes.
- `cargo run -p mission --features cli -- mission migrate-dossiers --dry-run` reports outcomes without modifying or deleting source dossiers.
- `mission get "$MISSION_ID"` reads back the migrated current roadmap and its provenance.
- Missing integration targets or zero matched tests fail acceptance.

## Non-goals

Deleting legacy dossiers, retargeting guidance, HTTP, viewers, and scheduling.