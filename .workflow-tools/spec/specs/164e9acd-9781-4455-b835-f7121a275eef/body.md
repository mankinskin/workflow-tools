<!-- aligned-structure:v2 -->
# Mission Lifecycle, Session Handoff and Audited Ticket Reopen

## Target Code Location

- [mission/crates/mission-api/src/lib.rs](../../../../mission/crates/mission-api/src/lib.rs) — `code_ref: mission/crates/mission-api/src/lib.rs`; position: `not-implemented` for this new contract (existing provider behavior is not reclassified).
- [mission/crates/mission/src/lib.rs](../../../../mission/crates/mission/src/lib.rs) — `code_ref: mission/crates/mission/src/lib.rs`; position: `not-implemented` for this new contract (existing provider behavior is not reclassified).
- [ticket/crates/ticket-api/src/lib.rs](../../../../ticket/crates/ticket-api/src/lib.rs) — `code_ref: ticket/crates/ticket-api/src/lib.rs`; position: `not-implemented` for this new contract (existing provider behavior is not reclassified).
- [ticket/crates/ticket-api/schemas/feature.toml](../../../../ticket/crates/ticket-api/schemas/feature.toml) — `code_ref: ticket/crates/ticket-api/schemas/feature.toml`; position: `not-implemented` for this new contract (existing provider behavior is not reclassified).
- [session/crates/session-api/src/store/config/runtime_workflow.rs](../../../../session/crates/session-api/src/store/config/runtime_workflow.rs) — `code_ref: session/crates/session-api/src/store/config/runtime_workflow.rs`; position: `not-implemented` for this new contract (existing provider behavior is not reclassified).

## Naming Conventions

Completion sequence is durable event order, not execution position; reopen_most_recent_completed consumes expected revision and current completion identity. Acceptance criterion identifiers are `MN-W5-NN`.

## Requester Input

> Vertiefe die Planung und erstelle und ordne Tickets und Specs in der Roadmap zu.

The full source scope and earlier explicit lifecycle/trait decisions are preserved in
[merged clean intent](../../../../../transcripts/29-10-2026_mission-native-mcp-planning/inputs/merged.clean.md).

## Reading Order

1. [Composition root](../92c78a48-cad8-4c08-8bba-4a523b717842/body.md) — shared invariants and component map.
2. [Owning Part](../../../../../transcripts/29-10-2026_mission-native-mcp-planning/05-lifecycle-handoff-and-reopen.md) — execution package, not a competing behavior authority.
3. [Mission Domain](../38a8ea3a-9a6a-4a0f-bc5a-97dc9ba670fc/body.md) — stable identity, serial order and foreign authority.

- Consumer W5 -> [provider W1](../908e1755-d486-4e49-8b63-3f9eee81396e/body.md) -> MN-W1-01, MN-W1-02, MN-W1-03, MN-W1-04.
- Consumer W5 -> [provider W3](../c3385d21-983e-4fb5-92f7-2a6ad757677a/body.md) -> MN-W3-01, MN-W3-02, MN-W3-05.
- Consumer W5 -> [provider W4](../9fe60c69-af53-4578-9310-69283cd09284/body.md) -> MN-W4-03, MN-W4-04, MN-W4-05.
- [Existing provider](../38a8ea3a-9a6a-4a0f-bc5a-97dc9ba670fc/body.md) -> Mission Domain: linked Ticket authority and serial prerequisite evidence. Reuse form 3: link unchanged.

## Responsibility

Consumers can pause/resume, hand off, cancel, complete, review results/add work and reopen the latest current completion while retaining history and Ticket authority. This new behavior slice is not already guaranteed by its historical providers.

## Interfaces And Dependencies

- Consumer W5 -> [provider W1](../908e1755-d486-4e49-8b63-3f9eee81396e/body.md) -> MN-W1-01, MN-W1-02, MN-W1-03, MN-W1-04.
- Consumer W5 -> [provider W3](../c3385d21-983e-4fb5-92f7-2a6ad757677a/body.md) -> MN-W3-01, MN-W3-02, MN-W3-05.
- Consumer W5 -> [provider W4](../9fe60c69-af53-4578-9310-69283cd09284/body.md) -> MN-W4-03, MN-W4-04, MN-W4-05.
- [Existing provider](../38a8ea3a-9a6a-4a0f-bc5a-97dc9ba670fc/body.md) -> Mission Domain: linked Ticket authority and serial prerequisite evidence. Reuse form 3: link unchanged.

Use explicit owning roots for each domain operation and same-root read-back.
Dependencies consume provider outcomes/criteria; they do not transfer provider ownership.

## Behavior

