Phase B. Extract the feedback tool into its own `feedback` repository (owner mankinskin), built as a single `feedback` domain crate per contract `0da6894c`: the crate lib re-exports the internal `feedback-api` crate and exposes transports as FEATURE-GATED binary targets (`feedback`, `feedback-mcp`) built on the shared `transport-harness` (`dbe0e955`).

Follow the parent-tracker recipe (`858c5286`); migrate feedback-scoped artifacts via the cross-store move tooling. Coordinate with the feedback-api design/curation tickets still in review.

## Acceptance criteria
- `feedback` builds independently: domain crate lib (primary) re-exporting internal `feedback-api` + feature-gated transport bins: bare `feedback` CLI plus `feedback-mcp` over the harness.
- transport bin smoke pass.
- feedback-scoped artifacts migrated with reference integrity.
- Registered as a workflow-tools dependency.