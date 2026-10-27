<!-- aligned-structure:v2 -->
# Shared-kernel Mission set relocation and recovery

## Target Code Location
- [mission/crates/mission-api/src/lib.rs](../../../../mission/crates/mission-api/src/lib.rs)
- [mission/crates/mission/src/lib.rs](../../../../mission/crates/mission/src/lib.rs)
- [session/crates/session-api/src/move_domain.rs](../../../../session/crates/session-api/src/move_domain.rs)

## Naming Conventions
Criterion prefix: `MOVE`. Public operations and carrier names are the concrete contract in Behavior and the linked execution package.

## Requester Input
> update the ticket and create all planning entities required. continue refinement until all blockers are solved


## Reading Order
1. [Recovery composition](../9f8d9f34-764f-4821-9b14-b6d8773ec2ae/body.md)
2. [repair provider](../44f100c4-215e-445a-b452-1a8a969d96cc/body.md)
3. [Historical or adjacent contract](../7487a6b5-39c6-4114-a697-6ed0556e888e/body.md)
4. [W4 execution package](../../../../../transcripts/24-10-2026_mission-typed-authoring/04-migration.md)
5. [Owning ticket](../../../ticket/tickets/ad664d30-d0b2-4c47-9120-ec4ed794bab9/ticket.toml)

## Responsibility
Motivation: Shared-kernel Mission set relocation and recovery closes the observed canonical-recovery gap without annexing adjacent historical work. If this spec is implemented, dependents can rely on every criterion below, not merely a successful command exit.

## Interfaces And Dependencies
Consumes only the directed provider criteria listed below.
Domain APIs own persistence and lifecycle; transports preserve structured outcomes. Explicit entity-store roots remain distinct from command cwd and checkout.

## Behavior
- MOVE-1: Mission API integrates the shared entity-move kernel, exposing move_preview/apply/journal/resume/rollback through the same thin CLI/MCP operation dispatch, not a new transport-owned engine.
- MOVE-2: Full source payload/identity/revision/fingerprint and destination identity/collision/containment/reference-base preconditions are verified under lock before transfer. Missing destination preview validates safe future initialization but creates nothing.
- MOVE-3: Move preserves UUID, accepted/source/Parts/history/projection and operation provenance. External Ticket/spec owners are not relocated; ambiguous ce://default or relative references block rather than silently rebind.
- MOVE-4: Historical snapshots remain at original paths with durable resolvable provenance. Complete record transfer is recoverable, no UUID collision merge/deletion or duplicate authority.
- MOVE-5: W3 overlay is verified/recomputed against current inputs and supports conditional preview only. Actual move needs successful repair read-back plus fresh real-state preflight; no predicted revision is live authorization.
- MOVE-6: Journal-first apply, explicit no-op, partial failure, same-operation restart/resume and operation-owned compensation are fixture-proven across API/CLI/initialized MCP, including later-edit refusal.

## Boundaries And Failure Cases
Reject source/destination collisions, unsafe topology and unresolved reference owners. Partial publish/delete resolves actual source/target ownership from durable kernel journal before any retry.

## Provider/Consumer Contract
- Consumer mission.canonical-recovery.move -> [mission.canonical-recovery.repair](../44f100c4-215e-445a-b452-1a8a969d96cc/body.md) -> `REPAIR-1`, `REPAIR-2`, `REPAIR-3`, `REPAIR-4`, `REPAIR-5`, `REPAIR-6`.

## Examples
A repaired Mission in the parent store moves with the same UUID into the canonical Mission store; its external Ticket stays in the parent store, and the original snapshot remains resolvable at its recorded provenance path.

## Evidence
- Position: `partial` existing surface; the new criteria are `not-implemented`. code_ref: [mission/crates/mission-api/src/lib.rs](../../../../mission/crates/mission-api/src/lib.rs).
- Position: `partial` existing surface; the new criteria are `not-implemented`. code_ref: [mission/crates/mission/src/lib.rs](../../../../mission/crates/mission/src/lib.rs).
- Position: `partial` existing surface; the new criteria are `not-implemented`. code_ref: [session/crates/session-api/src/move_domain.rs](../../../../session/crates/session-api/src/move_domain.rs).
Planned ValidationSpec guard ID: `mission-recovery-move`, not yet registered or executed. Its owning authorized execution package registers and reads back this guard through test-api before recording results; no test entity is created during planning. Approval is not verification and no guard success is claimed.
- `rtk cargo test -p mission --features cli,mcp --test recovery_move_contract move_`
Commands run only during separately authorized execution, each focused selector once per owning changed revision with nonzero intended cases. Negative cases require exact diagnostic and unchanged-byte assertions. Runtime evidence belongs in the explicit workflow-tools test store and links this spec/owning ticket; a result write is read back by ID.
Traceability: [actual owning ticket](../../../ticket/tickets/ad664d30-d0b2-4c47-9120-ec4ed794bab9/ticket.toml).
The current content is a planning contract, not an implemented/verified claim. For reused approved migration/guidance specs, the supported lifecycle has no approved-to-draft edge; body revision preserves the existing state/history and requires the new route reviews before handoff.

## Scope
A second conversion policy, moving linked entities, or live selected-record application while implementing the provider.
