See transcripts/01-09-2026_move-performance-benchmarking/03-rule-audit-benchmark-harness.md for full detail.

## Outcome
workflow-tools/rule/crates/rule-api/src/move_domain.rs and workflow-tools/audit/crates/audit-api/src/move_domain.rs are the remaining two MoveDomain implementors. Create benches/move_health.rs in each crate, same scenario matrix and fixture-realism approach as the ticket/spec/session sibling tickets, adapted to whatever link/reference shape each domain's manifests actually have -- inspect the manifest schema first; if a domain has no concept of inter-entity links, the link-density axis should legitimately collapse rather than fabricating a sweep that doesn't apply.

## Non-goals
Does not change rule-api's or audit-api's move implementation.

## Acceptance Criteria
- cargo bench -p rule-api --bench move_health and cargo bench -p audit-api --bench move_health both run to completion and produce Criterion output for every scenario applicable to that domain's schema.
