<!-- aligned-structure:v2 -->
# Shared Lifecycle Projections and Waypoint Capabilities

## Target Code Location

- [mission-api lifecycle](../../../../mission/crates/mission-api/src/lifecycle.rs) - `code_ref: mission/crates/mission-api/src/lifecycle.rs`; position: `implemented` for W1 types, journal validation and projections; durable control mutations remain W5.
- [mission-api public exports](../../../../mission/crates/mission-api/src/lib.rs) - `code_ref: mission/crates/mission-api/src/lib.rs`; position: `implemented`.
- [Mission lifecycle adapter](../../../../mission/crates/mission/src/lifecycle.rs) - `code_ref: mission/crates/mission/src/lifecycle.rs`; position: `implemented` for explicit-root reads and lawful Ticket-domain transition adapters.
- [Mission public exports](../../../../mission/crates/mission/src/lib.rs) - `code_ref: mission/crates/mission/src/lib.rs`; position: `implemented`.
- [Ticket provider](../../../../ticket/crates/ticket-api/src/lib.rs) - `code_ref: ticket/crates/ticket-api/src/lib.rs`; existing provider reused unchanged, with no Ticket-to-Mission dependency.

## Naming Conventions

LifecycleState, abstract Waypoint identity and separate read/transition, approval, handoff and reopen traits are proposed public concepts, not claims about existing symbols. Acceptance criterion identifiers are `MN-W1-NN`.

## Requester Input

> Vertiefe die Planung und erstelle und ordne Tickets und Specs in der Roadmap zu.

The full source scope and earlier explicit lifecycle/trait decisions are preserved in
[merged clean intent](../../../../../transcripts/29-10-2026_mission-native-mcp-planning/inputs/merged.clean.md).

## Reading Order

1. [Composition root](../92c78a48-cad8-4c08-8bba-4a523b717842/body.md) — shared invariants and component map.
2. [Owning Part](../../../../../transcripts/29-10-2026_mission-native-mcp-planning/01-shared-lifecycle-abstractions.md) — execution package, not a competing behavior authority.
3. [Mission Domain](../38a8ea3a-9a6a-4a0f-bc5a-97dc9ba670fc/body.md) — stable identity, serial order and foreign authority.

- [Existing provider](../38a8ea3a-9a6a-4a0f-bc5a-97dc9ba670fc/body.md) -> Mission Domain: stable waypoint identity and foreign-state authority. Reuse form 3: link unchanged.
- [Existing provider](../9fd527d8-e290-44e3-9edb-9f7d6658551a/body.md) -> Typed lifecycle: API-owned persistence and accepted identity. Reuse form 3: link unchanged.

## Responsibility

Consumers can query planned, active, paused, blocked, completed and cancelled consistently without a second writable Ticket state or a generic workflow database. This new behavior slice is not already guaranteed by its historical providers.

## Interfaces And Dependencies

Use the existing dependency direction: `mission-api` already depends on
`ticket-api`; Ticket must not depend back on Mission. Public types
`LifecycleState`, `LifecycleSnapshot` and capability traits belong in the
planned `mission/crates/mission-api/src/lifecycle.rs`, re-exported by
[mission-api/src/lib.rs](../../../../mission/crates/mission-api/src/lib.rs).
`mission/crates/mission/src/lifecycle.rs` owns the cross-store adapters.
There is **no new shared crate**, Ticket-to-Mission dependency or generic store.
Ticket-native transitions stay inside Ticket; the Mission facade exposes their
shared projection and capabilities. The existing
[manifest](../../../../mission/crates/mission-api/Cargo.toml) proves the
dependency direction.

`LifecycleSnapshot` includes shared state, native domain/state/revision, owning
root, control sequence and effective execution eligibility. Read and transition
traits are separate; approval, pause/resume, cancel, result review, handoff and
reopen are separately implementable capabilities. Unsupported operations error.
W1 owns types, conversion and serialized control contracts; W5 owns durable
control transitions and recovery. W2/W3 consume the W1 schema without inventing
another lifecycle.

