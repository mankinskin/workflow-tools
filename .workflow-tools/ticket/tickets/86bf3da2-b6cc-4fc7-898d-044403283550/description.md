# Summary

Bootstrap the first `test-api` entities for validation specifications, executions, and outcomes used by expectation-oriented spec fulfillment.

# Why

Spec acceptance cannot become store-owned if executable validation still lives only as ad hoc command text. The first slice needs a minimal but real `test-api` model.

# Scope

- add the first `test-api` crate and workspace wiring needed for native validation identities
- define minimal entities for validation specifications, executions, and outcomes such as `passed`, `failed`, and `blocked`
- define the minimal link metadata needed to connect validation executions to specs, tickets, docs, and future logs
- keep the slice focused on store-owned model and tests rather than full CLI/MCP/HTTP rollout

# Assumptions To Prove

- the first `test-api` slice can stay small and still provide stable identities for downstream evidence linking
- outcomes such as `passed`, `failed`, and `blocked` are enough for the pilot and audit rollups before richer status modeling exists
- the new crate can be introduced without forcing every transport surface to land in the same ticket

# Acceptance Criteria

- A minimal `test-api` crate exists in the workspace.
- Native entities exist for validation specifications, executions, and outcomes such as `passed`, `failed`, and `blocked`.
- The first slice exposes stable identifiers and link metadata for specs, tickets, docs, and future logs.
- The resulting model is sufficient for the pilot migration and downstream audit rollups to consume.

# Validation

- Focused model and linkage tests for the first `test-api` entities.
- A narrow workspace-level validation that the new crate integrates cleanly into the existing build.