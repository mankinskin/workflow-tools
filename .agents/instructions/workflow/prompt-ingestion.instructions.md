---
description: "Use before turning a raw prompt into tickets, a spec, or any other complex downstream workflow. Defines the structural shell of the prompt-ingestion pipeline — an extension of the transcript-transformation pipeline that onboards a raw prompt through denoising, research, two informed review/interview loops, and roadmap compilation into a fully refined, zero-open-question deliverable — plus the decision boundary and when to run it."
applyTo: "**/*.md"
---

## Purpose

A raw prompt — a rambling transcript, a dictated ask, a stream-of-consciousness request — must not be handed directly to `tickets.prompt.md`, `spec.prompt.md`, or an implementation session. Structure and scope are extracted first, cheaply, in a bounded pipeline, and only the resulting dossier is used to seed tickets/specs. This closes the gap the raw-prompt path otherwise leaves open: unbounded scope, no verification lens, and no evidence that the eventual tickets actually cover what the requester said.

This pipeline is an extension of [audio-transcript.instructions.md](../../../../context-engine/.agents/instructions/transcripts/audio-transcript.instructions.md), not a parallel process: it reuses that pipeline's denoise stage and dossier-folder conventions verbatim, then carries the cleaned signal onward through research, verification, and planning. Think of it as spell-crafting — the user hands over the raw spell (an unrefined ask) and the pipeline elevates it, preserving the original intent exactly, into the mechanical steps that execute it.

This file is the ingestion shell: it owns the dossier folder layout and the six-stage sequence. [intent-refinement.instructions.md](intent-refinement.instructions.md) owns the recurring technique used at Stages 3 and 5 — the informed review + interview loop that clears ambiguity before it reaches the shipped roadmap.

When the workflow is invoked through `Execute Ingest`, roadmap compilation is
not the terminal state. The lifecycle continues through these explicit states:

1. **Planning** — run the six-stage ingestion pipeline, including interviews
   whenever repository evidence cannot resolve a requirement, and surface the
   decisions made during planning to the user.
2. **Planned and accepted** — Stage 5 closes with no open question and Stage 6
   produces the current `ROADMAP.md`.
3. **Final user review** — present the compiled roadmap and request one explicit
   outcome from the user: `replan` returns the request to the planning loop;
   `approve` authorizes execution of the current roadmap.
4. **Execution** — only after `approve`, hand the same dossier path and
   `ROADMAP.md` to [execute-roadmap](../../../../.agents/prompts/execute-roadmap.prompt.md).

Planning interviews and final roadmap approval are separate interactions. A
successful planning verdict never implies execution approval.

## The Six Stages

Run each stage as a distinct pass; do not collapse them. Each stage has one job and one exit artifact.

