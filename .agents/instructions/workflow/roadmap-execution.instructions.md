---
description: "Use when methodically executing a compiled ROADMAP.md from a prompt-ingestion dossier. Covers reading the roadmap and dossier together, walking waypoints in dependency order, collecting per-waypoint context from the dossier before acting, and delegating oversized waypoints as one isolated unit."
applyTo: "**/*.md"
---

## Purpose

A compiled `ROADMAP.md` (see [prompt-ingestion.instructions.md](prompt-ingestion.instructions.md)) is a route, not a summary to skim once and improvise from. This instruction is the execution-side counterpart: it governs how an executing session — most often the Orchestrator Agent — walks a roadmap's waypoints methodically instead of reconstructing context from memory partway through.

## Required Procedure

1. **Read the roadmap and the dossier together.** Before acting on anything, read `ROADMAP.md` in full — starting with its outcome summary — plus the dossier's `README.md` index and `ARTIFACTS.md`, so every cited id/path resolves to a real artifact instead of an assumed one.
2. **Establish readiness before starting a waypoint.** Confirm that every declared dependency is satisfied, every cited artifact resolves, the intended outcome has unambiguous acceptance criteria, the target setting is known, and the waypoint has an exact validation strategy. A missing, stale, contradictory, or untestable item is a blocker; do not set the waypoint `in-progress` or make a mutation.
3. **Execute waypoints in order.** Walk the Roadmap Waypoints section top to bottom, respecting dependency order. Do not start a waypoint whose declared dependency (an earlier waypoint, a ticket, a decision) is not satisfied. Mark a ready waypoint `in-progress` immediately so the roadmap remains a live progress record.
4. **Collect per-waypoint context from the dossier before acting.** For each waypoint, resolve its cited artifact ids/paths — tickets, specs, numbered work-package documents (`01-...md`, ...), and code/config paths — via the dossier before writing anything. A waypoint's one-line roadmap summary is a pointer, not sufficient implementation context; the cited work-package document is normative and must be read.
5. **Use the task routing table.** Route the ready waypoint by task type before dispatching or acting. A task's route is determined by the work still required, not by its heading or a previous session's route.
6. **Delegate large waypoints as one isolated unit.** A waypoint marked cross-session, or one backed by a ticket created during roadmap compilation (per [prompt-ingestion.instructions.md](prompt-ingestion.instructions.md)'s "Ticket Creation During Refinement"), is executed as a single delegated dispatch scoped to that waypoint and ticket — not split into ad hoc smaller asks and not folded into neighboring waypoints.
7. **Validate before advancing.** Run the waypoint's declared validation gate and confirm it passes before marking the waypoint done and moving to the next one. Do not defer validation to the end of the route.

### Execute Ingest Approval Gate

When [execute-ingest.prompt.md](../../../.agents/prompts/execute-ingest.prompt.md)
hands a roadmap to this execution procedure, the handoff must contain the
explicit final user outcome `approve`. The outcome `replan` returns control to
the planning loop and is not an executable handoff. A planning verdict such as
`Approved as scoped` is evidence that the roadmap is ready for review, not
evidence of execution approval.

The executor must use the exact dossier path and `ROADMAP.md` supplied by the
approved handoff. The executor must not reconstruct, alter, or substitute the
roadmap before checking waypoint readiness.

## Waypoint Readiness Gate

A waypoint is ready only when all of the following are true:

- Its declared dependencies are `done` or otherwise have recorded, verified satisfaction.
- Its governing ticket, specification, decision record, and dossier artifacts resolve and agree on the requested outcome.
- Its acceptance criteria describe observable success and its target setting (repository, worktree, relevant configuration, and applicable constraints) is known.
- Its scope names the owning files, interfaces, or research question closely enough to prevent an implementation agent from rediscovering requirements.
- Its validation lines name executable commands or an explicitly documented non-executable review method that can prove each criterion.
- The user has not supplied a newer request that changes or conflicts with the waypoint's objective, scope, order, acceptance criteria, or non-goals.

Evaluate the readiness gate before the first mutation and again whenever new evidence changes the waypoint's premises. Do not infer that a waypoint is ready from an `in-progress` status left by a prior session.

## Blocker and User-Escalation Protocol

An unexpected hurdle, unresolved requirement, failed prerequisite, stale artifact, inconsistent validation result, or conflict between a newer user request and the roadmap is a blocker. A blocker terminates work on the current waypoint immediately.

When a blocker is found:

1. Stop the current waypoint. Do not make further mutations, start a dependent waypoint, substitute a requirement, or work around the hurdle by widening scope.
2. Mark the waypoint `blocked` in `ROADMAP.md` and add a dated entry to **Active blockers** containing the waypoint id, exact unmet condition, evidence pointer, impact, and the single decision needed to resume.
3. Inform the user before taking another implementation action. Explain the current plan, conflicting fact or request, blocked consequence, and concrete resolution options. Ask one self-contained, verifiable decision at a time using [question-quality.instructions.md](question-quality.instructions.md).
4. Route the resolution through a user-interactive planning or review batch. The batch updates the roadmap and the affected dossier artifact, ticket, or specification until the blocker is removed rather than treating the user answer as permission to bypass the plan.
5. Re-run the readiness gate after the roadmap update. Resume only when the blocked waypoint has complete requirements, a known setting, and validation evidence strategy.

Do not continue with unrelated roadmap work after detecting a blocker in the current execution route unless the user explicitly defers the blocked waypoint and approves the independent next waypoint. Record the deferral and its reason in `ROADMAP.md`; a deferral is not a completed dependency.

### User-Request Consistency Check

Treat a new user request as a planning input, not an implicit override. Before changing files, compare the request against the active waypoint's objective, dependencies, acceptance criteria, target paths, validation gates, and non-goals.

- When the request is consistent, carry it out within the ready waypoint.
- When the request adds scope, changes a decision, reverses a non-goal, changes dependency order, or weakens required validation, mark the current waypoint `blocked` and run the blocker protocol.
- When the request defines a separate independent outcome, add or revise a separate roadmap waypoint or ticket with its own requirements and validation; never hide the new work inside the active waypoint.

## Task Routing

| Task type | Enter when | Required route | Exit evidence |
| --- | --- | --- | --- |
| Planning or decision | The roadmap, requirement, scope, priority, architecture, ticket, or specification needs creation or revision. | Work with the user through an Interview Agent batch. Each batch presents evidence-backed, self-contained decisions; update the dossier and then the roadmap, ticket, or specification after each resolved decision. | Revised durable artifact; no open decision remains for the affected waypoint. |
| Research | A factual question must be answered before planning or implementation. | Dispatch a bounded Research Agent or Explore Agent unit. Persist the result in a dossier artifact with source paths/ids, findings, and planning implications. | Durable dossier evidence resolves the research question or records a blocker. |
| Implementation | A ready waypoint has a complete handoff, owned code slice, and validation strategy. | Create or update the ticket/spec as required, then dispatch one bounded Implement Agent unit. Follow [phase-separation.instructions.md](phase-separation.instructions.md). | Scoped change plus the waypoint's focused validation result. |
| Validation or review | An implementation change exists and the criteria require confirmation. | Run the declared deterministic validation first; use a review batch only for subjective or non-executable criteria. Persist validation evidence against the ticket/spec where available. | Passed command output or approved recorded review. |
| Documentation or reconciliation | Verified work changes the plan, contract, operation, or user-facing behavior. | Update the governing dossier, roadmap, ticket, specification, and user documentation in dependency order. | Documentation matches verified behavior and the waypoint's validation remains passing. |

An Interview Agent batch is a planning interaction, not an implementation workaround. The Interview Agent batch must stop after user decisions are recorded and return a revised, internally consistent artifact set. The next implementation action begins only after the readiness gate passes.

## Delegating a Waypoint

Follow [orchestrator-delegation.instructions.md](orchestrator-delegation.instructions.md) for model-tier selection when dispatching a waypoint. A single-session waypoint routes like any other bounded implementation unit; a ticket-backed waypoint routes to Ticket Refinement, Scoping, or Implement per that ticket's own state, not per the roadmap alone.

## Handling Drift

If a waypoint's dossier context has gone stale — a cited artifact no longer resolves, a validation command no longer exists, or source behavior contradicts the plan — do not silently improvise a substitute. Treat drift as a blocker and follow the blocker and user-escalation protocol. The resumed plan must replace the stale reference with a verified artifact and complete validation strategy.

**Completion record.** Immediately after a waypoint's validation and required review pass, change `Status: in-progress` to `Status: done` in `ROADMAP.md`. Add a short completion note naming the validation command or evidence record and, for a ticket-backed waypoint, the terminal ticket state. A completed waypoint is never merely implied by a later waypoint starting.

## Task Lifecycle

A waypoint's `Status:` line (see [roadmap-authoring.instructions.md's Syntax Rules](roadmap-authoring.instructions.md#syntax-rules)) moves through exactly four states, in this order, and never skips backward except via an explicit revert:

1. **`pending`** — not yet started; its declared dependencies may or may not be satisfied yet.
2. **`in-progress`** — the executing session has started work on it. Set this the moment work begins, not after it finishes, so a concurrent or later reader sees accurate live state.
3. **`blocked`** — work stopped on an unmet precondition. A waypoint MUST NOT sit at `blocked` without a stated reason in its own text or in "Active blockers" (per "Handling Drift" above).
4. **`done`** — its validation gate passed. Never mark a waypoint `done` before running its `Validate:` command per "Required Procedure" step 5.

**Ticket-backed waypoints track two lifecycles at once.** A waypoint whose `Scope:` names a ticket (per [roadmap-authoring.instructions.md](roadmap-authoring.instructions.md#scoping-guidelines)) has its own `Status:` line in the roadmap AND the ticket's own state machine (see [lifecycle.instructions.md](../ticket/lifecycle.instructions.md)). Keep the two in sync at the coarse level a roadmap needs: `pending`/`in-progress` maps loosely to the ticket being unclaimed/claimed, and the waypoint moves to `done` only once the ticket itself reaches a terminal `done` state — never mark the waypoint `done` while its ticket is still `in-review` or earlier.

## Review Handling

Some waypoints require a review pass before they can be marked `done`, not just a passing validation command — typically ticket-backed waypoints, per [loop-closure.instructions.md](loop-closure.instructions.md)'s Review → Interview → Commit → Handoff cycle. For a roadmap-tracked waypoint:

- Do not move a waypoint to `done` while its underlying ticket sits in `in-review` — leave it `in-progress` and note in the waypoint body that it is awaiting review.
- Record a review verdict as a short inline note on the waypoint (`Review: approved, see ticket <short-id>`) rather than a separate document, unless the review itself produced a substantial artifact worth its own file (a full [review.agent.md](../../agents/review.agent.md) report) — in that case cite the report by path instead of pasting it.
- A review that surfaces new scope or a new blocker is handled per [escalation-gate.instructions.md](escalation-gate.instructions.md), not by silently expanding the waypoint's own objective — open a new waypoint or ticket instead.
