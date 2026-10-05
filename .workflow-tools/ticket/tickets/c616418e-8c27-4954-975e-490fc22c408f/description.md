# Implement mission CLI and MCP transports

## Objective

Implement the public `mission` facade crate on top of `mission-api`, as defined by [Mission Domain](../../../spec/specs/38a8ea3a-9a6a-4a0f-bc5a-97dc9ba670fc/body.md) (`ce://default/spec/38a8ea3a-9a6a-4a0f-bc5a-97dc9ba670fc`). This ticket follows the mission-api ticket in the strictly serial mission roadmap.

## Scope

- Re-export `mission-api` from the facade and expose feature-gated `mission` CLI and `mission-mcp` MCP binaries.
- Implement the contract's validate/collect-preview, import-with-expected-revision, render-preview, publish, and check-generated operations through the same domain API for CLI and MCP.
- Make publication atomic from the consumer's perspective: a failed render/write must preserve the previously published document set and accepted revision.
- Return equivalent structured diagnostics and status snapshots through CLI and MCP; check-generated detects drift without rewriting files or executing validation commands.
- Use `transport-harness` only for shared transport scaffolding; keep domain-specific command/MCP wiring local.

## Acceptance Criteria

- `cargo build -p mission --features cli,mcp` passes.
- `cargo test -p mission` passes.
- `cargo test -p mission --test roadmap_commands` executes and passes `validate_preview_does_not_mutate`, `cli_and_mcp_return_equivalent_diagnostics`, `publish_failure_preserves_previous_file_set`, `check_generated_detects_document_drift`, and `stale_revision_import_is_rejected`.
- The integration target and every required named test exist and execute; a zero-test filtered command is not acceptance evidence.
- Tests read back persisted revision metadata and actual generated ROADMAP/Part files.

## Non-goals

HTTP, a mission viewer, session pin routing, dossier migration, and agent scheduling.