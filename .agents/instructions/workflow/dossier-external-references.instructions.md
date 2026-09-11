---
description: "Use when a roadmap or dossier artifact must reference material outside the dossier folder — other dossiers, tickets, specs, code, or non-repository documentation. Covers reference format, verification before citing, and when to inline vs point."
applyTo: "**/*.md"
---

## Purpose

A dossier under `transcripts/DD-MM-YYYY_<slug>/` constantly points outward — to tickets, specs, code paths, other dossiers, and occasionally something outside the repository entirely — instead of inlining that material. This file is the single place defining how those outward references are made, so every dossier and roadmap cites consistently and a reader can tell, at a glance, how trustworthy and how current a given reference is.

## Reference Categories

Every outward reference falls into exactly one of three categories. State which one applies when the category is not obvious from context.

1. **Repository artifacts** — a ticket, spec, or file that lives in this repository and is resolvable by MCP/CLI tooling right now. Cite by id/path only (ticket short-id, spec slug, file path), never by paraphrased name, and verify it resolves before citing it (a bounded `ticket-mcp`/`spec-mcp`/`peek-mcp` probe, not an assumption).
2. **Other dossiers** — a different `transcripts/DD-MM-YYYY_<slug>/` folder's content. Cite the exact folder path plus the specific file inside it (`transcripts/03-09-2026_other-topic/ROADMAP.md`, not "the other dossier"). Copying content across dossiers, rather than just citing it, is governed by [dossier-porting.instructions.md](dossier-porting.instructions.md), not this file.
3. **True external references** — a URL, a vendor page, or any document outside the repository and outside the dossier system. Flag these explicitly as external and unresolvable by repository tooling. Never fabricate or guess a URL to fill this category — per the workspace's security requirements, only cite a URL that was actually supplied by the user or actually produced by a tool in this session.

## Verification Before Citing

- A repository artifact reference must be checked to resolve at the time it is written into the dossier. A ticket id that no longer resolves, or a file path that has moved, is a drift signal — record it against the roadmap's "Active blockers" (per [roadmap-authoring.instructions.md](roadmap-authoring.instructions.md)) rather than citing it anyway.
- A true external reference cannot always be re-verified live. When it can't, mark it with an as-of date (`as of 03-09-2026`) instead of asserting it is still current — this tells a later reader the citation is a snapshot, not a live guarantee.
- Never assert a fact about an external reference's content beyond what was actually read or quoted. If only a title or URL is known, say so plainly rather than inferring the content.

## Inline vs. Point

Follow the authoritative-location-plus-reference pattern already established in [duplication-consolidation.instructions.md](duplication-consolidation.instructions.md): a fact lives in exactly one place, and every other mention points at it.

- **Inline** a fact only when it is small and load-bearing enough that forcing a lookup would slow the reader down more than the duplication costs — a single line, a single exact command, a single id.
- **Always point** (never inline) an entire artifact body: a ticket's full description, a spec's full body, another dossier's full `ROADMAP.md`. Cite the id/path and let the reader fetch it.
- When a roadmap needs several facts from the same external artifact, cite the artifact once and let the reader pull the rest, rather than scattering several separately-cited fragments of the same source through the roadmap.

## Format

Use the [Clickable Reference Policy](../../../AGENTS.md) format for any reference that has one (ticket, spec, doc, file, log) — this file does not restate that policy, only which category a dossier reference belongs to and when to trust it.
