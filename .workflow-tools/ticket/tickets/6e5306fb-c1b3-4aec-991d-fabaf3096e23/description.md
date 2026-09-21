# Summary

Pilot the expectation-oriented spec contract on one workflow spec and one README-rollout spec.

# Why

Without a real pilot, the architecture will stay theoretical and the repository will continue to mix contract and rollout text opportunistically. The first migration slice should stay small enough to prove the path before broader homogeneous migration begins.

# Scope

- rewrite one existing workflow spec under the new contract
- rewrite one of the newly created README-rollout specs under the same contract
- prove acceptance, evidence, health, and derived audit behavior end to end on those two artifacts
- document the remaining inventory and the migration mapping needed for the follow-on broader migration ticket

# Assumptions To Prove

- one workflow spec plus one README-rollout spec is enough to prove the migration path
- the new contract can be validated from native store links and derived audit reporting rather than from manual prose review alone
- the pilot can leave a concrete remaining-artifact inventory for the broader migration ticket instead of trying to complete the whole migration itself

# Acceptance Criteria

- One existing workflow spec and one README-rollout spec are rewritten under the new contract and pass the new health and evidence expectations.
- The pilot demonstrates end-to-end fulfillment using native store links and derived audit reporting.
- The pilot records the explicit remaining inventory and the homogeneous mapping needed for the broader migration ticket.
- The pilot leaves a clear follow-on baseline instead of another mixed-format interim state.

# Validation

- Focused validation for the pilot workflow spec and pilot README-rollout spec.
- A pilot checklist capturing the remaining inventory and migration mapping for the follow-on migration ticket.

## Session handoff findings — 2026-06-30

- Review confirmed the pilot artifacts exist in the spec store and the rewritten root specs carry native expectation/evidence manifest fields.
- Health checks on the inspected pilot/root specs were clean.
- The blocker is evidence alignment, not missing contract fields: validation-store query returned zero executions linked to this ticket.
- The rollout review also found related evidence-plan text still written in future tense on the explicit-init owning spec, which means the pilot is not yet backed by recorded native evidence links end to end.
- Next action: record or link the native `.test` / `.log` executions and the derived audit evidence for the pilot artifacts, then re-check this ticket and the umbrella rollout ticket together.