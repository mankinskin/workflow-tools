# Journaled forward Mission repair with durable recovery

## Goal and scope
REPAIR-1: API owns repair_preview/apply/journal/resume/rollback; thin CLI kebab-case commands and existing MCP mission_operation variants preserve the same diagnostics and results.

## Acceptance and validation
The explicit objective and requirements Parts are the complete current plan; do not infer completion from a preview or old note.
- `rtk cargo test -p mission --features cli,mcp --test recovery_move_contract repair_`

## Non-goals
Entity relocation, live five-record application during provider implementation, or restoring old snapshots over new edits.

## Status
Planning preparation only; implementation and live operations have not started. Use the linked owning specification and requirements through the plan view.
