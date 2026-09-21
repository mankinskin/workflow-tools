# Waypoint 6: Criteria, provider edges, and templates

## Governing specifications

- `.spec/specs/aebcbab4-2827-4ea1-8244-0a2e6277b571/body.md`
- `.spec/specs/ad0685f5-cb35-4c61-b1dc-f69232521e25/body.md`
- `.spec/specs/e82b9727-0ea2-4d1d-ab8a-98141f85caef/body.md`

## Owned implementation surface

- `workflow-tools/spec/crates/spec-api/src/manifest.rs` typed relationship types after W6.1 lands
- `workflow-tools/spec/crates/spec-api/src/store.rs` relationship persistence and validation
- `workflow-tools/spec/crates/spec-api/schemas/specification.toml`
- root-local `.spec/criterion-prefixes.toml` and `.spec/criterion-templates.toml` support

Implement ordinary provider-owned `CriterionArtifact` records, parent-owned composition criteria, provider-owned deterministic root-local edges, and root-local versioned templates. This ticket starts only after W6.1: it is the sole downstream owner of those shared manifest/schema edits. It excludes migration mechanics, TypedTarget/document resolution, health policy, source annotations, ticket governance, and any worktree behavior change.

## Acceptance criteria

1. Criteria require identity/owner/behavior/measurement, resolve one in-root owner, permit absent validation evidence, validate prefix registry entries, and keep parents from copying child/provider criteria.
2. Edges are provider-owned TOML rows with deterministic root-local ids, in-root immutable component endpoints, lexically ordered nonempty provider criterion claims, cycles allowed, and duplicate/overlap/self/cross-root claims rejected.
3. Root-local templates support only `string`, `identifier`, and `component_id` parameters; literal substitution produces exact-version, provider-owned artifacts with idempotence and deterministic collision behavior.
4. Template upgrades use declared review-required binding-map migrations, never implicit latest-version selection or rewrites.

## Focused validation

- `cargo test -p spec-api`
- criterion ownership/prefix/round-trip tests; edge cycle/canonical-id/overlap tests; template family, idempotence, collision, and version-migration tests

## Done condition

Persisted criteria, composition assertions, provider edges, and template bindings form one typed relationship model ready for downstream resolution and health.