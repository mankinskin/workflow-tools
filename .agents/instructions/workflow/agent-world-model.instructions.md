---
description: "Canonical narrative and lifecycle map for repository-guided agents."
applyTo: "**"
---

# Agent World Model

This document is the compact guidance entry point for understanding how an
agent moves through the repository. The more extensively authored narrative is
available in [narrative/README.md](../../../../narrative/README.md). Read the
narrative chapters in order when the world model needs explanation; use this
document when the same model must guide an active task.

## Chapter 0: Arrival

The agent is a client-side worker operating through a harness. Repository-wide
operating principles live in [context-engine/AGENTS.md](../../../../context-engine/AGENTS.md),
while the canonical agent templates and instructions live under
`workflow-tools/.agents/`. The retained `context-engine/.agents/` directory is
catalog and evidence material, not a second guidance root.

## Chapter 1: The Map

The repository map gives the agent scale and location before a file is opened.
Use [file-inspection.instructions.md](file-inspection.instructions.md) for
bounded reads, interface skeletons, and `repo_map.toon`. Entity stores are
separate from code: ticket, spec, test, and session records are resolved from
their owning workspace rather than inferred from a command directory.

## Chapter 2: Exploration

Exploration turns a request into a bounded slice of evidence. The
[research agent](../../agents/research.agent.md) gathers the owning files,
existing records, dependencies, and validation surfaces. Exploration remains
read-only until the goal and ownership are clear; the result is a compact map
of the relevant world, not an implementation guess.

## Chapter 3: Clarification

When evidence leaves a decision unresolved, the agent stops and asks one
answerable question. [question-quality.instructions.md](question-quality.instructions.md)
defines the question shape, and [escalation-gate.instructions.md](escalation-gate.instructions.md)
owns the rule that ambiguity or an incomplete handoff blocks implementation.

## Chapter 4: Decision

The goal becomes a concise, testable statement with explicit acceptance
criteria. [evidence-grounded-refinement.instructions.md](evidence-grounded-refinement.instructions.md)
owns the review and interview loop. The compiled handoff names the ticket,
specification, target paths, validation commands, constraints, and return
contract before execution starts.

## Chapter 5: Execution

The [implement agent](../../agents/implement.agent.md) performs one bounded
implementation unit from a complete handoff. [phase-separation.instructions.md](phase-separation.instructions.md)
keeps discovery before implementation, while
[write-and-die.instructions.md](write-and-die.instructions.md) keeps a worker
on one step and makes the next step a fresh, compiled dispatch.

Execution has four observable action types:

| Action | World-state evidence | Required proof |
|---|---|---|
| Edit | changed file or diff | focused check immediately after the edit |
| Commit | commit id and staged paths | clean worktree and current ticket/spec state |
| Tool call | returned tool result or store record | verified output, never an inferred success |
| Program run | exit status and captured output | bounded output inspected before trusting the result |

## Chapter 6: World-State Change

An edit, commit, tool call, or program run changes the observable repository
world. Commit discipline belongs to the repository workflow and the
[commit agent](../../agents/commit.agent.md); delegation economics belong to
[model-routing.instructions.md](model-routing.instructions.md). The agent must
keep the requested world-state change distinct from changes that improve the
toolset itself.

## Chapter 7: Validation and Return

Validation converts execution into evidence. The return contract in
[subagent-return-contract.instructions.md](subagent-return-contract.instructions.md)
requires command-backed success claims and explicit blockers. The
[iteration prompt](../../prompts/iteration.prompt.md) closes the loop through
review, interview when needed, commit, and handoff; the
[handoff agent](../../agents/handoff.agent.md) packages the next implementation
unit.

## Chapter 8: Improving the Tools

Tool use means invoking an existing ticket, spec, terminal, or agent surface to
change the requested world. Tool improvement means changing the guidance,
schema, cost model, or implementation of that surface so future calls become
more correct, capable, or economical. A tool-use call changes the current
request; a tool-improvement change changes the conditions for later requests.

The distinction is concrete: reading a bounded file with `peek-mcp` is tool
use; re-syncing `model_prices.json` so the cost gate makes different decisions
is tool improvement. Both are execution, but they have different targets and
must be validated against different outcomes.

## Chapter 9: Testing the World

Documentation and tests describe whether the repository world behaves as
claimed. Test fixtures can model simulated worlds; A narrative claim is
complete only when its repository anchor and validation method are named.

## Ownership Rules

Each lifecycle transition has one primary owner:

| Transition | Canonical owner |
|---|---|
| Exploration | `file-inspection.instructions.md` and `research.agent.md` |
| Clarification | `escalation-gate.instructions.md` and `question-quality.instructions.md` |
| Decision | `evidence-grounded-refinement.instructions.md` |
| Execution | `implement.agent.md` and `write-and-die.instructions.md` |
| World-state change | repository workflow and `commit.agent.md` |
| Validation | `subagent-return-contract.instructions.md` plus the validation workflow |
| Return | `iteration.prompt.md`, `handoff.agent.md`, and `shared-context-bundle.instructions.md` |

Secondary surfaces point to the owner instead of restating the same rule. The
formal specification for this lifecycle is
`workflow/agent-world-model` in the canonical spec store; implementation work
is tracked by ticket `97463359`.