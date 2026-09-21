## Objective
Record and sanitize real `PostToolUse` and `Stop` hook fixtures from live Copilot artifacts.

## Context
`memory-api/crates/session-api/tests/fixtures/capture_hook_workspace_e2e.jsonl` contains only three hand-authored records.
Audit: `tmp/test-coverage-audit/04-capture-hook.md`.

## Acceptance criteria
- Replace the toy fixture with sanitized producer output.
- Preserve event shape needed for deterministic replay.

## Out of scope
- Matrix and contention test implementation.