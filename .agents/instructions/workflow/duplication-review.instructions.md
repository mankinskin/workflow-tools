---
description: "Use when running or resuming a structured duplication review of the repository's agent guidance corpus: pairwise comparison, similarity classification, and the duplication report."
---

## Purpose

A duplication review finds every duplicated or semantically similar passage across the repository's agent guidance corpus and reports it. It never rewrites, condenses, or deletes the corpus itself — that is Simplify Agent's job, using this review's report as input.

## Scope Resolution

- **Default scope** (no scope given by the caller): every file under `.agents/` — `.agents/agents/*.agent.md`, `.agents/instructions/**/*.instructions.md`, `.agents/prompts/*.prompt.md`, `.agents/skills/**/SKILL.md` — plus the repository-root [AGENTS.md](../../AGENTS.md).
- **Narrowed scope**: the caller (a human argument to the agent, or the argument text passed to the `/duplication-review` prompt) may limit the review to a named subdirectory (e.g. `.agents/instructions/ticket/`) or an explicit file subset. A narrowed scope still includes `AGENTS.md` only if the caller names it or names the whole `.agents/` tree; otherwise treat the named subset as the complete scope.
- Treat an ambiguous narrowing request (a name that could match more than one directory or file) as a blocker to clarify before comparing, rather than guessing.
- This resolved scope is the **comparison scope**: the fixed file universe every pair is drawn from, and the input to the stable sort order `F_1 .. F_n` (see Coverage and Efficiency Rules). It does not change across runs of the same campaign — see Anchor-Subset Scope for Large Corpora for the separate, run-scoped `anchor scope`.

## Workspace

Create one dedicated review folder per run, sibling to `transcripts/`:

```
duplication-reviews/<DD-MM-YYYY>_<scope-slug>/
  pair-ledger.md          # one row per unordered file pair: batch id, verdict, status
  duplicate-passages.md   # one row per marked-section finding: both files' line ranges, classification, verbatim excerpt
  duplication-report.md   # final deliverable
```

When a review runs in Anchor-Subset campaign mode (large comparison scope), nest each run's folder under one persistent, dated campaign folder instead of the flat per-run form above:

```
duplication-reviews/<DD-MM-YYYY>_<scope-slug>_campaign/
  campaign-coverage.md                    # persistent across every run against this comparison scope
  <DD-MM-YYYY>_<anchor-slice-slug>/
    pair-ledger.md
    duplicate-passages.md
    duplication-report.md
```

The `<DD-MM-YYYY>` prefix on the campaign folder records only when the campaign was **first created** and never changes across later runs, even if those runs happen on a different date; the `_campaign` suffix distinguishes it from an ordinary single-run folder sharing the same scope-slug. A campaign is uniquely identified by its `<scope-slug>_campaign` suffix, not by that creation date.

**Locating an existing campaign.** Every campaign folder lives directly under `duplication-reviews/`, never nested deeper and never named after anything outside that folder. Before creating a new campaign folder, list `duplication-reviews/` non-recursively and match by the `_campaign` suffix and scope-slug alone (ignore the date prefix) — this single directory listing is always sufficient; do not grep or semantic-search the rest of the workspace to find a campaign's ledger. If a match exists, resume it (read its `campaign-coverage.md`); only create a new dated campaign folder when no folder with that scope-slug's `_campaign` suffix exists yet.

Never write review artifacts into `.agents/` or `AGENTS.md` themselves.

## Classification

Classify every marked-section match (see Per-Pair Comparison Procedure) as exactly one of:

- `exact duplicate` — near-identical text in both files.
- `near-duplicate` — the same statement, reworded or reordered.
- `thematic overlap` — the same underlying rule or idea, expressed with different wording, structure, or example.
- `no overlap` — no meaningful similarity.

## Coverage and Efficiency Rules

- Read each in-scope file's full content once and hold it for every subsequent comparison; never re-read the same file per pair.
- Seed `pair-ledger.md` with every unordered pair up front so coverage is verifiable and a review can resume mid-run.
- Sort in-scope files by directory path, then filename, into a stable order `F_1 .. F_n`; this ordering drives both pair assignment (below) and same-directory batching.
- Every pair must reach a verdict before the review is complete; an unclassified pair blocks completion rather than being silently skipped.
- Quote real text with real file paths and line ranges for every `exact duplicate` or `near-duplicate` finding; never paraphrase a duplicate into existence.

## Per-Pair Comparison Procedure

This is the exact procedure a batch worker runs for **every** file pair it owns. It marks similar sections directly instead of judging a whole-file verdict in one step.

