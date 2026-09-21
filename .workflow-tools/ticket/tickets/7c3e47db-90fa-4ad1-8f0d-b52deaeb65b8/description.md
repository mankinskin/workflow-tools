## Objective
Adopt `proptest` for parser, selector, and path equivalence classes.

## Context
Audit `tmp/test-coverage-audit/01b-coverage-verification.md`: no `proptest`, `quickcheck`, `arbitrary`, or `insta` exists; all tests are example-based.

## Acceptance criteria
- Add reusable proptest patterns for the stated equivalence classes.
- Define deterministic regression capture for generated failures.

## Out of scope
- Replacing every example-based test.