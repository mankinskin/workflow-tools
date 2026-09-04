---
description: "Use when merging two or more completed prompt-ingestion dossiers (transcripts/DD-MM-YYYY_<slug>/ folders) that turned out to be thematically related into one consolidated dossier."
applyTo: "**/*.md"
---

## Purpose

A prompt-ingestion dossier (see [prompt-ingestion.instructions.md](prompt-ingestion.instructions.md)) is refined independently per raw prompt. Sometimes several already-refined dossiers turn out to cover the same theme or effort after the fact — refined from separate transcripts, but pointing at one session's worth of work. This file governs folding them into a single dossier without losing any source or silently duplicating shared content.

## When to Merge

Two or more dossiers under `transcripts/` are thematically related enough that a single `ROADMAP.md` and artifact set should govern them going forward, rather than tracking overlapping waypoints across separate folders.

## Required Procedure

1. **Resolve source dossiers.** Validate each named folder is a completed dossier — it has `README.md` and `ROADMAP.md`. Treat an incomplete dossier (mid-pipeline, no `ROADMAP.md` yet) as a blocker: finish [refine-ingest.prompt.md](../../prompts/refine-ingest.prompt.md) on it first, or exclude it.
2. **Create the merged dossier folder.** `transcripts/DD-MM-YYYY_<merged-slug>/`, dated today, named for the combined theme.
3. **Relocate originals unchanged, outside the merged dossier.** `git mv` each source dossier folder in full into `transcripts/_merged-sources/<original-folder-name>/` — an archive location that is a sibling of `transcripts/`'s dossier folders, never a subfolder of the merged dossier itself. Never edit a source file in place; this archive exists only so the pre-merge history stays inspectable, not as content the merged dossier depends on.
4. **Duplication pass.** Run [duplication-review.prompt.md](../../prompts/duplication-review.prompt.md) with its scope narrowed to the archived sources' `ARTIFACTS.md`/`ROADMAP.md`/`README.md`/work-package files — an explicit file subset is a valid narrowed scope per [duplication-review.instructions.md's Scope Resolution](duplication-review.instructions.md#scope-resolution). Its report is the shared-vs-unique map the merge draws from; do not consolidate from eyeballing the sources instead.
5. **Consolidate the merged artifacts**, applying [duplication-consolidation.instructions.md](duplication-consolidation.instructions.md)'s Concept Grouping, Authoritative Location, and Snippet Compilation mechanics (only the mechanics — this step is dossier-specific, not a corpus-wide consolidation run). Every source's content — shared and unique — is absorbed into the merged artifact's own text; nothing is left behind as a pointer back to a source:
   - **`ARTIFACTS.md`**: union of rows across sources, deduplicated by artifact id/path, written as the merged dossier's own rows.
   - **`ROADMAP.md`**: one outcome summary describing the combined effort (not a concatenation of the sources' summaries); the union of active blockers, validation gates, and heads-up notes with duplicates collapsed to one entry each; one ordered waypoint list combining every source's waypoints, collapsing an exact- or near-duplicate waypoint pair to a single entry and keeping both only when the review found genuine thematic overlap describing two distinct waypoints.
   - **`README.md`**: index pointing at the merged `ROADMAP.md` as the entry point. It does not name, link, or otherwise reference `transcripts/_merged-sources/` or any original dossier folder — the merged dossier must read as a normal, freshly-produced dossier, not as a wrapper over its sources.
6. **Dry-run the merged roadmap.** Apply [prompt-ingestion.instructions.md's Roadmap Improvement Loop](prompt-ingestion.instructions.md#roadmap-improvement-loop) to the merged `ROADMAP.md` before treating it as ready — merging can introduce the same ordering and dependency defects a single dossier's drafting pass can.
7. **Version later revisions.** If the merged dossier itself is revised further, follow [prompt-ingestion.instructions.md's versioned-supersession pattern](prompt-ingestion.instructions.md#roadmap-compilation-and-versioning) rather than overwriting the merge in place.

## Constraints

- Same read-only/ticket-creation boundary as [prompt-ingestion.instructions.md's Decision Boundary](prompt-ingestion.instructions.md#decision-boundary): the merge does not create or edit a spec, and ticket creation follows the same exception (a merged waypoint too large for one session becomes a ticket, not an inline block).
- Never overwrite or delete a source dossier's content — it is moved to the archive, not edited, and stays inspectable under `transcripts/_merged-sources/`.
- **The merged dossier must be completely resolvable standalone.** No file inside the merged dossier folder may reference a source dossier's folder name, path, or id, or link to `transcripts/_merged-sources/` — a fresh session reading only the merged dossier's own files must get everything it needs without ever opening a source. The archive exists for audit/history, not as a dependency of the merged dossier's content.
- A merged artifact must not restate a shared statement in more than one place; keep the authoritative-location-plus-reference pattern from [duplication-consolidation.instructions.md](duplication-consolidation.instructions.md) rather than letting the merge reintroduce the duplication the review just found.

## Reporting

Return: source dossiers moved and their new `transcripts/_merged-sources/` paths, the merged dossier's folder path, the duplication review's workspace location, concepts consolidated (shared vs. unique), and the final merged artifact paths ending with `ROADMAP.md`.
