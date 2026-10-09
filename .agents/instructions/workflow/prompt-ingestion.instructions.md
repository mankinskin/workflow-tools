---
description: "Use before turning a raw prompt into tickets, a spec, or any other complex downstream workflow. Defines the structural shell of the prompt-ingestion pipeline — an extension of the transcript-transformation pipeline that onboards a raw prompt through denoising, research, two informed review/interview loops, and roadmap compilation into a fully refined, zero-open-question deliverable — plus the decision boundary and when to run it."
applyTo: "**/*.md"
---

## Purpose

A raw prompt — a rambling transcript, a dictated ask, a stream-of-consciousness request — must not be handed directly to `tickets.prompt.md`, `spec.prompt.md`, or an implementation session. Structure and scope are extracted first, cheaply, in a bounded pipeline. Settled scope may seed necessary planning tickets/specs; only explicit execution approval authorizes the durable mission handoff and implementation. This closes the gap the raw-prompt path otherwise leaves open: unbounded scope, no verification lens, and no evidence that the resulting work actually covers what the requester said.

This pipeline is an extension of [audio-transcript.instructions.md](https://github.com/mankinskin/context-engine/blob/main/.agents/instructions/transcripts/audio-transcript.instructions.md), not a parallel process: it reuses that pipeline's denoise stage and dossier-folder conventions verbatim, then carries the cleaned signal onward through research, verification, and planning. Think of it as spell-crafting — the user hands over the raw spell (an unrefined ask) and the pipeline elevates it, preserving the original intent exactly, into the mechanical steps that execute it.

This file is the ingestion shell: it owns the dossier folder layout and the six-stage sequence. [intent-refinement.instructions.md](intent-refinement.instructions.md) owns the recurring technique used at Stages 3 and 5 — the informed review + interview loop that clears ambiguity before it reaches the shipped roadmap.

When the workflow is invoked through `Execute Ingest`, roadmap compilation is
not the terminal state. The lifecycle continues through these explicit states:

1. **Planning** — run the six-stage ingestion pipeline, including interviews
   whenever repository evidence cannot resolve a requirement, and surface the
   decisions made during planning to the user.
2. **Planned and accepted** — Stage 5 closes with no open question and Stage 6
   produces the current `ROADMAP.md` with a passing review-only dry run of
   every waypoint and the complete expected execution path, followed by a
   passing final formal review of that same revision.
3. **Final user review** — present the compiled roadmap and request one explicit
   outcome from the user: `replan` returns the request to the planning loop;
   `approve` authorizes execution of the current roadmap. The user checks the
   decisions and tasks against intent, not repairs to numbering, structure,
   dependencies or other formal planning defects.
4. **Execution** — only after `approve`, hand the same dossier path and
   `ROADMAP.md` to `/execute-ingest`. It binds the approved roadmap to a mission
   and then hands the mission-backed route to
   [execute-roadmap](https://github.com/mankinskin/meta-workspace/blob/main/.agents/prompts/execute-roadmap.prompt.md).

Planning interviews and final roadmap approval are separate interactions. A
successful planning verdict never implies execution approval.

### Planning subphases and phase tracking

Record `Workflow phase` in the dossier's `README.md` and every handoff,
separately from roadmap/waypoint execution status:

- `planning / dossier-discovery`: Stages 1-3 gather evidence, consult the user,
  and record scope and decisions inside the dossier only. Existing entities
  may be read; do not create or update planning tickets/specs yet.
- `planning / entity-preparation`: after Stage 3 has settled scope, Stages 4-6
  prepare the dossier, compile the route, and create or update only necessary
  planning tickets/specs under [Planning Entities During Refinement](#planning-entities-during-refinement).
  If no entity is needed, record that and continue; this is not a mandatory
  ticket/spec-creation step.
- Any new scope question or required interview, including Stage 5 findings,
  returns the affected work to dossier-discovery. Resume entity-preparation
  only after the answers are recorded and scope is settled again.
- Dry run and final formal review remain read-only except for dossier
  evidence and corrections. Finish any permitted entity preparation before
  those passes; neither pass mutates entities or implements a waypoint.
- `awaiting-approval` begins only after both current-revision gates pass.
  `replan` returns to planning; explicit `approve` enters `execution`.
  Closure records `complete` only after the validated route is complete.

These are lifecycle states and planning subphases, not additional ingestion
stages. Planning never implements repository/product changes, activates or
dispatches work, or creates/mutates a mission. Lifecycle bookkeeping and
required feedback remain governed by their own session/feedback procedures.

## The Six Stages

Run each stage as a distinct pass; do not collapse them. Each stage has one job and one exit artifact.

Treat every observation whose inputs are already available as a **Planning
read** under [roadmap-authoring.instructions.md](roadmap-authoring.instructions.md#planning-reads-and-execution-side-effects).
Complete and record all such reads and gates during these stages. Do not defer
an independent Planning read into an executable roadmap waypoint.

1. **Transcript preparation and cleaning (cheap).** Delegate entirely to [audio-transcript.instructions.md](https://github.com/mankinskin/meta-workspace/blob/main/.agents/instructions/transcripts/audio-transcript.instructions.md) and the [Transcription Agent](https://github.com/mankinskin/meta-workspace/blob/main/.agents/agents/transcription.agent.md) — the authoritative five-pass paragraphs/punctuation-review/denoise/restructure/verify pipeline and the same multi-part naming convention. These five transcript passes all belong inside this outer Stage 1; they do not renumber the six ingestion stages. Output: `input.md`/`input-2.md`/... (raw) and the matching `input.clean.md`/`input-2.clean.md`/... (denoised), plus `merged.clean.md` once more than one part exists. **This stage is not complete while the clean artifact carries a flagged ambiguity** — see "Ambiguity Resolution Gate" immediately below, which runs before Stage 2 starts.
2. **Research and artifact inventory.** Dispatch a read-only [Explore Agent](https://github.com/mankinskin/context-engine/blob/main/.agents/agents/explore.agent.md) or [Research Agent](https://github.com/mankinskin/context-engine/blob/main/.agents/agents/research.agent.md) pass to gather every existing artifact relevant to the cleaned prompt. The sweep covers all six domain stores — `ticket`, `spec`, `test`, `session`, `feedback`, and the `transcripts/` dossiers — plus docs and the concrete code/config file paths the eventual work will touch or depend on. [entity-discovery.instructions.md](entity-discovery.instructions.md) owns the per-store surfaces, fallbacks and reuse rule; Stage 2 supplies the discovery evidence later preparation/creation boundaries consume. Do not re-derive this list later — every downstream stage cites entries from it instead of re-discovering paths. Output: `ARTIFACTS.md`, one row per artifact with id/path, a one-line relevance note, and its current state (e.g. ticket state, spec state, file exists/does not exist yet), **plus a mandatory store-coverage table** with the columns `store | searched? | surface used | result`, one row per store. A store that could not be searched is recorded as `partial — degraded` with the failure named; it is never recorded as an empty result.
3. **First informed review + interview loop.** Owned by [intent-refinement.instructions.md](intent-refinement.instructions.md). Critique the cleaned prompt against the research just gathered — never against the raw words alone — then apply that file's [interview-dispatch rule](intent-refinement.instructions.md#applying-the-refinement-loop-here) (interview only what the research cannot resolve). Output: `REVIEW.md` with an `Approved as scoped` verdict and a scope decision.
4. **Fully informed dossier creation or restructure.** With the scope decision and the artifact inventory both in hand, dispatch the [Roadmap Authoring Agent](https://github.com/mankinskin/meta-workspace/blob/main/.agents/agents/roadmap-authoring.agent.md) to produce, in one informed pass: the numbered work-package documents (`01-...md`, `02-...md`, ...), a draft `ROADMAP.md`, and a draft `README.md` index. Each work package carries an outcome, a non-goal, and a validation method. This is entity-preparation: dossier edits and the narrowly permitted planning ticket/spec preparation below are allowed, never implementation, activation or other execution side effects. When a reviewed concern needs adversarial testing before it can be trusted, dispatch [Structured Research Agent](https://github.com/mankinskin/context-engine/blob/main/.agents/agents/structured-research.agent.md) first and hand its synthesis to the Roadmap Authoring Agent as input evidence — Structured Research Agent has no `edit` tool and must never be the one writing the dossier or `ROADMAP.md` itself.
5. **Second informed review + interview loop.** Owned by [intent-refinement.instructions.md](intent-refinement.instructions.md). Critique the drafted dossier and `ROADMAP.md` for anything newly ambiguous or low-confidence that the drafting pass surfaced, and interview the requester to close it. This loop replaces a separate traceability-checklist stage — coverage already lives in `ARTIFACTS.md` and `ROADMAP.md`, and open questions get resolved by interview, not logged and left open.
6. **Adjustments and roadmap compilation (iterative).** Dispatch the [Roadmap Authoring Agent](https://github.com/mankinskin/meta-workspace/blob/main/.agents/agents/roadmap-authoring.agent.md) again to apply resolved answers and finish permitted entity preparation, then dry-run and refine `ROADMAP.md`/`README.md`/the work packages until no blocker or open question surfaces. Perform the final formal review after the dry run; both current-revision verdicts and zero open questions are this stage's exit condition, not an aspiration. See "Roadmap Compilation and Versioning" and "Roadmap Improvement Loop" below for the procedures and iteration rule.

## Ambiguity Resolution Gate (before Stage 2)

Resolving every transcript ambiguity is the pipeline's actual first step, not a
by-product of the later review loops. [audio-transcript.instructions.md](https://github.com/mankinskin/context-engine/blob/main/.agents/instructions/transcripts/audio-transcript.instructions.md)
permits a standalone denoise pass to ship with an explicitly flagged
ambiguity (an unresolved mis-transcription, an ambiguous referent, a term
without an unambiguous reading) — that is the correct behavior for a
standalone transcription. Inside this pipeline it is not: no dossier note,
research pass, work-package document, or `ROADMAP.md` line may be written
while `input.clean.md` / `merged.clean.md` still carries an open ambiguity
flag.

- **Check the gate immediately after Stage 1**, before Stage 2 (research and
  artifact inventory) begins. Read the delivered clean artifact's flagged
  ambiguities (Stage 3's "Verify" checklist in `audio-transcript.instructions.md`
  produces these as explicit notes, not silent guesses) and enumerate each one.
- **Resolve every flag through interview, not inference.** Dispatch the
  [Interview Agent](https://github.com/mankinskin/context-engine/blob/main/.agents/agents/interview.agent.md)
  with the exact ambiguous term/referent/phrase and its surrounding context
  from the transcript, and ask the requester to confirm the intended reading.
  Do not resolve a flagged ambiguity from repository evidence alone and do not
  guess a "most likely" reading yourself — the flag exists precisely because
  the transcript did not make the intended reading unambiguous, and only the
  requester can supply the missing fact.
- **Fold the answer back into the clean artifact.** Update `input.clean.md`
  (or the relevant `input-N.clean.md` and `merged.clean.md`) with the resolved
  reading before moving on, so every later stage — research, both informed
  review loops, drafting, and roadmap compilation — reads a transcript that is
  already unambiguous. Do not carry a resolved-in-conversation answer forward
  only in memory; the artifact itself must reflect the resolution.
- **Exit condition**: zero remaining ambiguity flags in the clean artifact(s)
  for this dossier. This gate is binding for every dossier, including a
  continuation (see "Resuming an In-Progress Dossier" below) — a newly added
  `input-N.md` part gets its own clean pass and its own ambiguity check before
  `merged.clean.md` is updated or any later stage re-runs.
- **Do not conflate this gate with Stage 3/5's review loops.** Those loops
  interview the requester about scope, requirements, and drafted-content
  gaps discovered against research; this gate resolves literal transcript
  ambiguity (an unclear term, a mis-heard word, an ambiguous referent) before
  any of that later reasoning starts. A transcript that still contains an
  unresolved ambiguity flag is not a valid input to Stage 2.

## Resuming an In-Progress Dossier

**Decide explicitly, in two steps, before writing any dossier file.** A shared topic, component, file, tool, or the same session never justifies extending an existing dossier on its own.

**Step 1 — is there an active dossier?** Two sessions have in practice worked on similar topics in parallel, and a fresh session reused another session's dossier folder by topic resemblance, collapsing two isolated dossiers into one. An existing dossier is therefore active only when at least one of these conditions holds, evaluated strictly against the **current session**:

1. This session's own conversation history shows it already created or resumed that exact dossier folder earlier in this session, or
2. `session_runtime_view` shows **exactly one** dossier pinned under relation `intent-ingestion-dossier` by this session (matching `ce://<workspace>/dossier/<folder-name>`), or
3. The requester names that exact existing dossier and explicitly authorizes this session to take it over.

For an authorized takeover, read the prior owner from the dossier's handover record. Before changing any other dossier content, record both the prior owner and the current session as the new owner in `README.md`; if the prior owner cannot be verified, stop and ask rather than inventing it. Prior-session status is irrelevant: never infer authorization from expiry, status, a pin, or history. If a requester names an existing dossier without explicitly authorizing takeover, stop and ask whether to authorize takeover or create a new dossier. Zero matching pins, more than one pin, or a similarly named dossier from another session otherwise mean there is no active dossier: create a new `transcripts/DD-MM-YYYY_<slug>/` folder and skip Step 2. Never scan `transcripts/` for a topically similar folder to reuse.

**Step 2 — classify the request against the active dossier's roadmap.**

| Classification | Signal | Action |
|---|---|---|
| Refinement | The requester explicitly asks to extend, continue, refine, or replan that dossier, or names it (an `/execute-ingest` `replan` counts); or the request clearly changes the same requested outcome — it corrects the scope, answers an open question, or adds a requirement to the same deliverable. | Extend the active dossier. |
| Authorized takeover | The requester explicitly named the dossier and authorized this session to take it over. | Record the prior and new owner in `README.md`, then extend the dossier; do not infer authorization from prior-session status. |
| Standalone | The request has its own outcome, even if it shares a topic, component, file, tool, or session with the active dossier. | Create a new dossier; the new `ARTIFACTS.md` may cite the active dossier as evidence. |
| Borderline | The request is closely related to the active roadmap and might count as an extension, but is not clearly a refinement. | Stop before writing any file. Ask the requester one question per [question-quality.instructions.md](question-quality.instructions.md) that names the active dossier and offers exactly `extend <dossier>` or `new dossier`, then follow the answer. Never infer the answer from silence. |

**Record the classification.** The `README.md` of the dossier used states the classification (refinement, authorized takeover, standalone, or confirmed borderline) and its signal. An authorized takeover also records the verified prior owner and current session as the new owner.

**Pinning.** Immediately after creating or resuming a dossier folder, pin its canonical URN via `session_runtime_pin` with relation `intent-ingestion-dossier` (e.g. `entity_urn: "ce://default/dossier/13-09-2026_my-slug"`) so a later stage, or a later pipeline invocation in the same session, can find it without re-deriving it. Do not pass a raw path like `path:...`, as entity pins require `ce://<workspace>/<store>/<entity>` format.

**Continuing instead of duplicating.** When continuing an existing dossier, follow the exact same multi-part convention [audio-transcript.instructions.md](https://github.com/mankinskin/context-engine/blob/main/.agents/instructions/transcripts/audio-transcript.instructions.md) uses for a multi-transcript topic:

- Do not recreate `input.md`. Write the new raw text to the next `input-N.md` (`input-2.md`, `input-3.md`, ...) in the same folder, and produce its matching `input-N.clean.md` via Stage 1.
- Update `merged.clean.md` from the full set of clean parts so it reflects the combined intent, not just the newest fragment.
- Re-run Stage 2 research against anything the new part changes, adding new rows to `ARTIFACTS.md` rather than starting a new file.
- Re-run the Stage 3 informed review + interview loop against the combined intent. Version the outgoing `REVIEW.md` (`REVIEW.v1.md`, ...) before writing the refined `REVIEW.md`, using the same versioned-supersession pattern `ROADMAP.md` uses (see "Roadmap Compilation and Versioning" below).
- Re-run Stages 4-6 (drafting, second loop, adjustments) against the updated scope, versioning each superseded work-package document, `ROADMAP.md`, and `README.md` rather than discarding them, so the dossier stays a single coherent history instead of a scatter of near-duplicate folders for what is really one evolving request.

## Roadmap Compilation and Versioning

`ROADMAP.md` is the single, current, most-refined artifact the pipeline produces. It is the entry point a fresh executing session reads first — it must be self-contained enough that a session starting cold from `ROADMAP.md` alone (plus the cited artifact ids/paths) can begin work without re-reading the whole dossier. See [roadmap-execution.instructions.md](roadmap-execution.instructions.md#purpose) for how an executing session treats and walks the compiled roadmap, and [roadmap-authoring.instructions.md](roadmap-authoring.instructions.md) for the required structure, waypoint scoping thresholds, and syntax rules a compiled roadmap must follow — this stage produces that structure, it does not redefine it.

**Iteration rule**: `ROADMAP.md` is expected to be revised as research deepens or execution surfaces new information. Never overwrite a prior iteration in place. Before writing an improved version, rename the existing `ROADMAP.md` to a versioned name (`ROADMAP.v1.md`, `ROADMAP.v2.md`, ...) inside the same dossier folder, then write the new, more refined content to `ROADMAP.md`. Only one file is ever named `ROADMAP.md` — it is always the most current, most refined iteration. The dossier's `README.md` index must point at `ROADMAP.md`, not at a versioned snapshot.

## Mission-backed execution handoff

Stages 1–6 keep the transcript dossier as planning evidence and do not create or
mutate a mission. Before the requester approves execution, the current
`ROADMAP.md` is the reviewable draft. After explicit `approve`, the
`Execute Ingest` handoff resolves or creates the mission for that exact dossier
using the procedure in
[roadmap-execution.instructions.md](roadmap-execution.instructions.md#mission-backed-execution).
The accepted mission record then owns the durable execution plan; record its
canonical URN in the dossier's `README.md` and `ARTIFACTS.md`, and retain the
raw inputs, reviews, and other planning evidence as source provenance.

Once bound, execute from the mission's accepted record and its generated
`ROADMAP.md`/Part projections. Treat the migrated dossier's `ROADMAP.md` as an
unchanged source snapshot, not a second editable status authority. Update the
mission through a validated bundle and `mission publish` with its expected
revision, then verify with `mission check-generated`; never hand-edit generated
mission documents. A blocked migration stops the handoff. A skipped migration
is successful only after `mission get` confirms the bound record.

## Roadmap Improvement Loop

A compiled roadmap remains a planning draft until its current revision passes
an explicit, review-only dry run of every waypoint. This is a mandatory gate
before suggesting the roadmap for approval, not an execution rehearsal: do
not implement waypoints, run mutating validation commands, or mutate repository,
entity-store, or external-system state during the pass.

The first waypoint must be fully executable after approval without a required
question or design decision when its stated expectations hold. The same applies
to the complete happy path: all expected-scenario decisions, preconditions,
intermediate results, and success criteria must be defined during planning.
Only genuinely unexpected evidence or previously unknown context encountered
during execution may require new questions, decisions, or interruptions. A
dry-run pass does not guarantee that execution will succeed.

**Dry-run procedure**:

1. Read `ROADMAP.md` cold, as the first executing session would — do not use any context from having written it.
2. Walk every waypoint in dependency order as an implementation agent would,
   checking readiness rather than doing the work. Verify existing input
   artifacts with bounded read-only probes. An output not yet created is valid
   only when a declared earlier waypoint creates it with a defined expected
   result and validation; do not require that future output to exist already.
   Check each measurable outcome, target boundary, acceptance criteria,
   validation method, commit checkpoint, and dependency against the expected
   state at that point. Read every explicit `Expected state`, confirm its
   observable postconditions satisfy dependent waypoint inputs, and chain
   those states through the final outcome. Confirm W1 can start after approval and that every
   later waypoint can proceed without rediscovering requirements or asking
   the user to choose expected behavior. Reject Planning reads whose inputs
   already exist; complete them during planning. A Dependent read-only
   waypoint remains subject to the authoring contract's branch-local rules.
3. Record every gap surfaced this way as one of two kinds:
   - **Blocker** — a required open decision, missing expectation or input,
     undefined prerequisite result, or dependency ordered too late.
   - **Informational gap** — a missing heads-up note or other clarity defect.
     An ambiguous acceptance check or ownership boundary affecting expected
     execution is a Blocker, not an optional note.
4. Resolve findings during planning. Correct evidence-backed structural defects
   in the dossier and roadmap; return open decisions and missing expectations
   to the informed review/interview loop and resolve them with the user before
   approval. Apply the planning branch of the
   [Blocker-to-Waypoint Policy](roadmap-authoring.instructions.md#blocker-to-waypoint-policy):
   fully specified prerequisite implementation may remain on the route, but
   do not defer known choices or interviews into execution waypoints.
5. Repeat the cold pass after corrections until no blocker, open decision,
   missing expectation, or structural defect remains. Record the verdict,
   current roadmap revision identity, and readiness evidence for every
   waypoint in the dossier's `REVIEW.md`, including W1 and the complete happy
   path. Then perform the final formal review below; a dry-run pass alone
   does not permit presentation. A change to scope, targets, expectations,
   decisions, dependencies, validation, numbering or revision mappings
   invalidates both gates and requires another dry run followed by final
   formal review before approval; execution-only
   status/evidence updates do not revise the approved plan.

Apply [roadmap-authoring.instructions.md's Scoping Guidelines](roadmap-authoring.instructions.md#scoping-guidelines) during this dry-run pass — forward references, bundled objectives, hidden parallelism, implicit dependencies, and uneven waypoint flow are exactly the defects that section defines and this loop exists to catch.

### Final formal review

After the cold dry run passes, make a distinct final review of the exact
current revision against the complete
[authoring contract](roadmap-authoring.instructions.md), not just new policy
keywords. Record a criterion/evidence/result table and a separate PASS/FAIL
verdict in `REVIEW.md`, tied to the same revision as the dry run.

Check all of the following before declaring the plan formally acceptable:

- Current labels are exactly contiguous `W1` through `Wn` in document order:
  reject W0, duplicates, gaps and suffixes such as W2a. Dependencies, Part
  navigation and all current label references resolve; historical labels are
  revision-bound with snapshots/mappings when renumbered.
- Required structure and schema, valid table delimiters, dedicated Parts and
  navigation, linked inputs/entities, scope/targets/non-goals, session prompts,
  dependency order and prerequisite ownership satisfy the authoring contract.
- Every waypoint has concrete preconditions, an explicit expected post-state,
  observable acceptance/validation and a checkpoint. Post-states satisfy
  dependent inputs and collectively reach the outcome. A completed waypoint
  in a continuation has verified completion evidence and is not rerun.
- No known blocker, missing expectation, design choice or interview remains
  on the expected execution path; W1 and the entire happy path are ready.
- Lifecycle phase, entity preparation/read-back and linked decisions agree.
  Mission-specific schema/order and provenance rules remain binding; the
  review cannot require invented fields or edits to generated projections.

Resolve any finding during planning, then repeat both passes on the corrected
revision. Only two passing current-revision verdicts allow awaiting-approval.
User approval is an intent/content decision, not a substitute for this gate.

## Planning Entities During Refinement

After scope is settled, entity-preparation may create or update necessary
planning tickets/specs through existing store APIs in their existing
planning/draft states. This exception does not authorize implementation,
activation, execution dispatch, mission mutation, or changes to unrelated or
active entities. A necessary change to an active entity is execution work,
not this exception. Never invent state flags or approval machinery.

- Use the ticket threshold from
  [`AGENTS.md`](../../../AGENTS.md) and
  [tickets.prompt.md](../../../ticket/.agents/prompts/tickets.prompt.md)
  to decide whether a planning ticket is necessary; a spec is prepared only
  when its contract is needed for the settled implementation scope.
- Before a preparation mutation, cite the Stage 2 coverage or a refreshed
  bounded search. If an existing entity owns the work, record a reuse form from
  [entity-discovery.instructions.md](entity-discovery.instructions.md) instead
  of creating a duplicate. Unavailable discovery evidence is a planning
  blocker, not proof of non-existence.
- Record objective, acceptance boundary, dependencies and validation before
  mutation. Read the resulting entity back by its specific id; record its
  canonical link, state and evidence in `ARTIFACTS.md` and the relevant Part.
  A bounded list is not read-back, and a guessed id is not an entity.
- Do not create a ticket for work that remains a single-session waypoint.

## Decision Boundary

The dossier produced by this pipeline is a bounded research-and-scoping artifact, not an implementation. State this explicitly in the dossier's `README.md`:

- Stages 1, 2, 3, and 5 (denoise, research/inventory, and both informed review + interview loops) are read-only with respect to the codebase and the ticket/spec store: they may read source, docs, tickets, and specs, and may write dossier notes (`REVIEW.md`, interview records), but do not mutate tickets or specs, and do not change workflow or store state.
- Stages 4 and 6 may write dossier artifacts and prepare only scoped planning tickets/specs under the exception above. Dry run and formal review themselves are review-only. All repository/product implementation, activation, mission mutation and other execution side effects begin only after explicit `approve`.
- `ROADMAP.md` is a scoping and sequencing artifact, not a spec. It links necessary prepared planning entities without duplicating their bodies; additional ticket/spec work outside the preparation exception remains approved execution work.

This mirrors [escalation-gate.instructions.md](escalation-gate.instructions.md) and [phase-separation.instructions.md](phase-separation.instructions.md): discovery/interview/review happen before implementation, and this pipeline is exactly that discovery phase for a raw prompt. Once an approved roadmap ships, [roadmap-execution.instructions.md](roadmap-execution.instructions.md) governs its external side effects and dependency-ordered waypoints.


## When to Run This Pipeline

Run it before `tickets.prompt.md`, `spec.prompt.md`, or any multi-file implementation session whenever the incoming prompt is:

- a raw transcript, dictation, or stream-of-consciousness prompt rather than an already-scoped ask,
- broad enough that "just start implementing" would produce an unbounded session (compare the "Feature or refactor" and "Unfamiliar module" rows in [`AGENTS.md`](../../../AGENTS.md)'s Task Routing table),
- ambiguous about whether it is one request or several interleaved concerns.

Skip it for an already-bounded, single-file fix or an ask that already names its acceptance criteria — running the full pipeline on a two-line, unambiguous prompt is pure overhead.

## Related Dossier Workflows

This file owns the six-stage shell for turning one raw prompt into one dossier. Related but distinct dossier workflows live in their own files rather than as sections here:

- [roadmap-authoring.instructions.md](roadmap-authoring.instructions.md) — the structure, scoping thresholds, and syntax rules a compiled `ROADMAP.md` must follow.
- [dossier-external-references.instructions.md](dossier-external-references.instructions.md) — how a dossier cites a ticket, spec, file, other dossier, or true external source.
- [dossier-porting.instructions.md](dossier-porting.instructions.md) — copying a bounded finding or artifact from one dossier into an unrelated one, short of a full [dossier-merge.instructions.md](dossier-merge.instructions.md).
- [dossier-idea-workspace.instructions.md](dossier-idea-workspace.instructions.md) — running a dossier as an open-ended idea workspace before committing to roadmap compilation.

## Cost Note

Stage 1 (denoise) runs on the cheap tier per `transcription.agent.md`'s own `model:` declaration. Stage 2 (research and artifact inventory) is mechanical read-only extraction and belongs on the T3 floor. Stages 3 and 5 (the informed review + interview loops) and Stages 4 and 6 (drafting and compilation inside the dossier) are judgement-bearing and route per the tier ladder in [model-routing.instructions.md](model-routing.instructions.md). Do not run the whole pipeline on the orchestrator-tier model when the denoise and inventory passes alone are mechanical.
