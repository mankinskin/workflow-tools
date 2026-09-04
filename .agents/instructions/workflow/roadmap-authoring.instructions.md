---
description: "Use when drafting or revising a ROADMAP.md for a prompt-ingestion dossier or any multi-step task plan. Covers the canonical roadmap structure, waypoint scoping thresholds, and syntax rules for status, dependencies, and cross-references."
applyTo: "**/*.md"
---

## Purpose

A roadmap is read cold by a fresh session and reread many times during execution — every structural and syntax choice either saves that reader time or costs it. This file is the canonical owner of what a high-quality `ROADMAP.md` contains and how it is written. [prompt-ingestion.instructions.md](prompt-ingestion.instructions.md) produces one during Stages 4/6 and defers to this file for its shape and scoping rules; [roadmap-execution.instructions.md](roadmap-execution.instructions.md) governs how a session then walks the finished artifact.

## Required Structure (six sections, this order)

1. **Outcome summary** — two to four sentences stating what this roadmap achieves and why, so a reader grasps the objective before reading anything else. State the destination in the reader's terms, not a restatement of the raw request.
2. **Relevant artifact IDs** — ticket ids, spec ids/slugs, doc paths, code/config file paths the roadmap depends on, referenced by id/path only. Never re-paste an artifact's body here — see [dossier-external-references.instructions.md](dossier-external-references.instructions.md) for the reference format.
3. **Active blockers** — anything currently unresolved that would stop an executing session cold. A blocker needing human judgment belongs to the review/interview loop, not this list; by the time `ROADMAP.md` ships it carries none.
4. **Validation gates** — the exact commands/checks that must pass during and after execution. Name exact commands; never leave a gate as prose like "run the tests."
5. **Roadmap Waypoints** — the complete ordered route, one scoped stop per waypoint. See "Scoping Guidelines" below for sizing and "Syntax Rules" for how each waypoint is written.
6. **Heads-up notes** — a flat list of quirks, gotchas, and good-to-know information gathered during research that would otherwise cost a fresh session time to rediscover.

**Size constraint.** `ROADMAP.md` is a root anchor for the whole effort, not an exhaustive plan — keep it readable in one pass. A sprawling waypoint list or deeply nested sub-tasks is a signal to push complexity into a ticket (see "Scoping Guidelines"), not to grow the file. The roadmap should read like a table of contents with status, not a full project plan.

## Scoping Guidelines

**One waypoint, one measurable outcome.** A waypoint bundling more than one loosely related change is a scoping defect — split it. Merge only when a prior split was too aggressive and produced trivially small fragments with no independent validation gate.

**The single-session threshold decides waypoint vs. ticket.** A waypoint completable by one session in one sitting stays inline and is marked single-session. A waypoint that is too large for one session, or whose internal dependencies are complex enough to need cross-session tracking, is **not** decomposed inline — it becomes a ticket (per [prompt-ingestion.instructions.md's Ticket Creation During Refinement](prompt-ingestion.instructions.md#ticket-creation-during-refinement)), and the roadmap keeps only the ticket id and a one-line summary.

**No forward references.** A roadmap reads as a dependency-ordered route: a waypoint must never depend on something only introduced by a later waypoint. Reorder before shipping rather than leaving an implicit backward dependency for the reader to untangle.

**Surface parallelism explicitly.** When multiple waypoints depend on the same upstream blocker or artifact and have no dependency on each other, say so in the waypoint's dependency notation (see below) instead of forcing a false sequential order that hides the parallel-execution opportunity.

**Watch for implicit dependencies.** A waypoint's real dependency is sometimes undeclared code coupling or a shared file rather than a stated blocker. Check candidate waypoints against the dossier's artifact inventory (`ARTIFACTS.md`) before finalizing order, not just against the stated blocker list.

**Uneven flow is a decomposition defect.** A roadmap with a few tiny waypoints followed by one sprawling one should push the sprawling waypoint's internal complexity into a ticket rather than leave it lumpy in the roadmap.

## Syntax Rules

Consistent syntax lets a reader (and a script) scan a roadmap without re-parsing prose each time. Use these conventions in every `ROADMAP.md`:

- **Waypoint heading.** `### W<n>. <one-line objective>` — sequential numbering (`W1`, `W2`, ...) that never gets reused, even across versioned revisions, so a dependency reference (`depends: W3`) always resolves unambiguously within that file's history.
- **Status marker.** Each waypoint opens its body with exactly one status line: `Status: pending | in-progress | blocked | done`. Use `blocked` only when the blocker is named in the waypoint's own text or in "Active blockers" — a bare `blocked` with no stated reason is not acceptable.
- **Sizing tag.** Immediately after status: `Scope: single-session` or `Scope: ticket <short-id>`. A ticket-scoped waypoint carries no further inline decomposition — see "Scoping Guidelines" above.
- **Dependency notation.** `Depends: W2, W4` (waypoint ids) and/or `Depends: ticket <short-id>` for a ticket-level prerequisite. Omit the line entirely when a waypoint has no dependency — do not write `Depends: none`.
- **Validation line.** `Validate: <exact command>` — one command per line if more than one gate applies. Never a prose description in place of a command.
- **Artifact references.** Cite an id/path exactly as it appears in `ARTIFACTS.md` (ticket short-id, spec slug, file path) — never a paraphrased name. See [dossier-external-references.instructions.md](dossier-external-references.instructions.md) for citing material outside the dossier.
- **Versioned-file naming.** When a prior `ROADMAP.md` is superseded, rename it `ROADMAP.v1.md`, `ROADMAP.v2.md`, ... in place before writing the new content to `ROADMAP.md` — see [prompt-ingestion.instructions.md's versioned-supersession pattern](prompt-ingestion.instructions.md#roadmap-compilation-and-versioning) for the full rule; this file owns only the naming syntax.

## Anti-Patterns

- A waypoint with two unrelated objectives joined by "and".
- A validation gate written as "make sure it works."
- A `Depends:` line pointing at a waypoint number that does not exist in this file (stale after a reorder or version bump).
- Inlining a ticket's full body into a waypoint instead of citing its short-id.
- A roadmap that grows past a single-pass read because ticket-worthy complexity was left inline.
