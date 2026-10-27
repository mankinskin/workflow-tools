<!-- aligned-structure:v2 -->
# Execute bounded live Mission repair and canonical relocation

## Target Code Location
- [mission/crates/mission-api/src/lib.rs](../../../../mission/crates/mission-api/src/lib.rs)
- [mission/crates/mission-api/src/typed_source.rs](../../../../mission/crates/mission-api/src/typed_source.rs)
- [mission/crates/mission/src/lib.rs](../../../../mission/crates/mission/src/lib.rs)

## Naming Conventions
Criterion prefix: `LIVE`. Public operations and carrier names are the concrete contract in Behavior and the linked execution package.

## Requester Input
> update the ticket and create all planning entities required. continue refinement until all blockers are solved

> The waypoint is wrong. The remidation of the ticket is part of planning work. The Waypoint needs to implement the live migration of missions

## Reading Order
1. [Recovery composition](../9f8d9f34-764f-4821-9b14-b6d8773ec2ae/body.md)
2. [authority provider](../0a8b3ea4-a3ce-4f26-90cc-1ce34474fb6c/body.md)
3. [repair provider](../44f100c4-215e-445a-b452-1a8a969d96cc/body.md)
4. [move provider](../2cd4db32-3987-4d6f-a2dc-774c356b69ad/body.md)
5. [Historical or adjacent contract](../dee92faf-b649-4577-8016-e6e465e6724a/body.md)
6. [W7 execution package](../../../../../transcripts/24-10-2026_mission-typed-authoring/07-migration-remediation.md)
7. [Owning ticket](../../../ticket/tickets/bf7ad7d6-8fab-498e-9114-2d16cd395ff2/ticket.toml)

## Responsibility
Motivation: Execute bounded live Mission repair and canonical relocation closes the observed canonical-recovery gap without annexing adjacent historical work. If this spec is implemented, dependents can rely on every criterion below, not merely a successful command exit.

## Interfaces And Dependencies
Consumes only the directed provider criteria listed below.
Domain APIs own persistence and lifecycle; transports preserve structured outcomes. Explicit entity-store roots remain distinct from command cwd and checkout.

## Behavior
- LIVE-1: Exact selection is five known identities, not every record at execution: meta-workspace 25287294-8211-42e9-b2de-29a4f22be289 and 2579a971-92a4-5332-a65c-0ed9b7f80bf9 repair in place; meta-workspace 6610348f-38c4-55a6-bb94-708849fe2f69 and f304b589-7738-500a-8184-c12c1dac7498 repair/read-back then move to workflow-tools/mission; execution Mission bd23f826-8f9f-4bae-8b75-0288ba1b4899 at workflow-tools reuses approved bootstrap repair/rebind lineage then moves last to workflow-tools/mission.
- LIVE-2: Before writes, preview all selected current roots/revisions/full fingerprints/reference/collision/provenance states and obtain separate exact-set live approval. Conditional move proposals are not apply authority. After each repair read-back obtain fresh real-state move preflight; a changed approved proposal needs renewed approval before transfer.
- LIVE-3: Execute only approved repair/move operations through domain tools, one durable step at a time. Record actual journal IDs and per-operation outcomes, read back same-root identity/revision/metadata/URI/results/prompt/Part/render/history/provenance and verify exact destination ownership before proceeding.
- LIVE-4: Preserve existing immutable histories/snapshots and historical reconciliation. Four materialized-default drifts are recorded, not erased; seven ownership refs and supported Part meaning must be present after forward repair. Never fabricate missing old journals or restore whole trees.
- LIVE-5: Stop on denial, blocked record, stale CAS/fingerprint, partial operation or failed independent review. Inspect actual journals/roots before retry, resume/compensate only operation-owned state, preserve success subset and keep ticket incomplete until all five verified dispositions and exactly three canonical relocations hold.
- LIVE-6: Before moving the executing Mission, complete prior source-root checkpoints, refresh preflight after any native progress/evidence write, and transfer it last. Verify native destination reads at explicit workflow-tools/mission and retarget later native operations; never fall back to old root or create a duplicate.
- LIVE-7: Existing failed review remains byte-identical. Current W7 has its own implementation and independent review evidence; only after real live outcome passes normal review may this original ticket reach done. W8 independently verifies the resulting migration.

## Boundaries And Failure Cases
Approval of this plan alone is not exact-set live authorization. Refusal waits at the gate; partial approval is re-scoped dependency-safely, not misreported as all-five completion. No internal pilot, unrelated record or linked Ticket/Session move is permitted.

## Provider/Consumer Contract
- Consumer mission.typed-authoring.migration -> [mission.canonical-recovery.authority](../0a8b3ea4-a3ce-4f26-90cc-1ce34474fb6c/body.md) -> `AUTH-1`, `AUTH-2`, `AUTH-3`, `AUTH-4`, `AUTH-5`.
- Consumer mission.typed-authoring.migration -> [mission.canonical-recovery.repair](../44f100c4-215e-445a-b452-1a8a969d96cc/body.md) -> `REPAIR-1`, `REPAIR-2`, `REPAIR-3`, `REPAIR-4`, `REPAIR-5`, `REPAIR-6`.
- Consumer mission.typed-authoring.migration -> [mission.canonical-recovery.move](../2cd4db32-3987-4d6f-a2dc-774c356b69ad/body.md) -> `MOVE-1`, `MOVE-2`, `MOVE-3`, `MOVE-4`, `MOVE-5`, `MOVE-6`.

## Examples
After approved repair of 6610348f at the superproject root, read its actual new fingerprints and obtain fresh move preflight. Transfer only that approved identity to workflow-tools/mission and verify source removal/target authority. A collision blocks the set rather than merging a similarly titled record.

## Evidence
- Position: `partial` existing surface; the new criteria are `not-implemented`. code_ref: [mission/crates/mission-api/src/lib.rs](../../../../mission/crates/mission-api/src/lib.rs).
- Position: `partial` existing surface; the new criteria are `not-implemented`. code_ref: [mission/crates/mission-api/src/typed_source.rs](../../../../mission/crates/mission-api/src/typed_source.rs).
- Position: `partial` existing surface; the new criteria are `not-implemented`. code_ref: [mission/crates/mission/src/lib.rs](../../../../mission/crates/mission/src/lib.rs).
Planned ValidationSpec guard ID: `mission-recovery-live-five`, not yet registered or executed. Its owning authorized execution package registers and reads back this guard through test-api before recording results; no test entity is created during planning. Approval is not verification and no guard success is claimed.
- `Exact five-ID repair/move preview, separate approval, actual journal read-back and per-operation same-root native get`
- `Independent review of all five actual dispositions, exactly three relocations and immutable historical evidence`
Commands run only during separately authorized execution, each focused selector once per owning changed revision with nonzero intended cases. Negative cases require exact diagnostic and unchanged-byte assertions. Runtime evidence belongs in the explicit workflow-tools test store and links this spec/owning ticket; a result write is read back by ID.
Traceability: [actual owning ticket](../../../ticket/tickets/bf7ad7d6-8fab-498e-9114-2d16cd395ff2/ticket.toml).
The current content is a planning contract, not an implemented/verified claim. For reused approved migration/guidance specs, the supported lifecycle has no approved-to-draft edge; body revision preserves the existing state/history and requires the new route reviews before handoff.

## Scope
Ticket paperwork as a replacement for migration, all-store/pilot migration, unrelated entities, speculative snapshot restoration or premature ticket closure.
