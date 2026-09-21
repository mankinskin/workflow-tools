# Goal

Define and publish the governance policy for profiling metadata retention, rotation/sampling, and redaction across observability artifacts.

## Scope

- Retention and rotation defaults for runtime logs and profiling artifacts.
- Sampling policy boundaries by log level.
- Redaction/hashing rules for sensitive and high-cardinality fields.
- Evidence obligations for profiling tickets and checklist handoff to `ff6637f5`.

## Implementation evidence

Policy implemented in owning observability architecture spec:

- `.spec/specs/aa769a27-2721-4b9d-880c-5c4e2f8136a7/body.md`
  - Section: `Profiling metadata retention and redaction policy`
  - Defines default retention/rotation/sampling behavior
  - Defines redaction/privacy rules for paths, secrets, payloads, and high-cardinality fields
  - Defines governance/evidence obligations and exception-handling notes

The same spec now marks open retention/privacy decisions as resolved by this policy section and adds traceability link to this ticket.

## Validation linkage to ff6637f5 checklist outputs

This ticket provides governance contract coverage for `ff6637f5` checklist linkage:

- Standardized run metadata can remain retained long enough for checklist verification while retaining deterministic replay boundaries.
- Evidence links remain resolvable after rotation/pruning via metadata-row preservation requirement.
- Profiling-only high-volume details are sampled/redacted according to policy without removing required lifecycle evidence.

## Depends-on context

Parent tracker: `84673399`.
Cross-tracker evidence target: `ff6637f5` validation checklist.