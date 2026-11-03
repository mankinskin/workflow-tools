# Effective W6 acceptance - named waypoint completion

This supersedes acceptance amendment `5cfcf00c-582b-4590-8b83-9908359a6f12`.
Use [the revised W6 Spec](../../../../spec/specs/551422d7-8a0f-4711-8642-a4c367649361/body.md)
and [Part 06](../../../../../../transcripts/29-10-2026_mission-native-mcp-planning/06-discoverable-mcp-surface.md).
The prior amendment and frozen originals remain historical.

- **MN-W6-01**: Actual initialized MCP tools/list exposes all 19 Spec-named
  tools with capability/version metadata and schemas, including
  `mission_waypoint_complete`. All delegate to W2-W5 services.
- **MN-W6-02**: Preserve explicit root/identity, retriable mutation
  `request_id` and expected revision, and the common Mission ID/revision,
  capability/version, result and authoritative-workspace response.
  `mission_waypoint_complete` accepts Mission and stable waypoint IDs,
  UUID-formatted `request_id`, expected plan revision,
  `expected_control_sequence`, and nonempty completion evidence.
  Map `request_id` to W5 `ControlRequest.op_id`; delegate to
  `WorkspaceLifecycle::complete_waypoint`, retaining provider approval,
  execution eligibility, current evidence, prerequisites, linked-Ticket
  completion, CAS and replay checks.
- **MN-W6-03**: Preserve goal/requirements/evidence and waypoint edits;
  review preparation returns the W4 pending challenge and exact host prompt;
  plan read exposes captured receipt references; approval accepts only a
  resolvable Session decision reference, never actor text, free-text decisions
  or booleans. Generic compatibility cannot bypass W4 checks.
- **MN-W6-04**: Preserve common code/message/mission_id?/revision?/details?
  errors and all listed provider error mappings, including idempotency
  conflict. Completion failures use the same envelope, not ad hoc responses.
- **MN-W6-05**: Real stdio initialize/list/call proves schema and representative
  success/error parity. Completion covers UUID mapping, successful completion
  and replay, stale revision/control sequence, missing evidence and provider
  approval/eligibility rejection. No client-created directories, handwritten
  manifests or hidden lifecycle choreography.

Planning-only authorization: no W6 product implementation, G6, Ticket-state
change or stale-board cleanup/reclaim. The exact G6 remains
`cargo test --manifest-path workflow-tools/Cargo.toml --locked -p mission --all-features -j 1`.
The interrupted test and stored G6 draft must be reconciled with this contract
during separately approved execution; neither is current passing evidence.
