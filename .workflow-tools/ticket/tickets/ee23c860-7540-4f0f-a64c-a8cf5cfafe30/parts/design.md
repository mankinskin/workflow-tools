Implement [the authoritative contract](../../../../spec/specs/908e1755-d486-4e49-8b63-3f9eee81396e/body.md), criteria MN-W1-01 through MN-W1-04, using the [execution package](../../../../../../transcripts/29-10-2026_mission-native-mcp-planning/01-shared-lifecycle-abstractions.md).

Owning code: mission/crates/mission-api/src/lib.rs, mission/crates/mission/src/lib.rs, ticket/crates/ticket-api/src/lib.rs.

Example: A cancelled linked Ticket projects cancelled, not blocked. An on-hold Ticket projects paused. Querying an unsupported reopen capability returns an explicit unsupported-capability result without changing either store.

Failure boundary: Unknown state, absent foreign authority and unsupported capability are explicit errors. No Ticket history mutation, automatic native-state migration or new shared entity registry.

Non-goal: Session intake, named MCP transport, generic entity storage and Ticket reopening.

Checkpoint after G1 shared-lifecycle: `feat(workflow): add shared lifecycle capability contracts`. Provider criteria and entry/exit conditions are linked in the execution package; a passed predecessor checkpoint and explicit execution approval are required before activation.