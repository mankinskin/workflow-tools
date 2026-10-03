# Migrate legacy dossiers to missions

## Objective

Implement the repeatable legacy-dossier migration defined by [Mission Domain](../../workflow-tools/.workflow-tools/spec/specs/38a8ea3a-9a6a-4a0f-bc5a-97dc9ba670fc/body.md) (`ce://default/spec/38a8ea3a-9a6a-4a0f-bc5a-97dc9ba670fc`).

## Scope

- Inventory the 69 legacy `transcripts/` dossier folders.
- Create or map mission entities idempotently, preserving source provenance and reporting migrated, skipped, and blocked records.
- Provide dry-run and recovery behavior without deleting or rewriting source dossiers.
- Migrate `transcripts/01-10-2026_roadmap-state-integration/` first and bind its mission identifier as `$MISSION_ID`.

## Acceptance Criteria

- `cargo test -p mission -- migration` passes.
- `cargo run -p mission --features cli -- mission migrate-dossiers --dry-run` passes.
- `mission get "$MISSION_ID"` reads the current dossier's created mission record.

## Dependencies

Depends on the mission CLI/MCP transport ticket.

## Non-goals

Deleting legacy dossiers, retargeting guidance, HTTP, viewers, and scheduling.