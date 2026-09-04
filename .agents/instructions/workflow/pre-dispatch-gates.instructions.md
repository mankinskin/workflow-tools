---
description: "Use whenever a compiled delegation prompt is complex, ambiguous, or risky enough that a mid-tier model is likely to hit a blocker: on-demand pre-dispatch dry-run gates. Covers per-delegation-class gate sets, tool calls implementing each check, when to reach for the gate, and fail-fast semantics."
applyTo: "**"
---

## Purpose

Quality gates that run AFTER dispatch cost full delegation loops when preconditions fail. This instruction defines pre-dispatch gate sets that catch bad units while they are still cheap — before spawning the sub-agent. The gate is a dry-run tool, not a tax on every delegation: reach for it when the compiled prompt itself is the risk.

## When to Apply

The gate is **on-demand, not mandatory for every delegation**. The orchestrator decides, per delegation, whether the compiled prompt is complex or risky enough that a mid-tier (T2/T3) sub-agent is likely to hit a blocker. Reach for the gate when one or more of these signals is present:

- The compiled prompt names a ticket/spec state, file path, or command the orchestrator has not itself confirmed in this session
- The plan has multiple hops and the first hop is unverified
- The unit is high-risk (touches shared infrastructure, a destructive command, or a cross-cutting scope)
- A prior delegation in this chain already came back blocked once, and the retry needs the same precondition re-checked

Skip the gate for routine, well-scoped units where the orchestrator already has confirmed evidence (ticket state read this session, path verified via `peek-mcp`, command already run once). Paying gate cost on every delegation defeats the purpose of a cheap dry-run tool — it should be reached for, not defaulted to.

## Gate Execution Model

**Mechanism**: The orchestrator cannot run gates directly (it has no tools), so when it decides a dry-run is warranted, it dispatches a cheap gate sub-agent — the workspace **Explore Agent** template (`.agents/agents/explore.agent.md`), formally designated as the pre-dispatch gate agent — see "Acting as the Pre-Dispatch Gate" in that template. It runs on the T3 floor model (`"GPT-5 mini (copilot)"`).

**Gate contract (explicit input/output)**:

- **Receives**: the delegation class (Implement/Review/Testing/Commit/Research-Explore), the candidate ticket/spec ids or handoff package draft, and the specific gate set below for that class. Nothing else — the gate agent is context-isolated like any sub-agent.
- **Must return exactly one of**:
  - `{pass: true, bundle: {...}}` — the resolved context bundle (ticket, specs, paths, validation commands per the gate set's "Output" line below), ready to hand to the real delegation's sub-agent unmodified.
   - `{pass: false, blocker: "<single exact reason>"}` — one concrete, actionable blocker (not a list, not a hedge). Template: `"ticket <ticket-short-id> is in state 'blocked', not dispatchable"`, not "there might be an issue with the ticket."

**Dispatch verification (binding)**: The gate input MUST include the target agent's capability mode and every exact command in the dispatch prompt. The gate MUST block a command that the target mode forbids, including a mutating command sent to a read-only agent. Each command MUST have same-session probe output or the literal marker `VERIFY BEFORE RELYING ON THIS COMMAND: <command>`; an unmarked, untested command is an orchestrator fault. A passing bundle preserves each marker so the target verifies the command before relying on it.

**Execution identity attestation (binding)**: Before evaluating class-specific
gates, the gate agent MUST resolve and compare all of the following values:

- `session_id`
- `workspace_root` from the active VS Code workspace
- `code_worktree` from authoritative session lookup
- `git_toplevel` from the proposed command directory
- `branch` from the proposed command directory
- `entity_store_root` for ticket, spec, test, and session operations
- `command_cwd` for every proposed command

The gate MUST return `pass: false` when `git_toplevel` or `branch` does not
match the session assignment, when the prompt names a worktree owned by another
session, or when an entity operation would use an undeclared shadow store. A
main-checkout read is allowed only when the prompt labels it `read-only
source-baseline`; that directory MUST NOT be reused for mutations, validation,
or check-in. A passing bundle carries the seven attested values forward unchanged.

**Fail-fast semantics (binding)**: `pass: false` means the delegation is **NOT dispatched**, full stop. The orchestrator MUST do exactly one of:

1. **Resolve** the precondition itself (create the missing spec, update the ticket state, fix the handoff package), then re-run the gate once, or
2. **Escalate** to the user if resolution requires a decision outside the orchestrator's authority (see [escalation-gate.instructions.md](escalation-gate.instructions.md)).

Re-dispatching the same blocked unit without resolving the blocker is the exact failure mode this ticket exists to close (`redispatch_count` in the AC5 benchmark below) and is never acceptable.

**Cost ceiling (structurally enforced in the gate contract, not just asserted)**: The gate agent's own template caps it at **≤5 turns and ≤10 tool calls** per invocation (see the "Hard Ceiling" clause in explore.agent.md's gate section). This is a HARD ceiling enforced by the dispatched template's own contract, not a target that drifts upward as delegation quality improves. If the gate agent cannot reach a verdict within the ceiling, it MUST return `{pass: false, blocker: "gate exceeded its 5-turn/10-tool-call ceiling before reaching a verdict"}` rather than continue investigating.

