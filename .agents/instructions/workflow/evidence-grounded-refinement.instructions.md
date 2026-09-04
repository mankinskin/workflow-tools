---
description: "Use when defining or running any critique -> verdict -> interview cycle that must be grounded in gathered evidence rather than raw words alone (prompt intent refinement, ticket refinement, mission planning). Defines the shared four-step refinement-loop shape reused by intent-refinement.instructions.md, ticket-refinement.agent.md, and mission-planning.agent.md so the loop's mechanics live in one place."
applyTo: "**/*.md"
---

## Purpose

Refining a prompt's intent, refining a ticket, and planning a mission goal are three different units of work going through the same loop: ground in evidence, critique against it, interview only the gap evidence can't close, repeat until resolved. Extracting that shape here keeps each consumer file short and stops the loop's mechanics from drifting out of sync across independent restatements.

## The Refinement Loop

1. **Ground first.** Gather the evidence relevant to the unit of work before critiquing or asking anything — existing tickets/specs, prior research, code, or (for a prompt) the artifact inventory. Never critique or interview from the raw ask's words alone.
2. **Critique against evidence.** Assess the unit against what the evidence actually supports: is it bounded, is it verifiable, does it conflate distinct concerns, what does the gathered evidence already answer. Produce a verdict and a findings list.
3. **Interview only what evidence cannot resolve.** For each finding evidence cannot settle — a genuine gap in goal, priority, or acceptance condition — ask a question grounded in a concrete finding, following [question-quality.instructions.md](question-quality.instructions.md). Do not ask about anything the evidence already answers.
4. **Repeat until resolved.** Re-run the critique against the updated state after answers arrive. Do not close the loop with an unresolved finding still open — carry it to another round rather than downgrading it to a note.

Escalate through [escalation-gate.instructions.md](escalation-gate.instructions.md) rather than guessing at any step: an unresolved finding is a blocked loop, not an approved one.

## Consumers

This loop shape is instantiated, not restated, by:

- [intent-refinement.instructions.md](intent-refinement.instructions.md) — applies it twice to a prompt's carried intent, before and after dossier drafting; evidence is `ARTIFACTS.md` (first pass) or the drafted dossier/`ROADMAP.md` (second pass).
- [ticket-refinement.agent.md](../../agents/ticket-refinement.agent.md) — applies it to a ticket; evidence is the ticket store, board, and spec stack.
- [mission-planning.agent.md](../../agents/mission-planning.agent.md) — applies it to a raw ask's mission goal; evidence is whatever the dispatching loop above hands over, most often as the interview target of another consumer's own loop rather than a standalone entry point.

Each consumer states only what evidence means for its unit of work, who it dispatches for the interview step, and what its verdict/output artifact is — the four-step shape itself lives here, once.