| Authority/input | Shared state and persistence contract |
|---|---|
| Ticket planning / ready | planned; Ticket is the only mutable authority |
| Ticket in-implementation / in-review | active |
| Ticket on-hold / done / cancelled | paused / completed / cancelled |
| Ticket schema-provided blocked | blocked; unmapped states return `ticket_state_unmapped` |
| Single-session Waypoint Pending / InProgress / Completed / Blocked | planned / active / completed / blocked; retain existing native values |
| Single-session pause / cancel | paused / cancelled from version-1 Mission-owned control events, never by rewriting the legacy WaypointState enum |
| Mission with no execution started, including empty draft | planned |
| Mission running / blocked / completed | active while the current eligible waypoint is active or results await closure; blocked when its required entry cannot progress; completed only after W5 explicit completion validates every required outcome/result |
| Mission explicit pause / cancel | paused / cancelled from Mission control events |

Store control events under the UUID Mission directory in the API-owned
versioned lifecycle journal, each with monotonic sequence, operation ID,
target identity, prior sequence, event and prior nonterminal state. W5 uses
CAS/locking and crash recovery; readers must report corrupt/unsupported versions,
not silently reset. Missing journal in a legacy Mission means no control events,
not a bulk migration or approval. No new fields are written into legacy plan
manifests. Mission completion is an explicit event with evidence, not inferred
solely from native progress; a legacy all-completed Mission without that event
projects active/result-review-needed, with native progress retained.
Paused/cancelled Mission controls override **Mission eligibility**, not linked
Ticket states. Ticket-backed Waypoints always expose the Ticket state, even in
a paused Mission; the separate eligibility field denies execution.
Single-session control events may pause/cancel only a nonterminal Waypoint;
resume restores its exact preceding nonterminal projection. Completing a paused
Waypoint is rejected. Reopening terminal work requires W5's audited operation
and reapproval where plan/evidence changed, not a fabricated base-state edit.
Precedence is terminal cancellation/completion, explicit pause, blocked entry,
then planned/active. Conflicting terminal/control events are invalid histories.
Progress/control evidence changes do not increment the semantic plan revision
or invalidate approval; semantic plan edits still do. Tests cover every row,
old native/typed reads, corrupt journals, legal/illegal edges and Ticket
authority independently of Mission eligibility.

- [Existing provider](../38a8ea3a-9a6a-4a0f-bc5a-97dc9ba670fc/body.md) -> Mission Domain: stable waypoint identity and foreign-state authority. Reuse form 3: link unchanged.
- [Existing provider](../9fd527d8-e290-44e3-9edb-9f7d6658551a/body.md) -> Typed lifecycle: API-owned persistence and accepted identity. Reuse form 3: link unchanged.

Use explicit owning roots for each domain operation and same-root read-back.
Dependencies consume provider outcomes/criteria; they do not transfer provider ownership.

## Behavior

- **MN-W1-01**: Expose LifecycleState/LifecycleSnapshot from mission-api::lifecycle and re-export through its lib.rs; facade adapters belong to mission/src/lifecycle.rs. Keep the existing mission-api -> ticket-api dependency direction, with no Ticket -> Mission dependency or new shared crate. Retain native state/revision, owning root, control sequence and separate execution eligibility. Ticket planning/ready -> planned, in-implementation/in-review -> active, on-hold -> paused, done -> completed, cancelled -> cancelled, schema-provided blocked -> blocked; unknown states return ticket_state_unmapped. Single-session Pending/InProgress/Completed/Blocked -> planned/active/completed/blocked; paused/cancelled come from Mission-owned control events, not new legacy enum values. Mission starts planned, is active for running/result-review-needed work, blocked at an unavailable required entry, and completed only by validated explicit completion; explicit pause/cancel projects paused/cancelled.
- **MN-W1-02**: Ticket-backed waypoints read current state from the explicit owning Ticket store. A Mission cannot overwrite that state; transition adapters invoke the Ticket domain and preserve its history and legal-state checks.
- **MN-W1-03**: Separate read/transition, planning, approval, pause/resume, cancellation, completion/result review, handoff and reopen capabilities. Unsupported capabilities return an explicit diagnostic. Mission pause/cancel gates execution without overriding the shared state of a Ticket-backed waypoint. Single-session pause/cancel applies only to nonterminal work; resume restores the prior nonterminal state, and paused completion is rejected.
- **MN-W1-04**: Preserve stable waypoint identity and native/typed read compatibility. Define a version-1 Mission-owned lifecycle journal/control schema with monotonic sequence, operation ID, target, prior sequence and prior nonterminal state. W5 implements writes/recovery; do not extend legacy plan manifests or duplicate Ticket state. Missing legacy journal means no control events; corrupt/unsupported history errors. Terminal cancellation/completion precedes pause, blocked entry, then planned/active; conflicting events error. Progress/control events preserve semantic plan identity, while semantic edits require reapproval.

