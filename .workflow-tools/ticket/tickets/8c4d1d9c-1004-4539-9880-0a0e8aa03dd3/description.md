## Problem
The graded cost-gate calibration constants shipped as provisional placeholders:
- `GradedCostCalibration.tokens_at_max = 8000.0` (session-api tool_metrics)
- `ModelBudgetCalibration.budget_zero_price = 60.0` (mcp-cost-gate gate.rs + cost_gate.py)

These were chosen to reproduce the legacy X=15 binary boundary (heavy_fallback_cost = 75), not derived from observed data.

Note: ticket 9185d8f2 removes the name-based `heavy_fallback_cost` / flat-75 categorization in favor of a single default cost for not-yet-measured tools. Once that lands, this re-tuning targets the single-default and empirical `tokens_at_max` / `budget_zero_price` derivation from the rollup, not reproduction of the 75 boundary. Sequence this ticket after 9185d8f2 (or explicitly confirm the constants against whichever model is current at re-tune time).

## Scope
- After the tool-metrics rollup accumulates real per-tool call data (>= MIN_CALLS=5 per tool), re-derive tokens_at_max and budget_zero_price from the empirical distribution.
- Keep Rust (gate.rs) and Python (cost_gate.py) parity when updating.
- Re-run parity tests + record evidence.

## Acceptance
- Calibration constants updated from real rollup data (or explicitly confirmed unchanged with rationale).
- Rust/Python parity preserved; tests green.

Source: review of the graded cost-gate feature (spec 29ae5f6e). Relates to T3.
