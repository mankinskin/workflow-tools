<!-- aligned-structure:v2 -->
# Journaled forward Mission repair with durable recovery

## Target Code Location
- [mission/crates/mission-api/src/repair.rs](../../../../mission/crates/mission-api/src/repair.rs)
- [mission/crates/mission-api/src/lib.rs](../../../../mission/crates/mission-api/src/lib.rs)
- [mission/crates/mission-api/src/typed_source.rs](../../../../mission/crates/mission-api/src/typed_source.rs)
- [mission/crates/mission/src/lib.rs](../../../../mission/crates/mission/src/lib.rs)
- [mission/crates/mission/src/bin/mission.rs](../../../../mission/crates/mission/src/bin/mission.rs)
- [mission/crates/mission/src/bin/mission-mcp.rs](../../../../mission/crates/mission/src/bin/mission-mcp.rs)
- [mission/crates/mission/tests/recovery_move_contract.rs](../../../../mission/crates/mission/tests/recovery_move_contract.rs)

## Naming Conventions
Criterion prefix: `REPAIR`. Public operations and carrier names are the concrete contract in Behavior and the linked execution package.

## Requester Input
> update the ticket and create all planning entities required. continue refinement until all blockers are solved


## Reading Order
1. [Recovery composition](../9f8d9f34-764f-4821-9b14-b6d8773ec2ae/body.md)
2. [authority provider](../0a8b3ea4-a3ce-4f26-90cc-1ce34474fb6c/body.md)
3. [workspace provider](../eb4444fd-1349-471f-b911-26b8c7cdf5d0/body.md)
4. [Historical or adjacent contract](../53e78fcb-df6f-4d74-91fa-163dbcb0e1a8/body.md)
5. [W3 execution package](../../../../../transcripts/24-10-2026_mission-typed-authoring/03-adapters.md)
6. [Owning ticket](../../../ticket/tickets/0e928c01-31c8-4f6b-8992-7f8cdb228431/ticket.toml)

## Responsibility
Motivation: Journaled forward Mission repair with durable recovery closes the observed canonical-recovery gap without annexing adjacent historical work. If this spec is implemented, dependents can rely on every criterion below, not merely a successful command exit.

## Interfaces And Dependencies
Consumes only the directed provider criteria listed below.
Domain APIs own persistence and lifecycle; transports preserve structured outcomes. Explicit entity-store roots remain distinct from command cwd and checkout.

## Behavior
- REPAIR-1: API owns repair_preview/apply/journal/resume/rollback; thin CLI kebab-case commands and existing MCP mission_operation variants preserve the same diagnostics and results.
- REPAIR-2: Preview emits per-ID readiness/blockers, full-payload-bound candidate_overlay and deterministic planned identity, with no record, directory, journal or snapshot writes.
- REPAIR-3: Full-payload fingerprint covers sorted relative filenames and raw hashes for accepted/source, referenced Parts, history, projections and provenance. Apply accepts/verifies this fingerprint and CAS under the operation lock, excluding only explicitly operation-owned staging/journals.
- REPAIR-4: Apply persists actual durable journal and recovery snapshot before mutation, appends a forward authoritative revision, preserves historical bytes and merges accepted lifecycle/ownership with current typed prompt/Part meaning.
- REPAIR-5: Explicit no-op allocates no journal/revision. Partial set failure stops later IDs and reports persisted subset. Same-root journal read/resume/rollback use actual ID, revalidate last durable step and refuse corrupt/missing journals or later unrelated edits.
- REPAIR-6: Rollback is history-preserving operation-owned compensation, never whole-tree restoration or a synthetic historical journal. Fixtures prove same-identity repair followed by safe corrected typed rebind/publish.

## Boundaries And Failure Cases
Stale Parts/history/projection must reject even when accepted/source JSON hashes are unchanged. Interrupted publish resumes only the same durable operation; uncertainty preserves evidence rather than replaying writes.

## Provider/Consumer Contract
- Consumer mission.canonical-recovery.repair -> [mission.canonical-recovery.authority](../0a8b3ea4-a3ce-4f26-90cc-1ce34474fb6c/body.md) -> `AUTH-1`, `AUTH-2`, `AUTH-3`, `AUTH-4`, `AUTH-5`.
- Consumer mission.canonical-recovery.repair -> [mission.canonical-recovery.workspace](../eb4444fd-1349-471f-b911-26b8c7cdf5d0/body.md) -> `ROOT-1`, `ROOT-2`, `ROOT-3`, `ROOT-4`.

## Examples
Preview selects revision N and all payload hashes. An intervening Part edit invalidates apply. A fresh approved apply records journal J before N+1; restart reads J and resumes or compensates only matching operation-owned state.

## Guards
- ValidationSpec: `mission-recovery-repair`

## Evidence
- Position: `implemented`; repair planning, fingerprints, durable journals, apply, resume and rollback are in [repair.rs](../../../../mission/crates/mission-api/src/repair.rs), exported through [lib.rs](../../../../mission/crates/mission-api/src/lib.rs).
- Position: `implemented`; typed-source validation and merge behavior used by repair are in [typed_source.rs](../../../../mission/crates/mission-api/src/typed_source.rs).
- Position: `implemented`; operation routing and the thin CLI/MCP surfaces are in [mission/src/lib.rs](../../../../mission/crates/mission/src/lib.rs), [mission.rs](../../../../mission/crates/mission/src/bin/mission.rs), and [mission-mcp.rs](../../../../mission/crates/mission/src/bin/mission-mcp.rs).
- Position: `implemented`; fixture coverage for CLI and initialized stdio MCP is in [recovery_move_contract.rs](../../../../mission/crates/mission/tests/recovery_move_contract.rs).
ValidationSpec `mission-recovery-repair` is registered in the workflow-tools test store and links this specification and its owning ticket. The latest focused execution `ae41c0ca-75fa-497e-b4ac-90e3fa04f6ec-mission-recovery-repair-focused-03` passed: the partial-batch and no-write rollback API regressions passed (1 each), and all eight repair transport contract tests passed.
- `rtk cargo test --manifest-path workflow-tools/Cargo.toml -p mission-api repair_apply_stops_after_partial_failure_and_resume_finishes_remaining_records`
- `rtk cargo test --manifest-path workflow-tools/Cargo.toml -p mission-api repair_rollback_records_an_unchanged_failed_record_as_durable_compensation`
- `rtk cargo test --manifest-path workflow-tools/Cargo.toml -p mission --features cli,mcp --test recovery_move_contract repair_`
Execution `ae41c0ca-75fa-497e-b4ac-90e3fa04f6ec-mission-recovery-repair-focused-03` is recorded in the explicit workflow-tools test store with the specification and owning ticket links. The test-domain recomputation subsequently succeeded and set this specification to `verified`.
Traceability: [actual owning ticket](../../../ticket/tickets/0e928c01-31c8-4f6b-8992-7f8cdb228431/ticket.toml).
This document remains the normative behavior contract; implementation status and broader-suite blockers are tracked on the owning ticket.

## Scope
Entity relocation, live five-record application during provider implementation, or restoring old snapshots over new edits.

## Bootstrap preservation criterion

REPAIR-7: Fixture-prove the exact twelve-waypoint same-identity bootstrap mapping, independent old/new completion evidence and safe repair/read-back/CAS rebind/publish. Apply consumes matching current full-payload fingerprints and actual preview digest; CLI --records/--proposal-id/--journal-id and equivalent existing MCP operation fields reject root mismatch or implicit proposal replacement.
