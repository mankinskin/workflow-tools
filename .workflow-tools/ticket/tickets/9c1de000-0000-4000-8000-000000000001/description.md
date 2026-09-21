# Reconcile the already-executed decommissioning tickets

## Context

Tickets `14c0995c` (epic, isolated node with no edges), `f43cb5cb`, `76d0ace3`, and `16cfd19f` are all state `new`, but the work they describe **already landed in git**:

- `4b74311d` deleted `10-agents.yaml`, `20-github-copilot.yaml`, `35-agents-skills.yaml`, `50-agents-instructions.yaml`, `55-session-optimization.yaml`
- `4697eb3f` deleted `30-agents-prompts.yaml`, `45-agents-agents.yaml`
- `a7810499` deleted `40-agents-prompts.yaml`

846 lines of agent-guidance target config in total. Marker stripping is complete: `.agents/instructions/`, `.agents/agents/`, and `.agents/prompts/` contain **zero** `<!-- rule-api:file generated=true -->` markers.

The ticket store therefore misrepresents reality, and `next`/`ready-overview` will keep surfacing finished work.

## Decision

Close forward. Do **not** revert the deleted configs — the new multi-client track re-imports from today's hand-owned files instead.

## Scope

- Walk `f43cb5cb`, `76d0ace3`, `16cfd19f` to `done`, recording the landing commits as evidence.
- Walk `14c0995c` to `done`; its four acceptance criteria are already satisfied in the description body.
- Note in each ticket that the direction is superseded by the multi-client track, with a link to the anchor spec.
- Verify `b13c5d89` (Epic: Agent Skill Foundation) and `0c72ecac` are unblocked as a result, and record which downstream tickets that frees.

## Acceptance criteria

1. All four tickets are `done` with the landing commits recorded.
2. No ticket in the store still claims that `.agents/**` must be decoupled from the rule system as pending work.
3. `ticket next` no longer surfaces any of the four.
