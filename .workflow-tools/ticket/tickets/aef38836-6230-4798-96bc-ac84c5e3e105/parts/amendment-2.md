# Effective design - cold-read correction

This supersedes part `213fa5ad-df2a-45a1-998d-cc5439ddb0b5` for W6. Implement [the amended behavior authority](../../../../spec/specs/551422d7-8a0f-4711-8642-a4c367649361/body.md) with unchanged criterion IDs and [current execution package](../../../../../../transcripts/29-10-2026_mission-native-mcp-planning/06-discoverable-mcp-surface.md). Frozen original text remains historical, not a competing plan.

## Concrete interface and provider contract

Register mission_intake_create, mission_workspace_get, mission_plan_get, mission_plan_update, mission_waypoint_create, mission_waypoint_update, mission_waypoint_remove, mission_waypoint_reorder, mission_plan_prepare_review, mission_plan_approve, mission_plan_replan, mission_pause, mission_resume, mission_handoff, mission_cancel, mission_complete, mission_review_results_add_waypoints and mission_reopen_most_recent_completed.

- **MN-W6-01**: Actual initialized MCP tools/list exposes every named tool in Naming Conventions with capability/version metadata and schemas. Each tool delegates to the W2-W5 service rather than duplicating business rules.
- **MN-W6-02**: Read tools require explicit root/identity; retriable mutations require request_id and expected revision where a Mission exists. Return Mission ID, current revision, capability/version and structured result, including authoritative workspace paths.
- **MN-W6-03**: Expose goal/requirements/evidence changes through mission_plan_update and all waypoint create/update/remove/reorder operations. mission_plan_prepare_review returns W4's persisted pending challenge and exact host-submission line; mission_plan_get exposes captured RequesterDecisionRef values after host ingress. mission_plan_approve accepts only a resolvable Session decision reference, not actor/free-text/boolean authority. Preparation and approval remain distinct named calls; generic compatibility cannot bypass W4 checks.
- **MN-W6-04**: Return the common code/message/mission_id?/revision?/details? envelope. Cover approval_required, approval_stale, approval_authority_unavailable, revision_conflict, intake_missing, workspace_unavailable, waypoint_not_found, no_completed_waypoint, ticket_transition_rejected, ticket_state_unmapped and session_finished plus idempotency conflict.
- **MN-W6-05**: Prove initialize, tools/list and tools/call against real stdio server with schema and representative success/error parity. Clients use no shell-created directories, hand-written manifests or hidden lifecycle choreography.

The [Spec interface contract](../../../../spec/specs/551422d7-8a0f-4711-8642-a4c367649361/body.md#interfaces-and-dependencies) and [correction inventory](../../../../../../transcripts/29-10-2026_mission-native-mcp-planning/ARTIFACTS.md) name the complete owner, authority, persistence, host/installer ingress and failure contracts. Do not replace them with guessed adapter or generated-output paths.

Planned guard: `cargo test --manifest-path workflow-tools/Cargo.toml --locked -p mission --all-features -j 1`.

Checkpoint: `feat(mission): expose named MCP lifecycle tools` after the guard. No execution or implementation evidence is asserted.
