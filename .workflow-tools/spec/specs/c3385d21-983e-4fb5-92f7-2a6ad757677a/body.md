<!-- aligned-structure:v2 -->
# Granular Mission Plan Mutation and Coherent Projection

## Target Code Location

- [mission/crates/mission-api/src/plan_operations.rs](../../../../mission/crates/mission-api/src/plan_operations.rs) - `code_ref: mission/crates/mission-api/src/plan_operations.rs`; position: `implemented`. `PlanOperation` (line 25), `ReviewPlan` (107), `EvidenceLedger` (124), `build_review_plan` (318), `MissionLifecycle::review_plan` (460) and `MissionLifecycle::apply_plan_operation` (497).
- [mission/crates/mission-api/src/typed_source.rs](../../../../mission/crates/mission-api/src/typed_source.rs) - `code_ref: mission/crates/mission-api/src/typed_source.rs`; position: `implemented`. `save_typed_locked` (366), `write_authority_with_extras` (597), staged request/ledger/projection writes (737), and `TypedLock::acquire_shared_existing` (964).
- [mission/crates/mission-api/src/output_history.rs](../../../../mission/crates/mission-api/src/output_history.rs) - `code_ref: mission/crates/mission-api/src/output_history.rs`; position: `implemented`. `projected_state` regenerates or removes the review projection for consistent repair fingerprints.
- [mission/crates/mission-api/src/repair.rs](../../../../mission/crates/mission-api/src/repair.rs) - `code_ref: mission/crates/mission-api/src/repair.rs`; position: `implemented`. Rollback fingerprint excludes the derived review plan.
- [mission/crates/mission-api/src/lib.rs](../../../../mission/crates/mission-api/src/lib.rs) - `code_ref: mission/crates/mission-api/src/lib.rs`; position: `implemented`. Module declaration and public re-exports.

Facade/MCP exposure through `mission` is deferred to W6, not an unimplemented
part of this behavior slice.

## Naming Conventions

Stable waypoint_id, expected revision, request_id, plan revision/digest and evidence snapshot identity are distinct; W display numbers are never mutation identifiers. Acceptance criterion identifiers are `MN-W3-NN`.

## Requester Input

> Vertiefe die Planung und erstelle und ordne Tickets und Specs in der Roadmap zu.

The full source scope and earlier explicit lifecycle/trait decisions are preserved in
[merged clean intent](../../../../../transcripts/29-10-2026_mission-native-mcp-planning/inputs/merged.clean.md).

## Reading Order

1. [Composition root](../92c78a48-cad8-4c08-8bba-4a523b717842/body.md) - shared invariants and component map.
2. [Owning Part](../../../../../transcripts/29-10-2026_mission-native-mcp-planning/03-granular-mission-operations.md) - execution package, not a competing behavior authority.
3. [Mission Domain](../38a8ea3a-9a6a-4a0f-bc5a-97dc9ba670fc/body.md) - stable identity, serial order and foreign authority.

- Consumer W3 -> [provider W1](../908e1755-d486-4e49-8b63-3f9eee81396e/body.md) -> MN-W1-02, MN-W1-04.
- Consumer W3 -> [provider W2](../8f53aa05-fb9c-4fec-9458-1eeb55b7342e/body.md) -> MN-W2-01, MN-W2-03, MN-W2-04.
- [Existing provider](../d0a96ede-6f79-4e50-8049-8ecbd5b96c7e/body.md) -> Waypoint outcomes: complete plans, legacy compatibility and advisory-only alternatives. Reuse form 3: link unchanged.
- [Existing provider](../9fd527d8-e290-44e3-9edb-9f7d6658551a/body.md) -> Typed lifecycle: save/publish integrity and safe paths. Reuse form 3: link unchanged.

## Responsibility

Consumers can create, edit, remove and reorder waypoints and update goal,
requirements and evidence through domain operations that generate one coherent
current plan. This behavior slice is implemented in mission-api on its
historical providers.

## Interfaces And Dependencies

- Consumer W3 -> [provider W1](../908e1755-d486-4e49-8b63-3f9eee81396e/body.md) -> MN-W1-02, MN-W1-04.
- Consumer W3 -> [provider W2](../8f53aa05-fb9c-4fec-9458-1eeb55b7342e/body.md) -> MN-W2-01, MN-W2-03, MN-W2-04.
- [Existing provider](../d0a96ede-6f79-4e50-8049-8ecbd5b96c7e/body.md) -> Waypoint outcomes: complete plans, legacy compatibility and advisory-only alternatives. Reuse form 3: link unchanged.
- [Existing provider](../9fd527d8-e290-44e3-9edb-9f7d6658551a/body.md) -> Typed lifecycle: save/publish integrity and safe paths. Reuse form 3: link unchanged.

