Implement [the authoritative contract](../../../../spec/specs/164e9acd-9781-4455-b835-f7121a275eef/body.md), criteria MN-W5-01 through MN-W5-07, using the [execution package](../../../../../../transcripts/29-10-2026_mission-native-mcp-planning/05-lifecycle-handoff-and-reopen.md).

Owning code: mission/crates/mission-api/src/lib.rs, mission/crates/mission/src/lib.rs, ticket/crates/ticket-api/src/lib.rs, ticket/crates/ticket-api/schemas/feature.toml, session/crates/session-api/src/store/config/runtime_workflow.rs.

Example: Waypoint A completes at sequence 8 after B at 7 even though A is earlier in plan order. Reopen selects A, calls its Ticket done->ready operation, retains both results and blocks descendants. Crash after Ticket commit resumes the same operation without another reopen.

Failure boundary: No final-order selection, raw terminal Ticket edits, automatic cancellation cascade, cross-store atomicity claim or historical event deletion.

Non-goal: Reopening cancelled Tickets, changing Session narrative scope or introducing a scheduler.

Checkpoint after G5 lifecycle-reopen: `feat(mission): add lifecycle and audited latest-completion reopen`. Provider criteria and entry/exit conditions are linked in the execution package; a passed predecessor checkpoint and explicit execution approval are required before activation.