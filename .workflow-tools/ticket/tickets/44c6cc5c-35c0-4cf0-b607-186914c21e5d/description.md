## Problem
Worker/small-model tiers are weak at designing comprehensive, correctly-scoped tests. Today nothing prevents a worker-tier agent from writing (and potentially weakening) its own test file to make its own implementation pass.

## Goal
Add an instruction establishing a split-responsibility testing contract:
- During planning (frontier/Planner tier), the required test file(s) are authored first, capturing the acceptance criteria as executable tests.
- Worker-tier agents are restricted to editing implementation files to make the pre-written tests pass; they must not modify the test files.
- Reconcile with existing Testing Agent responsibilities and .agents/instructions/testing/* (assertions.instructions.md, test-execution.instructions.md) so the boundary between "who writes tests" and "who runs/validates them" is unambiguous.

## Acceptance criteria
- Instruction states explicitly that worker-tier agents may not edit test files once the frontier tier has authored them for a given ticket.
- Cross-referenced from testing/assertions.instructions.md or a new file under .agents/instructions/testing/.
- Documents the exception path (e.g. if a test itself is found to be wrong) and who is authorized to change it.

## Source
Derived from AGENT_WORKFLOW_OPTIMIZATIONS.md conversation, "Step 4: Solving the Small Models Suck at Testing Problem".
Implemented .agents/instructions/testing/split-responsibility-testing.instructions.md (new) forbidding worker-tier edits to frontier-authored test files, with the exception path routed through the spec's blocker mechanism (Planner/Architect or human reviewer authorized to change a test), Playwright/browser-verification handling, and a cross-reference from assertions.instructions.md. Review verdict pass. All 3 ACs met.