1. **Denoise (cheap).** Delegate entirely to [audio-transcript.instructions.md](../../../../context-engine/.agents/instructions/transcripts/audio-transcript.instructions.md) and the [Transcription Agent](../../../../context-engine/.agents/agents/transcription.agent.md) — the same three-stage denoise/restructure/verify pipeline and the same multi-part naming convention. Output: `input.md`/`input-2.md`/... (raw) and the matching `input.clean.md`/`input-2.clean.md`/... (denoised), plus `merged.clean.md` once more than one part exists.
2. **Research and artifact inventory.** Dispatch a read-only [Explore Agent](../../../../context-engine/.agents/agents/explore.agent.md) or [Research Agent](../../../../context-engine/.agents/agents/research.agent.md) pass to gather every existing artifact relevant to the cleaned prompt: tickets (ids + state), specs (ids + slugs), docs, prior transcripts/dossiers, and concrete code/config file paths the eventual work will touch or depend on. Do not re-derive this list later — every downstream stage cites entries from it instead of re-discovering paths. Output: `ARTIFACTS.md`, one row per artifact with id/path, a one-line relevance note, and its current state (e.g. ticket state, spec state, file exists/does not exist yet).
3. **First informed review + interview loop.** Owned by [intent-refinement.instructions.md](intent-refinement.instructions.md). Critique the cleaned prompt against the research just gathered — never against the raw words alone — then apply that file's [interview-dispatch rule](intent-refinement.instructions.md#applying-the-refinement-loop-here) (interview only what the research cannot resolve). Output: `REVIEW.md` with an `Approved as scoped` verdict and a scope decision.
4. **Fully informed dossier creation or restructure.** With the scope decision and the artifact inventory both in hand, dispatch [Research Agent](../../../../context-engine/.agents/agents/research.agent.md) or [Structured Research Agent](../../../../context-engine/.agents/agents/structured-research.agent.md) (dialectic pass, when a conclusion needs adversarial testing) to check each reviewed concern against actual repository capability and produce, in one informed pass: the numbered work-package documents (`01-...md`, `02-...md`, ...), a draft `ROADMAP.md`, and a draft `README.md` index. Each work package carries an outcome, a non-goal, and a validation method.
5. **Second informed review + interview loop.** Owned by [intent-refinement.instructions.md](intent-refinement.instructions.md). Critique the drafted dossier and `ROADMAP.md` for anything newly ambiguous or low-confidence that the drafting pass surfaced, and interview the requester to close it. This loop replaces a separate traceability-checklist stage — coverage already lives in `ARTIFACTS.md` and `ROADMAP.md`, and open questions get resolved by interview, not logged and left open.
6. **Adjustments and roadmap compilation (iterative).** Apply the second loop's resolved answers, then dry-run and refine `ROADMAP.md`/`README.md`/the work packages until no new blocker or open question surfaces. See "Roadmap Compilation and Versioning" and "Roadmap Improvement Loop" below for required contents, dry-run procedure, and the iteration rule. `ROADMAP.md` must ship with zero open questions — that is this stage's exit condition, not an aspiration.

## Resuming an In-Progress Dossier

A single raw prompt is refined iteratively as the standard mode of use, not an edge case: after the pipeline runs, the requester reviews the dossier and can trigger it again with an additional transcript. Treat every such re-invocation as continuing the same dossier, never as a new request — but only when that continuation is anchored to **this session**, per the strict rule below.

**Strict two-condition reuse rule.** Two sessions have in practice worked on similar or overlapping topics in parallel, and a new pipeline invocation in a fresh session reused another session's transcript folder by topic resemblance, collapsing what should have been two isolated dossiers and roadmaps into one. To close that failure mode, an existing dossier folder may be addressed only when at least one of these two conditions holds, evaluated strictly against the **current session**:

1. This session's own conversation history shows it already created or resumed that exact dossier folder earlier in this session, or
2. `session_runtime_view` shows **exactly one** path pinned under relation `intent-ingestion-dossier` by this session.

If neither condition holds — including when `session_runtime_view` shows zero matching pins, or shows more than one (ambiguous) — a new `transcripts/DD-MM-YYYY_<slug>/` folder MUST be created, even if a similarly-named or topically similar dossier already exists from another session. Never scan the `transcripts/` directory for a topically similar existing folder to reuse: continuation detection is scoped strictly to this session's own record (conversation history or its own pins), never to the repository-wide `transcripts/` folder or to another session's work. When condition 1 or 2 holds, still confirm the new ask is thematically continuous with that dossier (a refinement, addition, or correction to the same request) before treating it as a continuation rather than an unrelated new request in the same session.

**Pinning.** Immediately after creating or resuming a dossier folder, pin its path via `session_runtime_pin` with relation `intent-ingestion-dossier` so a later stage, or a later pipeline invocation in the same session, can find it without re-deriving it.

**Continuing instead of duplicating.** When continuing an existing dossier, follow the exact same multi-part convention [audio-transcript.instructions.md](../../../../context-engine/.agents/instructions/transcripts/audio-transcript.instructions.md) uses for a multi-transcript topic:

