---
description: "Use at the start of and throughout every session, before every sub-agent dispatch: the canonical tier ladder and model preference ordering. Covers capability gating, context isolation, model selection, dominated models, delegation rules, and parallel fan-out."
applyTo: "**"
---

## Model Cost Awareness & Routing

> **Capability gate — read first.** This strategy requires a subagent-capable surface that exposes a `runSubagent` (or equivalent) tool with a selectable `model`. If no such tool is loadable in the current session, this entire section is **inert**: do the work inline using the tactical mechanics in the other orchestration files and skip the ladder. Do not narrate a routing plan you cannot execute, and do not spend reasoning budget describing delegation that will not happen.

> **Context isolation — the single most important rule.** A subagent inherits **none** of the current session's context: no conversation history, no prior findings, no shared "we". A context-dependent prompt does not fail loudly — it burns a full agent spawn just to reply "I have no prior context" and hand the question back to you. Every subagent prompt **must be self-contained**. Checklist before dispatch:
> - name every file with its full workspace-relative path (never "the file we discussed")
> - paste the exact snippet, error, or scope the subagent must act on
> - state the repository root and any command/cwd assumptions
> - define every referent — no "this", "that fix", or "the earlier change"
> - state the exact return shape you want back (e.g. `scope | finding | pointer`)

When a subagent tool *is* available, token cost is a function of *which model* does the work, not only how much context it sees. Be model-cost-aware: reserve expensive, high-capability models for work that genuinely needs them, and delegate routine or bulk work to smaller, cheaper models.

This matters most in sessions driven by a large, expensive model. Treat that model as an active **router**: it plans and reasons at a high level, then dispatches routine subtasks to cheaper models via `runSubagent` (passing an explicit cheaper `model`) instead of spending premium tokens itself.

## Tiered Model Ladder (Smartness vs Cost)

This table is the **canonical** tier ladder for the repository. [orchestrator-delegation.instructions.md](orchestrator-delegation.instructions.md) references it rather than restating it — do not fork a second copy.

Prices come from `workflow-tools/session/crates/model-prices/model_prices.json` and are USD per 1M tokens. Re-check with `sync_model_prices.py --query <model>` rather than trusting these numbers indefinitely — see [model-prices.instructions.md](model-prices.instructions.md).

**Context windows are not in the price table.** No `github-copilot` row publishes `context_window`; the `ctx` values below are carried from each model's upstream (genai-prices) row or vendor documentation. Where an entry reads *unpublished*, no source gives a number — treat that as "unknown", not "large".

**Every model named below was verified to be offered by `runSubagent` on 2026-07-27** (see "Roster is not the catalogue"). The price table lists every vendor's catalogue; this table lists only what can actually be dispatched.

