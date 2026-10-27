<!-- aligned-structure:v2 -->
# Session schema migration lifecycle infrastructure at production v1

## Target Code Location
- [session/crates/session-api/src/model.rs](../../../../session/crates/session-api/src/model.rs)
- [session/crates/session-api/src/store/helpers/storage.rs](../../../../session/crates/session-api/src/store/helpers/storage.rs)
- [memory-kernel/src/model/migration.rs](../../../../memory-kernel/src/model/migration.rs)
- [session/src/cli.rs](../../../../session/src/cli.rs)
- [session/src/mcp/server.rs](../../../../session/src/mcp/server.rs)

## Naming Conventions
Criterion prefix: `SCHEMA`. Public operations and carrier names are the concrete contract in Behavior and the linked execution package.

## Requester Input
> update the ticket and create all planning entities required. continue refinement until all blockers are solved


## Reading Order
1. [Recovery composition](../9f8d9f34-764f-4821-9b14-b6d8773ec2ae/body.md)
2. [W6 execution package](../../../../../transcripts/24-10-2026_mission-typed-authoring/06-session-migration.md)
3. [Owning ticket](../../../ticket/tickets/86816751-6212-407f-bed3-dfbc16231d1e/ticket.toml)

## Responsibility
Motivation: Session schema migration lifecycle infrastructure at production v1 closes the observed canonical-recovery gap without annexing adjacent historical work. If this spec is implemented, dependents can rely on every criterion below, not merely a successful command exit.

## Interfaces And Dependencies
Uses existing domain/shared-kernel infrastructure; no dependency on unrelated Mission/Session work is fabricated.
Domain APIs own persistence and lifecycle; transports preserve structured outcomes. Explicit entity-store roots remain distinct from command cwd and checkout.

## Behavior
- SCHEMA-1: Provide Session schema migration plan/journal/apply/resume/rollback with explicit workspace, finite selected IDs, current schema/hash preconditions and per-record results.
- SCHEMA-2: Production remains schema v1. v1-to-v1 returns zero transformations, unchanged bytes and no actual journal; unknown/future schemas reject before writes.
- SCHEMA-3: Only test-owned injected converters perform nonempty transformations. No production registration, flags or transport escape hatch enables fictitious schemas.
- SCHEMA-4: Nonempty fixture transformations stage/validate/journal/publish with provenance; failures before/after staging/publish and restart exercise durable per-step resume or operation-owned compensation.
- SCHEMA-5: Missing/corrupt journal, changed record/schema or unrelated later edits block recovery explicitly. CLI/MCP prove production no-op and rejection parity without needing a fake production converter.

## Boundaries And Failure Cases
Converter or transformed-schema failure cannot publish invalid data. Partially published sets report persisted subset, never success-shaped fallback or assumed atomic rollback.

## Provider/Consumer Contract
No new sibling provider prerequisite. Existing implementation reuse is not a dependency on a new sibling outcome.

## Examples
A real v1 Session plan/apply is write-free. Separately, fixture schema A to B uses an injected test converter, faults after staging, restarts from its actual journal and verifies publish/compensation deterministically.

## Evidence
- Position: `partial` existing surface; the new criteria are `not-implemented`. code_ref: [session/crates/session-api/src/model.rs](../../../../session/crates/session-api/src/model.rs).
- Position: `partial` existing surface; the new criteria are `not-implemented`. code_ref: [session/crates/session-api/src/store/helpers/storage.rs](../../../../session/crates/session-api/src/store/helpers/storage.rs).
- Position: `partial` existing surface; the new criteria are `not-implemented`. code_ref: [memory-kernel/src/model/migration.rs](../../../../memory-kernel/src/model/migration.rs).
- Position: `partial` existing surface; the new criteria are `not-implemented`. code_ref: [session/src/cli.rs](../../../../session/src/cli.rs).
- Position: `partial` existing surface; the new criteria are `not-implemented`. code_ref: [session/src/mcp/server.rs](../../../../session/src/mcp/server.rs).
Planned ValidationSpec guard ID: `session-recovery-schema`, not yet registered or executed. Its owning authorized execution package registers and reads back this guard through test-api before recording results; no test entity is created during planning. Approval is not verification and no guard success is claimed.
- `rtk cargo test -p session-api --test schema_migration`
- `rtk cargo test -p session --features cli --lib schema_migration_`
- `rtk cargo test -p session --features mcp --lib schema_migration_`
Commands run only during separately authorized execution, each focused selector once per owning changed revision with nonzero intended cases. Negative cases require exact diagnostic and unchanged-byte assertions. Runtime evidence belongs in the explicit workflow-tools test store and links this spec/owning ticket; a result write is read back by ID.
Traceability: [actual owning ticket](../../../ticket/tickets/86816751-6212-407f-bed3-dfbc16231d1e/ticket.toml).
The current content is a planning contract, not an implemented/verified claim. For reused approved migration/guidance specs, the supported lifecycle has no approved-to-draft edge; body revision preserves the existing state/history and requires the new route reviews before handoff.

## Scope
Production v2, live Session upgrades, track-link backfills or new schema registration.
