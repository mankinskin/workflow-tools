## Objective
Act as the required Track 5 research sub-ticket and define the deterministic rules-first classifier before live migration of legacy `task` and `tracker-improvement` records. This ticket uses the legacy `tracker-improvement` type because the future `research` schema type is introduced by Track 3.

## Requirements
- Specify inputs: title, description, fields, state, and relation edges.
- Specify targets: `research`, `planning`, `implementation`, `review`, `interview`, and `testing`.
- Define deterministic weights, score calculation, tie handling, and classifier versioning.
- Permit automatic migration only for a unique top score >= 0.80.
- Require a linked review-ticket decision for tied, lower, or missing scores, with no timeout or default.
- Define immutable classifier evidence: classifier version, candidates, scores, selected target, rules/explanation, and approval; the latest approved decision is authoritative and conflicting pending parts block migration.

## Acceptance Criteria
Fixtures and focused tests demonstrate deterministic scores, ties, threshold behavior, required review paths, immutable evidence, latest-approved precedence, and conflict blocking. Track 5 live classification remains blocked until this research ticket is complete.