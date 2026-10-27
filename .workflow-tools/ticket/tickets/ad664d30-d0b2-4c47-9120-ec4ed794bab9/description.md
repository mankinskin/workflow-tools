# Shared-kernel Mission set relocation and recovery

## Goal and scope
MOVE-1: Mission API integrates the shared entity-move kernel, exposing move_preview/apply/journal/resume/rollback through the same thin CLI/MCP operation dispatch, not a new transport-owned engine.

## Acceptance and validation
The explicit objective and requirements Parts are the complete current plan; do not infer completion from a preview or old note.
- `rtk cargo test -p mission --features cli,mcp --test recovery_move_contract move_`

## Non-goals
A second conversion policy, moving linked entities, or live selected-record application while implementing the provider.

## Status
Planning preparation only; implementation and live operations have not started. Use the linked owning specification and requirements through the plan view.
