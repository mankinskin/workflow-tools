## Objective
Implement the kernel-owned core `DomainStore` contract and typed operation capabilities defined by the existing workspace Spec.

## Scope
- Define a domain-neutral `DomainStore` core trait exposing domain identity, selected workspace/store resolution, diagnostics, and explicit read-only versus create/open behavior.
- Define typed `CreateEntity`, `ReadEntity`, `UpdateEntity`, and `DeleteEntity` capabilities, with `ListEntities` as a separate optional capability.
- Keep kernel-owned types free of domain API crate dependencies. Preserve domain-specific identifiers, inputs, entities, patches, results, errors, and extensions.
- Integrate the resolver semantics after existing selector and path tickets `31e70dab`, `6cf90b00`, and `db3ebe07` are completed; do not duplicate those bug scopes.

## Acceptance criteria
1. The core trait is usable by domain APIs without depending on Ticket, Spec, Test, Session, Feedback, Audit, or Log crates.
2. Capability traits encode only operations the implementing API actually supports and do not impose a universal entity schema.
3. Read-only opens do not initialize stores; workspace-based writes resolve to canonical storage and carry structured diagnostics.
4. Trait and resolver tests cover active-domain identity, explicit access mode, diagnostics, and the existing workspace contract.
5. Rule remains excluded.

## Validation
Run `cargo test --manifest-path workflow-tools/memory-kernel/Cargo.toml` and record focused test results against the workspace Spec.