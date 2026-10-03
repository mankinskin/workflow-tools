# Attach sessions to missions by URN

## Objective

Extend session pin routing so sessions can attach to the shared mission contract defined by [Mission Domain](../../workflow-tools/.workflow-tools/spec/specs/38a8ea3a-9a6a-4a0f-bc5a-97dc9ba670fc/body.md) (`ce://default/spec/38a8ea3a-9a6a-4a0f-bc5a-97dc9ba670fc`).

## Scope

- Add `mission` to the legal pinned-URN store enumeration.
- Validate canonical mission URNs and resolve the shared mission record from session attachments.
- Test multiple sessions attaching to one mission while each session retains its own workflow graph.

## Acceptance Criteria

- `cargo test -p session-api -- mission` passes.
- `cargo test -p session-api -- runtime_pin` passes.
- Invalid mission URNs fail actionably; session-local `session_workflow_*` remains unchanged.

## Dependencies

Depends on the mission-api store ticket.

## Non-goals

Moving or mirroring session workflow graphs, transports, dossier migration, and scheduling.