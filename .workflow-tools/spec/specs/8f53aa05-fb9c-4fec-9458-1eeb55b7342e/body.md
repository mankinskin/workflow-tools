<!-- aligned-structure:v2 -->
# Session Input Intake and Managed Mission Workspace

## Target Code Location

- [Session intake types](../../../../session/crates/session-api/src/session_intake.rs) - `code_ref: session/crates/session-api/src/session_intake.rs`; position: `implemented`.
- [Session intake storage](../../../../session/crates/session-api/src/store/config/session_intake.rs) - `code_ref: session/crates/session-api/src/store/config/session_intake.rs`; position: `implemented`, reusing the existing Session mutation lock without changing capture transcripts.
- [Mission typed creation](../../../../mission/crates/mission-api/src/typed_source.rs) - `code_ref: mission/crates/mission-api/src/typed_source.rs`; position: `implemented`, including journaled identity, bounded lock and intent-authorized interrupted-create recovery.
- [Managed intake facade](../../../../mission/crates/mission/src/intake.rs) - `code_ref: mission/crates/mission/src/intake.rs`; position: `implemented`.

## Naming Conventions

Use a Session-owned immutable submitted-input identifier/digest, caller request_id, API-assigned Mission UUID and returned canonical workspace reference. Acceptance criterion identifiers are `MN-W2-NN`.

## Requester Input

> Vertiefe die Planung und erstelle und ordne Tickets und Specs in der Roadmap zu.

The full source scope and earlier explicit lifecycle/trait decisions are preserved in
[merged clean intent](../../../../../transcripts/29-10-2026_mission-native-mcp-planning/inputs/merged.clean.md).

## Reading Order

1. [Composition root](../92c78a48-cad8-4c08-8bba-4a523b717842/body.md) — shared invariants and component map.
2. [Owning Part](../../../../../transcripts/29-10-2026_mission-native-mcp-planning/02-session-intake-managed-workspace.md) — execution package, not a competing behavior authority.
3. [Mission Domain](../38a8ea3a-9a6a-4a0f-bc5a-97dc9ba670fc/body.md) — stable identity, serial order and foreign authority.

- Consumer W2 -> [provider W1](../908e1755-d486-4e49-8b63-3f9eee81396e/body.md) -> MN-W1-01, MN-W1-02, MN-W1-04.
- [Existing provider](../9f8d9f34-764f-4821-9b14-b6d8773ec2ae/body.md) -> Canonical recovery: preserve explicit roots and immutable recovery evidence; does not provide intake. Reuse form 3: link unchanged.

## Responsibility

Consumers can submit input, create and find exactly one planning Mission workspace from an empty store without manually creating a transcript directory. This new behavior slice is not already guaranteed by its historical providers.

## Interfaces And Dependencies

- Consumer W2 -> [provider W1](../908e1755-d486-4e49-8b63-3f9eee81396e/body.md) -> MN-W1-01, MN-W1-02, MN-W1-04.
- [Existing provider](../9f8d9f34-764f-4821-9b14-b6d8773ec2ae/body.md) -> Canonical recovery: preserve explicit roots and immutable recovery evidence; does not provide intake. Reuse form 3: link unchanged.

Use explicit owning roots for each domain operation and same-root read-back.
Dependencies consume provider outcomes/criteria; they do not transfer provider ownership.

## Behavior

- **MN-W2-01**: Session owns append-only submitted-input bytes and their digest. Mission records the canonical source reference and an immutable provenance copy; no capture transcript is overwritten and a finished Session rejects new intake with session_finished.
- **MN-W2-02**: The same owning roots, Session and request_id with identical bytes resolve to the same Mission and paths across restart. Reusing the request_id with different bytes returns an idempotency conflict without another Mission.
- **MN-W2-03**: API-managed creation returns Mission ID/revision, explicit owning root, UUID directory, canonical input and generated-plan entry paths. Reuse the existing Mission directory layout, separate source from projections and reject traversal, symlink escape and incompatible stores.
- **MN-W2-04**: Journal Session-to-Mission linkage before exposing success. Faults before/after Mission creation or Session linkage return a pending/error receipt with the persisted subset; retry resumes that operation without orphan adoption or duplicate creation.
- **MN-W2-05**: Distinguish intake_missing, session_finished and workspace_unavailable from not-found. Planning Mission creation is permitted before execution approval but grants no executable state.

## Boundaries And Failure Cases

No finished-Session mutation, source normalization, manually authored path fallback or success-shaped partial linkage. No product implementation or evidence of passing behavior is asserted by approval of this contract.

## Provider/Consumer Contract

The consumer may rely on the criterion IDs above only after their implementation evidence passes.

- Consumer W2 -> [provider W1](../908e1755-d486-4e49-8b63-3f9eee81396e/body.md) -> MN-W1-01, MN-W1-02, MN-W1-04.
- [Existing provider](../9f8d9f34-764f-4821-9b14-b6d8773ec2ae/body.md) -> Canonical recovery: preserve explicit roots and immutable recovery evidence; does not provide intake. Reuse form 3: link unchanged.

The next consumer links these IDs rather than copying their acceptance into its own authority.

## Examples

Request r-42 submits bytes once, returns Mission m and its UUID directory. Restart and replay r-42 return m; different bytes conflict. A crash after Mission creation resumes the linkage and never creates m2.

## Evidence

Motivation: remove manual Mission intake/lifecycle chores while retaining requester control.
Dependent expectation: Consumers can submit input, create and find exactly one planning Mission workspace from an empty store without manually creating a transcript directory.

Registered guard: **G2 intake-workspace**, from
[ROADMAP validation gates](../../../../../transcripts/29-10-2026_mission-native-mcp-planning/ROADMAP.md#validation-gates).
Command from meta-workspace root:

```sh
cargo test --manifest-path workflow-tools/Cargo.toml --locked -p session-api -p mission-api -p mission --all-features -j 1
```

ValidationSpec `mission-native-w2-session-intake-g2` links this Spec, the
owning Ticket and MN-W2-01 through MN-W2-05. Exact execution
`mission-native-w2-g2-fixture-repair-20261030` passed with exit 0:
590 passed, 0 failed, 3 ignored across 37 suites. Capture:
`mission-native-w2-g2-fixture-repair-20261030-log`.
Prior failed runs are preserved, not waived. Eight baseline handoff failures
were resolved by the separately requester-approved testfixture-only repair.

Focused evidence covers immutable bytes/digests, transcript preservation,
restart/conflict, safe paths, finish synchronization, interrupted linkage,
bounded crash-released locks and exact-request create intent.
The finish-marker review correction has regression-first red/green evidence.
See [execution results](../../../../../transcripts/29-10-2026_mission-native-mcp-planning/RESULTS.md)
and the owning [Ticket](../../../ticket/tickets/55c99751-2801-42ef-bc29-7e3d01d6f4ca/ticket.toml).
Checkpoint and lifecycle read-back are recorded there, not inferred from approval.

### Guards

- `mission-native-w2-session-intake-g2`

## Scope

Excludes: Executing the created Mission, migrating old archives or changing worktree assignments. Implementation sequence and commit checkpoints remain in the
owning Ticket and roadmap; this Spec owns observable behavior only.
