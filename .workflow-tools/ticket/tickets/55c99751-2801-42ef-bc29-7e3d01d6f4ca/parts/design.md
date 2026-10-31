Implement [the authoritative contract](../../../../spec/specs/8f53aa05-fb9c-4fec-9458-1eeb55b7342e/body.md), criteria MN-W2-01 through MN-W2-05, using the [execution package](../../../../../../transcripts/29-10-2026_mission-native-mcp-planning/02-session-intake-managed-workspace.md).

Owning code: session/crates/session-api/src/store/config/capture_query.rs, session/crates/session-api/src/store/config/runtime_workflow.rs, mission/crates/mission-api/src/typed_source.rs, mission/crates/mission/src/lib.rs.

Example: Request r-42 submits bytes once, returns Mission m and its UUID directory. Restart and replay r-42 return m; different bytes conflict. A crash after Mission creation resumes the linkage and never creates m2.

Failure boundary: No finished-Session mutation, source normalization, manually authored path fallback or success-shaped partial linkage.

Non-goal: Executing the created Mission, migrating old archives or changing worktree assignments.

Checkpoint after G2 intake-workspace: `feat(mission): add session intake and managed workspace`. Provider criteria and entry/exit conditions are linked in the execution package; a passed predecessor checkpoint and explicit execution approval are required before activation.