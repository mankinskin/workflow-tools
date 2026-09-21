# Waypoint 6: Rust source annotations and offline resolution

## Governing specification

- `.spec/specs/7766e61d-dea9-4292-bde5-dfc287b8da3b/body.md`

## Owned implementation surface

- `workflow-tools/spec/crates/spec-annotation/` new proc-macro crate
- spec workspace annotation registration/offline resolver and health integration
- `workflow-tools/spec/crates/spec-api/src/code_ref.rs` fallback integration only

Implement `#[implements(component_id = "...")]` and `#[validates(criterion_id = "...")]` compile-time syntax checks, discoverable registration metadata, and an offline resolver. It consumes W6.1 component identities and W6.2 criterion identities/edges. Local variables are expressly excluded. It must not create identities, change criterion ownership, or make `CodeRef` authoritative when a valid annotation exists.

## Acceptance criteria

1. The dedicated proc-macro validates supported attribute syntax at compile time and emits offline-discoverable registration metadata without executing the annotated crate.
2. Resolver support covers structs, enums, traits, impl methods/associated functions, free functions, consts, and statics; each resolution reports parsed item identity and current file/line range.
3. Resolution validates immutable component ids and provider-owned criterion ids, makes valid annotations authoritative, retains CodeRef only as absent-annotation navigation fallback, and reports unknown/malformed/conflicting/unsupported/unresolved findings.

## Focused validation

- proc-macro compile-pass/compile-fail fixtures
- offline resolver fixtures for each supported item, movement of source locations, unknown ids, duplicate conflicts, and CodeRef fallback
- focused spec health integration tests

## Done condition

Supported Rust items can supply durable component/criterion traceability, resolved offline from the persisted v2 model.