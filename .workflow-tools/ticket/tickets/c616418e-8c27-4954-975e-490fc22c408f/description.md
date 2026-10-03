# Implement mission CLI and MCP transports

## Objective

Implement the public `mission` facade crate on top of `mission-api`, as defined by [Mission Domain](../../workflow-tools/.workflow-tools/spec/specs/38a8ea3a-9a6a-4a0f-bc5a-97dc9ba670fc/body.md) (`ce://default/spec/38a8ea3a-9a6a-4a0f-bc5a-97dc9ba670fc`).

## Scope

- Re-export `mission-api` from the facade crate.
- Add feature-gated `mission` CLI and `mission-mcp` MCP binaries.
- Use `transport-harness` only for shared transport scaffolding and keep domain wiring local.

## Acceptance Criteria

- `cargo build -p mission --features cli,mcp` passes.
- `cargo test -p mission` passes.
- CLI and MCP expose mission reads and mutations without an HTTP binary.

## Dependencies

Depends on the mission-api store ticket.

## Non-goals

HTTP, a mission viewer, session pin routing, dossier migration, and scheduling.