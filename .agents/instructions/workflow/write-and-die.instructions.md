---
description: "Use whenever dispatching a Worker-tier sub-agent for a single plan step: the write-and-die contract — one isolated step, then terminate. Covers how the next step becomes a fresh session and how this differs from Planner/frontier-tier multi-step sessions."
applyTo: "**"
---

## The Contract

A Worker-tier sub-agent receives exactly one isolated step, executes it, returns its declared result, and terminates. **It does not continue the conversation, does not chain to the next step, and does not remain resident waiting for further instructions.** The next step in the plan is dispatched as a brand-new sub-agent session, not a continuation of the current one.

This is the operational form of the Worker capability boundary defined in spec [1b654f30](../../../.spec/specs/1b654f30-d1a4-4cb4-ab2e-8355dfe5a758/body.md) ("Two-tier Planner/Worker model routing architecture"), which states plainly:

> Stop after completing (or blocking on) its one step — MAY NOT Chain to the next `step_id` on its own initiative.
> Report a blocker via `{pass: false, blocker: "..."}` — MAY NOT Re-plan around the blocker itself.

## Why "die"

A Worker that stays conversational across steps re-derives "what's next" from its own accumulating transcript, which burns tokens on redundant re-reasoning and risks silent scope drift (deciding to reorder steps, skip ahead, or widen the target). Terminating after one step removes both failure modes structurally: there is no transcript left to drift from, and no session left to keep talking.

## How the next step gets its context

The terminating Worker does not hand off state directly — the fresh session that runs the next step receives its context via a **compiled prompt**: prior context, the step's `target_path`, `allowed_tools`, validation commands, and `return_contract` are compiled into the new session's spawn prompt by the Planner/dispatcher, with any larger artifact named by resolved path/id for the fresh session to fetch itself, exactly as any other sub-agent spawn is made self-contained. See [shared-context-bundle.instructions.md](shared-context-bundle.instructions.md) for compiled-prompt composition and size targets.

## Worker vs Planner/frontier-tier

**Worker (write-and-die) and Planner/frontier-tier agents are not held to the same session-lifetime rule.** Workers are single-step and terminate: each step is a new spawn, receiving its slice of the plan via the compiled prompt. Planner/Architect agents (T0, frontier tier) may still run multi-step sessions — they reason once over the whole task to produce the Plan, and nothing in this instruction restricts how many turns that planning pass takes. Do not apply write-and-die to a Planner/Architect dispatch; it governs Worker execution only.

## Reconciling with existing statelessness language

[orchestrator-delegation.instructions.md](orchestrator-delegation.instructions.md) already establishes that "A sub-agent inherits NONE of the current session's context. No conversation history, no prior findings, no shared 'we'." Write-and-die does not contradict that — it extends the same isolation guarantee from *inbound* context (a sub-agent never inherits the prior session) to *outbound* lifecycle (a sub-agent never carries its own session forward past one step either). Isolation applies in both directions.
