<!-- aligned-structure:v2 -->
# Whole-State Mission Generation History and Separate Approval Events

## Motivation

Input requirements R2-001–012 request coherent whole-state revision labels,
canonical current paths, preserved/reconstructable history with explicit gaps,
separate continuation inputs, generated history, and a JSON contract before a
separate JSON approval/reaction event. Existing Mission revision history and
create-new writes are prior art; this contract adds generated dossier/layout
history and an honest human-reaction event rather than claiming all history is
absent.

If implemented, consumers can distinguish a coherent current generated state,
its recoverable prior states and the requester's separate recorded reaction.

## Target Code Location

- [Typed Mission source and history](../../../../mission/crates/mission-api/src/typed_source.rs)
- [Mission API persistence](../../../../mission/crates/mission-api/src/lib.rs)
- [Mission documentation](../../../../mission/README.md)

## Naming Conventions

Use `mission-output-revision-*` and `mission-approval-event-*` criteria.
Revision labels identify complete states, not per-file counters. A
human-reaction `signature` is an explicit contract-binding value, not a
cryptographic signature.

## Reading Order

1. [Mission Domain](../38a8ea3a-9a6a-4a0f-bc5a-97dc9ba670fc/body.md) — source identity and history.
2. [Typed lifecycle](../9fd527d8-e290-44e3-9edb-9f7d6658551a/body.md) — persistence and immutable history.
3. [Typed file-backed authoring](../dee92faf-b649-4577-8016-e6e465e6724a/body.md) — generated file composition.
4. [Mission source](../../../../mission/crates/mission-api/src/typed_source.rs) — existing revision history.

## Responsibility

Define complete-state output revisions, canonical current/archive layouts,
human-readable history and the separate JSON contract/reaction-event formats.

## Interfaces And Dependencies

Consumes a Mission's current typed source, complete generated file set,
metadata and immutable historical revisions. It emits a pre-approval JSON
contract with file BOM/checksums and a separate human event that binds to that
contract by identity/digest and records decision and event metadata.

## Behavior

One revision label identifies one coherent generated state; current files stay
at canonical locations and prior versions are archived under a unique
revision-specific location. Existing revisions are never overwritten.
History renders available old states and explicitly marks unrecoverable gaps.
Complementary inputs remain independent continuations: multipart inputs live
under `inputs/`, only the merged request remains at the root, and current
waypoint Parts live under `waypoints/` with an index/map where useful. Other
multipart documents use a folder and nearby index/map where that improves
navigation. A concise automatic note on revision creation is optional. The
JSON contract
precedes approval and contains captured files, checksums and metadata. A
separate event records `approve` or `changes-requested`, the contract-linked
signature value and event metadata; the event does not repeat the BOM.

## Boundaries And Failure Cases

Do not fabricate missing revisions, rewrite occupied archives, collapse
complementary inputs, backdate or reuse a previous approval, infer
cryptographic identity, or confuse publication locks with human decisions.
Invalid JSON, digest mismatch, missing contract, unsupported decision and
duplicate revision label fail without overwriting prior state.

## Provider/Consumer Contract

Consumes the Mission API's immutable source/history and file-backed output
contract. It provides coherent revisions and a separately auditable requester
decision event to Mission generation and review. This does not authorize
Mission writes or implement any approval event during W8.

## Examples

A contract records revision `v4`, the generated roadmap and Parts, byte sizes,
SHA-256 values and captured metadata. A later separate event records
`changes-requested`, timestamp, actor source and a digest link to that exact
contract, with no copied file inventory and no claim of cryptographic signing.

## Acceptance Criteria

- Revision labels identify complete coherent output states and occupied
  revisions are never overwritten.
- Current files remain canonical; history can reconstruct every available
  state and explicitly identifies gaps.
- Continuation inputs remain separate; individual inputs are stored under
  `inputs/`, the merged request remains at the root, and current Parts are
  stored under `waypoints/` with a nearby map where useful.
- History output is generated from preserved state. A brief revision-created
  note is optional and cannot replace the history record.
- A JSON pre-approval contract records the complete file BOM, digests, sizes
  and required metadata.
- A separate JSON human event records only the decision, contract-binding
  value and event metadata; it never duplicates the BOM or claims cryptographic
  signing.
- Invalid/mismatched/duplicate records fail without altering prior history.

## Guards

`w8-mission-revision-history-event` is the declared test-api ValidationSpec.
No Mission record, product test or execution was run in W8.

## Positions

- [Typed Mission source](../../../../mission/crates/mission-api/src/typed_source.rs) — `partial`:
  accepted Mission history and revision preconditions exist; complete generated
  output-history and separate human-reaction formats are not established.
- [Mission API persistence](../../../../mission/crates/mission-api/src/lib.rs) — `partial`:
  current rendering and persistence exist; this whole-state contract is not
  established.

## Evidence

The guard command is
`cargo test -p mission-api --test revision_history_approval_event_contract`.
It must cover whole-state snapshots, create-new collision rejection, explicit
gaps, BOM digest/size validation, separate decision events, and invalid or
replayed event handling.

## Scope

Owns future Mission output layout/history and contract/reaction records. It
does not revise this dossier's immutable approvals or create a Mission.
