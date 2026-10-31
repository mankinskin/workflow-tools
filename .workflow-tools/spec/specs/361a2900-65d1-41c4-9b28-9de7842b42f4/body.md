<!-- aligned-structure:v2 -->
# Installed MCP Native Planning Acceptance and Requester Walkthrough

## Target Code Location

- [mission/crates/mission/src/bin/mission-mcp.rs](../../../../mission/crates/mission/src/bin/mission-mcp.rs) — `code_ref: mission/crates/mission/src/bin/mission-mcp.rs`; position: `not-implemented` for this new contract (existing provider behavior is not reclassified).
- [mission/crates/mission/tests/direct_transports.rs](../../../../mission/crates/mission/tests/direct_transports.rs) — `code_ref: mission/crates/mission/tests/direct_transports.rs`; position: `not-implemented` for this new contract (existing provider behavior is not reclassified).
- [install/install-ctl](../../../../install/install-ctl) — `code_ref: install/install-ctl`; position: `not-implemented` for this new contract (existing provider behavior is not reclassified).
- [mission/README.md](../../../../mission/README.md) — `code_ref: mission/README.md`; position: `not-implemented` for this new contract (existing provider behavior is not reclassified).

## Naming Conventions

Add mission_native_mcp_pilot as a future test target. Separate simulated decision fixture evidence from an actual requester-decision walkthrough receipt. Acceptance criterion identifiers are `MN-W8-NN`.

## Requester Input

> Vertiefe die Planung und erstelle und ordne Tickets und Specs in der Roadmap zu.

The full source scope and earlier explicit lifecycle/trait decisions are preserved in
[merged clean intent](../../../../../transcripts/29-10-2026_mission-native-mcp-planning/inputs/merged.clean.md).

## Reading Order

1. [Composition root](../92c78a48-cad8-4c08-8bba-4a523b717842/body.md) — shared invariants and component map.
2. [Owning Part](../../../../../transcripts/29-10-2026_mission-native-mcp-planning/08-installed-pilot-acceptance.md) — execution package, not a competing behavior authority.
3. [Mission Domain](../38a8ea3a-9a6a-4a0f-bc5a-97dc9ba670fc/body.md) — stable identity, serial order and foreign authority.

- Consumer W8 -> [provider W7](../ec2f0709-180f-4ccf-b692-601796f33e05/body.md) -> MN-W7-01, MN-W7-02, MN-W7-03, MN-W7-04, MN-W7-05.
- Consumer W8 -> [provider W6](../551422d7-8a0f-4711-8642-a4c367649361/body.md) -> MN-W6-01, MN-W6-04, MN-W6-05.
- Consumer W8 -> [provider W5](../164e9acd-9781-4455-b835-f7121a275eef/body.md) -> MN-W5-03, MN-W5-04, MN-W5-05, MN-W5-06.
- Consumer W8 -> [provider W4](../9fe60c69-af53-4578-9310-69283cd09284/body.md) -> MN-W4-02, MN-W4-03, MN-W4-04, MN-W4-05.
- [Existing provider](../9e714b78-ef31-46e5-8103-a944e7b1a278/body.md) -> Installer distribution: real installed artifact discovery, not generic CLI availability. Reuse form 3: link unchanged.

## Responsibility

A disposable installed-server pilot proves the full user-facing native lifecycle from empty stores and no precreated Mission directory, including real requester authority. This new behavior slice is not already guaranteed by its historical providers.

## Interfaces And Dependencies

- Consumer W8 -> [provider W7](../ec2f0709-180f-4ccf-b692-601796f33e05/body.md) -> MN-W7-01, MN-W7-02, MN-W7-03, MN-W7-04, MN-W7-05.
- Consumer W8 -> [provider W6](../551422d7-8a0f-4711-8642-a4c367649361/body.md) -> MN-W6-01, MN-W6-04, MN-W6-05.
- Consumer W8 -> [provider W5](../164e9acd-9781-4455-b835-f7121a275eef/body.md) -> MN-W5-03, MN-W5-04, MN-W5-05, MN-W5-06.
- Consumer W8 -> [provider W4](../9fe60c69-af53-4578-9310-69283cd09284/body.md) -> MN-W4-02, MN-W4-03, MN-W4-04, MN-W4-05.
- [Existing provider](../9e714b78-ef31-46e5-8103-a944e7b1a278/body.md) -> Installer distribution: real installed artifact discovery, not generic CLI availability. Reuse form 3: link unchanged.

Use explicit owning roots for each domain operation and same-root read-back.
Dependencies consume provider outcomes/criteria; they do not transfer provider ownership.

## Behavior

