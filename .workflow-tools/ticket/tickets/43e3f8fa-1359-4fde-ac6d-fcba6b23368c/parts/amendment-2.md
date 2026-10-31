# Effective design - cold-read correction

This supersedes part `d665641f-73de-4440-9ab5-9584e6e9164e` for W8. Implement [the amended behavior authority](../../../../spec/specs/361a2900-65d1-41c4-9b28-9de7842b42f4/body.md) with unchanged criterion IDs and [current execution package](../../../../../../transcripts/29-10-2026_mission-native-mcp-planning/08-installed-pilot-acceptance.md). Frozen original text remains historical, not a competing plan.

## Concrete interface and provider contract

Add mission_native_mcp_pilot as a future test target. Separate simulated decision fixture evidence from an actual requester-decision walkthrough receipt.

- **MN-W8-01**: Build/install through existing tooling into a disposable prefix and start the installed server, not a source-only test substitute. Record executable/version, initialize/list/call transcript, explicit roots and returned IDs/revisions/digests.
- **MN-W8-02**: Use only installed MCP for user-facing intake/planning/lifecycle operations: empty-store bootstrap, lookup, granular edits/reorder/remove, plan review and no execution before exact approval. No shell-created Mission folders or hand-written lifecycle records.
- **MN-W8-03**: Use W4's installed UserPromptSubmit hook with an active requester Session. Show its persisted pending challenge and exact standalone mission-decision approval line, have the actual requester submit it as a new host prompt, resolve the returned immutable RequesterDecisionRef and approve through installed Mission MCP using that reference only. Record source event/decision IDs, raw-byte digest, explicit Session root, plan/review digests, consumed challenge and same-root grant read-back; then replan and collect a new real challenge-bound approval. Automated stdin/receipt fixtures remain synthetic evidence. Missing host delivery, receipts or real requester interaction fails acceptance; never manufacture a receipt or substitute CLI actor text.
- **MN-W8-04**: Exercise pause/resume, checked Session handoff, cancel versus blocked, validated complete, result review/add, latest-current-completion CAS reopen and successful authoritative Ticket done->ready read-back.
- **MN-W8-05**: Inject restart and interrupted cross-store cases and replay request IDs without duplicate identity or lost history; demonstrate precise missing/unavailable/stale errors. Retain separate automated transcript and real decision receipts with final same-root Mission/Ticket read-back.
- **MN-W8-06**: Clean up only explicitly created disposable resources after evidence capture. Preserve production Missions, Sessions, source dossiers and user archives; claim neither production rollout nor archive migration.

The [Spec interface contract](../../../../spec/specs/361a2900-65d1-41c4-9b28-9de7842b42f4/body.md#interfaces-and-dependencies) and [correction inventory](../../../../../../transcripts/29-10-2026_mission-native-mcp-planning/ARTIFACTS.md) name the complete owner, authority, persistence, host/installer ingress and failure contracts. Do not replace them with guessed adapter or generated-output paths.

Planned guard: `cargo test --manifest-path workflow-tools/Cargo.toml --locked -p mission --all-features --test mission_native_mcp_pilot -j 1`.

Checkpoint: `test(mission): verify installed native MCP planning lifecycle` after the guard. No execution or implementation evidence is asserted.
