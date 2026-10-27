# Dedicated Mission review and completeness role

## Objective

Define a Mission-specific reviewer role that uses the generated entry to
assess completeness and cite evidence without replacing general repository
audit.

## Requirements

- Provide a workflow-aware role with explicit inputs, outputs, invocation and
  evidence-citation behavior.
- Assess promised-versus-delivered artifacts, declared completeness and
  missing/contradictory/unavailable evidence from the generated entry.
- Work without undocumented context and report unknowns instead of inventing
  product facts.
- Remain compatible with the existing Audit Agent; replacement is not required.
- Create or edit no guidance in this W8 planning work.

## Design and boundaries

Plan a workflow-tools-local role entry authored through the
[Guidance Lifecycle Agent](../../../../.agents/agents/guidance-lifecycle.agent.md).
Existing Audit Agent scope is adjacent prior art, not an owner to take over.

## Acceptance criteria

- Complete, missing and contradictory entry examples produce evidence-linked
  outcomes with explicit residual uncertainty.
- Audit checks remain available and unchanged.
- Any future role guidance follows the Guidance Lifecycle Agent's roster and
  publication contract.
- No guidance change or audit replacement is part of this ticket.

## Validation plan

Future guard `w8-mission-reviewer-role`: manual workflow simulation against
complete, missing and contradictory entries; verify citations, unknown
handling and audit compatibility. No guidance or product tests were run.

## Sources and non-goals

R1-008–009; A23. The roster and bounded local guidance search found no
Mission-specific reviewer. No audit takeover, guidance edit or product
readiness claim.
