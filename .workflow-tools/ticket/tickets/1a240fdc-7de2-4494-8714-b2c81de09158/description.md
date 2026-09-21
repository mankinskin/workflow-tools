## Problem
Small/worker-tier agents can burn large amounts of tokens attempting repeated self-fixes after a failing test run, with no hard stop today. Existing fail-fast semantics in pre-dispatch-gates.instructions.md only govern whether a delegation is dispatched at all, not what happens mid-execution after a test failure.

## Goal
Add an instruction (new section or new file under .agents/instructions/orchestration/) defining a hard retry cap for worker-tier agents:
- On a test failure after an implementation edit, the worker gets exactly one self-fix retry.
- If the retry also fails, the worker must stop and escalate back to the dispatching/frontier agent or the user with the failing test output and diagnosis attempt, rather than continuing to iterate.
Cross-reference pre-dispatch-gates.instructions.md so the two fail-fast concepts (pre-dispatch vs mid-execution) are distinguished, not conflated.

## Acceptance criteria
- Instruction text states the exact retry count (1) and the required escalation action on second failure.
- Distinguishes this mid-execution retry cap from the existing pre-dispatch fail-fast gate.
- Referenced from orchestrator-delegation.instructions.md or model-routing.instructions.md wherever worker dispatch is described.

## Source
Derived from AGENT_WORKFLOW_OPTIMIZATIONS.md conversation, "Step 4: Break the Infinite Iteration Loop", "Fail-Fast Hard Stop".
Implemented .agents/instructions/orchestration/retry-limit.instructions.md (new) defining the one-self-fix-retry cap with escalation to Planner/Architect, distinguished from pre-dispatch fail-fast; cross-referenced from model-routing.instructions.md Failure Path. Review verdict pass-with-fixes: a clarifying paragraph was added resolving the retry-vs-terminate ambiguity against write-and-die.instructions.md. All 3 ACs met.