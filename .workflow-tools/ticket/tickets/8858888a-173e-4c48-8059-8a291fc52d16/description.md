## Objective
Implement replay tests for the 31 uncovered rows of the 38-row capture-hook matrix.

## Context
`memory-api/crates/session-api/src/bin/copilot-capture-hook.rs` has 14 tests; audit `tmp/test-coverage-audit/04-capture-hook.md` section 4 identifies 31 uncovered matrix rows.

## Acceptance criteria
- Cover all 31 named matrix rows with replay assertions.
- Preserve event-specific semantics for UserPromptSubmit, PreToolUse, PostToolUse, Stop, and SessionEnd.

## Out of scope
- New hook features.