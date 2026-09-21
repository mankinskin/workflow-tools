# Summary

Extend `spec-api` with native fields and validation for expected properties, acceptance clauses, and evidence requirements.

# Why

Without a first-class store model, the contract remains markdown folklore. The repository needs structured spec data that can be queried, validated, and linked from other stores.

# Scope

- store the first implementation slice as spec-owned structured manifest data in `spec.toml`, with stable identifiers for expected properties, acceptance clauses, evidence requirements, and fulfillment summaries
- keep explanatory prose, rationale, and human-oriented narrative in `body.md` and `sections/*.md`
- extend health and validation behavior so missing or unsatisfied requirements surface mechanically
- preserve existing body, section, and generated-document behavior

# Assumptions To Prove

- manifest-backed structured metadata is sufficient for the first slice and does not require a second spec-owned artifact store
- richer contract metadata can coexist with existing manifests and section files
- the store can evaluate missing evidence or unsatisfied acceptance clauses without forcing every spec into the same authored prose
- existing generated-document capabilities remain compatible with the richer model

# Acceptance Criteria

- The first slice chooses manifest-backed structured contract data under `spec-api` control rather than introducing a separate wrapper artifact path.
- `spec-api` stores native expectation and evidence metadata instead of requiring the contract to live only in markdown prose.
- Health or equivalent validation surfaces missing expected properties, missing acceptance clauses, and missing required evidence deterministically.
- Existing authored `body.md`, `sections/*.md`, and generated spec artifacts continue to work.
- The resulting model is queryable enough for transport parity and downstream evidence integration work.

# Validation

- Focused unit and integration tests for the new `spec-api` fields and health behavior.
- Focused regression checks for existing spec create/update and generated-document flows.