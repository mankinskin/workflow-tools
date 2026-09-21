Phase B. Extract the session tool into its own `session` repository (owner mankinskin), built as a single `session` domain crate per contract `0da6894c`: the crate lib re-exports the internal `session-api` crate and exposes transports as FEATURE-GATED binary targets (`session`, `session-mcp`) built on the shared `transport-harness` (`dbe0e955`).

Follow the parent-tracker recipe (`858c5286`); migrate session-scoped artifacts via the cross-store move tooling. Coordinate with session identity/optimization tickets still in review.

## Acceptance criteria
- `session` builds independently: domain crate lib (primary) re-exporting internal `session-api` + feature-gated transport bins: bare `session` CLI plus `session-mcp` over the harness.
- transport bin smoke pass.
- session-scoped artifacts migrated with reference integrity.
- Registered as a workflow-tools dependency.