# Summary

Redefine the specification contract and the native `spec-api` model so the repository can represent expected properties, acceptance clauses, and evidence requirements without relying on free-form prose alone.

# Why

Today the format pressure comes from prompts and generated guidance, while the store itself only knows generic spec metadata. That gap forces contract semantics into markdown sections and makes transport behavior inconsistent.

# Scope

- define the blackbox authoring contract for expectation-oriented specs
- add native `spec-api` fields and validation for the contract
- expose the new model consistently through CLI, MCP, and HTTP surfaces

# Assumptions To Prove

- authoring rules can be tested from observable create/get/update/health behavior
- the spec store can grow richer contract metadata without breaking existing body and section workflows
- transport parity can be demonstrated against one shared fixture set

# Acceptance Criteria

- The child tickets under this tracker define the contract, the native store model, and the public transport behavior together.
- The resulting spec contract can be evaluated from structured store data instead of requiring free-form rollout prose.
- The new model keeps existing spec body and section workflows compatible while adding the missing contract semantics.

# Validation

- `ticket.exe health <tracker-id> --workspace-root . --json`
- Child tickets provide the failing and passing behavior-scoped checks for the contract and transport surfaces.