| Tier | Preferred models | in$/M · cread$/M · out$/M · ctx | Use for | Avoid for |
|---|---|---|---|---|
| T0 — Orchestrator (most expensive) | Claude Opus 5 | 5 · 0.5 · 25 · 1M | Large-scope planning, cross-cutting architecture, high-level reasoning, review of dense content or a single critical artifact, final synthesis/decisions | Command batches, bulk output summarization, mechanical edits, wide file sweeps |
| | Claude Opus 4.8 | 5 · 0.5 · 25 · 1M | Prior-generation equivalent; prefer Opus 5 | |
| T1 — Complex implementation (gate ceiling) | GPT-5.6 Terra | 2.5 · 0.25 · 15 · 1.05M | High-risk or cross-cutting implementation, dense refactors, gnarly debugging where the default tier already failed once. Prefer Terra when the slice needs a very large context window | Routine slices the default tier handles; anything the cheap tier can pre-digest |
| | GPT-5.3-Codex | 1.75 · 0.175 · 14 · 400k | Heavy code-generation slices with bounded input | |
| **T2 — Default implementation** | **Claude Sonnet 5** | **2 · 0.2 · 10 · 1M** | **The default for all delegated implementation, review, targeted debugging, and moderate multi-file edits.** Dominates Sonnet 4.5 (3 · 0.3 · 15 · 1M) on every priced axis at the same window, so prefer it over any older Sonnet | Long research sweeps or bulk summarization a cheap model can pre-digest |
| T3 — Cheap worker (floor) | **GPT-5 mini** *(default)* | **0.25 · 0.025 · 2 · 400k** | Running and summarizing command/tool-call batches, condensing large tool outputs, summarizing many large files or artifacts, first-pass research triage, and judgement-free extraction. Nothing offered on this surface is cheaper on input — this is the floor | Final architectural decisions, subtle correctness review of dense artifacts |
| | Kimi K2.7 Code *(T3 code-specialist)* | 0.95 · 0.19 · 4 · 262k | Use for bulk code-shaped work where a code-specialist model materially improves edit quality; pick only when the task requires code-specialist capabilities — GPT-5 mini remains the T3 default | |
| | MAI-Code-1-Flash *(T3 code-specialist)* | 0.75 · 0.075 · 4.5 · *unpublished* | Use for code-shaped bulk work at the cheap tier; priced identically to GPT-5.4 mini on input, cache read, and output. No context window is published for it, so prefer GPT-5.4 mini whenever the input size is near a limit — and GPT-5 mini remains the T3 default | |
| | GPT-5.6 Luna | 1 · 0.1 · 6 · 1.05M | The wide-context cheap option: use when the input does not fit 400k, or when a cheap model must digest a huge input *and* emit non-trivial code | |
| | GPT-5.4 mini | 0.75 · 0.075 · 4.5 · 400k | Step up from GPT-5 mini when the unit needs real reasoning over what it read | |

There is no tier below T3 on this surface: no offered model undercuts GPT-5 mini on input or cache read, so judgement-free extraction and bulk triage share the same band. There is no T4 — a document citing one is stale.

### Two mechanisms, two jobs

Routing has two independent controls. Confusing them produces both false bans and false confidence.

| | **This ladder** | **`mcp-toolmon`** |
|---|---|---|
| Answers | *Which model should receive this unit?* | *May this model call this tool directly?* |
| Scope | `runSubagent` dispatch | MCP `tools/call` traffic |
| Shape | Preference ordering — judgement, applied by you | Graded budget — arithmetic, applied by the middleware |
| Outcome | A recommended model, deviable with a stated reason | `Allow`, `Delegate`, or `Reject` on an unresolvable `caller_model` |

Neither is a prohibition list. The gate **never denies a model outright** — it computes `base_budget = round((1 − output_mtok / 60) × 100)` and compares it to an empirical per-tool cost, so a pricier model keeps full access to cheap tools and loses only the token-heavy ones. An unmeasured tool costs 0 and is always allowed, and a grant offset can lift any model's budget. See [model-prices.instructions.md](model-prices.instructions.md).

The `X = 15` threshold in [orchestrator-delegation.instructions.md](orchestrator-delegation.instructions.md) belongs to neither column: it decides only **whether a model runs as orchestrator**. Do not reuse it as dispatch eligibility — "at or below X" is not a selection rule, and reading it as one is what lets an arbitrary same-priced model look defensible.

### Dominated models: default away, do not ban

These models are beaten by a laddered peer on **every** priced axis, so there is no cost argument for reaching for them. Route to the dominating peer by default. They remain dispatchable — pick one only for a reason you state (the surface rejects your first choice, or the model has a capability its peer lacks), never from familiarity.

| Model | Dominated by | On |
|---|---|---|
| `Claude Sonnet 4.5` (3 · 0.3 · 15) | `Claude Sonnet 5` (2 · 0.2 · 10) | input, cache read, output — same 1M window |
| `Claude Haiku 4.5` (1 · 0.1 · 5) | `GPT-5 mini` (0.25 · 0.025 · 2) | input, cache read, output, window |
| `Gemini 3.5 Flash` (1.5 · 0.15 · 9) | `GPT-5.6 Luna` (1 · 0.1 · 6) | input, cache read, output, window |
| `Gemini 3.6 Flash` (1.5 · 0.15 · 7.5) | `GPT-5.6 Luna` (1 · 0.1 · 6) | input, cache read, output, window |

Older Opus and Sonnet generations are likewise superseded by their current equivalents.

