---
description: "Use at the start of and throughout every session: when to operate as orchestrator, the sub-agent delegation contract, and how to classify work into capability roles. Model selection itself lives in model-routing.instructions.md."
applyTo: "**"
---

## When to Activate

Activate this rule when:
- The running model's `output_mtok` (USD per 1M output tokens) **strictly exceeds** threshold `X = 15` (1500 credits/1M at 100 credits = $1)
- The task is bulky, numerous, or context-heavy enough to benefit from delegated execution even if below threshold

**Trigger rule**: At or below X, execute directly. Strictly above X, operate as orchestrator and delegate routine execution.

**Driving field**: `output_mtok` (the `out$/M` column). Do NOT substitute a blended or input-based metric.

## Cost Gating

**Source of truth**: `workflow-tools/session/crates/model-prices/model_prices.json`
- Keys: `provider_id`/`model_id`
- Fields: `input_mtok`, `output_mtok`, `cache_read_mtok`, `cache_write_mtok`, `context_window`, `deprecated`
- Never hardcode prices

**Tooling**:
- Query/regenerate the table: `workflow-tools/session/crates/model-prices/sync_model_prices.py` with `--query <model>`, `--list`, `--format {table,csv,json}`, `--check`, `--force`
- Enforcement middleware: the Rust crate `workflow-tools/session/crates/mcp-toolmon`. There is no `cost_gate.py` — earlier revisions referenced one that never shipped. See [model-prices.instructions.md](model-prices.instructions.md) for its flags and failure modes.

**MCP boundary enforcement**: `mcp-toolmon` middleware injects a mandatory `caller_model` field into every MCP tool schema, then grades each call: `base_budget = round((1 − output_mtok / 60) × 100)` versus an empirical per-tool cost. A pricier model keeps full access to cheap tools and is asked to delegate only the token-heavy ones; an unmeasured tool costs 0 and is always allowed, and a grant offset can raise any model's budget. No model is denied outright. Fails open if the price table is unavailable, and intercepts MCP `tools/call` traffic only — it never sees `runSubagent`, so it does not police dispatch targets.

**Setting `caller_model` correctly**: Pass the **actual id of the model issuing the call** — the running model's real price-table `model_id`, e.g. `claude-opus-5`, `claude-sonnet-5`, `claude-haiku-4-5`, `gpt-5.3-codex`, `gpt-5.6-terra`. Match a `model_id` key in `workflow-tools/session/crates/model-prices/model_prices.json` (query with `sync_model_prices.py --list`).
- Do **not** pass a generic vendor or product label such as `github-copilot`, `copilot`, `openai`, or `anthropic`. An unrecognized `caller_model` resolves to a **zero-cost budget**, which silently breaks price-awareness enforcement (the gate can no longer distinguish orchestrator-tier from cheap models).
- When delegating, the sub-agent sets `caller_model` to **its own** model id, not the orchestrator's.

**Model selection is not this file's job.** The canonical tier ladder, preference ordering, dominated-model notes, prices, and context windows live in [model-routing.instructions.md](model-routing.instructions.md). Read it before pinning any model. Nothing here restates it — a second copy is how the two tables drifted apart.

The only thing threshold X decides is **who orchestrates**: a model whose `output_mtok` strictly exceeds 15 runs as orchestrator; everything at or below executes directly. Do **not** reuse "at or below X" as dispatch eligibility — it is not a selection rule, and reading it as one makes any same-priced model look defensible.

Never hardcode prices into tooling; re-resolve with `sync_model_prices.py --query <model>` when a routing decision depends on the exact price. See [model-prices.instructions.md](model-prices.instructions.md) for working with the table.

**Roster check**: the price table is a vendor catalogue and lists models `runSubagent` will refuse. Pin only models present in the surface's model list — a rejected dispatch errors outright and wastes the spawn. See "Roster is not the catalogue" in [model-routing.instructions.md](model-routing.instructions.md).

## What Stays vs What to Delegate

**Keep on expensive model**:
- Strategic decisions and tradeoffs
- Decomposing task into small delegable units
- Planning sub-agent dispatch (which agent, which model, what order)
- Aggregating/reconciling/quality-checking results
- Deciding when done or when to escalate

**Delegate (never do directly when orchestrating)**:
- Reading/editing files
- Searching workspace or web
- Running commands/tests/builds/tool-call batches
- Summarizing large tool outputs or many artifacts

## Delegation Contract

Each sub-agent dispatch MUST include:

