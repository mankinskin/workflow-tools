# Waypoint 6: TypedTarget resolution, document provider, and projections

## Governing specifications

- `.spec/specs/ef4cbcd7-9544-4c43-9095-59822b4211b6/body.md`
- `.spec/specs/224f9384-c38f-4d8b-855e-a8b2457887ca/body.md`

## Owned implementation surface

- shared `TypedTarget` parser/resolver in `workflow-tools/spec/crates/spec-api/`
- `workflow-tools/spec/src/cli/` read-only `dump`, `links`, and health projections plus matching spec MCP projection
- `workflow-tools/doc/crates/doc-api/src/lib.rs`, `evidence.rs`, and `workspace.rs`

Implement the v1 typed-target grammar and resolution outcome model, read-only spec CLI/MCP projections, and doc-api's typed repository index/resolver. Spec-api consumes document outcomes only; it must not own document path parsing, repository scans, or a document MCP/HTTP transport. This ticket excludes persistence model changes beyond consuming W6.1's typed fields, migration, relationship creation, health policy, annotations, and ticket gating.

## Acceptance criteria

1. `TypedTarget` accepts exactly `<kind>/v1/<workspace_slug>/<repo_relative_ref>[#<locator>]` for spec, code, ticket, document, component, and criterion; malformed input is a request error while recognized unsupported version/kind is `unsupported`.
2. `spec dump <id>` and `spec links <id>` share one API projection, return v2 structured data or source/normalized-target/resolution/failure detail, and remain read-only.
3. Doc-api persists `(workspace_slug, repo_relative_path)` identity, builds an explicit deterministic repository index/refresh lifecycle, and returns Resolved, Missing, Unsupported, or typed errors without implicit scans/free-form fallbacks.

## Focused validation

- `cargo test -p spec-api`
- `cargo test --manifest-path workflow-tools/doc/Cargo.toml -p doc-api`
- CLI/MCP parity tests for every target kind and resolved/missing/unsupported/cross-workspace result; document scan/refresh/collision/ambiguity fixtures

## Done condition

All structured target consumers use one TypedTarget outcome model, while document indexing and resolution remain exclusively in doc-api.