Note that the gate cannot express this: dominance is a *relative* judgement across four axes, while `base_budget` reads only `output_mtok`. Claude Sonnet 4.5 and GPT-5.6 Terra both price at `out 15` and therefore receive the **identical** budget of 75 — the gate has no opinion between them. Preference ordering is exactly the gap the gate leaves open, which is why it lives here as guidance rather than there as enforcement.

**Do not derive a model from a vendor family name.** "A Sonnet", "a mini", "a Flash" is not a selection — it is the habit that produces these mistakes. Name a model from the table, or state why you are going outside it.

### Roster is not the catalogue

`workflow-tools/session/crates/model-prices/model_prices.json` is a **vendor catalogue**. It lists models `runSubagent` will refuse. Routing a unit to a catalogue-only model does not degrade gracefully — the dispatch errors outright and the spawn is wasted.

- Only route to a model verified present in the surface's model list. To re-verify, dispatch one trivial subagent with a deliberately wrong model string; the error response enumerates every available model.
- **Model-name format is exact.** `"GPT-5.3-Codex (copilot)"` is hyphenated; `"GPT-5.3 Codex (copilot)"` errors. Copy strings from the surface list, not from prose.
- If a model in this table is rejected, fall back to the next entry in the same band and update this table — do not silently land at T2.
- **Verified available, 2026-07-27** — **dominated** entries are fully dispatchable but have no cost argument in their favour (see "Dominated models" above): Claude Fable 5, Claude Opus 4.6, Claude Opus 4.7, Claude Opus 4.8, Claude Opus 4.8 (fast mode) (preview), Claude Opus 5, Claude Sonnet 4.5 *(dominated)*, Claude Sonnet 4.6, Claude Sonnet 5, Claude Opus 4.5, Claude Haiku 4.5 *(dominated)*, Gemini 3.1 Pro, Gemini 3.5 Flash *(dominated)*, Gemini 3.6 Flash *(dominated)*, Gemini 2.5 Pro, GPT-5.3-Codex, GPT-5.4 mini, GPT-5.4, GPT-5.5, GPT-5.6 Luna, GPT-5.6 Sol, GPT-5.6 Terra, GPT-5 mini, Kimi K2.7 Code, MAI-Code-1-Flash, Auto *(escape hatch only)*. Notably **absent**: every `*-flash-lite` model, GPT-5.4 nano, and GPT-5.4 Pro.
- `MAI-Code-1-Flash` is now priced (github-copilot provider row) and placed in the T3 band above as a code-specialist peer of GPT-5.4 mini.
- `Auto` delegates model selection away from you and defeats cost-aware routing when used as a default. It is permitted only as an explicit escape hatch: use `Auto (copilot)` when no tier model fits or when the surface rejects the intended model. Because it delegates model selection away from the caller and prevents the cost gate from reasoning about the choice, any use must be stated explicitly in the dispatch rationale and not used as a default.

### Optimizing the cheap tier (T3)

Most delegated volume lands here, so T3 dominates real spend. Route it on the right metric:

- **Select on `input_mtok` + `cache_read_mtok`, not `output_mtok`.** T3 units are input-heavy and output-tiny — they swallow a long log or many files and return a few lines. The output column is nearly irrelevant to their bill; the input and cache-read columns are the whole cost.
- **This inverts the naive ranking.** Claude Haiku 4.5 (in 1 · cread 0.10 · 200k) looks cheap on output ($5/M) but is the *most* expensive input option in the band with the smallest window. GPT-5 mini (in 0.25 · cread 0.025 · 400k) costs 4x less on input, 4x less on cache read, and carries a 2x larger window. Do not reach for Haiku by habit.
- **Fit the context window before anything else.** A unit whose input does not fit forces truncation, a re-dispatch, or a silently incomplete answer — each of which costs more than the tier ever saved. Send wide sweeps to GPT-5.6 Luna (1.05M); reserve GPT-5 mini for inputs that genuinely fit 400k.
- **Batch aggressively at T3.** One subagent digesting twenty files beats twenty subagents digesting one file each; spawn overhead is per-dispatch and does not shrink with the model.
- Kimi K2.7 Code is available as a T3 code-specialist option: it costs ~4x the default's input and ~8x its cache-read, so pick it only when the task is genuinely code-specialist, not for generic bulk work.
- **Do not verify cheap output with an expensive model.** If a T3 result needs checking, check it with ground truth (a grep, a `--check` run, a bounded read), not by re-running the unit a tier up. Re-running upward erases the saving twice over.
- **One retry, then step up exactly one band.** T3 failure goes to T2. Jumping a cheap unit straight to T1/T0 is how low-tier routing turns into a net loss.
- **The Gemini Flash models are dominated.** Gemini 3.5 Flash and 3.6 Flash (in 1.5 · cread 0.15 · 1M) are beaten by GPT-5.6 Luna on input, cache read, output, and context window. Between the two, 3.6 Flash dominates 3.5 Flash. Neither belongs in T3.

