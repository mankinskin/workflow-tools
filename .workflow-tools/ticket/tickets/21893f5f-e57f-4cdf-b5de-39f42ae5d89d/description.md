Phase B. Extract the rule tool into its own `rule` repository (owner mankinskin), built as a single `rule` domain crate per contract `0da6894c`: the crate lib re-exports the internal `rule-api` crate and exposes transports as FEATURE-GATED binary targets (`rule`, `rule-mcp`, and any `rule-http`) built on the shared `transport-harness` (`dbe0e955`).

Follow the parent-tracker recipe (`858c5286`); migrate rule-scoped artifacts via the cross-store move tooling. Coordinate with the migrate-off-generator work (rule-targets coupling).

## Acceptance criteria
- `rule` builds independently: domain crate lib (primary) re-exporting internal `rule-api` + feature-gated transport bins: bare `rule` CLI plus `rule-mcp` and any `rule-http` over the harness.
- transport bin smoke pass.
- rule-scoped artifacts migrated with reference integrity.
- Registered as a workflow-tools dependency.