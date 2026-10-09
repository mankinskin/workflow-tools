# Attach sessions to missions by URN

## Objective

Extend session pin routing so sessions can attach to the shared mission contract defined by [Mission Domain](../../../spec/specs/38a8ea3a-9a6a-4a0f-bc5a-97dc9ba670fc/body.md) (`ce://default/spec/38a8ea3a-9a6a-4a0f-bc5a-97dc9ba670fc`). This ticket follows the mission CLI/MCP ticket in the roadmap's required serial execution order.

## Scope

- Add `mission` to the legal pinned-URN store enumeration.
- Validate canonical mission URNs and resolve the shared mission record from session attachments.
- Test multiple sessions attaching to one mission while each session retains its own workflow graph.
- Do not begin until the immediately preceding mission CLI/MCP ticket has passed its checkpoint, even though this session-routing change does not call the CLI.

## Acceptance Criteria

- `cargo test -p session-api --lib runtime_pin_accepts_mission_urn -- --exact` passes.
- `cargo test -p session-api --lib runtime_pin_rejects_invalid_mission_urn -- --exact` passes.
- `cargo test -p session-api --lib two_sessions_attach_same_mission_without_shared_workflow_graph -- --exact` passes.
- Each exact test command executes one named test; zero matched tests are a failure.
- Invalid mission URNs fail actionably and session-local `session_workflow_*` remains unchanged.

## Non-goals

Moving or mirroring session workflow graphs, transports, dossier migration, and scheduling.