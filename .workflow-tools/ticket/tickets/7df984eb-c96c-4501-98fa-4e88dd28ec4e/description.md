## Objective
Inventory and migrate legacy `task`, `feature`, `tracker-improvement`, and upgraded `bug`/`epic` records safely.

## Requirements
- Produce a type/state inventory before migration.
- Treat all pre-model legacy records as one workflow-exempt cohort until individually migrated; grandfather prior history.
- For active bug/epic records (`open`, `planned`, `in-implementation`, `in-review`, `on-hold`), require either a direct lifecycle route or a reviewer-selected `remediation_approved_by` relation to an immutable review decision; missing proof blocks that canonical upgrade and the release.
- Classify `task`/`tracker-improvement` only through a rules-first research sub-ticket. Only a unique score ≥0.80 auto-migrates; tied, lower, or missing scores require a linked review-ticket decision.
- Migrate legacy `feature` records deterministically: active child/dependency -> epic; unchecked acceptance criterion only -> implementation; both -> epic; neither -> archived spec.
- Use idempotent transactional batches. Persist immutable migration parts. A committed approved batch removes the individual exemption.

## Acceptance Criteria
Dry-run, idempotence, batch rollback, remediation authorization, classifier approval, feature conversion, and live-cutover tests pass. The exact inventory and decisions are auditable.


## Recovered Cutover, Preflight, and Classification Contract
- Retain canonical `bug` and `epic` IDs; upgrade active records in place only after global preflight. Active means `open`, `planned`, `in-implementation`, `in-review`, or `on-hold`.
- Each active bug/epic must have a direct lifecycle route or a typed `remediation_approved_by` relation to a `review` ticket with an immutable approval decision part. Missing proof blocks its upgrade and the release.
- Current manifest rules govern transitions. All pre-model-cutover records are one workflow-exempt cohort: historical evidence is grandfathered, workflow checks suspend until individual migration, and the first post-migration transition follows the current model. `on-hold` restores the last non-on-hold lifecycle category.
- Create a required Track 5 research child before live classification. The child defines deterministic rules, weights, and tie calculation for title, description, fields, state, and relation edges. Targets are `research`, `planning`, `implementation`, `review`, `interview`, and `testing`.
- Only a unique top score >= 0.80 auto-migrates. Tied, lower, missing, or conflicting-pending results require linked review approval, with no timeout or default. Immutable evidence records classifier version, candidate scores, selected target, rules/explanation, and approval. The latest approved decision is authoritative.
- Reuse the active-state list for deterministic legacy feature conversion: active child/dependency -> epic; unchecked acceptance criteria only -> implementation; both -> epic; neither -> archived spec.
- Write a migration-completion record only after all approved idempotent transactional batches commit.

## Additional Acceptance Criteria
Tests cover ID retention, each preflight proof and release block, full cutover/exemption/history/on-hold behavior, classifier research-child gate and evidence/precedence/conflict rules, no-timeout review path, feature conversion table, and migration-completion record.


## Required Prerequisite
Live classification and migration are blocked until [d8bd4c53 Research deterministic legacy-ticket classifier](.ticket/tickets/d8bd4c53-898e-4984-97e5-6ef605569f91/ticket.toml) is complete. The prerequisite is temporarily typed `tracker-improvement` because Track 3 has not yet introduced the `research` schema type.