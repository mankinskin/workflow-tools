# Correct persisted typed Mission lifecycle authority

## Goal and scope
AUTH-1: All persisted typed and compatible legacy mutation routes share one revision/CAS/lifecycle authority owned by mission-api; transports cannot reconstruct or maintain a second accepted source.

## Acceptance and validation
The explicit objective and requirements Parts are the complete current plan; do not infer completion from a preview or old note.
- `rtk cargo test -p mission-api --test typed_source`
- `rtk cargo test -p mission --test typed_migration`

## Non-goals
Root selection, live record repair/move and Session changes.

## Status
Planning preparation only; implementation and live operations have not started. Use the linked owning specification and requirements through the plan view.
