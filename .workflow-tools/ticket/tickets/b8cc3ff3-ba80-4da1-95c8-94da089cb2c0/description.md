## Gap

`.agents/instructions/orchestration/retry-limit.instructions.md` is 43 lines with `applyTo: "**/*.rs,**/*.ts,**/tests/**"`. Its retry cap is stated purely for test failures: a step that fails a test receives exactly one self-fix retry. Compile and build failures are never addressed.

## Session Evidence

An implementer stopped after two failed feature builds and abandoned a half-migrated worktree because the test-failure cap was interpreted as applying to compile failures.

## Required Corrected State

State that a mechanical compile or build failure may be fixed locally in an iterative cycle, while test failures retain the existing one-retry-then-escalate rule.