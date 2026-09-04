---
description: "Use when a dossier under transcripts/ is being used to capture and develop open-ended ideas rather than to produce an execution-ready ROADMAP.md. Covers when a dossier may skip roadmap compilation, how to keep an idea-workspace dossier discoverable, and the promotion path to a full roadmap."
applyTo: "**/*.md"
---

## Purpose

Not every dossier is destined for immediate execution. Some exist to accumulate and refine ideas — candidate directions, tradeoffs, half-formed requests — across multiple sessions before a roadmap is worth compiling at all. Without an explicit mode for this, an unfinished dossier with no `ROADMAP.md` looks identical to an abandoned or broken one. This file names the idea-workspace mode explicitly and defines its minimal shape and its promotion path into the full [prompt-ingestion.instructions.md](prompt-ingestion.instructions.md) pipeline.

## Recognizing an Idea-Workspace Dossier

A dossier is in idea-workspace mode when one of the following is true:

- It has no `ROADMAP.md` yet, and its `README.md` states its status as an idea workspace rather than a stalled pipeline run.
- It has a `ROADMAP.md` explicitly marked `Status: draft / exploratory` at the top, signaling it is not yet the zero-open-question deliverable [prompt-ingestion.instructions.md](prompt-ingestion.instructions.md) requires before shipping.

A dossier with no status statement at all is ambiguous — treat it as a stalled or interrupted pipeline run, not as an idea workspace, until its `README.md` is updated to say otherwise.

## Minimal Required Shape

Even in idea-workspace mode, keep:

- **`README.md`** as the index, stating the idea-workspace status and a one-line summary of what is being explored.
- **`ARTIFACTS.md`**, started as soon as any concrete repository artifact (a ticket, a file, a spec) becomes relevant to the idea — do not wait until promotion to start tracking it.

Everything else — the full Stage 3/5 review-and-interview loops, the numbered work-package documents, waypoint compilation — is **optional** while the dossier stays in idea-workspace mode. Forcing those stages early defeats the point of a lightweight workspace.

## Allowed Operations in Idea-Workspace Mode

- Append new `input-N.md`/`input-N.clean.md` parts using the same multi-part convention [prompt-ingestion.instructions.md's Resuming an In-Progress Dossier](prompt-ingestion.instructions.md#resuming-an-in-progress-dossier) defines, each time a new session contributes to the same idea.
- Keep freeform notes files (e.g. `notes.md`, `options.md`) capturing candidate directions and their tradeoffs. When comparing distinct directions in a structured way, dispatch the [Brainstorm Agent](../../agents/brainstorm.agent.md) rather than inventing an ad hoc comparison format.
- Update `README.md`'s summary as the idea's shape changes, without needing to version it the way a shipped `ROADMAP.md` is versioned — an idea workspace is expected to be edited in place until it is promoted.

## Promotion Path

When the idea workspace has accumulated enough shape to justify committing to a roadmap, run the full pipeline against the accumulated notes exactly as [prompt-ingestion.instructions.md's Resuming an In-Progress Dossier](prompt-ingestion.instructions.md#resuming-an-in-progress-dossier) describes for a continuation: treat the notes as the merged clean input, run Stage 2 research/inventory if not already current, then Stages 3 through 6 to produce a real `ROADMAP.md`. At that point the dossier stops being an idea workspace and is governed like any other dossier by [roadmap-authoring.instructions.md](roadmap-authoring.instructions.md) and [roadmap-execution.instructions.md](roadmap-execution.instructions.md).

## Guardrail

Idea-workspace status is not a way to avoid the review/interview loop once real implementation work starts. The moment a ticket is created against material from the workspace, [phase-separation.instructions.md](phase-separation.instructions.md) and [escalation-gate.instructions.md](escalation-gate.instructions.md) apply as normal — an idea workspace defers roadmap compilation, it does not defer discovery discipline once execution begins.
