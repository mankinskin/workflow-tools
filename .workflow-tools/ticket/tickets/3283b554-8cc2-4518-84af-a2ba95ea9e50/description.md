## Objective
Test cost-gate, caller-model resolution, rejection guidance, and session-guard behavior.

## Context
Owner decisions: “versioned policy table (graded budget)” and “N=5 ... highest-tier-first, cheapest within each tier.” Also: “mcp-toolmon acts as a pre-tool hook and requires a valid session id.”

## Acceptance criteria
- Cover policy versions, caller-model resolution, and graded outcomes.
- Cover all rejection-guidance branches and guarded-session failures.

## Out of scope
- Policy or hook redesign.