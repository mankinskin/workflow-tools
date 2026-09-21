Phase B. Extract the test tool into its own `test` repository (owner mankinskin), built as a single `test` domain crate per contract `0da6894c`: the crate lib re-exports the internal `test-api` crate and exposes transports as FEATURE-GATED binary targets (`test`, `test-mcp`, and any `test-http`) built on the shared `transport-harness` (`dbe0e955`).

Follow the parent-tracker recipe (`858c5286`); migrate test-scoped artifacts via the cross-store move tooling.

## Acceptance criteria
- `test` builds independently: domain crate lib (primary) re-exporting internal `test-api` + feature-gated transport bins: bare `test` CLI plus `test-mcp` and any `test-http` over the harness.
- transport bin smoke pass.
- test-scoped artifacts migrated with reference integrity.
- Registered as a workflow-tools dependency.