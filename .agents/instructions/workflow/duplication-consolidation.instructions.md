---
description: "Use when consolidating findings from a completed duplication review (pair-ledger.md, duplicate-passages.md, duplication-report.md) into one authoritative source per duplicated concept, with reference-only replacements at every other occurrence."
---

## Purpose

Consolidation takes a duplication review's structured findings and turns each real duplicate into one authoritative snippet plus reference-only occurrences elsewhere. It does not re-derive findings (that's the Duplication Review Agent's job) and does not run Simplify Agent's judgment-call interview loop — it acts mechanically on evidence the review already classified.

## Input Contract

Expect a completed review workspace per [duplication-review.instructions.md Workspace](duplication-review.instructions.md#workspace). Default to the most recently modified matching folder under `duplication-reviews/` when the caller does not name one. Treat a missing file, or any unclassified pair remaining in `pair-ledger.md`, as a blocker — do not consolidate from a partial review.

## Consolidation Candidates

Only `exact duplicate` and `near-duplicate` findings are mechanical consolidation candidates — the same statement is genuinely present in more than one place. `thematic overlap` findings name the same topic with materially different content and are **not** auto-consolidated: report them as candidates for Simplify Agent's judgment-call interview loop instead of compiling a replacement for them.

## Concept Grouping

1. Start from `duplication-report.md`'s "Top Duplicated Ideas" clusters (three-or-more-file recurrences) as the primary grouping.
2. Also scan `duplicate-passages.md` directly for `exact duplicate`/`near-duplicate` findings that did not make the report's threshold (e.g. a two-file exact duplicate) — group these into their own concept when the passage expresses one coherent, reusable rule.
3. Merge a report cluster and a below-threshold finding into one concept when they describe the same rule with the same wording family, even if the report treated them separately.
4. Each concept must name every occurrence explicitly: file, line range, and verdict — a concept with only one remaining occurrence after grouping is not a duplicate and is dropped.

## Authoritative Location Selection

For each concept, choose exactly one occurrence's file as the authoritative source, in this priority order:

1. **`AGENTS.md`** — if any occurrence lives there, it stays authoritative; `AGENTS.md` is the global, outranking source per its own precedence rules, and no other file may hold a competing copy of the same rule.
2. **An `.instructions.md` file** among the occurrences — instruction files are the designated single-owner location for a reusable rule; prefer the file whose existing topic most closely matches the concept.
3. **A `SKILL.md`** among the occurrences, only if no `.instructions.md` occurrence exists.
4. **Never** an `.agent.md` or `.prompt.md` template, when any occurrence above is available — a rule that only lives in templates today still moves to an instructions file (create a new focused one under `.agents/instructions/<workflow>/` per [README.md](../README.md)'s layout convention if no existing file fits) rather than letting one template become the copy every other template must reference.

## Snippet Compilation

1. Read every occurrence's current text in full (do not trust the review's quoted excerpt alone — files may have changed since the review ran).
2. Compile one canonical statement of the concept: prefer the authoritative occurrence's existing wording when it is already the clearest and most complete; otherwise merge the clearest phrasing across occurrences into a single statement, preserving every distinct constraint any occurrence carries (never drop a constraint one variant had that another lacked).
3. Place the compiled snippet at the authoritative location: update the existing section in place if the concept already has a home there, or add a new section there when it does not.

## Replacement Compilation

For every non-authoritative occurrence:

1. Replace the duplicated passage with a short reference to the authoritative location: a clickable link (repository-relative path, with a line range once the authoritative edit is applied) per the Clickable Reference Policy in [AGENTS.md](../../AGENTS.md).
2. Keep only the occurrence's genuinely local content next to the reference — a short clause naming how the rule applies in that file's context — per the Authoring Contract rule that a template "may state only the local application of a rule needed to establish the template's responsibility, input, output, or boundary." Do not reproduce the full rule text next to the reference.
3. Record the exact `oldString`/`newString` pair for the replacement before moving to the next occurrence; do not apply it yet.

## Mechanical Execution

1. Compile every concept's authoritative-snippet edit and every occurrence's replacement edit fully, across all concepts, before applying anything.
2. Group the compiled edits by file. Within a file that receives edits from more than one concept, apply them **bottom-to-top by current line number** so an earlier edit never shifts the location an unapplied edit still needs to match.
3. Apply each edit by matching its exact current text (not the review's stored line numbers, which may be stale) — re-read the file immediately before editing if any prior edit in this run touched it.
4. After all edits to a file are applied, re-read the file once to confirm every intended replacement landed and no unrelated content was altered.
5. Never edit a file, integrate a branch, or delete a worktree with unmerged commits that another agent actively owns, and never revert, stage, or commit another agent's in-progress work — check `board_show` per [board.instructions.md](../ticket/board.instructions.md) before the first edit to any file in the changeset.

## Reporting Contract

Return, per concept: the concept name, its authoritative location (file + line range after edits), every occurrence and whether it was replaced or was the authoritative copy, and the exact edits applied. Separately list every `thematic overlap` candidate deferred to Simplify Agent, and any concept skipped because it collapsed to a single occurrence. Close with a reminder that committing the changeset is Commit Agent's job, not this workflow's.
