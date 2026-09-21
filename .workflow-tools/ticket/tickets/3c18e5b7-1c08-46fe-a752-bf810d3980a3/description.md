## Gap

`.agents/instructions/orchestration/pre-dispatch-gates.instructions.md` is 168 lines and says gates apply to every delegation. `Per-Delegation-Class Gate Sets` at line 34 defines exactly four gate sets: Implement at line 36, Review at line 64, Testing at line 82, and Commit at line 100. Research and Explore read-only delegations have no gate set.

## Session Evidence

Read-only Research and Explore work in the restructuring session repeatedly skipped a pre-dispatch gate because no applicable gate set existed.

## Required Corrected State

Add a Research/Explore gate set or an explicit documented exemption with a stated rationale. The corrected guidance must make the every-delegation mandate executable for read-only delegation. Reference tickets `46d8b25d` `Move quality gates before dispatch` and `84aa1d3e` `Pre-dispatch gate: Implement delegation checks`.