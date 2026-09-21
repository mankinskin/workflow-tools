# Summary

Coordinate the bounded evidence-store bootstrap across `doc-api`, `test-api`, and `log-api` so spec acceptance clauses can resolve authoritative documentation, validation, and log evidence without wrapper-owned artifacts.

# Why

`doc-api` already exists, while `test-api` and `log-api` are still only planned. Turning that asymmetry into a smooth implementation path requires smaller bounded slices instead of one overstuffed umbrella ticket.

# Scope

- add a bounded `doc-api` child ticket for documentation-validation identities, manual verification steps, and coverage-gap reporting
- add a bounded `test-api` child ticket for validation specifications, executions, and outcomes
- add a bounded `log-api` child ticket for validation-log identities and links from test executions
- keep the shared link semantics sufficient for the pilot migration and downstream audit rollups

# Assumptions To Prove

- the evidence stores can be bootstrapped as separate first-slice tickets without losing the shared ownership model
- spec fulfillment can depend on native evidence links without reintroducing wrapper-owned truth
- the resulting evidence model is sufficient for the pilot migration and audit rollups without solving every transport surface in one pass

# Acceptance Criteria

- Distinct child tickets exist for `doc-api`, `test-api`, and `log-api`, each with a bounded first-slice scope.
- Evidence ownership remains in the data-owning stores rather than moving into `audit-api` or a wrapper artifact path.
- Shared link semantics are sufficient for the pilot migration ticket to validate one end-to-end slice.
- This tracker closes only after the bounded child tickets are implemented and wired together coherently.

# Validation

- `ticket.exe health <tracker-id> --workspace-root . --json`
- Child tickets provide the focused validation for store-owned evidence and derived audit reporting.