1. Read file A in full (already cached if it was read for an earlier pair in the same batch — see Anchor-Fixed Batching).
2. Read file B in full.
3. Walk both files section by section (headings, bullet groups, or paragraphs) and identify every section in A that has a matching or overlapping counterpart in B.
4. For each match, mark both sides explicitly: file A's line range, file B's line range, and a per-match classification (see Classification). Quote the verbatim overlapping text for `exact duplicate` and `near-duplicate` matches.
5. If no section in A matches any section in B, emit exactly one finding for the pair with classification `no overlap` — every pair produces at least one finding row, even when nothing was found.
6. Record the pair's **verdict** as the most severe classification among its findings, using this severity order: `exact duplicate` > `near-duplicate` > `thematic overlap` > `no overlap`. The verdict is what goes into `pair-ledger.md`; the individual marked-section findings go into `duplicate-passages.md`.

## Anchor-Fixed Batching

A batch is efficient for a worker when one file — the **anchor** — stays fixed across every pair in the batch, and the other files (the anchor's **targets**) are read once each. Build batches this way instead of grouping arbitrary multi-file clusters:

1. With files sorted `F_1 .. F_n` (see Coverage and Efficiency Rules), anchor `F_i` (for `i = 1 .. n-1`) owns every pair with `F_(i+1) .. F_n` — this assigns each unordered pair to exactly one anchor, with no gaps or double coverage, and total pairs equal `n × (n-1) / 2`.
2. Because `F_i`'s targets are contiguous in directory-sorted order, slicing them in order naturally groups same-directory files into the same batch, which is the preferred grouping (easier reading, higher expected overlap).
3. Cap every batch on two independent limits so a worker never manages too many files or too much text:
   - `MAX_FILES_PER_BATCH` — hard cap on files per batch (anchor + targets), default **8** (anchor + up to 7 targets).
   - `MAX_BATCH_CHARS` — character budget per batch, computed from the dispatched model's context window (see Batch Sizing Formulas below).
4. **Greedy packing per anchor**: starting from `F_i`'s first remaining target, keep adding targets to the current batch while doing so stays within both caps; when the next target would exceed either cap, close the current batch and open a new batch on the same anchor `F_i` with that target. Always include at least one target per batch even if it alone exceeds `MAX_BATCH_CHARS`, so packing always makes progress. Move to anchor `F_(i+1)` once `F_i` has no targets left.
5. This greedy packing keeps batches close to `MAX_BATCH_CHARS` in size (roughly equal, aside from a possibly-smaller final batch per anchor), satisfies the file-count cap, and yields many small batches rather than few large ones.

### Batch Sizing Formulas

- `CHARS_PER_TOKEN = 4` — repository-standard rough heuristic for English/Markdown text.
- `INPUT_FRACTION = 0.5` — reserve the rest of the dispatched model's context window for prompt scaffolding, the per-pair reasoning, and the returned findings.
- `MAX_BATCH_CHARS = context_window_tokens × CHARS_PER_TOKEN × INPUT_FRACTION`. For the default worker model `GPT-5 mini` (400k-token window, see [model-routing.instructions.md](model-routing.instructions.md)): `400,000 × 4 × 0.5 = 800,000` characters. Recompute with `GPT-5.6 Luna`'s 1.05M-token window (`≈ 2,100,000` characters) only when an anchor's remaining targets still cannot form a batch under the mini budget without violating `MAX_FILES_PER_BATCH` — do not raise the model tier for ordinary batches.
- Total unordered pairs in scope: `n × (n-1) / 2` — use this to verify `pair-ledger.md`'s final row count.

## Anchor-Subset Scope for Large Corpora

Anchor-Fixed Batching assigns every anchor `F_i` its full target range `F_(i+1) .. F_n` in one run. For a large comparison scope (e.g. `n = 140`) that produces far too many batches and phases to dispatch and merge in one blocking pass. Anchor-subset scope fixes this by running the campaign across several invocations, each owning only a slice of the anchors, while every pair is still assigned to exactly one anchor exactly once across the whole campaign.

- **Comparison scope** — the full file universe and its fixed sort order `F_1 .. F_n`, resolved once per campaign per Scope Resolution above. It never changes between runs of the same campaign.
- **Anchor scope** — the subset of anchors (a contiguous slice `F_a .. F_b` of the comparison scope's sort order, by default) that this run builds batches and dispatches for. Every anchor in the anchor scope still compares against the **full** remaining comparison scope (`F_(i+1) .. F_n`, not just the slice), per Anchor-Fixed Batching — narrowing the anchor scope only reduces which files sit on the left side of a pair this run, never which files can appear on the right side.
- A file may occupy the anchor role in exactly one run per campaign. The `campaign-coverage.md` ledger (see Workspace) is the single source of truth for which anchors are closed; never re-open a closed anchor or let two runs claim the same anchor.

**When to switch to campaign mode.** Estimate total batches for the whole comparison scope before building anything: `estimated_total_batches ≈ (n × (n-1) / 2) / (MAX_FILES_PER_BATCH - 1)`. If this exceeds `MAX_BATCHES_PER_RUN` (default **40**, ≈7 phases at the default `PHASE_WIDTH`), resolve an anchor scope per the algorithm below instead of building batches for every anchor at once.

**Selecting the next anchor scope.** Read `campaign-coverage.md` to find the first anchor index not yet closed; call it `a`. Walk forward from `a`, accumulating each candidate anchor's own estimated batch count `batches(i) = ceil((n - i) / (MAX_FILES_PER_BATCH - 1))`, and stop extending the slice at the last index `b` before the running total would exceed `MAX_BATCHES_PER_RUN` — always include at least anchor `a` itself even if `batches(a)` alone exceeds the budget, so the campaign always makes progress. Because `batches(i)` shrinks as `i` grows (fewer targets remain), early slices are naturally small (few anchors, each with many targets) and later slices are naturally large (many anchors, each with few targets) — this is expected and requires no special-casing.
- Pairs resolved by this run: `pairs_in_run = Σ_{i=a}^{b} (n - i)`.
- Campaign complete when anchor `n - 1` (the last anchor with any targets) is closed; at that point every pair across every run's `pair-ledger.md` sums to exactly `n × (n-1) / 2` with no gaps or overlaps.

**Coverage ledger.** `campaign-coverage.md` records one row per closed anchor slice: run folder, anchor index range `[a, b]`, file range, pairs resolved, and the run's verdict counts. After a run's pairwise phase and synthesis phase both complete, append its slice as closed before reporting back. A run must never mark a slice closed until every pair it owns has a verdict in that run's `pair-ledger.md`.

**Cross-run synthesis.** The per-run synthesis phase (Two-Phase Workflow) only sees that run's own `duplicate-passages.md` and may under-count ideas that recur across files spanning multiple slices. Once `campaign-coverage.md` shows the campaign complete, run one additional campaign-level synthesis pass over every run folder's `duplicate-passages.md` concatenated together, and write the combined result to `duplication-report.md` at the campaign folder root. Do not attempt this cross-run synthesis while anchors remain open — its clustering would be incomplete and misleading.

## Phased Dispatch

Many small batches, dispatched all at once, would spawn an unmanageable number of parallel subagents. Instead, dispatch batches in sequential phases of bounded width:

1. `PHASE_WIDTH` — maximum concurrent `runSubagent` dispatches per phase, default **6**. Lower it if the surface signals dispatch contention; do not raise it without a stated reason.
2. Number every constructed batch `1..m` in anchor order (per Anchor-Fixed Batching). Compute `num_phases = ceil(m / PHASE_WIDTH)`.
3. For phase `p = 1 .. num_phases`: dispatch batches `[(p-1) × PHASE_WIDTH + 1 .. min(p × PHASE_WIDTH, m)]` in parallel, each targeting [Duplication Batch Worker Agent](../../agents/duplication-batch-worker.agent.md) on the T3 worker model.
4. Wait for every dispatch in phase `p` to return, merge its findings into `pair-ledger.md` and `duplicate-passages.md`, and only then start phase `p + 1`. Do not overlap phases.
5. If a batch worker returns an incomplete or malformed set of rows (fewer findings than its assigned pair count), re-dispatch only that batch once, within the same phase, before escalating.

## Two-Phase Workflow

1. **Pairwise phase**: build batches per Anchor-Fixed Batching, run Phased Dispatch to completion, and merge every worker's returned verdicts and marked-section findings into `pair-ledger.md` and `duplicate-passages.md`.
2. **Synthesis phase** (only after every pair in the ledger has a verdict): re-evaluate `duplicate-passages.md` as a whole — cluster entries that express the same underlying idea across three or more files, simplify each cluster to one representative statement, and drop clusters that turn out to be coincidental phrasing rather than a real duplicated rule. Rank the surviving clusters by occurrence count into `duplication-report.md`, listing every occurrence (file + line range) per idea. Run the synthesis phase on the orchestrator's own model — it needs the cross-batch judgement a cheap model should not be trusted with.

## Reporting Contract

- Render every file reference as a clickable link with repository-relative path and line range per the Clickable Reference Policy in [AGENTS.md](../../AGENTS.md).
- Report counts by verdict and confirm the pairs-evaluated count matches `pair-ledger.md`'s row count (`n × (n-1) / 2`).
- Close with a handoff note pointing to Simplify Agent for any consolidation of the reported duplicates; do not apply consolidation in this workflow.