1. **An explicit model string chosen from the tier ladder** in [model-routing.instructions.md](model-routing.instructions.md), which owns the T2 default, the bulk/mechanical drop to T3, context-window fit, and the budget-pressure step-down — do not re-derive a model from the price table or restate its default/step-down rules here.
   - Never delegate to another orchestrator-tier model (T0) — that constraint is not covered by the ladder file and lives only here.
2. **Single well-scoped objective** — one unit per sub-agent, never the whole task
3. **Compact return contract** — ask for exactly the facts/edits/results needed (file paths, line ranges, diff summary, decision, short findings list), not a transcript
   - Suggested shape: `scope | finding | outcome | blocker | pointer`
4. **Compiled prompt** — compile prior context, objective, plan, and boundaries into a compact self-contained brief; name artifacts by resolved path/id for the sub-agent to fetch itself rather than pasting their full content. See [shared-context-bundle.instructions.md](shared-context-bundle.instructions.md).
5. **Workspace agent template only** — dispatch to a workspace `.agents/agents/*.agent.md` template (e.g. Research Agent, Implement Agent, Explore Agent); never dispatch to a VS Code built-in agent (e.g. the built-in Explore), which is not integrated with our MCP toolset. For read-only probes use the workspace **Explore Agent** template.

## Context Isolation

**The single most important delegation rule**: A sub-agent inherits NONE of the current session's context. No conversation history, no prior findings, no shared "we". Context-dependent prompts do not fail loudly — they burn a full agent spawn to reply "I have no prior context."

The pre-dispatch self-containment checklist lives in [model-routing.instructions.md](model-routing.instructions.md). Two additions specific to orchestration:
- Compile prior context, objective/validation, steps, and non-goals into the prompt; name artifacts by path/id rather than pasting their full content.
- Include the target agent's contract excerpt inline; do not make the sub-agent read its own template.

## Pre-Dispatch Quality Gates

The pre-dispatch gate is an on-demand dry-run, not a step run before every delegation. Reach for it (dispatching the Explore Agent template, `.agents/agents/explore.agent.md`, `"GPT-5 mini (copilot)"`) when a compiled prompt is complex or risky enough that a mid-tier sub-agent is likely to hit a blocker; skip it for routine, well-scoped units. Each delegation class (Implement, Review, Testing, Commit, Research/Explore) has its own gate set. See [pre-dispatch-gates.instructions.md](pre-dispatch-gates.instructions.md) for complete definitions and trigger signals.

Gate failures (`pass: false`) mean the delegation is NOT dispatched as-is: fix the precondition or escalate BEFORE dispatch, never re-dispatch a blocked unit and hope it works.

## Required Workflow

5-step orchestration loop:

1. **Plan** — ordered delegable units + done-criteria + dependencies
2. **Dispatch** — sequential when dependent, batch when independent
3. **Aggregate** — collect compact results, reconcile conflicts, fill gaps with follow-up units, keep running synthesis; compile each result into the "available prior context" of subsequent delegation prompts in the same chain
4. **Verify** — confirm acceptance criteria; delegate validation and read the verdict
5. **Report or escalate** — escalate to user only on genuine ambiguity/conflicting evidence after focused delegation

## When NOT to Delegate

See "When NOT to Delegate (The Floor)" in [model-routing.instructions.md](model-routing.instructions.md). Short version: spawn overhead is real, so delegate only bulky, numerous, or context-heavy units. **Bulk, not trivial.**

## Case → Capability-Role → Cost-Class Mapping

Define work allocation using **capability roles** rather than raw prices, where each role maps to a cost-class band in the price table.

### Three Capability-Role Bands

| Role | Purpose | Cost Class | Allocation Strategy |
|------|---------|------------|---------------------|
| **Reasoner** | One-time context capture, strategic decisions, high-level reasoning, cross-cutting tradeoffs | T0 (orchestrator tier) | Use sparingly — only for planning and decomposition |
| **Sequencer** | Decompose and sequence implementation steps, moderate-complexity editing, cross-file coordination | T1-T2 | Default delegation target (T2 = Claude Sonnet 5) |
| **Executor** | Step execution, mechanical edits, read-only triage, interaction with external systems, unforeseen error recovery, large-data handling | T3 | Use liberally for bulk/routine work |

**Allocation principle**: Use **Executors** (small models) as much as possible; reserve **Reasoners** for one-time strategic work; use **Sequencers** (mid-tier) as the default implementation workhorse.
### Work Case Classification

