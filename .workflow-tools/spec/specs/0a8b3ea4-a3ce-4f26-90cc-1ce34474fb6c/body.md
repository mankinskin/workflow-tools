<!-- aligned-structure:v2 -->
# Correct persisted typed Mission lifecycle authority

## Target Code Location
- [mission/crates/mission-api/src/typed_source.rs](../../../../mission/crates/mission-api/src/typed_source.rs)
- [mission/crates/mission-api/src/typed_migration.rs](../../../../mission/crates/mission-api/src/typed_migration.rs)
- [mission/crates/mission-api/src/lib.rs](../../../../mission/crates/mission-api/src/lib.rs)
- [mission/crates/mission/src/lib.rs](../../../../mission/crates/mission/src/lib.rs)

## Naming Conventions
Criterion prefix: `AUTH`. Public operations and carrier names are the concrete contract in Behavior and the linked execution package.

## Requester Input
> update the ticket and create all planning entities required. continue refinement until all blockers are solved


## Reading Order
1. [Recovery composition](../9f8d9f34-764f-4821-9b14-b6d8773ec2ae/body.md)
2. [Historical or adjacent contract](../9fd527d8-e290-44e3-9edb-9f7d6658551a/body.md)
3. [Historical or adjacent contract](../9413a0a2-0d53-407d-b088-3953074393fb/body.md)
4. [W1 execution package](../../../../../transcripts/24-10-2026_mission-typed-authoring/01-ownership-refactor.md)
5. [Owning ticket](../../../ticket/tickets/f60ee370-09a7-41d9-9d4a-6b97145f0e18/ticket.toml)

## Responsibility
Motivation: Correct persisted typed Mission lifecycle authority closes the observed canonical-recovery gap without annexing adjacent historical work. If this spec is implemented, dependents can rely on every criterion below, not merely a successful command exit.

## Interfaces And Dependencies
Uses existing domain/shared-kernel infrastructure; no dependency on unrelated Mission/Session work is fabricated.
Domain APIs own persistence and lifecycle; transports preserve structured outcomes. Explicit entity-store roots remain distinct from command cwd and checkout.

## Behavior
- AUTH-1: All persisted typed and compatible legacy mutation routes share one revision/CAS/lifecycle authority owned by mission-api; transports cannot reconstruct or maintain a second accepted source.
- AUTH-2: Preserve accepted metadata, external ownership URIs, stable waypoint identities, lifecycle/progress/results, immutable existing history and snapshot bytes, and independent typed prompt/Part content.
- AUTH-3: Valid incomplete drafts save; publication requires nonblank validation commands and commit checkpoints, serial order and structurally valid independent Parts. Unsupported content rejects with exact field/location diagnostics without loss or opaque fallback.
- AUTH-4: Legacy-after-typed and typed-after-legacy mutations, stale CAS and injected failure leave a consistent authority. Corrected save/publish can rebind the existing execution identity without losing old completed IDs/results.
- AUTH-5: Rendered Part semantics preserve supported headings/lists/code/tables/hard breaks; semantic equivalence is tested, not assumed from different encodings.

## Boundaries And Failure Cases
Malformed or irreconcilable content rejects before mutation. Stale CAS and stage/publish failures preserve historical bytes and surface diagnostics; repair capability is not simulated by legacy publication.

## Provider/Consumer Contract
No new sibling provider prerequisite. Existing implementation reuse is not a dependency on a new sibling outcome.

## Examples
After typed conversion at revision N, a compatible legacy result update and a typed save both advance the same authority; a second save using stale N rejects without adding history or replacing the generated projection.

## Evidence
- Position: `partial` existing surface; the new criteria are `not-implemented`. code_ref: [mission/crates/mission-api/src/typed_source.rs](../../../../mission/crates/mission-api/src/typed_source.rs).
- Position: `partial` existing surface; the new criteria are `not-implemented`. code_ref: [mission/crates/mission-api/src/typed_migration.rs](../../../../mission/crates/mission-api/src/typed_migration.rs).
- Position: `partial` existing surface; the new criteria are `not-implemented`. code_ref: [mission/crates/mission-api/src/lib.rs](../../../../mission/crates/mission-api/src/lib.rs).
- Position: `partial` existing surface; the new criteria are `not-implemented`. code_ref: [mission/crates/mission/src/lib.rs](../../../../mission/crates/mission/src/lib.rs).
Planned ValidationSpec guard ID: `mission-recovery-authority`, not yet registered or executed. Its owning authorized execution package registers and reads back this guard through test-api before recording results; no test entity is created during planning. Approval is not verification and no guard success is claimed.
- `rtk cargo test -p mission-api --test typed_source`
- `rtk cargo test -p mission --test typed_migration`
Commands run only during separately authorized execution, each focused selector once per owning changed revision with nonzero intended cases. Negative cases require exact diagnostic and unchanged-byte assertions. Runtime evidence belongs in the explicit workflow-tools test store and links this spec/owning ticket; a result write is read back by ID.
Traceability: [actual owning ticket](../../../ticket/tickets/f60ee370-09a7-41d9-9d4a-6b97145f0e18/ticket.toml).
The current content is a planning contract, not an implemented/verified claim. For reused approved migration/guidance specs, the supported lifecycle has no approved-to-draft edge; body revision preserves the existing state/history and requires the new route reviews before handoff.

## Scope
Root selection, live record repair/move and Session changes.

## Bootstrap preservation criterion

AUTH-6: Rebind retains old ownership-refactor/typed-persistence/adapters completed IDs/results; adds recovery-authority/mission-root-resolution/mission-repair/mission-move/session-set-transports/session-schema-infrastructure/migration-verification; reuses migration/guidance-evidence. Native total order is twelve, with retained old three before recovery nine, immediate predecessors and collision rejection.