- Do not recreate `input.md`. Write the new raw text to the next `input-N.md` (`input-2.md`, `input-3.md`, ...) in the same folder, and produce its matching `input-N.clean.md` via Stage 1.
- Update `merged.clean.md` from the full set of clean parts so it reflects the combined intent, not just the newest fragment.
- Re-run Stage 2 research against anything the new part changes, adding new rows to `ARTIFACTS.md` rather than starting a new file.
- Re-run the Stage 3 informed review + interview loop against the combined intent. Version the outgoing `REVIEW.md` (`REVIEW.v1.md`, ...) before writing the refined `REVIEW.md`, using the same versioned-supersession pattern `ROADMAP.md` uses (see "Roadmap Compilation and Versioning" below).
- Re-run Stages 4-6 (drafting, second loop, adjustments) against the updated scope, versioning each superseded work-package document, `ROADMAP.md`, and `README.md` rather than discarding them, so the dossier stays a single coherent history instead of a scatter of near-duplicate folders for what is really one evolving request.

## Roadmap Compilation and Versioning

`ROADMAP.md` is the single, current, most-refined artifact the pipeline produces. It is the entry point a fresh executing session reads first — it must be self-contained enough that a session starting cold from `ROADMAP.md` alone (plus the cited artifact ids/paths) can begin work without re-reading the whole dossier. See [roadmap-execution.instructions.md](roadmap-execution.instructions.md#purpose) for how an executing session treats and walks the compiled roadmap, and [roadmap-authoring.instructions.md](roadmap-authoring.instructions.md) for the required structure, waypoint scoping thresholds, and syntax rules a compiled roadmap must follow — this stage produces that structure, it does not redefine it.

**Iteration rule**: `ROADMAP.md` is expected to be revised as research deepens or execution surfaces new information. Never overwrite a prior iteration in place. Before writing an improved version, rename the existing `ROADMAP.md` to a versioned name (`ROADMAP.v1.md`, `ROADMAP.v2.md`, ...) inside the same dossier folder, then write the new, more refined content to `ROADMAP.md`. Only one file is ever named `ROADMAP.md` — it is always the most current, most refined iteration. The dossier's `README.md` index must point at `ROADMAP.md`, not at a versioned snapshot.

## Roadmap Improvement Loop

A compiled roadmap is a draft until it has been dry-run at least once. Repeat this loop until a dry-run pass surfaces no new blocker or structural defect, then treat the current `ROADMAP.md` as ready to hand off.

**Dry-run procedure**:

1. Read `ROADMAP.md` cold, as the first executing session would — do not use any context from having written it.
2. Walk the waypoint list in order. For each waypoint, check whether everything it needs is actually present: the artifacts it cites resolve (via a bounded `peek-mcp`/`ticket-mcp`/`spec-mcp` probe, not by assumption), its stated objective is a single measurable outcome, its validation gate is an exact command, and its declared dependencies (prior waypoints, tickets, decisions) are already satisfied by that point in the order.
3. Record every gap surfaced this way as one of two kinds:
   - **Blocker** — something that would stop the executing session cold (unresolved decision, missing precondition, an artifact that does not exist, a dependency ordered after the waypoint that needs it).
   - **Informational gap** — something the session could stumble on but is not fully blocking (an ambiguous acceptance check, a missing heads-up note, an unclear ownership boundary between two waypoints).
4. Fix cheap findings directly in `ROADMAP.md`: reorder a waypoint, sharpen an objective to be single-outcome, add a missing validation command, add a heads-up note. Route expensive findings to a ticket per "Ticket Creation During Refinement" below instead of expanding the roadmap prose.

Apply [roadmap-authoring.instructions.md's Scoping Guidelines](roadmap-authoring.instructions.md#scoping-guidelines) during this dry-run pass — forward references, bundled objectives, hidden parallelism, implicit dependencies, and uneven waypoint flow are exactly the defects that section defines and this loop exists to catch.

## Ticket Creation During Refinement

Unlike the read-only stages above, roadmap drafting and adjustment (Stages 4 and 6, including the dry-run loop) are explicitly allowed to create or update tickets and to add entries to the linked artifact set. This is the mechanism for keeping `ROADMAP.md` small: complex waypoint dependencies and large blockers are modeled in the ticket system, not inlined into the roadmap.

- Create or update a ticket when a waypoint is too large for one session, has internal sub-dependencies worth tracking across sessions, or is itself a blocker significant enough to need its own status and history.
- Follow the ticket threshold and workflow in `AGENTS.md`'s Task Routing and `.agents/prompts/tickets.prompt.md` when creating tickets — this stage does not bypass that threshold, it is simply where the decision to cross it gets made for roadmap-sized work.
- After creating or updating a ticket, add its id to `ARTIFACTS.md` and reference it from the roadmap's "Relevant artifact IDs" and "Roadmap Waypoints" sections instead of duplicating its content.
- Do not create a ticket for something the roadmap can state in one line (a single-session waypoint, a simple blocker with an obvious resolution) — that is scope creep in the other direction.

## Decision Boundary

The dossier produced by this pipeline is a bounded research-and-scoping artifact, not an implementation. State this explicitly in the dossier's `README.md`:

- Stages 1, 2, 3, and 5 (denoise, research/inventory, and both informed review + interview loops) are read-only with respect to the codebase and the ticket/spec store: they may read source, docs, tickets, and specs, and may write dossier notes (`REVIEW.md`, interview records), but do not mutate tickets or specs, and do not change workflow or store state.
- Stages 4 and 6 (dossier drafting and roadmap adjustment, including the dry-run loop) are the one exception: they are explicitly allowed to create or update tickets and to add entries to the linked artifact set, per "Ticket Creation During Refinement" above. They still do not create or edit a spec — that remains a separate, later step.
- `ROADMAP.md` is a scoping and sequencing artifact, not a spec — it names the waypoints an executing session should pick up, with complex decomposition delegated to tickets created during refinement. Turning roadmap items into a spec happens in a **separate**, later step — `spec.prompt.md` — consuming `ROADMAP.md` and its linked tickets as input.

This mirrors [escalation-gate.instructions.md](escalation-gate.instructions.md) and [phase-separation.instructions.md](phase-separation.instructions.md): discovery/interview/review happen before implementation, and this pipeline is exactly that discovery phase for a raw prompt — with ticket creation as the one deliberate, scoped exception that keeps the roadmap itself small. Once a roadmap ships, [roadmap-execution.instructions.md](roadmap-execution.instructions.md) governs walking its waypoints methodically.


## When to Run This Pipeline

Run it before `tickets.prompt.md`, `spec.prompt.md`, or any multi-file implementation session whenever the incoming prompt is:

- a raw transcript, dictation, or stream-of-consciousness prompt rather than an already-scoped ask,
- broad enough that "just start implementing" would produce an unbounded session (compare the "Feature or refactor" and "Unfamiliar module" rows in `AGENTS.md`'s Task Routing table),
- ambiguous about whether it is one request or several interleaved concerns.

Skip it for an already-bounded, single-file fix or an ask that already names its acceptance criteria — running the full pipeline on a two-line, unambiguous prompt is pure overhead.

## Related Dossier Workflows

This file owns the six-stage shell for turning one raw prompt into one dossier. Related but distinct dossier workflows live in their own files rather than as sections here:

- [roadmap-authoring.instructions.md](roadmap-authoring.instructions.md) — the structure, scoping thresholds, and syntax rules a compiled `ROADMAP.md` must follow.
- [dossier-external-references.instructions.md](dossier-external-references.instructions.md) — how a dossier cites a ticket, spec, file, other dossier, or true external source.
- [dossier-porting.instructions.md](dossier-porting.instructions.md) — copying a bounded finding or artifact from one dossier into an unrelated one, short of a full [dossier-merge.instructions.md](dossier-merge.instructions.md).
- [dossier-idea-workspace.instructions.md](dossier-idea-workspace.instructions.md) — running a dossier as an open-ended idea workspace before committing to roadmap compilation.

## Cost Note

Stage 1 (denoise) runs on the cheap tier per `transcription.agent.md`'s own `model:` declaration. Stage 2 (research and artifact inventory) is mechanical read-only extraction and belongs on the T3 floor. Stages 3 and 5 (the informed review + interview loops) and Stage 4 (drafting) are judgement-bearing and route per the tier ladder in [model-routing.instructions.md](model-routing.instructions.md); Stage 6 (adjustments and roadmap compilation) is where ticket creation can happen and should route accordingly — do not run the whole pipeline on the orchestrator-tier model when the denoise and inventory passes alone are mechanical.
