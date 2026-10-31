Implement [the authoritative contract](../../../../spec/specs/c3385d21-983e-4fb5-92f7-2a6ad757677a/body.md), criteria MN-W3-01 through MN-W3-05, using the [execution package](../../../../../../transcripts/29-10-2026_mission-native-mcp-planning/03-granular-mission-operations.md).

Owning code: mission/crates/mission-api/src/typed_source.rs, mission/crates/mission-api/src/output_history.rs, mission/crates/mission/src/direct.rs, mission/crates/mission/src/lib.rs.

Example: Removing W2's stable ID while W3 depends on it fails without changing files. Supplying a valid replacement and serial order in the same edit succeeds and yields one new plan digest; old result-bearing identity remains historical.

Failure boundary: No hidden dependency deletion, result erasure, silent legacy downgrade, partial generated publication or W-label identity.

Non-goal: Requester approval, execution and final named MCP registration.

Checkpoint after G3 plan-operations: `feat(mission): add granular planning operations`. Provider criteria and entry/exit conditions are linked in the execution package; a passed predecessor checkpoint and explicit execution approval are required before activation.