# Goal

Define and publish the deterministic replay versus profiling evidence boundary so replay/rollback state remains deterministic while performance diagnostics remain queryable and linked.

## Scope

- Classify allowed fields for deterministic replay payloads.
- Classify profiling-only fields that must not enter replay-critical payload state.
- Define enforcement and linkage rules between journals/replay and profiling evidence surfaces.

## Implementation evidence

Implemented in owning observability architecture spec:

- `.spec/specs/aa769a27-2721-4b9d-880c-5c4e2f8136a7/body.md`
  - Section: `Deterministic replay versus profiling evidence boundary`
  - Defines deterministic replay artifacts and allowed content
  - Defines profiling-only artifacts excluded from replay payloads (`*_ms`, `p50/p95/p99`, host telemetry)
  - Defines enforcement rules so resume/rollback behavior derives from deterministic journal state only
  - Defines ff6637f5 checklist evidence expectation for timing-free replay payload snapshots with linked profiling artifacts

Traceability link added in the same spec to this ticket.

## Validation linkage to ff6637f5 checklist outputs

This ticket contributes direct contract coverage for `ff6637f5`:

- Replay payload remains deterministic and excludes profiling-only timing fields.
- Timing and percentile outputs remain available on linked profiling/log artifacts keyed by correlation identifiers.

## Depends-on context

Parent tracker: `84673399`.
Cross-tracker evidence target: `ff6637f5` validation checklist.