# Summary

Homogeneously migrate the remaining affected expectation-oriented specs and tickets after the pilot proves the contract.

# Why

The broader migration still matters, but it should not be crammed into the pilot ticket. Once the pilot exposes the real edge cases, the remaining work needs its own bounded tracker with a fixed inventory.

# Scope

- capture a fixed in-scope inventory of the remaining affected specs and tickets after the pilot completes
- apply one documented mapping for where contract, plan, current state, and evidence live across that inventory
- keep unrelated future cleanup or optional refinements out of scope for this tracker

# Assumptions To Prove

- the pilot will reveal enough edge cases to let the broader migration use one consistent mapping
- a fixed in-scope inventory is enough to keep the broader migration bounded and reviewable
- the broader migration can remain a documentation-and-metadata rollout rather than turning back into architecture discovery

# Acceptance Criteria

- A fixed in-scope inventory exists before the broader migration begins.
- All listed artifacts are migrated using one documented homogeneous mapping for contract, plan, current state, and evidence ownership.
- No in-scope artifact is left in a mixed-ownership state once this tracker closes.
- The migration leaves a clear baseline for any later follow-on cleanup.

# Validation

- A migration checklist covering every in-scope artifact.
- Focused validation on representative migrated artifacts, plus the checks needed to show the fixed inventory is fully covered.