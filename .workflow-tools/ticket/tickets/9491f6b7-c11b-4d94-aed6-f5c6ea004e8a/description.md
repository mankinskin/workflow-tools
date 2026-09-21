Plan and scaffold a bounded first `session-api` slice under `memory-viewers/memory-api` for saving Copilot chat sessions into a memory-api-backed store.

# Implemented Slice
- Added a new workspace member at `memory-api/crates/session-api`.
- Added typed models for `SessionRecord`, `SessionTurn`, `SessionMetadata`, and `SessionLinks`.
- Added hook-ingest models for `CopilotHookPayload`, `CopilotHookMessage`, and `SessionCaptureRequest`.
- Added deterministic store-layout planning with `SessionStoreConfig`, `SessionStorePaths`, and `SessionStorePlan`.
- Kept actual store writes, hook installation, and query/retrieval surfaces explicitly out of scope for this first slice.

# Validation
- ValidationSpec: focused compile and shallow unit coverage for the new `session-api` crate.
- ValidationExecution: passed `cargo test -p session-api`.
- Covered behaviors: serde round-trip for the session record, hook payload to session record mapping, deterministic store path planning, and invalid path-segment rejection.

# Evidence Trail
- DocEvidenceRecord candidates: `Cargo.toml`, `memory-api/crates/session-api/Cargo.toml`, and the new crate source files under `memory-api/crates/session-api/src/`.
- ValidationLogCapture / ValidationLogRetrieval: `cargo test -p session-api` output captured in the current session terminal.
- The linked spec records the crate boundary, related tickets, generated guidance surfaces, and the passing validation commands.

# Remaining Work
- Persist the planned session record into a concrete memory-api filesystem store.
- Add Copilot hook installation or ingestion surfaces that emit `SessionCaptureRequest` payloads.
- Add read/query tooling once the stored record format and write path are stabilized.