---
description: "Use when copying a specific finding, waypoint, or artifact row from one prompt-ingestion dossier into a different, otherwise-unrelated dossier, without merging the two dossiers wholesale. Covers when porting applies instead of dossier-merge, provenance notes, and avoiding orphaned back-references."
applyTo: "**/*.md"
---

## Purpose

Not every cross-dossier reuse is a merge. [dossier-merge.instructions.md](dossier-merge.instructions.md) folds two whole, thematically-overlapping dossiers into one. Porting is smaller and more common: a single finding, decision, artifact row, or waypoint from a completed dossier turns out to matter for a **different** dossier that is not otherwise related to it — for example, a later request depends on one specific decision an earlier session already made. This file governs that bounded copy.

## When Porting Applies (vs. Merge)

- **Port** when only a bounded slice of a source dossier is relevant to the target — one `ARTIFACTS.md` row, one heads-up note, one waypoint's outcome — and the two dossiers otherwise cover unrelated efforts.
- **Merge** instead when the two dossiers' entire scope overlaps thematically enough that tracking them as one going forward makes more sense than two — use [dossier-merge.instructions.md](dossier-merge.instructions.md) for that case.
- If it is unclear which applies, check whether the target dossier would need most of the source's `ROADMAP.md` to make sense of the ported item. If yes, that is scope creep toward a merge; stop and merge instead.

## Required Procedure

1. **Identify the exact source.** Resolve the source dossier's folder path and the specific file/section the item comes from — never "the other dossier" (see [dossier-external-references.instructions.md](dossier-external-references.instructions.md) for citation format when only referencing, not porting).
2. **Absorb the content fully into the target.** Copy the finding, row, or waypoint's content verbatim into the target dossier's own artifact (`ARTIFACTS.md` row, a heads-up note, a new or existing waypoint). The target dossier must make sense on its own — a reader must not need to open the source dossier to understand the ported item.
3. **Add a one-line provenance note.** Attach a short citation of where the item came from (`Ported from transcripts/DD-MM-YYYY_<slug>/ARTIFACTS.md`) immediately next to the ported content. This is a historical trace, not a live dependency: the provenance note is informational only, and the target dossier's correctness never depends on the source dossier still existing or being reachable.
4. **Version the target roadmap if it changes.** If porting touches `ROADMAP.md`, follow the standard versioned-supersession pattern in [prompt-ingestion.instructions.md's Roadmap Compilation and Versioning](prompt-ingestion.instructions.md#roadmap-compilation-and-versioning) — rename the prior version, do not overwrite in place.

## Do Not Orphan the Source

Porting is a copy, not a move, by default: the source dossier keeps its own copy of the item unless the work it describes is explicitly being reassigned away from the source's scope entirely (for example, a ticket originally tracked in the source dossier is formally moved to the target's roadmap). When reassigning rather than copying, update the source dossier to note the item was ported out and where, so a later reader of the source is not left looking for work that quietly disappeared.

## Constraints

- Never leave a bare pointer as the only copy of a ported item ("see the other dossier for details") — that fails the standalone-readability bar this file shares with [dossier-merge.instructions.md's constraint](dossier-merge.instructions.md#constraints) that a dossier must be resolvable without opening another one.
- Do not port an entire dossier's content item-by-item as a workaround to avoid running [dossier-merge.instructions.md](dossier-merge.instructions.md)'s duplication-review step — if the port grows to cover most of the source, switch to a merge.
- A ported item never fabricates detail the source did not actually contain; port exactly what was written, not an inferred elaboration of it.