- **MN-W8-01**: Build/install through existing tooling into a disposable prefix and start the installed server, not a source-only test substitute. Record executable/version, initialize/list/call transcript, explicit roots and returned IDs/revisions/digests.
- **MN-W8-02**: Use only installed MCP for user-facing intake/planning/lifecycle operations: empty-store bootstrap, lookup, granular edits/reorder/remove, plan review and no execution before exact approval. No shell-created Mission folders or hand-written lifecycle records.
- **MN-W8-03**: Use W4's installed UserPromptSubmit hook with an active requester Session. Show its persisted pending challenge and exact standalone mission-decision approval line, have the actual requester submit it as a new host prompt, resolve the returned immutable RequesterDecisionRef and approve through installed Mission MCP using that reference only. Record source event/decision IDs, raw-byte digest, explicit Session root, plan/review digests, consumed challenge and same-root grant read-back; then replan and collect a new real challenge-bound approval. Automated stdin/receipt fixtures remain synthetic evidence. Missing host delivery, receipts or real requester interaction fails acceptance; never manufacture a receipt or substitute CLI actor text.
- **MN-W8-04**: Exercise pause/resume, checked Session handoff, cancel versus blocked, validated complete, result review/add, latest-current-completion CAS reopen and successful authoritative Ticket done->ready read-back.
- **MN-W8-05**: Inject restart and interrupted cross-store cases and replay request IDs without duplicate identity or lost history; demonstrate precise missing/unavailable/stale errors. Retain separate automated transcript and real decision receipts with final same-root Mission/Ticket read-back.
- **MN-W8-06**: Clean up only explicitly created disposable resources after evidence capture. Preserve production Missions, Sessions, source dossiers and user archives; claim neither production rollout nor archive migration.

## Boundaries And Failure Cases

No synthetic production grant, source-only pass substitution, broad cleanup, manual lifecycle fallback or success when host authority is unavailable. No product implementation or evidence of passing behavior is asserted by approval of this contract.

## Provider/Consumer Contract

The consumer may rely on the criterion IDs above only after their implementation evidence passes.

- Consumer W8 -> [provider W7](../ec2f0709-180f-4ccf-b692-601796f33e05/body.md) -> MN-W7-01, MN-W7-02, MN-W7-03, MN-W7-04, MN-W7-05.
- Consumer W8 -> [provider W6](../551422d7-8a0f-4711-8642-a4c367649361/body.md) -> MN-W6-01, MN-W6-04, MN-W6-05.
- Consumer W8 -> [provider W5](../164e9acd-9781-4455-b835-f7121a275eef/body.md) -> MN-W5-03, MN-W5-04, MN-W5-05, MN-W5-06.
- Consumer W8 -> [provider W4](../9fe60c69-af53-4578-9310-69283cd09284/body.md) -> MN-W4-02, MN-W4-03, MN-W4-04, MN-W4-05.
- [Existing provider](../9e714b78-ef31-46e5-8103-a944e7b1a278/body.md) -> Installer distribution: real installed artifact discovery, not generic CLI availability. Reuse form 3: link unchanged.

The next consumer links these IDs rather than copying their acceptance into its own authority.

## Examples

The installed MCP creates Mission m from Session input with no precreated folder. Actor-only approval is rejected. An actual requester message grants digest A; changing A blocks execution until reapproval. Restart retains m and successful Ticket reopening is read back.

## Evidence

Use the [actual workspace hook registration](../../../../../.github/hooks/hooks.json)
as the registration model for the isolated pilot host. Record the installed
capture-hook executable/version it resolves, as well as the Mission server;
an older PATH-resolved hook cannot demonstrate the new W4 receipt adapter.

Motivation: remove manual Mission intake/lifecycle chores while retaining requester control.
Dependent expectation: A disposable installed-server pilot proves the full user-facing native lifecycle from empty stores and no precreated Mission directory, including real requester authority.

Planned guard: **G8 installed-pilot**, from
[ROADMAP validation gates](../../../../../transcripts/29-10-2026_mission-native-mcp-planning/ROADMAP.md#validation-gates).
Command from meta-workspace root:

```sh
cargo test --manifest-path workflow-tools/Cargo.toml --locked -p mission --all-features --test mission_native_mcp_pilot -j 1
```

This is a declared validation plan, not an already registered test-api ValidationSpec
or a passing execution. No test-store records are created during this planning-only
request. Execution must register the owning guard and criterion/Ticket references
through test tools before recording the new behavior's results. Approval of a
Spec does not mean implemented or verified. Positions/code_ref values are above.

Owning implementation [Ticket](../../../ticket/tickets/43e3f8fa-1359-4fde-ac6d-fcba6b23368c/ticket.toml). State is planning until final review and supported lifecycle advancement; later read-back is in the dossier ledger.

## Scope

Excludes: Executing arbitrary work, production rollout or archive migration. Implementation sequence and commit checkpoints remain in the
owning Ticket and roadmap; this Spec owns observable behavior only.
