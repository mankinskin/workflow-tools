# W1: Correct persisted typed Mission lifecycle authority

- AUTH-1: All persisted typed and compatible legacy mutation routes share one revision/CAS/lifecycle authority owned by mission-api; transports cannot reconstruct or maintain a second accepted source.
- AUTH-2: Preserve accepted metadata, external ownership URIs, stable waypoint identities, lifecycle/progress/results, immutable existing history and snapshot bytes, and independent typed prompt/Part content.
- AUTH-3: Valid incomplete drafts save; publication requires nonblank validation commands and commit checkpoints, serial order and structurally valid independent Parts. Unsupported content rejects with exact field/location diagnostics without loss or opaque fallback.
- AUTH-4: Legacy-after-typed and typed-after-legacy mutations, stale CAS and injected failure leave a consistent authority. Corrected save/publish can rebind the existing execution identity without losing old completed IDs/results.
- AUTH-5: Rendered Part semantics preserve supported headings/lists/code/tables/hard breaks; semantic equivalence is tested, not assumed from different encodings.

## Failure and continuation
Malformed or irreconcilable content rejects before mutation. Stale CAS and stage/publish failures preserve historical bytes and surface diagnostics; repair capability is not simulated by legacy publication.
Apply common and per-waypoint branches from [outcome contract](../../../../../../transcripts/24-10-2026_mission-typed-authoring/OUTCOMES.md); after native binding that content must be embedded in the accepted Mission, not require this dossier at runtime.

## Validation and checkpoint
- `rtk cargo test -p mission-api --test typed_source`
- `rtk cargo test -p mission --test typed_migration`
Focused selectors run once per owning changed revision, nonzero cases mandatory; no Python, active-denial machinery or unchanged broad final reruns. Planned guard `mission-recovery-authority` is registered/read back as execution evidence, never claimed run in planning.

## Scope and authority
Root selection, live record repair/move and Session changes.
Planning correction is not execution. Do not mutate live Mission/Session entities until the applicable execution and exact-set approval gates pass.
[Owning specification](../../../../spec/specs/0a8b3ea4-a3ce-4f26-90cc-1ce34474fb6c/body.md)
[Detailed execution package](../../../../../../transcripts/24-10-2026_mission-typed-authoring/01-ownership-refactor.md)

## Bootstrap preservation criterion

AUTH-6: Rebind retains old ownership-refactor/typed-persistence/adapters completed IDs/results; adds recovery-authority/mission-root-resolution/mission-repair/mission-move/session-set-transports/session-schema-infrastructure/migration-verification; reuses migration/guidance-evidence. Native total order is twelve, with retained old three before recovery nine, immediate predecessors and collision rejection.
