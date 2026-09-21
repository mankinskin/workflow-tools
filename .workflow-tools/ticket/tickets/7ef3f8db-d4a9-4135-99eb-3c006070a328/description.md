## Objective
Implement the shared directed schema engine required by [e9c38d24 Schema modernization lifecycle and migration](.spec/specs/e9c38d24-42cc-4044-8b2c-6811b918530f/spec.toml).

## Requirements
- Add strict linear zero-or-one-parent inheritance with atomic full-load/reload validation for missing parents and cycles.
- Resolve child lifecycle graphs as category-contained refinements of universal `plan`, `act`, and `verify` nodes.
- Require exactly one plan-category root per resolved schema and a valid path through all categories for each concrete work-item type.
- Enforce directed lifecycle edges: same category, `plan→act`, `act→verify`, `verify→act`, and `act→plan`; reject skips.
- Keep lifecycle graphs distinct from relation graphs; ticket/spec/rule registries reuse primitives but keep separate schema-kind namespaces and local graphs.
- Make `cancelled` a derived verify terminal leaf; terminal nodes have no outgoing edge.
- Explicit reload atomically swaps registry generation, resolved caches, manifest/catalog index, and client cache version, or retains the prior valid generation.

## Acceptance Criteria
Focused tests prove directionality, inheritance resolution, category containment, terminal validation, atomic reload rollback, and cache invalidation across ancestor changes.


## Decision-Complete Validation Contract
- Define explicit resolved-schema entry and terminal semantics, including each category-refinement boundary.
- Require reachability of every resolved lifecycle node from the one global plan entry and a valid terminating path through `plan`, `act`, and `verify`.
- Treat category refinement as a contained tunnel: validate permitted boundary edges and reject bypasses, skipped categories, and illegal category escapes.
- Validate only declared rework/replan loops; test permitted `verify→act` and `act→plan` loops separately from forbidden loops.
- Preserve five distinct model concepts: schema type, concrete lifecycle state, lifecycle category, ticket relation/dependency edge, and validation gate. Tests must reject cross-namespace or graph-semantic conflation.

## Additional Acceptance Criteria
Focused tests cover entry/terminal boundaries, all-node reachability, contained refinement tunnels, allowed and disallowed loops, illegal escape rejection, and five-way concept separation.
Focused tests prove cancellation direct-entry/terminal behavior, separate ticket/spec/rule local graphs and namespaces, and relation-edge validation independence from lifecycle rules.
Rule validation proves the governing rule resolves, links the owning spec and validation guards, and preserves separate ticket/spec/rule local graphs and type-ID namespaces.


## Recovered Interview Requirements
- Model `cancelled` as a derived `verify` terminal leaf with no outgoing edge; allow direct entry to `cancelled` from `plan`, `act`, or `verify` only as the sole cross-category exception.
- Reuse directed-lifecycle primitives for ticket, spec, and existing rule schemas while keeping independent local graphs and per-kind type-ID namespaces.
- Relation-edge validation checks declared relation-kind rules and endpoint existence, but never applies lifecycle categories or lifecycle directionality.


## Governing Policy Rule
Create or link the governing policy rule that introduces [e9c38d24 Schema modernization lifecycle and migration](.spec/specs/e9c38d24-42cc-4044-8b2c-6811b918530f/spec.toml) and its lifecycle-validation guards. The rule must apply the shared directed-lifecycle primitives to existing rule schemas without creating a new rule entity.


## Acceptance Criteria (from planning ticket 9e450826, 2026-08-06)

### Target files
- `memory-api/crates/memory-api/src/model/schema.rs` — shared source/resolved inheritance and lifecycle primitives.
- `memory-api/crates/memory-api/src/model/schema_registry.rs` — candidate validation, immutable generation, cache, manifest, and atomic reload.
- `memory-api/crates/memory-api/src/error.rs` — deterministic schema-load and lifecycle diagnostics.
- `memory-api/crates/ticket-api/src/model/schema_registry.rs` — ticket registry adapter over shared atomic reload.
- `memory-api/crates/ticket-api/src/model/default_schema.rs` — delivered legacy schema registration compatibility.
- `memory-api/crates/ticket-api/src/storage/store.rs` — independent dependency relation gate and directed lifecycle resolution.
- `memory-api/crates/ticket-api/src/storage/tests/update_regression_tests.rs` — ticket update boundary regressions.
- `memory-api/crates/ticket-api/src/storage/tests/workflow_tests.rs` — dependency-progress relation-gate regressions.

### Numbered acceptance criteria
1. Loading a derived schema with one declared parent resolves a linear parent chain and materializes every resolved lifecycle node with its inherited `plan`, `act`, or `verify` category; declaring more than one parent is rejected.
2. Loading a schema whose declared parent does not exist fails the whole load with a diagnostic naming the source and parent, and leaves the prior registry generation available.
3. Loading schemas that form a parent cycle fails the whole load with a diagnostic naming the cycle/parent chain, and leaves the prior registry generation available.
4. Loading duplicate type IDs within one schema kind fails the whole load and reports the colliding source paths deterministically; the same type ID remains permitted in a different ticket/spec/rule namespace.
5. A lifecycle-enabled concrete work-item schema with no directed path that visits `plan`, then `act`, then `verify` is rejected during load.
6. A resolved lifecycle-enabled work-item schema with zero or more than one `plan` node having no inbound lifecycle edge is rejected during load.
7. A declared terminal lifecycle node with an outgoing lifecycle edge is rejected during load.
8. Runtime transition validation accepts only declared directed lifecycle edges: within-category, `plan -> act`, `act -> verify`, `verify -> act`, and `act -> plan`; reversing a declared normal edge or skipping a category is rejected.
9. `cancelled` is accepted only as a derived `verify` terminal leaf with no outgoing edge, and a direct transition to `cancelled` from a `plan`, `act`, or `verify` node is accepted as the sole cross-category bypass.
10. An explicit `reload_from_dir` that encounters parse, missing-parent, cycle, duplicate-type-ID, refinement, category-path, root, or terminal validation failure retains the entire previous resolved registry, manifest, cache contents, and `registry-v<generation>` version.
11. An explicit successful reload recomputes each affected resolved schema, manifest record, and cache entry when its own source hash or any ancestor source hash changes, then exposes all replacements under one advanced generation.
12. `TicketStore::update` executes `enforce_dependency_progress_gate` before directed lifecycle path resolution; unresolved `depends_on` behavior, cancellation, on-hold, and non-forward transitions retain the prior relation-policy outcome, while relation edges are not read by lifecycle traversal.
13. `spec-api` and `rule-api` compile and retain separate local graphs/type-ID namespaces while consuming the shared lifecycle model and registry generation API; legacy schemas without `lifecycle` remain loadable without category enforcement.

### Validation commands
- `cargo test -p memory-api --lib` — verified
- `cargo test -p ticket-api --lib` — verified
- `cargo test -p spec-api --lib` — verified
- `cargo test -p rule-api --lib` — verified

### Explicit non-goals
- Dual TOML/JSON parsing, lexical mixed-format ordering, and TOML/JSON fixture parity (Track 2).
- Generated catalogs, shipped-ticket JSON conversion, deleting shipped TOML, catalog-index generation, and catalog publishing (Track 3).
- CLI, MCP, HTTP, VS Code, or other client cache-version transport/invalidations (Track 4).
- Legacy-record migration, active-record preflight, historical exemption removal, remediation approvals, and cutover execution (Track 5).
- Release-wide validation matrix and corrective migration batches (Track 6).
- New rule entities or lifecycle-category constraints on relation edges.