Each case maps to a capability role based on **driving signals**: scope breadth, data volume, error-recovery need, and reasoning depth. Resolve the resulting tier to an actual model string through the ladder in [model-routing.instructions.md](model-routing.instructions.md) — no model names appear below, by design.

| Work Case | Role | Cost Class | Driving Signal |
|-----------|------|------------|----------------|
| **Strategic planning / decomposition** | Reasoner | T0 | Cross-cutting scope, requires full context once |
| **Initial task breakdown** | Reasoner | T0 | Must understand entire problem space |
| **Quality synthesis / conflict resolution** | Reasoner | T0 | Reconciling contradictory findings |
| **Multi-file feature implementation** | Sequencer | T2 (default) | Moderate scope, needs code reasoning |
| **Bug fix with diagnosis** | Sequencer | T2 | Requires tracing causal chain |
| **Cross-crate refactor** | Sequencer | T1 | Large scope, high risk |
| **Test authoring** | Sequencer | T2 | Needs understanding of behavior |
| **Targeted single-file edit** | Executor | T3 | Narrow scope, clear target |
| **Read-only search / grep** | Executor | T3 | Mechanical extraction |
| **Docs generation from template** | Executor | T3 | Formula-driven, low judgement |
| **Run validation command** | Executor | T3 | Execute + summarize result |
| **Bulk file reads for triage** | Executor | T3 | High data volume, low reasoning |
| **Error log summarization** | Executor | T3 | Extract failure pattern |
| **Retry after failed validation** | Executor | T3 | Unforeseen event recovery |

**Cost-class boundaries**: tier membership is defined by the ladder in [model-routing.instructions.md](model-routing.instructions.md), not by a price range computed here. This file maps **work cases to roles**; that file maps **roles to models**.

**When a case is ambiguous**: start one tier lower than your intuition suggests. If the result is insufficient, step up **exactly one band** (T3→T2→T1→T0), never skipping, and record the reason in your planning notes.

### Driving Signals Detail

| Signal | Reasoner (T0) | Sequencer (T1-T2) | Executor (T3) |
|--------|---------------|-------------------|------------------|
| **Scope breadth** | Cross-cutting, entire subsystem | Multi-file feature slice | Single file or bounded target |
| **Data volume** | Must digest full context once | Moderate (5-15 files) | High volume but mechanical, or tiny focused scope |
| **Reasoning depth** | Strategic tradeoffs, architectural decisions | Causal chains, behavior inference | Pattern matching, formula application |
| **Error-recovery need** | Reconcile conflicting evidence | Diagnose root cause | Retry with fixed input, summarize logs |
| **Risk tolerance** | High-risk cross-cutting change | Moderate risk with focused tests | Low-risk mechanical change or read-only |

These signals classify **work**, which is the unique contribution of this file. They deliberately name no prices and no models: resolve the role to a model through the ladder in [model-routing.instructions.md](model-routing.instructions.md), which is the only place tier membership is maintained.

## Verify Sub-Agent Output

See "Verify Subagent Output Before Acting" in [model-routing.instructions.md](model-routing.instructions.md). Short version: every sub-agent summary is an UNVERIFIED claim; spot-check load-bearing findings against ground truth before they drive an edit or decision.

## Parallel Fan-Out

See "Parallel Fan-Out" in [model-routing.instructions.md](model-routing.instructions.md) for when and how to fan out. One orchestration-specific rule it does not cover:

**Template**: route every fan-out probe through the workspace **Explore Agent** template (`.agents/agents/explore.agent.md`), never the VS Code built-in Explore, so probes keep MCP access.

## Failure Path

Classify every failed delegation before retrying:

- **Orchestrator fault**: the prompt contains an unverified command, omits its required verification marker, or assigns work outside the target agent's capability mode. Correct the dispatch and re-run the pre-dispatch gate.
- **Sub-agent fault**: the response violates [subagent-return-contract.instructions.md](subagent-return-contract.instructions.md), fabricates a successful result, or declares an item impossible without reporting an attempted alternative probe and its output. Correct the target contract before retrying.
- **Tooling fault**: the same command or tool fails after a bounded reproduction with captured output. Record the command and output; do not restate the ticket CLI broken-pipe defect, which is owned by ticket `2e07430b`.

Retries of mechanical units MUST retain the same or a cheaper tier. Escalate exactly one tier only for demonstrated quality insufficiency, and record the reason. See "Failure Path" in [model-routing.instructions.md](model-routing.instructions.md) for the retry limit.
