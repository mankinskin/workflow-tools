# Problem

The ticket query surface is still fragmented and under-specified.

Current behavior is split across several partially overlapping mechanisms:

- `ticket list --where key=value` supports only repeated equality filters.
- `ticket search` uses the shared `memory-api` query parser, but that parser currently supports only `AND`, bare full-text tokens, exact field predicates, and simple string ranges.
- workflow discovery surfaces (`ticket next`, `ticket board show`, MCP next/board, HTTP workflow endpoints) apply custom scope narrowing and custom ordering instead of consuming one expressive query contract.
- ordering is inconsistent: `list` implicitly sorts by effort, workflow surfaces use dependency-convergence ranking, and there is no clear contract for when explicit ordering should override defaults.

The requested improvement is broader than the earlier scoped-selector work: users need a refined query interface that can express deep field search, common text-search patterns, logical combinators, comparison operators such as contains / greater-than / not, and predictable ordering semantics that stay close to Rust iterator-style pipeline behavior without degrading UX.

# Scope

1. Own the end-to-end query-interface expansion across query language, adapter surfaces, ordering, and validation.
2. Coordinate the specification and implementation tickets for:
   - logical combinators (`and`, `or`, `not`)
   - richer predicates (`contains`, comparison operators, range semantics, existence checks if needed)
   - deep field paths over structured ticket metadata
   - explicit text-search integration
   - ordering clauses and stable tie-break semantics
   - workflow-surface integration for `next` and `board`
3. Keep prior selector and workflow tickets as historical prerequisites, but treat this tracker as the place where the more complete query interface is defined and delivered.

# Acceptance Criteria

- A child spec ticket defines the canonical expressive query contract and ordering semantics.
- A child implementation ticket applies that contract to the shared query/parser layer and ticket adapters.
- Existing next-work trackers depend on this tracker where relevant so the dependency graph shows the new scope clearly.
- This tracker remains open until the spec and implementation children are done and validated.

# Related Existing Work

- closed selector contract work around board / next scope narrowing
- workflow ordering and HTTP transport tickets
- existing `ticket list --where` structured equality filtering
- existing `ticket search` parser in `memory-api`
