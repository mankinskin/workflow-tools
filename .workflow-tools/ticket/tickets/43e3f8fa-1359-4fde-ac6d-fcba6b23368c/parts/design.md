Implement [the authoritative contract](../../../../spec/specs/361a2900-65d1-41c4-9b28-9de7842b42f4/body.md), criteria MN-W8-01 through MN-W8-06, using the [execution package](../../../../../../transcripts/29-10-2026_mission-native-mcp-planning/08-installed-pilot-acceptance.md).

Owning code: mission/crates/mission/src/bin/mission-mcp.rs, mission/crates/mission/tests/direct_transports.rs, install/install-ctl, mission/README.md.

Example: The installed MCP creates Mission m from Session input with no precreated folder. Actor-only approval is rejected. An actual requester message grants digest A; changing A blocks execution until reapproval. Restart retains m and successful Ticket reopening is read back.

Failure boundary: No synthetic production grant, source-only pass substitution, broad cleanup, manual lifecycle fallback or success when host authority is unavailable.

Non-goal: Executing arbitrary work, production rollout or archive migration.

Checkpoint after G8 installed-pilot: `test(mission): verify installed native MCP planning lifecycle`. Provider criteria and entry/exit conditions are linked in the execution package; a passed predecessor checkpoint and explicit execution approval are required before activation.