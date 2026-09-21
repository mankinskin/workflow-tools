## Objective
Add capture-hook contention and failure-path tests.

## Context
Audit `tmp/test-coverage-audit/04-capture-hook.md` identifies concurrency and malformed-input gaps.

## Acceptance criteria
- Cover two concurrent hook processes, locked/read-only session file, truncated final JSONL line, non-UTF8 bytes, path with spaces, and absent store directory.
- Assert observable recovery or error behavior.

## Out of scope
- General session storage redesign.