<!-- aligned-structure:v2 -->
# Discoverable Mission-native MCP Tool Contracts

## Target Code Location

- [mission/crates/mission/src/bin/mission-mcp.rs](../../../../mission/crates/mission/src/bin/mission-mcp.rs) — `code_ref: mission/crates/mission/src/bin/mission-mcp.rs`; position: `not-implemented` for this new contract (existing provider behavior is not reclassified).
- [mission/crates/mission/src/lib.rs](../../../../mission/crates/mission/src/lib.rs) — `code_ref: mission/crates/mission/src/lib.rs`; position: `not-implemented` for this new contract (existing provider behavior is not reclassified).
- [mission/crates/mission/tests/direct_transports.rs](../../../../mission/crates/mission/tests/direct_transports.rs) — `code_ref: mission/crates/mission/tests/direct_transports.rs`; position: `not-implemented` for this new contract (existing provider behavior is not reclassified).

## Naming Conventions

Register mission_intake_create, mission_workspace_get, mission_plan_get, mission_plan_update, mission_waypoint_create, mission_waypoint_update, mission_waypoint_remove, mission_waypoint_reorder, mission_plan_prepare_review, mission_plan_approve, mission_plan_replan, mission_pause, mission_resume, mission_handoff, mission_cancel, mission_complete, mission_review_results_add_waypoints and mission_reopen_most_recent_completed. Acceptance criterion identifiers are `MN-W6-NN`.

## Requester Input

> Vertiefe die Planung und erstelle und ordne Tickets und Specs in der Roadmap zu.

The full source scope and earlier explicit lifecycle/trait decisions are preserved in
[merged clean intent](../../../../../transcripts/29-10-2026_mission-native-mcp-planning/inputs/merged.clean.md).

## Reading Order

1. [Composition root](../92c78a48-cad8-4c08-8bba-4a523b717842/body.md) — shared invariants and component map.
2. [Owning Part](../../../../../transcripts/29-10-2026_mission-native-mcp-planning/06-discoverable-mcp-surface.md) — execution package, not a competing behavior authority.
3. [Mission Domain](../38a8ea3a-9a6a-4a0f-bc5a-97dc9ba670fc/body.md) — stable identity, serial order and foreign authority.

- Consumer W6 -> [provider W2](../8f53aa05-fb9c-4fec-9458-1eeb55b7342e/body.md) -> MN-W2-02, MN-W2-03, MN-W2-04, MN-W2-05.
- Consumer W6 -> [provider W3](../c3385d21-983e-4fb5-92f7-2a6ad757677a/body.md) -> MN-W3-01, MN-W3-02, MN-W3-04, MN-W3-05.
- Consumer W6 -> [provider W4](../9fe60c69-af53-4578-9310-69283cd09284/body.md) -> MN-W4-01, MN-W4-02, MN-W4-04.
- Consumer W6 -> [provider W5](../164e9acd-9781-4455-b835-f7121a275eef/body.md) -> MN-W5-01, MN-W5-02, MN-W5-03, MN-W5-05, MN-W5-06, MN-W5-07.
- [Existing provider](../9413a0a2-0d53-407d-b088-3953074393fb/body.md) -> Typed transport adapters: API delegation and actual initialized MCP parity. Reuse form 3: link unchanged.

## Responsibility

Consumers discover specialized tools and schemas for every planning/lifecycle operation instead of constructing generic scripts or folder operations. This new behavior slice is not already guaranteed by its historical providers.

## Interfaces And Dependencies

- Consumer W6 -> [provider W2](../8f53aa05-fb9c-4fec-9458-1eeb55b7342e/body.md) -> MN-W2-02, MN-W2-03, MN-W2-04, MN-W2-05.
- Consumer W6 -> [provider W3](../c3385d21-983e-4fb5-92f7-2a6ad757677a/body.md) -> MN-W3-01, MN-W3-02, MN-W3-04, MN-W3-05.
- Consumer W6 -> [provider W4](../9fe60c69-af53-4578-9310-69283cd09284/body.md) -> MN-W4-01, MN-W4-02, MN-W4-04.
- Consumer W6 -> [provider W5](../164e9acd-9781-4455-b835-f7121a275eef/body.md) -> MN-W5-01, MN-W5-02, MN-W5-03, MN-W5-05, MN-W5-06, MN-W5-07.
- [Existing provider](../9413a0a2-0d53-407d-b088-3953074393fb/body.md) -> Typed transport adapters: API delegation and actual initialized MCP parity. Reuse form 3: link unchanged.

