# Problem

The repository has the pieces of a query engine, but not the complete interface needed for focused ticket discovery.

Current technical state:

- memory-api already owns a shared query AST and Tantivy translation layer.
- ticket-api already owns workflow ranking and deterministic candidate ordering.
- CLI, MCP, and HTTP adapters still split query concerns across separate flag sets, ad hoc title filtering, equality-only field filters, and transport-specific request structs.

This follow-on work should turn those building blocks into one more complete query interface instead of layering more bespoke filter flags onto each command.

# Scope

1. Extend the shared query model and parser so it can represent the approved expressive query contract: boolean combinators (and, or, not), deep field predicates, comparison operators (contains, gt, gte, lt, lte, range, existence), and text-search clauses.
2. Apply that shared query model to ticket surfaces: ticket list, ticket search, workflow selection (next/board), MCP request types, HTTP query types.
3. Add explicit ordering support where the spec allows it, preserving specialized workflow ranking where that remains the default contract.
4. Keep ordering deterministic and close to iterator-style semantics: query/scope filtering first, then ordering, then truncation/limit.
5. Add focused regression coverage for parser behavior, storage evaluation, adapter parity, and workflow filtering/ordering interactions.

# Acceptance Criteria

- The shared query layer supports the new operators and deep-field predicates defined by the spec.
- Ticket adapters no longer duplicate incompatible query semantics for list/search/next/board flows.
- Workflow surfaces can consume the expressive query contract for narrowing candidate sets, with ordering behavior matching the spec.
- Explicit ordering clauses work where supported and preserve deterministic tie-break behavior.
- Tests cover combinators, comparisons, text search, deep fields, and ordering interactions.

# Progress

## Slice 1 (done): shared parser layer — comparison operators + deep-field addressing

Contract: spec 08aa283e (ticket-query/expressive-query-and-ordering).

Implemented in memory-api/crates/memory-api/src/model/query.rs:
- Added CompareOp enum (Eq/Contains/Gt/Gte/Lt/Lte/Range/Exists).
- Added ValueExpr::Empty marker for the existence predicate.
- Added Expr::Compare { key, op, value }; kept Expr::Field as the canonical Eq/Range form for backward compatibility (existing parser output and tests unchanged).
- parse_field_value: parses comparison tokens key:~, key:*v*, key:>, key:>=, key:<, key:<=, key:[a TO b], key:? with longest-prefix-first operator matching and a deterministic "missing a value" error.
- normalize_field_path: dotted dynamic addressing x.<type>.<field> normalizes to the canonical flat x_<type>_<field> key before strict validation.
- tokenize: now bracket-aware so unquoted [a TO b] ranges keep their embedded space in one token.

In memory-api/crates/memory-api/src/storage/search.rs:
- expr_to_query handles the new Expr::Compare arm: Contains reuses the substring regex query; Exists matches documents with a non-empty indexed field; ordering comparisons (Gt/Gte/Lt/Lte) and unresolved deep fields degrade to AllQuery (never silently drop candidates) pending fast-field evaluation in a later slice.
- field_expr_to_query handles ValueExpr::Empty.

Tests: memory-api/tools/cli/ticket-cli/tests/contracts_query_parser.rs extended with 9 new cases (contains aliases, longest-prefix comparisons, exists, negated-exists, range stays Field::Range, dotted->flat normalization, strict-mode dotted validation, missing-value error, plain-equality backward-compat). 16/16 pass.

Validation run:
- cargo test -p ticket-cli --test contracts_query_parser -> 16 passed
- cargo check -p memory-api -p ticket-api -p ticket-mcp -p ticket-http -p ticket-cli -> clean
- cargo test -p memory-api search -> passed

## Remaining slices

- Fast-field evaluation of Gt/Gte/Lt/Lte/Range against numeric/temporal fields in storage/search.rs.
- Explicit ordering clause (order:f:dir, multi-key lexicographic, id tie-break) and composition with workflow ranking.
- Expose the shared query model through one adapter at a time: ticket list (replace --where equality-only lowering), search, MCP request types, HTTP query types.
- Workflow-surface integration for next/board (predicate-only narrowing; explicit order as secondary tie-break).
- Adapter parity + workflow composition regression tests.

# Dependencies and Coordination

- Depends on the expressive-query specification ticket (f6aa9048, in-review).
- Aligns with completed selector and workflow-ordering specs (best-next-ordering, blocker-trees) rather than replacing them.

# Likely Surfaces

- memory-api/crates/memory-api/
- memory-api/crates/ticket-api/
- memory-api/tools/cli/ticket-cli/
- memory-api/tools/mcp/ticket-mcp/
- memory-api/tools/http/ticket-http/
- related tests under memory-api/**/tests/
