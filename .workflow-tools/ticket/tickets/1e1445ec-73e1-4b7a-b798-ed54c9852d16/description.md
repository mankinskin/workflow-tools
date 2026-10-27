# Mission generated review entry and publication consistency

## Objective

Implement a compact Mission-generated review entry and make typed publication
and complete-set verification agree. Keep the entry a projection of
authoritative Mission state.

## Requirements

- Generate one compact entry with links to every declared output, promise,
  delivery and validation evidence.
- Navigate representative product questions through evidence; state unknown or
  unavailable where facts are absent.
- Resolve links from actual consumer destinations and retain explicit
  missing/changed/extra file failures.
- After successful publication, verify the roadmap/Part set byte-for-byte,
  including empty external status; distinguish transport `ok` from publication
  state.
- Do not infer acceptance of historical specimen rows or copy artifact text.

## Design and boundaries

Use the existing typed Mission and adapter contracts; no new renderer/backend
or human review automation. Output remains generated, concise, and
evidence-linked.

## Acceptance criteria

- Complete generated artifact reachability is checked from one entry.
- Entry links, promise/delivery/verification orientation and explicit unknowns
  pass focused tests.
- Successful publish and verification render identical complete file sets;
  changed/missing/extra cases remain distinct.
- No historical review item is accepted by implication.

## Validation plan

Future guard `w8-mission-review-entry`:
`cargo test -p mission-api --test review_entry_contract`.
No product validation was run during this planning handoff.

## Sources and non-goals

R1-001–007; F01 is an observed discrepancy, not an independently inferred
human disposition. Link approved Typed Mission Transport and Publication
Adapters. No product implementation or evidence rerun in W8.
