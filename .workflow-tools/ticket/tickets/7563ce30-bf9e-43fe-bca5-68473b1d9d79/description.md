## Problem
Worker-tier sub-agents currently can remain conversational across multiple steps within one session, which burns tokens as they re-derive "what to do next" instead of executing a single predetermined step. There is no instruction requiring single-step termination.

## Goal
Add an instruction defining a "write-and-die" dispatch pattern for worker-tier sub-agents:
- A worker sub-agent is dispatched with exactly one isolated step/edit to perform.
- After completing that one step (one tool call or one cohesive edit), the worker's session ends; it does not continue planning further steps.
- The next step, if any, is dispatched as a fresh sub-agent session rather than continuing the prior one.
This should be reconciled with existing shared-context-bundle.instructions.md (context handoff between dispatches) so the fresh-session requirement doesn't lose necessary context.

## Acceptance criteria
- Instruction states the one-step-then-terminate contract explicitly for worker-tier dispatch.
- Cross-references shared-context-bundle.instructions.md for how the next fresh session receives its context.
- Distinguished from Planner/frontier-tier agents, which may still run multi-step sessions.

## Source
Derived from AGENT_WORKFLOW_OPTIMIZATIONS.md conversation, "Step 2: Streamlining Handoffs and Context", "The Write-and-Die Pattern".
Implemented .agents/instructions/orchestration/write-and-die.instructions.md (new) defining the one-step-then-terminate Worker dispatch contract, with a reciprocal cross-reference in shared-context-bundle.instructions.md for how the next fresh session receives context, and an explicit Worker vs Planner/frontier-tier distinction. Review verdict pass. All 3 ACs met.