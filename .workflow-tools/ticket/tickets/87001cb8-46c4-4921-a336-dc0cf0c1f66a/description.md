# Summary

Extend `doc-api` with documentation-validation identities and coverage metadata that can satisfy or block spec acceptance clauses.

# Why

`doc-api` is the one evidence-owning store that already exists. It should become the first implemented source of authoritative documentation evidence instead of waiting on the broader validation stack.

# Scope

- add native `doc-api` records for authored-document checks, generated-guidance checks, manual verification steps, and explicit documentation coverage gaps
- define the minimal identifiers and link fields needed to attach those records to spec acceptance clauses and related tickets
- keep the slice focused on store-owned model and validation behavior rather than full transport rollout

# Assumptions To Prove

- `doc-api` can represent both executable and manual documentation evidence without conflating them
- explicit coverage-gap reporting can participate in spec blockage reporting without requiring a complete documentation linter first
- the first slice can stay within `doc-api` and its tests without solving every CLI/MCP/HTTP surface immediately

# Acceptance Criteria

- `doc-api` records represent authored-doc checks, generated-guidance checks, manual verification steps, and explicit coverage gaps.
- The records expose stable identifiers and link metadata that specs can use to resolve acceptance evidence.
- Missing or partial documentation coverage can be represented explicitly rather than only as free-form prose.
- The resulting model is sufficient for the pilot migration and downstream audit rollups to consume.

# Validation

- Focused crate-level tests for the new `doc-api` entities and link behavior.
- Focused regression checks for existing `doc-api` behavior that should remain unchanged.