Implement [the authoritative contract](../../../../spec/specs/551422d7-8a0f-4711-8642-a4c367649361/body.md), criteria MN-W6-01 through MN-W6-05, using the [execution package](../../../../../../transcripts/29-10-2026_mission-native-mcp-planning/06-discoverable-mcp-surface.md).

Owning code: mission/crates/mission/src/bin/mission-mcp.rs, mission/crates/mission/src/lib.rs, mission/crates/mission/tests/direct_transports.rs.

Example: Discover mission_waypoint_reorder, call it with stable IDs and expected revision, and read the generated plan with mission_plan_get. A stale call returns revision_conflict and preserves source/projection hashes.

Failure boundary: No generic-only discoverability claim, transport-owned state machine or claim of installed distribution before W8.

Non-goal: Installed rollout, product scheduling or removing generic compatibility prematurely.

Checkpoint after G6 named-mcp: `feat(mission): expose named MCP lifecycle tools`. Provider criteria and entry/exit conditions are linked in the execution package; a passed predecessor checkpoint and explicit execution approval are required before activation.