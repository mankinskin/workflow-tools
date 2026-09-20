Phase B. Extract the ticket tool into its own `ticket` repository (owner mankinskin), built as a single `ticket` domain crate per contract `0da6894c`: the crate lib re-exports the internal `ticket-api` crate and exposes transports as FEATURE-GATED binary targets (`ticket`, `ticket-mcp`, `ticket-http`) built on the shared `transport-harness` (`dbe0e955`). Frontends stay separate crates: `ticket-viewer` (Dioxus) and `ticket-vscode` (+ ticket-vscode-core).

Follow the parent-tracker recipe (`858c5286`); migrate ticket-scoped artifacts via the cross-store move tooling.

## Acceptance criteria
- `ticket` builds independently: domain crate lib (primary) re-exporting internal `ticket-api` + feature-gated transport bins: bare `ticket` CLI plus `ticket-mcp` and `ticket-http` over the harness.
- cli/mcp/http bin smoke pass; ticket-viewer browser-verified (screenshot + Playwright).
- ticket-scoped artifacts migrated with reference integrity.
- Registered as a workflow-tools dependency.