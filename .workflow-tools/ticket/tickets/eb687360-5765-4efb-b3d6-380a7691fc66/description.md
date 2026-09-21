## Problem
The graded cost-gate work added a new `feature` ticket type schema to shared ticketing infra without its own tracking ticket:
- NEW `memory-api/crates/ticket-api/schemas/feature.toml`
- MODIFIED `memory-api/crates/ticket-api/src/model/default_schema.rs` (registers feature.toml)

It was introduced to unblock creating type=feature tickets. It is committed in memory-api 2269f49 but never reviewed on its own merits.

## Scope
- Review the `feature` schema state machine (new -> ready -> in-implementation -> in-review -> done, + new -> in-review) for consistency with sibling schemas (task/bug/epic).
- Confirm adding a delivered default schema is the intended extension mechanism vs workspace-local schema dir.
- Decide whether this belongs as a permanent delivered type.

## Acceptance
- Explicit accept/revert decision recorded for the feature schema.
- If kept, schema documented alongside other delivered types.

Source: review of the graded cost-gate feature (spec 29ae5f6e).
