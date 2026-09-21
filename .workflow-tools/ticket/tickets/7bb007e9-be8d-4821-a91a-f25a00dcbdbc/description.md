Blocking conformance gate runs in HANDOFF WRITE PATH (session_handoff). Non-conforming handoff REJECTED at write time, fail closed. Not at session start/resume.

**Code anchors**: memory-api/tools/mcp/session-mcp/src/server.rs (session_handoff tool), memory-api/crates/session-api/src/quality_gate.rs

**Acceptance**: Non-conforming handoff rejected at session_handoff write, error returned to caller.