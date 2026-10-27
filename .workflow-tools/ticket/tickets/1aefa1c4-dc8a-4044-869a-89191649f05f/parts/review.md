## W1 closure validation

Fixed the confirmed failed-create facade regression in commit `68c8e244ba79093d349001fb4bc8aeb5973d0b14`.

- Rejected `Operation::CreateMission` now returns `status: error` with its API-allocated, parseable mission UUID and no accepted record.
- `cargo test -p mission --test direct_authoring rejected_create_returns_its_allocated_id_without_an_accepted_record -- --exact`: 1 passed.
- `cargo test -p mission-api --test ownership_api`: 3 passed.
- `cargo check -p mission --features cli,mcp`: 0 errors; pre-existing ticket-api dead-code warning only.

Evidence was recorded on Mission `bd23f826-8f9f-4bae-8b75-0288ba1b4899` from the explicit workspace root and published with generated-output verification.