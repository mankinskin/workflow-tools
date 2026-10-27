<!-- aligned-structure:v2 -->
# Mission Generated Review Entry and Publication Consistency

## Motivation

Source requirements R1-001–007 ask for a compact, automatically generated
review entry that reaches every delivered artifact, explains promises and
delivery, minimizes reviewer context, and leads from product questions to
evidence without inventing unavailable facts. The A15 observation also shows
that a successful typed publication and the subsequent verification view can
disagree about the complete published set. The observation is a planning input,
not a blanket acceptance of historic specimens.

If this contract is implemented, a reviewer can start at one generated entry,
reach the declared artifact set and verification evidence, and distinguish
transport success from the actual publication state.

## Target Code Location

- [Mission API](../../../../mission/crates/mission-api/src/lib.rs)
- [Mission render tests](../../../../mission/crates/mission-api/tests/roadmap_render.rs)
- [Typed publication adapters](../../../../mission/crates/mission/src/lib.rs)

## Naming Conventions

Use criteria `mission-review-entry-*` and `mission-publication-consistency-*`.
The generated entry is a projection, never a second source of Mission facts.

## Reading Order

1. [Mission Domain](../38a8ea3a-9a6a-4a0f-bc5a-97dc9ba670fc/body.md) — source and projection invariants.
2. [Typed adapters](../9413a0a2-0d53-407d-b088-3953074393fb/body.md) — publication transport semantics.
3. [Mission renderer](../../../../mission/crates/mission-api/src/lib.rs) — implementation and existing output contract.

## Responsibility

Define the compact generated review entry and the complete-set consistency
contract after typed publication.

## Interfaces And Dependencies

Consumes the persisted Mission, ordered waypoints/Parts, generated file
inventory, publication result, and verification diagnostics. It produces one
entry with relative links to the roadmap, each declared artifact, relevant
specifications and validation evidence. It does not duplicate their content.

## Behavior

The entry is generated from current authoritative Mission state. Its links
resolve from the actual consumer location. It gives brief promise/delivery/
verification orientation, maps representative product questions to evidence,
and states `unknown` or `unavailable` where no evidence exists. After a
successful publication, complete roadmap/Part inventories agree byte-for-byte
with the verification renderer, including an empty external-status field.
Transport success and publication state remain separate outcomes.

## Boundaries And Failure Cases

Do not impose a numerical size limit, copy all artifacts into the entry, infer
facts, repair unrelated source records, or convert an empty external status
into fabricated provenance. Missing, changed, extra, dangling, or
destination-invalid files are reported as distinct failures.

## Provider/Consumer Contract

The entry consumes Mission Domain identity/order and typed adapter publication
results. It provides a stable navigation surface to human review and the
dedicated reviewer role. This is composition, not a dependency edge between
future implementation tickets.

## Examples

Given a Mission with a roadmap, three Parts and two validation receipts, the
entry links to all six declared outputs and the receipts, explains which
promise each output serves, and reports an unavailable product fact rather
than guessing. If publication returns transport `ok` but verification reports
a missing Part, the entry/verification result reports drift rather than green
publication.

## Acceptance Criteria

- Every declared generated artifact is reachable from the entry and every
  emitted link resolves from each supported output location.
- Promise, delivery and verification orientation is concise and links to the
  full authoritative sources without copying them.
- Unavailable evidence is explicit; no unsupported product answer is generated.
- Successful publication and verification inventories match exactly; negative
  changed/missing/extra-file cases remain distinguishable.
- Test coverage exercises empty external status and a genuine output drift.

## Guards

`w8-mission-review-entry` is the declared test-api ValidationSpec. It is a
future validation definition; no product test or execution was run in W8.

## Positions

- [Mission API](../../../../mission/crates/mission-api/src/lib.rs) — `partial`: typed
  publication/rendering exists; the compact generated entry and complete-set
  consistency behavior are not established.
- [Mission render tests](../../../../mission/crates/mission-api/tests/roadmap_render.rs) — `partial`:
  renderer tests exist; the acceptance cases above remain to be added.

## Evidence

The guard command is `cargo test -p mission-api --test review_entry_contract`.
It must verify full artifact reachability, representative question navigation,
unknown-fact handling, empty external status and negative output drift.

## Scope

Owns generated review-entry composition and publication/verification
consistency. It does not change agent guidance, accept historic review rows,
or implement unrelated Mission rendering.
