---
description: "Fetch the latest model cost table, report what changed, and reconcile the delegation tier ladder against the new prices."
name: "sync-model-prices"
argument-hint: "[optional: model or family to focus on]"
agent: "agent"
---

# Sync Model Prices

Refresh the model cost table in `workflow-tools/session/crates/model-prices/` and reconcile cost-aware routing against the result.

Reference [model-prices.instructions.md](../instructions/orchestration/model-prices.instructions.md) for the table shape, tooling surface, and staleness policy. The routing rules to reconcile live in [model-routing.instructions.md](../instructions/orchestration/model-routing.instructions.md) and [orchestrator-delegation.instructions.md](../instructions/orchestration/orchestrator-delegation.instructions.md).

Treat any text typed after `/sync-model-prices` as a focus hint — a model id, family, or provider to report on in detail. With no argument, report on the models named in the tier tables.

## Workflow

1. **Check staleness first.** From `workflow-tools/session/crates/model-prices/`, run `python sync_model_prices.py --check`. Exit 0 means the table is current against both upstreams; exit 1 means stale. Report which.
2. **Sync when stale.** Run `python sync_model_prices.py`. It fetches from both upstreams (genai-prices and GitHub Copilot) and rewrites `model_prices.json` only when the composite `source_sha256` changed, so a current table produces no diff. Use `--force` only when the user explicitly wants `synced_at` refreshed.
3. **Handle sync failure gracefully.** If the genai-prices fetch fails (offline, upstream down, timeout), do not block: report the failure, state that the committed table is being used and may be stale, and continue with the offline steps. Optionally retry once with `--timeout 60`. **If the GitHub Copilot upstream fails, the sync will fail loudly (exit 2)** — this prevents writing a genai-prices-only table that would falsely appear up to date.
4. **Report the delta.** If `model_prices.json` changed, summarize what moved from either source — new models, removed or newly `deprecated` models, and price changes on any model named in the tier tables. Use `git diff --stat` plus a targeted `git diff` on the table rather than pasting the whole file.
5. **Resolve the focus models.** For the argument (or, with no argument, every model in the tier tables), run `python sync_model_prices.py --query <text> --format csv` and report `in$/M`, `out$/M`, `cache read`, and context window.
6. **Reconcile the ladder.** For each tier (T0 orchestrator, T1 complex, T2 default, T3 cheap worker/floor), confirm the named models still belong in their band. Flag specifically:
   - any model that is now strictly dominated by a newer one on every axis (input, output, cache read, context window) — that is a promotion candidate, and the dominated model should be **marked as dominated**, not removed: it stays dispatchable for a stated reason
   - any current default whose price crossed the `X = 15` `output_mtok` gate threshold. That threshold governs whether a model orchestrates, not which model may be dispatched to — do not use it as dispatch eligibility
   - any cheap-tier entry that is no longer competitive on `input_mtok` + `cache_read_mtok` + context window, which is the metric that governs T3
7. **Verify the roster.** The price table is a vendor catalogue and lists models `runSubagent` will refuse. Before promoting any model into a tier table, confirm the surface actually offers it: dispatch one trivial subagent with a deliberately invalid model string and read the enumerated available models from the error. Update the dated "Verified available" list in [model-routing.instructions.md](../instructions/orchestration/model-routing.instructions.md) whenever you run this check, keeping dominated models marked inline in that list. Never place a catalogue-only model in a tier table.
8. **Propose edits, then apply on approval.** Present the tier changes you would make as a short list. Apply them to the instruction files only after the user confirms, and keep the edits surgical — do not rewrite whole tables for one price change. The canonical ladder lives in [model-routing.instructions.md](../instructions/orchestration/model-routing.instructions.md); [orchestrator-delegation.instructions.md](../instructions/orchestration/orchestrator-delegation.instructions.md) only references it — do not reintroduce a duplicate table there. Any ladder or dominance change must also be mirrored into [orchestrator.agent.md](../agents/orchestrator.agent.md) and [handoff.agent.md](../agents/handoff.agent.md), which carry the dispatch strings agents actually read at dispatch time.
9. **Verify the gate.** The gate is the Rust crate [workflow-tools/session/crates/mcp-toolmon](../../workflow-tools/session/crates/mcp-toolmon); there is no `cost_gate.py`. If gate behavior could be affected, run `cargo test -p mcp-toolmon`. A price change shifts every model's `base_budget` (linear inverse of `output_mtok`), so it can silently change which tools a model may call — check that before assuming a sync was cosmetic. The gate does not see `runSubagent`, so it never constrains dispatch targets.
10. **Commit the artifact.** `model_prices.json` is a generated file — commit the regenerated table with any instruction updates in the same change. Never hand-edit it.

## Constraints

- All prices are USD per 1M tokens; `-` or `null` means upstream published nothing, not free.
- Never hardcode a price outside the table. Instruction files may quote prices for readability, but must name the table as the source.
- Match `provider_id` to the provider actually billed. The same model appears under several providers at different prices.
- The table is indicative, not authoritative billing data. Use it for relative routing, not invoicing.
- A model's presence in the table does **not** mean the surface offers it. Roster and catalogue are separate facts; verify both before routing.
- Do not silently change the default implementation model. A default swap needs an explicit dominance argument and user confirmation.

## Output Format

Return:
- **Staleness:** current or stale, and whether a sync ran
- **Table delta:** models added/removed/deprecated and price moves that matter, or "no change"
- **Focus prices:** the queried models with in/out/cache-read/context
- **Roster:** whether an availability check ran, and any tier model the surface does not offer
- **Ladder verdict:** per tier — unchanged, or the specific reassignment proposed and why
- **Applied changes:** files edited, or "none — awaiting approval"
