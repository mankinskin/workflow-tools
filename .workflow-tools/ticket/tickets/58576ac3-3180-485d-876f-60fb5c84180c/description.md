Phase B. Extract the spec tool into its own `spec` repository (owner mankinskin), built as a single `spec` domain crate per contract `0da6894c`: the crate lib re-exports the internal `spec-api` crate and exposes transports as FEATURE-GATED binary targets (`spec`, `spec-mcp`, and `spec-http` if present) built on the shared `transport-harness` (`dbe0e955`). Frontend `spec-viewer` stays a separate crate.

Follow the parent-tracker recipe (`858c5286`); migrate spec-scoped artifacts via the cross-store move tooling.

## Acceptance criteria
- `spec` builds independently: domain crate lib (primary) re-exporting internal `spec-api` + feature-gated transport bins: bare `spec` CLI plus `spec-mcp` and `spec-http` if present over the harness.
- transport bin smoke pass; spec-viewer browser-verified (screenshot + Playwright).
- spec-scoped artifacts migrated with reference integrity.
- Registered as a workflow-tools dependency.