### Picking a tier

- **Start at T2.** Claude Sonnet 5 is the default implementation model; deviate only for a stated reason.
- **Drop to T3** when the unit is bulk, mechanical, or read-only triage, or needs no judgement at all — that is where most delegated volume belongs.
- **Climb to T1** only after a T2 attempt was wrong or too shallow, or when the slice is plainly cross-cutting and high-risk. Record why in the delegation note.
- **Budget pressure** shifts the whole ladder down one step: T2 work goes to T3, T1 work goes to T2.
- Among models of equal cost, prefer the latest generation and the larger context window.

## Per-Template `model:` Declaration (Ticket [66acb737](../../../.ticket/tickets/66acb737-71d6-4585-a921-b597f7c88e8e/ticket.toml))

This is the canonical contract for the `model:` frontmatter field on every `.agents/agents/*.agent.md` template. Spec [ec3b13f1 Per-template MCP tool grants](../../../.spec/specs/ec3b13f1-ae9f-4f11-b3f9-e8fa3877afbd/spec.toml) explicitly lists this field as a **non-goal** — it does not define this contract. There is no separate spec for `model:`; this instruction file is the sufficient contract surface because the field is a routing default, not a product behavior: it has one producer (the template loader), one consumer (`runSubagent`'s no-`model` path), and its correctness is fully checked by the validation commands below rather than by acceptance-criteria-driven product testing. If the resolution or override mechanics ever need enforcement in code (not just convention), promote this section to a spec at that point.

**Schema.** `model:` is a single string frontmatter field, value equal to a `model_id` in [workflow-tools/session/crates/model-prices/model_prices.json](../../../workflow-tools/session/crates/model-prices/model_prices.json) and to a "Preferred models" entry in the Tiered Model Ladder table above (bare name, no `"(Vendor)"` suffix — the vendor suffix is a `runSubagent` dispatch-time concern, not a template-declaration concern). Every template under `.agents/agents/*.agent.md` MUST declare exactly one `model:` value. A template with no `model:` field is a bug in that template, not a valid "inherit default" state.

**Resolution (AC2).** When `runSubagent` is invoked against a template and the caller does not pass an explicit `model` argument, the dispatcher resolves the model to that template's declared `model:` value. Explicit `model` arguments on the call always take precedence over the template default — the template value is a fallback, not a floor or ceiling.

**Override-audit rule (AC4).** An explicit `model` argument that names a model *more expensive* than the template's declared tier (i.e., higher on the ladder) is an override. Every such override MUST be recorded with a one-line reason in the session record (the handoff/session-workflow entry for that dispatch) — e.g. "escalated ticket-refinement.agent.md from Claude Sonnet 5 to GPT-5.6 Terra: prior T2 attempt under-specified acceptance criteria on a cross-cutting spec." Overrides *downward* (cheaper than declared) do not require a recorded reason — the ladder already treats dropping a tier as the safe default direction; only climbing needs justification. This mirrors the existing "Climb to T1 only after..." and "Escalate a subtask up a tier only for quality insufficiency, and record why" rules elsewhere in this file — the override-audit rule generalizes those to *any* declared-tier override, not just T2→T1.

**Class-to-tier mapping applied to the current templates:**

| Template | Declared `model:` | Tier | Rationale |
|---|---|---|---|
| `orchestrator.agent.md` | Claude Opus 5 | T0 | Sole planning/delegation entry point; no direct execution tools |
| `implement.agent.md` | Claude Sonnet 5 | T2 | Default implementation class |
| `audit.agent.md` | Claude Sonnet 5 | T2 | Findings-first review needs judgement over checks |
| `handoff.agent.md` | Claude Sonnet 5 | T2 | Synthesizes session state into a coherent handoff, not pure extraction |
| `interview.agent.md` | Claude Sonnet 5 | T2 | Requirement clarification needs judgement |
| `iteration.agent.md` | Claude Sonnet 5 | T2 | Sequences gate decisions across Review/Interview/Commit/Handoff |
| `review.agent.md` | Claude Sonnet 5 | T2 | Verifies acceptance criteria against evidence |
| `roast.agent.md` | Claude Sonnet 5 | T2 | Critique requires judgement, not just extraction |
| `spec.agent.md` | Claude Sonnet 5 | T2 | Spec authoring/traceability needs judgement |
| `brainstorm.agent.md` | Claude Sonnet 5 | T2 | Generates distinct, grounded opportunity directions and compares tradeoffs |
| `testing.agent.md` | Claude Sonnet 5 | T2 | Validation slice selection needs judgement |
| `ticket-refinement.agent.md` | Claude Sonnet 5 | T2 | Ticket authoring/planning needs judgement |
| `explore.agent.md` | GPT-5 mini | T3 | Read-only, judgement-free bounded probes (AC3) |
| `research.agent.md` | GPT-5 mini | T3 | First-pass research triage, bulk artifact digestion (AC3) |
| `commit.agent.md` | GPT-5 mini | T3 | Mechanical: pre-commit hooks, rule regen, conventional messages (AC3) |
| `transcription.agent.md` | GPT-5.4 mini | T3 | Bulk text transform needing real reasoning over content, not zero-judgement extraction |
| `context-enrichment.agent.md` | GPT-5.6 Terra | T1 | Enriches context across related artifacts |
| `simplify.agent.md` | GPT-5.6 Terra | T1 | Simplifies guidance and instruction content |
| `guidance-lifecycle.agent.md` | Claude Sonnet 5 | T2 | Authors and publishes repository guidance through the required lifecycle |
| `session-bootstrap.agent.md` | GPT-5 mini | T3 | Initializes session context and routine state |
| `merge.agent.md` | GPT-5.6 Terra | T1 | Coordinates merge-ready integration work |
| `cleanup.agent.md` | GPT-5.4 mini | T3 | Performs bounded cleanup tasks |
| `structured-research.agent.md` | GPT-5.6 Terra | T1 | Synthesizes structured repository research |
| `online-research.agent.md` | GPT-5 mini | T3 | Performs focused external research |
| `writing.agent.md` | GPT-5.6 Terra | T1 | Produces considered written artifacts |
| `framing.agent.md` | GPT-5.4 mini | T3 | Frames bounded work for downstream agents |
| `refactoring.agent.md` | GPT-5.6 Terra | T1 | Plans and executes code refactoring work |
| `code-architect.agent.md` | GPT-5.6 Terra | T1 | Designs code-level architecture |
| `surface-design.agent.md` | GPT-5.6 Terra | T1 | Designs user-facing surfaces |
| `live-validation.agent.md` | GPT-5 mini | T3 | Runs focused live validation checks |
| `installer.agent.md` | GPT-5 mini | T3 | Handles routine installation work |
| `bug-report.agent.md` | GPT-5.4 mini | T3 | Captures and structures bug reports |
| `session-learning.agent.md` | GPT-5.6 Terra | T1 | Extracts durable session learnings |
| `scoping.agent.md` | GPT-5.6 Terra | T1 | Defines implementation scope and boundaries |

This satisfies AC3: Explore, Research, and Commit all route to GPT-5 mini, cheaper than Claude Sonnet 4.5 on every priced axis (see "Dominated models" above) and cheaper than the Claude Sonnet 5 default used by the remaining classes.

**AC5 — deferred.** The benchmark in ticket [10d21210 Define a synthetic benchmark session](../../../.ticket/tickets/10d21210-7168-4ed4-8e99-f6fb0e6e08db/ticket.toml) is not yet built as of this ticket's implementation. AC5 ("measured against that benchmark, model distribution is no longer uniform and the mechanical delegation does not run on the strong tier") cannot be validated without fabricating numbers, so it is marked **DEFERRED-pending-10d21210** in ticket 66acb737. The structural change that AC5 will measure (non-uniform per-template tiers, mechanical classes on T3) is in place as of this section.

## Delegation Rules

- In a large-model session, delegate to a cheaper subagent model when the subtask is: a batch of command or tool calls, summarization of large or numerous tool outputs, or research/summarization across many large files or artifacts.
- Give the subagent a **self-contained prompt** (see the context-isolation checklist above) and pin the intended cheaper `model` explicitly on `runSubagent`.
- **Model-name format:** the `model` field must be `"Model Name (Vendor)"`, e.g. `"Claude Sonnet 5 (copilot)"` or `"GPT-5 mini (copilot)"`. A bare label like `"mini"` or `"cheap"` will error or be silently ignored. The string must match the surface's model list exactly, punctuation included — see "Roster is not the catalogue" above.
- **Default model:** when a dispatch does not justify a different tier, pin `"Claude Sonnet 5 (copilot)"`. Going outside the ladder is allowed but never silent — name the model and the reason in the dispatch rationale.
- Ask the subagent to return only the distilled finding — scope, result, blocker, pointer — not raw output. The expensive model reasons over the summary, not the bulk.
- Reserve the high-capability tier for planning, high-level reasoning, and review of dense content or individual artifacts.

## When NOT to Delegate (The Floor)

- Each subagent is a full agent loop with real spawn overhead. Delegating a single bounded read (one `peek --grep`, one small file window) costs **more** than doing it inline.
- Delegate only when the subtask is *bulky*, *numerous*, or *context-heavy* — many files, a long log to digest, a wide search to triage. For one small bounded operation, just do it yourself.
- "Delegate routine or bulk work" means **bulk**, not **trivial**. Over-delegation is its own token bonfire.

## Verify Subagent Output Before Acting

- Treat every subagent summary as an **unverified claim**. Subagents hallucinate like any model.
- Before acting on a claimed workspace, branch, file state, command result, or external effect, verify the claim against same-session ground truth (for example, a real grep, `--check` run, bounded read, or status command). A contradictory claim is a sub-agent fault, not state to infer around.
- Reasoning over a summary is fine; trusting it blindly is how a hallucinated refactor ships.

## Parallel Fan-Out (Highest-Throughput Pattern)

- Independent **read-only** probes can be dispatched **concurrently** in a single block, then reasoned over as merged results. This is the single highest-throughput delegation pattern.
- Good fan-out targets: survey N files/crates at once, run several independent searches, gather evidence from multiple subsystems in parallel.
- Keep fan-out to read-only work; do not parallelize writes to overlapping scope.
- Each parallel prompt must still be independently self-contained.

## Failure Path

- If a subagent errors, returns empty, refuses, or replies that it lacks context: first retry **once** with a more self-contained prompt (the usual cause is context isolation, not a broken tool).
- If it still fails, abandon delegation and do the subtask inline; record the failure as a one-line finding.
- Escalate a subtask *up a tier* only for **quality** insufficiency (the cheaper model's answer is wrong or too shallow), and record why.
- For a worker-tier step that fails a *test* specifically (not a generic error/refusal), the retry cap and required escalation action are defined in [retry-limit.instructions.md](retry-limit.instructions.md): exactly one self-fix retry, then escalate — do not extend this general failure path's retry-once rule into a second same-tier attempt for test failures.

## Inspection Before Delegation or Premium Reasoning

- Use bounded inspection tooling (`peek` CLI, `repo_map.toon`, interface skeletons) to render reduced, focused views of artifacts before either spending expensive-model tokens or handing the artifact to a subagent.
- A focused, reduced view is often enough for the expensive model; the full artifact usually is not needed.
