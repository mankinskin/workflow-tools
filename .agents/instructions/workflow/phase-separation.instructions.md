---
description: "Use when deciding whether to search, clarify, or implement. Covers phase separation: planning/discovery/interview happen before implementation, and the Implement Agent operates only on complete handoff packages."
---

## Phase Separation

Agent workflows separate planning from execution.

### Discovery and Planning Phase

Before implementation:
- **Research Agent**: explores codebase, gathers context, identifies owning slices
- **Interview Agent**: clarifies requirements, refines specs and tickets, locks acceptance criteria
- **Orchestrator Agent**: decomposes work, delegates slices to cheaper agents

These phases produce a **handoff package** containing:
- The target ticket/spec with clear acceptance criteria
- The owning code path or slice to edit
- Required context (tests, docs, dependencies)

### Implementation Phase

**Implement Agent** operates **only** on a complete handoff package:
- Does **not** search the codebase for unclear context
- Does **not** clarify requirements inline with the user
- **Escalates immediately** if the package is incomplete or ambiguous

The Implement Agent's `tools` list excludes `search` and `vscode/askQuestions` to enforce this contract.

### Escalation Protocol

If the Implement Agent receives incomplete context:
1. Stop work immediately
2. Report exactly what is missing (owning slice, acceptance criteria, dependency context)
3. Escalate to the delegating agent or user
4. Wait for a complete handoff package before proceeding

Do not attempt to fill gaps by searching or interviewing. Those phases are owned by specialized agents optimized for that work.

### Rationale

Phase separation:
- Prevents token waste on repeated discovery inside implementation loops
- Enforces clear handoff contracts between agents
- Allows cheaper implementation agents to focus only on surgical edits
- Makes incomplete delegation visible and actionable

## Related

- Handoff-package schema spec `5e52039d-aabc-434d-bdf3-eca63e312476` (required fields for a self-contained handoff)
- Iteration loop workflow spec `b71658f1-8de2-444a-9be1-64b1d8ecce70` (defines the Review → Interview → Commit → Handoff transition cycle)