## Per-Delegation-Class Gate Sets

### Implement Delegation

**Purpose**: Verify the implementation unit is dispatchable and has the context it needs.

**Gates** (all must pass):

1. **Ticket exists and is dispatchable**
   - Tool: `ticket_get <id>`
   - Block if: ticket does not exist, or `state ∉ {new, ready, in-implementation}`
   - Pass: ticket path, current state, title

2. **Spec coverage exists**
   - Tool: `spec_search <ticket-title>` or `spec_list --where ticket_ids=<id>`
   - Block if: no spec references this ticket
   - Pass: matching spec id(s)

3. **Target paths exist**
   - Tool: `peek_skeleton <path>` or `list_dir <dir>`
   - Block if: declared target file/directory does not exist in workspace
   - Pass: confirmed path set

4. **Validation commands are present and non-empty**
   - Tool: read handoff package or session validation gates
   - Block if: validation section is empty, missing, or contains only placeholder text
   - Pass: exact command list to run

**Output**: `{pass: true, ticket: <resolved-ticket>, specs: [<spec-ids>], paths: [<confirmed-paths>], validation_cmds: [<commands>]}` OR `{pass: false, blocker: "<exact-reason>"}`

### Review Delegation

**Purpose**: Verify the implementation produced the evidence that review will check.

**Gates**:

1. **Implementation delegation declared test/validation obligations**
   - Tool: read prior implement sub-agent's return or handoff package
   - Block if: no test evidence, no validation results, no "done" criteria declared
   - Pass: test result pointers, validation command output references

2. **Ticket state allows review**
   - Tool: `ticket_get <id>`
   - Block if: `state ∉ {in-review, done}`
   - Pass: current state

**Output**: `{pass: true, evidence: [<test-pointers>], ticket_state: <state>}` OR `{pass: false, blocker: "<exact-reason>"}`

### Testing Delegation

**Purpose**: Verify validation spec ids resolve and commands are executable.

**Gates**:

1. **Validation spec ids resolve**
   - Tool: `test_get_spec <id>` (when test-mcp is available)
   - Block if: spec id does not resolve or is not linked to the target ticket
   - Pass: resolved spec with command

2. **Test commands are well-formed**
   - Tool: parse command for obvious syntax errors (missing binary, malformed args)
   - Block if: command is obviously broken (e.g., `cargo test -p <missing-package>`)
   - Pass: command ready to run

**Output**: `{pass: true, specs: [<resolved-specs>], commands: [<validated-commands>]}` OR `{pass: false, blocker: "<exact-reason>"}`

### Research/Explore Delegation