- **MN-W5-01**: Implement the W1 version-1 Mission lifecycle/control journal under the UUID workspace with locking/CAS, durable operation identities, restart recovery and explicit corrupt-version errors. planned->active requires current approval; active->paused/blocked records reason; resume restores the prior nonterminal state only with current approval, prerequisites and no pending cross-store operation. Cancel remains distinct from blocked; complete appends explicit validated completion after all required outcomes/result review. Mission eligibility is separate from unchanged linked Ticket state; do not mutate legacy plan manifests to record controls.
- **MN-W5-02**: Handoff transfers active Session binding with source/destination Session checks and durable read-back. It never silently transfers worktrees or Ticket ownership. Mission pause/cancel does not blindly cascade linked Ticket transitions.
- **MN-W5-03**: Append immutable completion events with waypoint ID, monotonically ordered completion sequence, plan identity and result reference. Reopen chooses the greatest still-current completion sequence of an eligible completed waypoint; empty selection returns no_completed_waypoint and already reopened events are ineligible.
- **MN-W5-04**: Reopen preserves immutable results, makes the waypoint planned, invalidates plan approval and marks descendant completion evidence stale for the revised route. Revised descendants cannot execute until current approval and prerequisite evidence are valid.
- **MN-W5-05**: For linked done Tickets, add/use an explicit audited Ticket-domain done->ready reopen with expected state/revision, reason and operation identity; retain prior review/completion history. Unsupported/cancelled or stale transitions return ticket_transition_rejected without pretending the Ticket reopened. Successful happy-path Ticket reopening is mandatory.
- **MN-W5-06**: Journal Mission/Ticket transitions and preconditions. During partial reopen all affected execution is gated; retry resumes the recorded step and reads both stores back. Inject failure before Ticket transition, after Ticket transition and before Mission finalization; never erase history to simulate rollback.
- **MN-W5-07**: Result review may propose added waypoints only through W3 semantic plan changes and W4 reapproval. Concurrent completion/reopen uses CAS so exactly one current completion is selected and stale competitors fail.

## Boundaries And Failure Cases

No final-order selection, raw terminal Ticket edits, automatic cancellation cascade, cross-store atomicity claim or historical event deletion. No product implementation or evidence of passing behavior is asserted by approval of this contract.

## Provider/Consumer Contract

The consumer may rely on the criterion IDs above only after their implementation evidence passes.

- Consumer W5 -> [provider W1](../908e1755-d486-4e49-8b63-3f9eee81396e/body.md) -> MN-W1-01, MN-W1-02, MN-W1-03, MN-W1-04.
- Consumer W5 -> [provider W3](../c3385d21-983e-4fb5-92f7-2a6ad757677a/body.md) -> MN-W3-01, MN-W3-02, MN-W3-05.
- Consumer W5 -> [provider W4](../9fe60c69-af53-4578-9310-69283cd09284/body.md) -> MN-W4-03, MN-W4-04, MN-W4-05.
- [Existing provider](../38a8ea3a-9a6a-4a0f-bc5a-97dc9ba670fc/body.md) -> Mission Domain: linked Ticket authority and serial prerequisite evidence. Reuse form 3: link unchanged.

The next consumer links these IDs rather than copying their acceptance into its own authority.

## Examples

Waypoint A completes at sequence 8 after B at 7 even though A is earlier in plan order. Reopen selects A, calls its Ticket done->ready operation, retains both results and blocks descendants. Crash after Ticket commit resumes the same operation without another reopen.

## Evidence

Motivation: remove manual Mission intake/lifecycle chores while retaining requester control.
Dependent expectation: Consumers can pause/resume, hand off, cancel, complete, review results/add work and reopen the latest current completion while retaining history and Ticket authority.

Planned guard: **G5 lifecycle-reopen**, from
[ROADMAP validation gates](../../../../../transcripts/29-10-2026_mission-native-mcp-planning/ROADMAP.md#validation-gates).
Command from meta-workspace root:

```sh
cargo test --manifest-path workflow-tools/Cargo.toml --locked -p mission-api -p mission -p ticket-api -p session-api --all-features -j 1
```

This is a declared validation plan, not an already registered test-api ValidationSpec
or a passing execution. No test-store records are created during this planning-only
request. Execution must register the owning guard and criterion/Ticket references
through test tools before recording the new behavior's results. Approval of a
Spec does not mean implemented or verified. Positions/code_ref values are above.

Owning implementation [Ticket](../../../ticket/tickets/9fcad997-7233-4305-9e69-534d4813ee2e/ticket.toml). State is planning until final review and supported lifecycle advancement; later read-back is in the dossier ledger.

## Scope

Excludes: Reopening cancelled Tickets, changing Session narrative scope or introducing a scheduler. Implementation sequence and commit checkpoints remain in the
owning Ticket and roadmap; this Spec owns observable behavior only.
