## Summary
Narrowed over-broad `applyTo` selectors in instruction frontmatter to reduce unintended rule activation.

## Files Updated
- .agents/instructions/token-efficiency.instructions.md
- .agents/instructions/spec-system.instructions.md
- .agents/instructions/tests.instructions.md
- .agents/instructions/ticket-system.instructions.md

## Changes
- Replaced wildcard-all / over-broad matching with scoped path patterns in token-efficiency, spec-system, and ticket-system instructions.
- Replaced `**/*test*` in tests instruction with explicit test directory and filename patterns.

## Validation
- Root-scoped next check run for `e1d8be15`.
- Confirmed `30606247` and `f19dcafa` remain frontier-ready.
- Ticket moved to `in-review` after implementation.
