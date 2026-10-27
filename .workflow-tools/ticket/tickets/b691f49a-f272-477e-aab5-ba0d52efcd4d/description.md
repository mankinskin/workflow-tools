# Whole-state Mission generation history and separate approval events

## Objective

Define whole-state generated revisions, reconstructable history, and a
pre-approval JSON contract followed by a separate JSON human-reaction event.

## Requirements

- A revision labels one coherent state; current files stay canonical and
  previous versions use unique revision-specific archives.
- Never overwrite occupied history. Reconstruct available states and mark
  gaps explicitly.
- Keep complementary inputs separate; store input parts under `inputs/`, keep
  the merged request at the root, and current waypoint Parts under
  `waypoints/`. Use a nearby index/map for multipart documents where useful.
- Generate a human-readable history. A small automatic revision-created note
  is optional and does not replace history.
- Capture a JSON contract before approval with a complete BOM, file hashes,
  sizes and metadata.
- Record a separate `approve` or `changes-requested` event containing
  contract-linked signature value and event metadata, without repeating the
  BOM or asserting cryptographic signing.

## Design and boundaries

Reuse Mission API history and create-new preconditions. Do not fabricate old
state, backdate approval, change immutable receipts, or treat a publication
lock as human consent.

## Acceptance criteria

- Whole-state identity and collision rejection are tested.
- History reconstructs retained states and visibly identifies gaps.
- JSON contract BOM and metadata validate before any reaction event.
- Separate event binds to exact contract and preserves actor/time/decision
  metadata without BOM duplication or cryptographic claims.
- Invalid/mismatched/repeated records fail without prior-state mutation.

## Validation plan

Future guard `w8-mission-revision-history-event`:
`cargo test -p mission-api --test revision_history_approval_event_contract`.
No Mission record, product test or execution was run in W8.

## Sources and non-goals

R2-001–012; existing history is prior art, not absence. No Mission creation
or mutation, product implementation, or rewrite of this dossier's approvals.
