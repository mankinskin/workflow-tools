# W6: Session schema migration lifecycle infrastructure at production v1

- SCHEMA-1: Provide Session schema migration plan/journal/apply/resume/rollback with explicit workspace, finite selected IDs, current schema/hash preconditions and per-record results.
- SCHEMA-2: Production remains schema v1. v1-to-v1 returns zero transformations, unchanged bytes and no actual journal; unknown/future schemas reject before writes.
- SCHEMA-3: Only test-owned injected converters perform nonempty transformations. No production registration, flags or transport escape hatch enables fictitious schemas.
- SCHEMA-4: Nonempty fixture transformations stage/validate/journal/publish with provenance; failures before/after staging/publish and restart exercise durable per-step resume or operation-owned compensation.
- SCHEMA-5: Missing/corrupt journal, changed record/schema or unrelated later edits block recovery explicitly. CLI/MCP prove production no-op and rejection parity without needing a fake production converter.

## Failure and continuation
Converter or transformed-schema failure cannot publish invalid data. Partially published sets report persisted subset, never success-shaped fallback or assumed atomic rollback.
Apply common and per-waypoint branches from [outcome contract](../../../../../../transcripts/24-10-2026_mission-typed-authoring/OUTCOMES.md); after native binding that content must be embedded in the accepted Mission, not require this dossier at runtime.

## Validation and checkpoint
- `rtk cargo test -p session-api --test schema_migration`
- `rtk cargo test -p session --features cli --lib schema_migration_`
- `rtk cargo test -p session --features mcp --lib schema_migration_`
Focused selectors run once per owning changed revision, nonzero cases mandatory; no Python, active-denial machinery or unchanged broad final reruns. Planned guard `session-recovery-schema` is registered/read back as execution evidence, never claimed run in planning.

## Scope and authority
Production v2, live Session upgrades, track-link backfills or new schema registration.
Planning correction is not execution. Do not mutate live Mission/Session entities until the applicable execution and exact-set approval gates pass.
[Owning specification](../../../../spec/specs/cef24f88-a24b-4d1c-9a9c-5c249e9991f9/body.md)
[Detailed execution package](../../../../../../transcripts/24-10-2026_mission-typed-authoring/06-session-migration.md)
