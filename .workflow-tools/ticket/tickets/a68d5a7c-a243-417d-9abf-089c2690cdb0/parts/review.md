## Review

- Reviewed CLI/MCP mapping: typed lifecycle inputs are inline `TypedDraft`; adapters delegate identity, CAS, validation, serial predecessor derivation, persistence, and rendering to `mission-api`.
- Validation: `cargo test -p mission --features cli,mcp --test direct_transports` — 5 passed.
- Commit: `d8cb1deab5c0522cda65a57631dd77504c91c707`.