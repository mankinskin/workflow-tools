## Objective
Build reusable multi-store sandbox and replay harness infrastructure.

## Context
Audit `tmp/test-coverage-audit/05-gap-analysis.md` identifies replay and concurrency infrastructure as missing foundations.

## Acceptance criteria
- Provide temporary store/root isolation and explicit cwd control.
- Coordinate child processes for concurrency tests.
- Provide repeatable replay entry points.

## Out of scope
- Individual surface test cases.