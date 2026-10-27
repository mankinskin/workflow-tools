# Session schema migration lifecycle infrastructure at production v1

## Goal and scope
SCHEMA-1: Provide Session schema migration plan/journal/apply/resume/rollback with explicit workspace, finite selected IDs, current schema/hash preconditions and per-record results.

## Acceptance and validation
The explicit objective and requirements Parts are the complete current plan; do not infer completion from a preview or old note.
- `rtk cargo test -p session-api --test schema_migration`
- `rtk cargo test -p session --features cli --lib schema_migration_`
- `rtk cargo test -p session --features mcp --lib schema_migration_`

## Non-goals
Production v2, live Session upgrades, track-link backfills or new schema registration.

## Status
Planning preparation only; implementation and live operations have not started. Use the linked owning specification and requirements through the plan view.
