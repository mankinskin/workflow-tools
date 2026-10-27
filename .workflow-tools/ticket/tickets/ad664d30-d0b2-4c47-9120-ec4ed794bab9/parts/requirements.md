# W4: Shared-kernel Mission set relocation and recovery

- MOVE-1: Mission API integrates the shared entity-move kernel, exposing move_preview/apply/journal/resume/rollback through the same thin CLI/MCP operation dispatch, not a new transport-owned engine.
- MOVE-2: Full source payload/identity/revision/fingerprint and destination identity/collision/containment/reference-base preconditions are verified under lock before transfer. Missing destination preview validates safe future initialization but creates nothing.
- MOVE-3: Move preserves UUID, accepted/source/Parts/history/projection and operation provenance. External Ticket/spec owners are not relocated; ambiguous ce://default or relative references block rather than silently rebind.
- MOVE-4: Historical snapshots remain at original paths with durable resolvable provenance. Complete record transfer is recoverable, no UUID collision merge/deletion or duplicate authority.
- MOVE-5: W3 overlay is verified/recomputed against current inputs and supports conditional preview only. Actual move needs successful repair read-back plus fresh real-state preflight; no predicted revision is live authorization.
- MOVE-6: Journal-first apply, explicit no-op, partial failure, same-operation restart/resume and operation-owned compensation are fixture-proven across API/CLI/initialized MCP, including later-edit refusal.

## Failure and continuation
Reject source/destination collisions, unsafe topology and unresolved reference owners. Partial publish/delete resolves actual source/target ownership from durable kernel journal before any retry.
Apply common and per-waypoint branches from [outcome contract](../../../../../../transcripts/24-10-2026_mission-typed-authoring/OUTCOMES.md); after native binding that content must be embedded in the accepted Mission, not require this dossier at runtime.

## Validation and checkpoint
- `rtk cargo test -p mission --features cli,mcp --test recovery_move_contract move_`
Focused selectors run once per owning changed revision, nonzero cases mandatory; no Python, active-denial machinery or unchanged broad final reruns. Planned guard `mission-recovery-move` is registered/read back as execution evidence, never claimed run in planning.

## Scope and authority
A second conversion policy, moving linked entities, or live selected-record application while implementing the provider.
Planning correction is not execution. Do not mutate live Mission/Session entities until the applicable execution and exact-set approval gates pass.
[Owning specification](../../../../spec/specs/2cd4db32-3987-4d6f-a2dc-774c356b69ad/body.md)
[Detailed execution package](../../../../../../transcripts/24-10-2026_mission-typed-authoring/04-migration.md)