**Purpose**: Verify that a read-only investigation has a bounded question and no capability/mode mismatch. This completes the every-delegation gate mandate for [ticket 46d8b25d](../../../.ticket/tickets/46d8b25d-e80c-4170-9601-1c26a7a0bcb8/ticket.toml) and applies the Implement delegation checks from [ticket 84aa1d3e](../../../.ticket/tickets/84aa1d3e-d98c-4c7c-8352-9ccecb2ca93e/ticket.toml) to read-only work.

**Gates**:

1. **Target mode and requested operations agree**
   - Tool: read the dispatch prompt and target agent contract
   - Block if: the target mode is absent, or a read-only target receives an edit, state-store mutation, or mutating command such as `git submodule update --init --recursive`
   - Pass: declared `read-only` mode and a command list containing only allowed read-only operations

2. **Research scope is bounded and evidence-backed**
   - Tool: read the dispatch prompt
   - Block if: no concrete question, target surface, or requested evidence is provided
   - Pass: research question, target paths or entities, and requested evidence shape

**Output**: `{pass: true, mode: "read-only", question: "<question>", targets: [<paths-or-entities>], commands: [<verified-read-only-commands>]}` OR `{pass: false, blocker: "<exact-reason>"}`

### Commit Delegation

**Purpose**: Verify working tree state is known and ticket is committable.

**Gates**:

1. **Working tree state is known**
   - Tool: `git status --short` (via terminal tool or committed handoff state)
   - Block if: unknown dirty state, untracked files that should be staged, or conflicts
   - Pass: clean or staged state

2. **Ticket is in committable state**
   - Tool: `ticket_get <id>`
   - Block if: `state ∉ {in-review, done}`
   - Pass: ticket state

**Output**: `{pass: true, git_state: "<clean|staged>", ticket_state: <state>}` OR `{pass: false, blocker: "<exact-reason>"}`

## Integration with Orchestrator Template

See the "Pre-Dispatch Gate (On-Demand Dry-Run)" section of [orchestrator.agent.md](../../agents/orchestrator.agent.md) for the canonical statement of this contract.

## Integration with Delegation Instructions

See the "Pre-Dispatch Quality Gates" section in [orchestrator-delegation.instructions.md](orchestrator-delegation.instructions.md), which points here for the on-demand gate mechanism, per-class gate sets, and fail-fast semantics.

## Validation

This is prose-only guidance that cannot be mechanically tested. The acceptance check is:
1. Gate definitions exist for all five delegation classes
2. Each gate specifies the exact tool call and pass/block criteria
3. The trigger signals for reaching for the gate (vs. skipping it) are stated explicitly
4. Fail-fast semantics are stated: `pass: false` blocks dispatch until resolved or escalated
5. Integration points with orchestrator template and delegation instructions are documented
6. Cost ceiling (≤5 turns, ≤10 tool calls) is stated as a hard requirement enforced in the gate's own contract, applying whenever the gate is used

## Relation to Benchmark

Benchmark ticket `10d21210` (now DONE) publishes the combined-baseline `redispatch_count` in [.benchmark/10d21210/README.md](../../../.benchmark/10d21210/README.md)'s thresholds table: **baseline 10 → target 0**, measured as `runSubagent` dispatches sharing `(agent_name, description)` with an earlier dispatch whose span recorded a failure. When the orchestrator reaches for the gate on a risky compiled prompt, the blocker is caught BEFORE dispatch, which is the mechanism this threshold measures — a post-change session replayed through the same harness is expected to show `redispatch_count = 0` on the units where the gate was used. Actual measurement of a post-change run is not owed by this ticket; only the evidence path is cited here. The gate cost (≤5 turns) is far cheaper than a full delegation loop (20-64 turns in the measured sessions), which is why it is worth reaching for on risky units even though it is not run on every delegation.

## Schema Gaps Discovered

The ticket description references schema gaps discovered during rework chains:
- `SessionValidationGate` missing `command` field (see tickets `8c67b96a`, `0d3fdba6`)

These gaps belong in their owning ticket/spec scopes, not in this gate definition. If a gate discovers a new schema gap, record it in the relevant ticket rather than expanding this gate file.