Use explicit owning roots for each domain operation and same-root read-back.
Dependencies consume provider outcomes/criteria; they do not transfer provider ownership.

## Behavior

- **MN-W6-01**: Actual initialized MCP tools/list exposes every named tool in Naming Conventions with capability/version metadata and schemas. Each tool delegates to the W2-W5 service rather than duplicating business rules.
- **MN-W6-02**: Read tools require explicit root/identity; retriable mutations require request_id and expected revision where a Mission exists. Return Mission ID, current revision, capability/version and structured result, including authoritative workspace paths.
- **MN-W6-03**: Expose goal/requirements/evidence changes through mission_plan_update and all waypoint create/update/remove/reorder operations. mission_plan_prepare_review returns W4's persisted pending challenge and exact host-submission line; mission_plan_get exposes captured RequesterDecisionRef values after host ingress. mission_plan_approve accepts only a resolvable Session decision reference, not actor/free-text/boolean authority. Preparation and approval remain distinct named calls; generic compatibility cannot bypass W4 checks.
- **MN-W6-04**: Return the common code/message/mission_id?/revision?/details? envelope. Cover approval_required, approval_stale, approval_authority_unavailable, revision_conflict, intake_missing, workspace_unavailable, waypoint_not_found, no_completed_waypoint, ticket_transition_rejected, ticket_state_unmapped and session_finished plus idempotency conflict.
- **MN-W6-05**: Prove initialize, tools/list and tools/call against real stdio server with schema and representative success/error parity. Clients use no shell-created directories, hand-written manifests or hidden lifecycle choreography.

## Boundaries And Failure Cases

No generic-only discoverability claim, transport-owned state machine or claim of installed distribution before W8. No product implementation or evidence of passing behavior is asserted by approval of this contract.

## Provider/Consumer Contract

The consumer may rely on the criterion IDs above only after their implementation evidence passes.

- Consumer W6 -> [provider W2](../8f53aa05-fb9c-4fec-9458-1eeb55b7342e/body.md) -> MN-W2-02, MN-W2-03, MN-W2-04, MN-W2-05.
- Consumer W6 -> [provider W3](../c3385d21-983e-4fb5-92f7-2a6ad757677a/body.md) -> MN-W3-01, MN-W3-02, MN-W3-04, MN-W3-05.
- Consumer W6 -> [provider W4](../9fe60c69-af53-4578-9310-69283cd09284/body.md) -> MN-W4-01, MN-W4-02, MN-W4-04.
- Consumer W6 -> [provider W5](../164e9acd-9781-4455-b835-f7121a275eef/body.md) -> MN-W5-01, MN-W5-02, MN-W5-03, MN-W5-05, MN-W5-06, MN-W5-07.
- [Existing provider](../9413a0a2-0d53-407d-b088-3953074393fb/body.md) -> Typed transport adapters: API delegation and actual initialized MCP parity. Reuse form 3: link unchanged.

The next consumer links these IDs rather than copying their acceptance into its own authority.

## Examples

Discover mission_waypoint_reorder, call it with stable IDs and expected revision, and read the generated plan with mission_plan_get. A stale call returns revision_conflict and preserves source/projection hashes.

## Evidence

Motivation: remove manual Mission intake/lifecycle chores while retaining requester control.
Dependent expectation: Consumers discover specialized tools and schemas for every planning/lifecycle operation instead of constructing generic scripts or folder operations.

Planned guard: **G6 named-mcp**, from
[ROADMAP validation gates](../../../../../transcripts/29-10-2026_mission-native-mcp-planning/ROADMAP.md#validation-gates).
Command from meta-workspace root:

```sh
cargo test --manifest-path workflow-tools/Cargo.toml --locked -p mission --all-features -j 1
```

This is a declared validation plan, not an already registered test-api ValidationSpec
or a passing execution. No test-store records are created during this planning-only
request. Execution must register the owning guard and criterion/Ticket references
through test tools before recording the new behavior's results. Approval of a
Spec does not mean implemented or verified. Positions/code_ref values are above.

Owning implementation [Ticket](../../../ticket/tickets/aef38836-6230-4798-96bc-ac84c5e3e105/ticket.toml). State is planning until final review and supported lifecycle advancement; later read-back is in the dossier ledger.

## Scope

Excludes: Installed rollout, product scheduling or removing generic compatibility prematurely. Implementation sequence and commit checkpoints remain in the
owning Ticket and roadmap; this Spec owns observable behavior only.
