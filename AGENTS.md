# workflow-tools guidance router

This repository owns workflow-tool implementation and canonical operational
guidance. Read only the owners relevant to the requested operation, before
acting; do not preload this table's targets.

## Universal safeguards

Resolve the target repository, explicit entity-store root, and active file
ownership before edits. Use bounded inspection and the smallest falsifiable
change. Main-checkout work is the default. Worktree creation requires an explicit
request from the current requester; overlap, risk, and dirty files do not
authorize it. Coordinate or stop on ownership conflicts. Preserve unrelated dirty changes; stage only
task-owned paths. Use domain APIs for entity mutations and read records back.
Validate the changed behavior, fix introduced regressions, and report evidence
and blockers honestly. Commit submodules before updating parent gitlinks.
Use installed compact command/inspection tools when available.

When referring to a numbered stage in guidance, qualify it with its owning
instruction file or complete workflow name (for example,
`prompt-ingestion.instructions.md Stage 4`). Never use a bare `Stage N` or
`Stage-N` when the process identity is not part of the same reference.

## Read on task entry

| Operation | Required owner |
| --- | --- |
| Repository editing, validation, publication | [repository workflow](.agents/instructions/repository/workflow.instructions.md), repository guide (`REPOSITORY_GUIDE.md`) |
| Investigation or uncertain requirements | [file inspection](.agents/instructions/workflow/file-inspection.instructions.md), [escalation](.agents/instructions/workflow/escalation-gate.instructions.md) |
| Implementation or regression | [regression-first development](.agents/instructions/workflow/regression-first-development.instructions.md), [test execution](test/.agents/instructions/testing/test-execution.instructions.md) |
| Delegation | [model routing](.agents/instructions/workflow/model-routing.instructions.md), [delegation contract](.agents/instructions/workflow/orchestrator-delegation.instructions.md) |
| Session identity, response lineage | [session identity](session/.agents/instructions/session/session-identity-and-handoff.instructions.md) |
| Ticket or board operation | [ticket workflow](ticket/.agents/instructions/ticket/workflow.instructions.md) |
| Specification or validation records | [spec workflow](spec/.agents/instructions/spec/spec-system.instructions.md), [validation evidence](test/.agents/instructions/testing/validation-evidence.instructions.md) |
| Planning or roadmap | [prompt ingestion](.agents/instructions/workflow/prompt-ingestion.instructions.md), [roadmap execution](.agents/instructions/workflow/roadmap-execution.instructions.md) |
| Worktree or submodule operation | [worktree workflow](session/.agents/instructions/worktree/worktree-workflow.instructions.md), [submodules](.agents/instructions/repository/submodule.instructions.md) |
| Deferred tooling problem, response feedback | [feedback visibility](.agents/instructions/workflow/feedback-turn-visibility.instructions.md) |
| Guidance creation, generation or installation | [Guidance Lifecycle Agent](.agents/agents/guidance-lifecycle.agent.md) |

Other task owners are discoverable through instruction descriptions. Detailed
definitions stay authoritative in their owning files; route summaries do not
override those definitions. Skills are opt-in by task, not startup imports.
