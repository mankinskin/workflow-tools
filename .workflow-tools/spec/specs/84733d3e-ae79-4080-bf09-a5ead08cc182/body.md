<!-- aligned-structure:v2 -->
# Mission Evidence Provenance, File Roles, and Destination References

## Motivation

Source requirements R1-010–021 and R1-031–034 require consumer-correct
references, clear evidence roles and provenance, repository-local producers,
classified authored/generated/helper files, explicit file guarantees, and
unambiguous identifiers. Existing Mission schema and relative-link support are
prior art; this contract adds complete destination-aware reachability and
provenance rather than claiming those basics are absent.

If implemented, each artifact has an understandable role and origin, and every
reference can be resolved from its actual consumer location or fails explicitly.

## Target Code Location

- [Mission API and validation](../../../../mission/crates/mission-api/src/lib.rs)
- [Typed Mission source](../../../../mission/crates/mission-api/src/typed_source.rs)
- [Mission documentation](../../../../mission/README.md)

## Naming Conventions

Use `mission-artifact-*`, `mission-provenance-*`, and
`mission-reference-*` criteria. Authored input, generated projection, helper
output and preserved historical evidence are distinct roles.

## Reading Order

1. [Mission Domain](../38a8ea3a-9a6a-4a0f-bc5a-97dc9ba670fc/body.md) — identity, schema and reference invariants.
2. [Typed file-backed authoring](../dee92faf-b649-4577-8016-e6e465e6724a/body.md) — existing authored-source/file composition.
3. [Typed adapters](../9413a0a2-0d53-407d-b088-3953074393fb/body.md) — transport-owned operations.
4. [Generated review entry](../87d20edd-240d-4533-b7ba-711b06bf8296/body.md) — artifact reachability consumer.
5. [Mission model](../../../../mission/crates/mission-api/src/lib.rs) — current managed file behavior.

## Responsibility

Define repository-local producer provenance, Mission file-role classification,
format/layout guarantees, and reference resolution against actual consumer
destinations.

## Interfaces And Dependencies

Consumes typed Mission source, producer identity/invocation metadata,
repository-relative paths, file role declarations, and consumer destinations.
It provides validated file inventories and resolved references to publication,
the review entry, and downstream planning/rendering.

## Behavior

Every newly generated dataset identifies its producer, purpose, invocation,
inputs, outputs and grouping. New helper tools and changes reside in the target
repository unless a separate express exception exists. Session-state is a
source of session information, not a helper workspace. Mission files identify
authored input versus generated output versus helper artifact; each role has
declared format, location and schema guarantees. Links resolve relative to the
actual consumer, and identifiers include enough owning-root context to avoid
ambiguity.

## Boundaries And Failure Cases

Do not retroactively relocate historical evidence or claim missing historical
provenance exists. Preserve the report that A15 had a consumer-resolution
problem without asserting an unreproduced mechanism. Missing producer data,
unknown file roles, unsupported formats, dangling references and destination
escape are explicit validation failures. Do not prescribe a universal
directory spelling merely because one example is useful.

## Provider/Consumer Contract

Consumes existing Mission Domain schema and file-backed authoring interfaces.
Provides validated artifact/reference metadata to Mission publication and the
generated review entry; no ordering edge is implied by shared metadata.

## Examples

A repository-local producer records its command, input manifest and output
paths beside a generated evidence bundle. A Markdown reference resolves from
both the generated roadmap and a consumer copy; a missing target fails with
the owning Mission path and consumer location.

## Acceptance Criteria

- New outputs identify the generating program independently of implementation
  language and carry purpose/invocation/input/output provenance.
- Every Mission file is classified as authored, generated or helper output and
  validated against its declared path, format and schema guarantees.
- Each emitted reference resolves from every supported consumer destination;
  dangling, wrong-root, escaping and unknown references fail explicitly.
- The A15 user-reported issue remains a reported case until its consumer
  mechanism is independently reproduced.
- Historical evidence remains immutable; absence of historic metadata is
  visible rather than backfilled by inference.

## Guards

`w8-mission-evidence-provenance` is the declared test-api ValidationSpec.
No product validation was executed during W8.

## Positions

- [Mission API](../../../../mission/crates/mission-api/src/lib.rs) — `partial`: typed path
  validation and relative references exist; full role/provenance and
  destination-consumer guarantees are not established.
- [Typed Mission source](../../../../mission/crates/mission-api/src/typed_source.rs) — `partial`:
  typed source exists; artifact-role provenance completeness is not established.

## Evidence

The guard command is
`cargo test -p mission-api --test evidence_reference_contract`. It must cover
role/schema validation, producer metadata, two consumer locations, and
missing/dangling/escape cases.

## Scope

Owns Mission artifact roles, provenance and consumer-correct references. It
does not mutate source specimens, move historical evidence, or create a
cross-workflow file backend.
