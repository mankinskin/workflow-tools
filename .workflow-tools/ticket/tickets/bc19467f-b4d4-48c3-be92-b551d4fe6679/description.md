# Summary

Coordinate the rollout that redefines specifications around intended properties, acceptance criteria, and store-owned evidence while migrating the affected specs and tickets homogeneously.

# Why

The current workflow stores specs as largely unstructured markdown plus lightweight metadata. That makes it too easy to mix product contract, implementation plan, current-state notes, and review evidence into one document. The intended architecture in this repository already points toward store-owned workflow metadata across `spec-api`, `doc-api`, future `test-api`, future `log-api`, and derived rollups in `audit-api`, but there is no ticket track that turns that direction into one coherent delivery program.

# Scope

- redefine the spec contract around expected properties, acceptance clauses, and evidence requirements
- add the native store model and transport parity needed to expose that contract through the default spec surfaces
- integrate store-owned documentation, validation, and log evidence rather than wrapper-owned artifacts
- report derived fulfillment status through `audit-api`
- pilot the new contract and migrate the affected specs and tickets homogeneously

# Assumptions To Prove

- the current markdown-oriented spec format can remain the authoring shell while the underlying definition changes
- first-class expectation and evidence metadata can be added to `spec-api` compatibly with existing stores
- `doc-api` can be extended and `test-api` / `log-api` can be bootstrapped incrementally without reintroducing wrapper-owned truth
- `audit-api` can stay a derived reporting surface rather than becoming the source of truth for fulfillment state
- one pilot migration can prove the contract before the broader homogeneous migration proceeds

# Acceptance Criteria

- The child tickets under this tracker define one coherent implementation path for contract definition, store modeling, evidence integration, audit rollups, and migration.
- The rollout is explicitly blackbox-tested rather than relying on prose review alone.
- The pilot migration proves the target contract end to end before broader homogeneous migration work is considered complete.
- The migration scope explicitly includes both specs and the tickets that currently carry spec-like contract text.

# Validation

- `ticket.exe health <tracker-id> --workspace-root . --json`
- Child tickets define the focused blackbox tests, transport checks, and migration validations they must satisfy before this tracker can close.