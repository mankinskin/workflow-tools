---
description: "Use when implementing, reviewing, or completing iteration work. Covers the closed-loop iteration workflow: Review→Interview→Commit→Handoff. Only approved work is committed, and each passing run produces a forward handoff package."
applyTo: "**/*.md,**/*.toml"
---

## Loop Closure

The iteration loop must close before handoff. Every iteration moves through discrete phases: Review → Interview (optional) → Commit → Handoff. Only approved work is committed. Each successful iteration produces a forward handoff package containing completed evidence, remaining scope, and implementation-ready context for the next session.

## Rules

1. **Commit only approved work.** Do not commit partial implementations, incomplete tests, or failing validation runs.
2. **Close the loop.** Every passing iteration must produce a handoff package. Handoff the completed evidence, updated specs, validated test results, and next session's implementation-ready context.
3. **Do not skip review.** Before moving a ticket to `in-review`, ensure the spec is updated and validation evidence is captured.
4. **Do not skip handoff.** After committing, generate the handoff package so the next session knows what was completed and what remains.
5. **A session's own diagnosis is not a deferred finding.** When a session's own diagnosis produces a new blocking ticket, that ticket becomes the next unit of work in the same session — not a future backlog item — unless the user explicitly defers it. Do not pivot to unrelated work in the same session after creating the ticket.

## Scope

- Implementation work must pass validation before commit.
- Tickets must move to `in-review` with updated specs and validation evidence.
- Handoff packages are mandatory at iteration boundaries.

## Cross-References

- Iteration loop behavior spec: [.spec/specs/b71658f1-8de2-444a-9be1-64b1d8ecce70/spec.toml](.spec/specs/b71658f1-8de2-444a-9be1-64b1d8ecce70/spec.toml)
- Handoff package schema: [.spec/specs/5e52039d-aabc-434d-bdf3-eca63e312476/spec.toml](.spec/specs/5e52039d-aabc-434d-bdf3-eca63e312476/spec.toml)
- Phase separation enforcement: [.agents/instructions/orchestration/phase-separation.instructions.md](.agents/instructions/orchestration/phase-separation.instructions.md)

## Anti-Patterns

- Committing partial work "to save progress."
- Skipping handoff because "the ticket is done."
- Marking work complete without review-ready validation evidence.
- Diagnosing a blocker, filing a ticket for it, then pivoting to unrelated work in the same session instead of picking up the ticket just filed.
