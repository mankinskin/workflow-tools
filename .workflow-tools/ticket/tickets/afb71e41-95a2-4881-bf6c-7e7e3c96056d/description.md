Phase B. Extract the audit tool into its own `audit` repository (owner mankinskin), built as a single `audit` domain crate per contract `0da6894c`: the crate lib re-exports the internal `audit-api` crate and exposes transports as FEATURE-GATED binary targets (`audit`, `audit-mcp`) built on the shared `transport-harness` (`dbe0e955`), plus the `.audit.toml` config contract.

Follow the parent-tracker recipe (`858c5286`); migrate audit-scoped artifacts via the cross-store move tooling.

## Acceptance criteria
- `audit` builds independently: domain crate lib (primary) re-exporting internal `audit-api` + feature-gated transport bins: bare `audit` CLI plus `audit-mcp` over the harness.
- transport bin smoke pass.
- audit-scoped artifacts migrated with reference integrity.
- Registered as a workflow-tools dependency.