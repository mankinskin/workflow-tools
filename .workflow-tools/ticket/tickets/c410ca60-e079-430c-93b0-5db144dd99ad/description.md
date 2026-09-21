Every sub-agent invocation gets its own durable isolated session stamped with parent_session_id and inherited track_id.

**Code anchors**: memory-api/crates/session-api/src/store.rs (session creation), memory-api/crates/session-api/src/store_routing_types.rs

**Acceptance**: Sub-agent sessions created with parent_session_id set, track_id inherited from parent.