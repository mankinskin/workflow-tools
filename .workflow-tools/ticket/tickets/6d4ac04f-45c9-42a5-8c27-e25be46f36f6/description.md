See transcripts/01-09-2026_move-performance-benchmarking/02-spec-session-benchmark-harness.md for full detail.

## Outcome
workflow-tools/spec/crates/spec-api/ and workflow-tools/session/crates/session-api/ have no benches/ directory today. Create one benches/move_health.rs per crate, modeled on the ticket-domain file (after its extension in the sibling ticket), exercising the identical scenario matrix (entity count x link presence x link density x phase) through each domain's own plan_move_preflight/execute_with_journal/rollback entrypoints (spec-api/src/move_domain.rs, session-api/src/move_domain.rs). Reuse the memory_fixtures-based fixture generation approach, adapted to each domain's manifest shape.

## Non-goals
Does not benchmark rule/audit. Does not change any domain's move implementation.

## Acceptance Criteria
- cargo bench -p spec-api --bench move_health and cargo bench -p session-api --bench move_health both run to completion and produce Criterion output covering every scenario in the shared matrix.
