Implement [the authoritative contract](../../../../spec/specs/9fe60c69-af53-4578-9310-69283cd09284/body.md), criteria MN-W4-01 through MN-W4-05, using the [execution package](../../../../../../transcripts/29-10-2026_mission-native-mcp-planning/04-revision-bound-approval.md).

Owning code: mission/crates/mission-api/src/output_history.rs, mission/crates/mission-api/src/typed_source.rs, mission/crates/mission/src/lib.rs, session/crates/session-api/src/store/config/capture_query.rs.

Example: Plan digest A passes two reviews; the requester message approving A grants A only. Editing a requirement creates B and blocks execution until B is reviewed and approved. An automated fixture saying actor=requester cannot approve B.

Failure boundary: Host-adapter absence is a failing supported-path gate, not permission to fabricate authority. Distinguish machine readiness, historical reaction and execution authorization.

Non-goal: Executing work, cryptographic identity claims or accepting a generic reaction as a grant.

Checkpoint after G4 requester-approval: `feat(mission): gate execution on reviewed requester approval`. Provider criteria and entry/exit conditions are linked in the execution package; a passed predecessor checkpoint and explicit execution approval are required before activation.