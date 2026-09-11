---
description: "Use when a handoff package is incomplete or requirements are ambiguous. Covers escalation over inline clarification: when discovery or interview is needed, escalate to the user or hand off to a fresh discovery/interview agent rather than searching or clarifying during implementation."
applyTo: "**/*.md,**/*.toml"
---

## Escalation Gate

When a handoff package is incomplete or requirements are ambiguous, escalate immediately. Do not attempt inline clarification or broad search during implementation. Escalate to the user or hand off to a fresh discovery/interview agent to complete the discovery phase before proceeding with implementation.

## Rules

1. **Incomplete handoff → escalate.** If the handoff package is missing implementation-ready context, acceptance criteria, or related ticket/spec pointers, stop and escalate.
2. **Ambiguous requirements → escalate.** If the acceptance criteria, user intent, or edge-case behavior is unclear, escalate for interview before writing code.
3. **Do not search during implementation.** If you need to discover related code, existing patterns, or design constraints, hand off to a discovery agent; do not interleave search with implementation.
4. **Do not clarify inline.** Implementation agents execute; they do not interview the user for missing requirements.

## Scope

- Implementation phases must receive complete handoff packages.
- Discovery and interview work happens before implementation, not during it.
- Missing context is an escalation signal, not an implementation task.

## Cross-References

- Iteration loop behavior spec: `b71658f1`
- Handoff package schema: `5e52039d`
- Phase separation enforcement: [.agents/instructions/orchestration/phase-separation.instructions.md](./phase-separation.instructions.md)

## Escalation Triggers

- Handoff package missing acceptance criteria or test scenarios.
- Conflicting guidance from multiple specs or tickets.
- Unclear edge-case handling or error-recovery requirements.
- Uncertainty about whether a behavior change requires user approval.

## Anti-Patterns

- "I'll search for related patterns while implementing."
- "Let me clarify this requirement with the user during the implementation pass."
- Proceeding with partial context "to make progress."
