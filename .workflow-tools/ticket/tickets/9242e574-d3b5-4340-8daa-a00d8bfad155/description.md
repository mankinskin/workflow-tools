## Objective
Use the completed classifier-research design to plan 7df984eb live migration work.

## Required Plan
Define inventory/preflight sequencing, exemption/cutover rules, bug/epic proof handling, classifier-review flow, feature conversion, transactional batches, immutable evidence, migration completion, rollback boundaries, and focused tests.

## Done
A reviewed implementation plan exists before Track 5 production edits.
## Planning Output Obligation

This planning ticket is not complete until it has written the following into its implementation ticket `7df984eb` as an appended `## Acceptance Criteria` section:

1. **Target files** — the definitive list of files the implementation will create or modify, as repo-root-relative forward-slash paths, each with a one-line reason.
2. **Numbered acceptance criteria** — each independently verifiable and phrased as an observable behavior, not an activity. A criterion a reviewer cannot mark pass/fail from evidence alone is a defect. "Inheritance works correctly" is wrong; "loading a schema whose declared parent does not exist fails the whole load and leaves the prior registry generation intact" is right.
3. **Validation commands** — the exact commands that prove those criteria, with real verified package names. An invented package name is a defect; if a name is uncertain, mark it explicitly as unverified rather than guessing.
4. **Explicit non-goals** — what the implementation ticket must NOT do, so scope creep is visible at review time.

Rationale: implementation tickets in this program were found to lack target files, test seams, and validation commands. Those fields are the deliverable of planning, not of implementation — an implementation ticket cannot name its own test seams before its research and planning predecessors have run. Writing them back into the implementation ticket at planning time is what makes the plan artifact's consumer contract concrete.