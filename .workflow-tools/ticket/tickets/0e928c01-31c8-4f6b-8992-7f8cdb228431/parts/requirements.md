# W3: Journaled forward Mission repair with durable recovery

- REPAIR-1: API owns repair_preview/apply/journal/resume/rollback; thin CLI kebab-case commands and existing MCP mission_operation variants preserve the same diagnostics and results.
- REPAIR-2: Preview emits per-ID readiness/blockers, full-payload-bound candidate_overlay and deterministic planned identity, with no record, directory, journal or snapshot writes.
- REPAIR-3: Full-payload fingerprint covers sorted relative filenames and raw hashes for accepted/source, referenced Parts, history, projections and provenance. Apply accepts/verifies this fingerprint and CAS under the operation lock, excluding only explicitly operation-owned staging/journals.
- REPAIR-4: Apply persists actual durable journal and recovery snapshot before mutation, appends a forward authoritative revision, preserves historical bytes and merges accepted lifecycle/ownership with current typed prompt/Part meaning.
- REPAIR-5: Explicit no-op allocates no journal/revision. Partial set failure stops later IDs and reports persisted subset. Same-root journal read/resume/rollback use actual ID, revalidate last durable step and refuse corrupt/missing journals or later unrelated edits.
- REPAIR-6: Rollback is history-preserving operation-owned compensation, never whole-tree restoration or a synthetic historical journal. Fixtures prove same-identity repair followed by safe corrected typed rebind/publish.

## Failure and continuation
Stale Parts/history/projection must reject even when accepted/source JSON hashes are unchanged. Interrupted publish resumes only the same durable operation; uncertainty preserves evidence rather than replaying writes.
Apply common and per-waypoint branches from [outcome contract](../../../../../../transcripts/24-10-2026_mission-typed-authoring/OUTCOMES.md); after native binding that content must be embedded in the accepted Mission, not require this dossier at runtime.

## Validation and checkpoint
- `rtk cargo test -p mission --features cli,mcp --test recovery_move_contract repair_`
Focused selectors run once per owning changed revision, nonzero cases mandatory; no Python, active-denial machinery or unchanged broad final reruns. Planned guard `mission-recovery-repair` is registered/read back as execution evidence, never claimed run in planning.

## Scope and authority
Entity relocation, live five-record application during provider implementation, or restoring old snapshots over new edits.
Planning correction is not execution. Do not mutate live Mission/Session entities until the applicable execution and exact-set approval gates pass.
[Owning specification](../../../../spec/specs/44f100c4-215e-445a-b452-1a8a969d96cc/body.md)
[Detailed execution package](../../../../../../transcripts/24-10-2026_mission-typed-authoring/03-adapters.md)

## Bootstrap preservation criterion

REPAIR-7: Fixture-prove the exact twelve-waypoint same-identity bootstrap mapping, independent old/new completion evidence and safe repair/read-back/CAS rebind/publish. Apply consumes matching current full-payload fingerprints and actual preview digest; CLI --records/--proposal-id/--journal-id and equivalent existing MCP operation fields reject root mismatch or implicit proposal replacement.