## Boundaries And Failure Cases

Unknown state, absent foreign authority and unsupported capability are explicit errors. No Ticket history mutation, automatic native-state migration or new shared entity registry. No product implementation or evidence of passing behavior is asserted by approval of this contract.

## Provider/Consumer Contract

The consumer may rely on the criterion IDs above only after their implementation evidence passes.

- [Existing provider](../38a8ea3a-9a6a-4a0f-bc5a-97dc9ba670fc/body.md) -> Mission Domain: stable waypoint identity and foreign-state authority. Reuse form 3: link unchanged.
- [Existing provider](../9fd527d8-e290-44e3-9edb-9f7d6658551a/body.md) -> Typed lifecycle: API-owned persistence and accepted identity. Reuse form 3: link unchanged.

The next consumer links these IDs rather than copying their acceptance into its own authority.

## Examples

A cancelled linked Ticket projects cancelled, not blocked. An on-hold Ticket projects paused. Querying an unsupported reopen capability returns an explicit unsupported-capability result without changing either store.

## Evidence

Motivation: remove manual Mission intake/lifecycle chores while retaining requester control.
Dependent expectation: Consumers can query planned, active, paused, blocked, completed and cancelled consistently without a second writable Ticket state or a generic workflow database.

Planned guard: **G1 shared-lifecycle**, from
[ROADMAP validation gates](../../../../../transcripts/29-10-2026_mission-native-mcp-planning/ROADMAP.md#validation-gates).
Command from meta-workspace root:

```sh
cargo test --manifest-path workflow-tools/Cargo.toml --locked -p mission-api -p mission -p ticket-api --all-features -j 1
```

G1 is now registered as
[mission-native-w1-shared-lifecycle-g1](../../../test/specs/mission-native-w1-shared-lifecycle-g1.json).
Execution `w1-g1-review-fix-20261030-mission-native-w1-shared-lifecycle-g1`
passed with 466 tests passed, 2 ignored, 0 failed across 31 suites.
It links the owning Ticket, this Spec and MN-W1-01 through MN-W1-04, plus log
capture `w1-g1-review-fix-20261030-mission-native-w1-shared-lifecycle-g1-log`.
The original passing run is retained; distinct review found four defects,
regressions were added before their fixes, and follow-up review passed.
Public-path imports, incomplete-completion rejection, cancelled required entry
eligibility and unknown journal targets are covered by the corrected guards.
See [API tests](../../../../mission/crates/mission-api/tests/lifecycle_contract.rs),
[facade tests](../../../../mission/crates/mission/tests/lifecycle_contract.rs),
[documentation](../../../../mission/README.md) and
[execution evidence](../../../../../transcripts/29-10-2026_mission-native-mcp-planning/RESULTS.md).

Owning implementation [Ticket](../../../ticket/tickets/ee23c860-7540-4f0f-a64c-a8cf5cfafe30/ticket.toml)
read back `done` after review and checkpoint
`5794d162317d8e5495ecd4f5e4a3ca5ed50414f9`.
Spec state is `implemented`, not a manually asserted `verified`.

### Guards

- `mission-native-w1-shared-lifecycle-g1`

## Scope

Excludes: Session intake, named MCP transport, generic entity storage and Ticket reopening. Implementation sequence and commit checkpoints remain in the
owning Ticket and roadmap; this Spec owns observable behavior only.
