# Mission evidence provenance, file roles, and destination references

## Objective

Define and implement provenance, file-role classification and
consumer-destination-aware references for new Mission outputs.

## Requirements

- New output metadata identifies producer, purpose, invocation, inputs,
  outputs and grouping; new tools/changes stay in the target repository unless
  separately approved.
- Classify authored inputs, generated projections and helper outputs with
  explicit path, format and schema guarantees.
- Resolve links from actual consumers and validate canonical owning-root
  context; unknown, dangling, escaping or wrong-root references fail.
- Preserve historical evidence and state missing historic provenance rather
  than inventing it.
- Keep A15's reported consumer failure distinct from an unreproduced cause.

## Design and boundaries

Reuse Mission schema, file-backed authoring and path validation. Do not
retroactively relocate evidence, prescribe one directory spelling, or turn
session-state into a helper workspace.

## Acceptance criteria

- Producer metadata and file roles are validated for every new generated
  dataset.
- Every emitted reference resolves from all supported consumer locations.
- Negative missing, wrong-root, unknown-role, unsupported-format and path
  escape cases fail with explicit diagnostics.
- Historical source/evidence bytes remain unchanged.

## Validation plan

Future guard `w8-mission-evidence-provenance`:
`cargo test -p mission-api --test evidence_reference_contract`.
No product validation was run during this planning handoff.

## Sources and non-goals

R1-010–021 and R1-031–034; A19–A22 and A25. Link approved Mission Domain and
Typed File-Backed Mission Authoring. No product implementation or history
cleanup in W8.