Use explicit owning roots for each domain operation and same-root read-back.
Dependencies consume provider outcomes/criteria; they do not transfer provider ownership.

## Behavior

- **MN-W3-01**: Every mutation uses stable identities and expected revision; retry uses request_id. Stale revision, invalid identity or failed validation preserves authoritative source and published bytes, with revision_conflict or waypoint_not_found as appropriate.
- **MN-W3-02**: Support waypoint create/update/remove/reorder plus goal, requirements and evidence changes. Removal of referenced unfinished work requires an explicit valid replacement in the same operation; completed/result-bearing waypoints remain retained history.
- **MN-W3-03**: Create new complete native-v2/typed-v3 outcome plans, including expected outcome and alternative handling. Preserve legacy reads without synthesizing absent requester requirements, results or outcomes.
- **MN-W3-04**: Generate one deterministic review plan containing goal, ordered waypoints, requirements, non-goals, references, validation/review status, plan digest and evidence identity. Publish a coherent set or preserve the prior set; do not expose mixed-revision files.
- **MN-W3-05**: Reordering preserves the accepted Mission total-order and immediate-predecessor checkpoint contract. Semantic plan edits advance plan identity; progress/results append evidence bound to a plan identity instead of rewriting its content.

## Boundaries And Failure Cases

No hidden dependency deletion, result erasure, silent legacy downgrade, partial generated publication or W-label identity.
Operator evidence is never requester review: `review_status: "evidenced"`
leaves `formal_review: "not-recorded"`. A coherent read never creates its
lock; missing, symlinked or legacy locks yield explicit `PublicationRecovery`.

## Provider/Consumer Contract

Consumers may rely on the criterion IDs above after the passing implementation
evidence below. Provider ownership remains unchanged.

- Consumer W3 -> [provider W1](../908e1755-d486-4e49-8b63-3f9eee81396e/body.md) -> MN-W1-02, MN-W1-04.
- Consumer W3 -> [provider W2](../8f53aa05-fb9c-4fec-9458-1eeb55b7342e/body.md) -> MN-W2-01, MN-W2-03, MN-W2-04.
- [Existing provider](../d0a96ede-6f79-4e50-8049-8ecbd5b96c7e/body.md) -> Waypoint outcomes. Reuse form 3: link unchanged.
- [Existing provider](../9fd527d8-e290-44e3-9edb-9f7d6658551a/body.md) -> Typed lifecycle. Reuse form 3: link unchanged.

The next consumer links these IDs rather than copying their acceptance into its own authority.

## Examples

Removing W2's stable ID while W3 depends on it fails without changing files.
A valid replacement in the same operation transfers requirement ownership and
preserves serial order; completed/result-bearing identities remain retained.

## Evidence

Motivation: remove manual Mission intake/lifecycle chores while retaining requester control.
Dependent expectation: domain planning operations generate one coherent current plan.

### Guards

- `mission-native-w3-plan-operations-g3`

Command from meta-workspace root:

```sh
cargo test --manifest-path workflow-tools/Cargo.toml --locked -p mission-api -p mission --all-features -j 1
```

Execution `mission-native-w3-g3-20261030`: passed, exit 0, 13816 ms;
291 passed, zero failed, two ignored across 33 suites (sum of captured suite
results). Criterion links cover MN-W3-01..05.

Regression suites: granular_plan_operations (9), granular_plan_coherence (6),
granular_plan_read_lock (4), plan_after_intake (1, actual W2 facade).
Distinct implementation review d415cebe: PASS, no significant issues.
The committed corpus was regenerated with the existing helper after checking
projection-only changes: review plans, inventories and matching event signature.
Checkpoint `12cc8d924f8ee5d4fc5c5b4411071d5674e5ede2`.

Owning [Ticket](../../../ticket/tickets/bb4d6c4c-913d-4f01-b774-a75bd14ccb7d/ticket.toml)
reads done. Execution/read-back evidence is retained in the dossier ledger.

## Scope

Excludes requester approval, execution and final named MCP registration.
Implementation sequence and checkpoints remain in the owning Ticket and
roadmap; this Spec owns observable behavior only.
