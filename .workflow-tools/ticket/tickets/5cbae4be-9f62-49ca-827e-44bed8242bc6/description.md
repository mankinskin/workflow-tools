## Capture a real post-9d527ad1 delegation session and replay it through the 10d21210 harness for epic AC4/AC5/AC6 evidence

## Problem

Epic [79c4ac3e](.ticket/tickets/79c4ac3e-fd53-48bf-babb-43d27555c4bd/ticket.toml)'s
ACs 1, 2, 3 are satisfied and all 11 dependency children are `done`. ACs 4,
5, and 6 are the only remaining gaps, and **all three are blocked on the
same missing artifact**: a delegation-orchestration session captured
*after* the sibling fixes (`fb14754e`, `66acb737`, `46d8b25d`, `77eb143b`,
`cc3324c9`, `9d527ad1`) landed, replayed through the existing `10d21210`
benchmark harness
(`memory-api/crates/session-api/tests/delegation_cost_benchmark.rs` +
`compute_delegation_cost_report_from_events`). The two checked-in baseline
sessions (`.benchmark/10d21210/baseline/sessions/{3e9bc20b,41966513}/events.json`)
predate `9d527ad1`'s token-load telemetry capture, so
`data_json.usage`/`input_tokens`/`output_tokens`/`cost_usd` are honestly
all-zero in that baseline — a before/after comparison against it would be
real-telemetry-vs-zero for AC6, and it cannot show a "drop" for AC4/AC5
either since it has nothing to diff against.

The `10d21210` README's "Thresholds table (AC5)" already documents that
**one** post-change replay through `compute_delegation_cost_report_from_events`
yields `subagent turn count`, `substitutable_shell_count`, and
`redispatch_count` simultaneously — the analyzer computes all metrics from
a single event log in one pass. This ticket is therefore the epic's single
remaining measurement/evidence ticket, not an AC6-only ticket.

**This ticket is the SOLE remaining blocker for closing epic `79c4ac3e`.**
Once its evidence is recorded against epic ACs 4, 5, and 6, the epic has no
further open acceptance criteria.

## Objective

Capture (at minimum) one delegation session recorded **after** all sibling
fixes and `9d527ad1` landed, with genuinely non-zero token-load telemetry,
and replay it through the existing `10d21210` harness to produce the real
`delegation_cost_report.json` evidence epic ACs 4, 5, and 6 need. Because a
valid cost comparison (AC6) additionally requires a non-zero *baseline*
side, also capture a post-9d527ad1 **pre-change** (or otherwise valid
comparison-anchor) session under the same scenario/task shape, replayed
through the same harness, so both sides of the AC6 cost comparison carry
real telemetry.

## Acceptance Criteria

1. **AC1 — Real non-zero telemetry captured**: a captured post-change
   session exists where `data_json.usage` is present and `input_tokens`,
   `output_tokens`, and `cost_usd` are non-zero, independently verified (not
   just asserted) by inspecting the captured session's raw event/turn data.
2. **AC2 — Session replayed through the existing harness**: the captured
   post-change session is replayed through
   `memory-api/crates/session-api/tests/delegation_cost_benchmark.rs` and
   `compute_delegation_cost_report_from_events`, producing a
   `delegation_cost_report.json` with real (non-zero, non-synthetic) token,
   cost, turn, and shell-command figures. No new analyzer code path — reuse
   the harness as-is.
3. **AC3 — Valid before/after cost comparison constructed**: a second,
   comparison-anchor session with real (non-zero) telemetry is captured or
   otherwise obtained — e.g. a post-9d527ad1 *pre-sibling-fix* baseline
   captured under the same task/scenario as the post-change session, or an
   equivalent already-captured session that postdates `9d527ad1` — and
   replayed through the same harness, so the before/after cost comparison
   compares two sides that both carry real telemetry. The comparison does
   **not** use the all-zero `.benchmark/10d21210/baseline` sessions as
   either side. State explicitly, in this ticket's status summary and/or a
   checked-in README, how the comparison anchor was constructed.
4. **AC4 — Epic AC4 evidence (turns + substitutable-shell, same replay)**:
   from the post-change replay in AC2, record:
   - Sub-agent turn count (Σ `tool_call_count`, combined across the
     replayed session(s)) against the `10d21210` baseline of **713**,
     threshold **>=25% drop, i.e. <=535**.
   - `substitutable_shell_count` against the `10d21210` baseline of
     **105/334**, threshold **>=50% relative drop, i.e. <=53** substitutable
     calls at comparable total `run_in_terminal` volume.
   Both numbers must come from the same post-change replay (per the epic
   AC4 requirement that both drop "on the same replay").
5. **AC5 — Epic AC5 evidence (redispatch, same replay)**: from the same
   post-change replay, record `redispatch_count` against the `10d21210`
   baseline of **10**, threshold **0**.
6. **AC6 — Epic AC6 evidence (real telemetry + valid cost comparison)**:
   record non-zero real telemetry (`input_tokens`/`output_tokens`/`cost_usd`)
   from the post-change replay (AC1/AC2), and the valid before/after cost
   comparison constructed per AC3 — explicitly **not** a comparison against
   the all-zero `10d21210` baseline.
7. **AC7 — Recorded as epic evidence**: the resulting report(s) and
   comparisons are recorded as evidence against epic
   [79c4ac3e](.ticket/tickets/79c4ac3e-fd53-48bf-babb-43d27555c4bd/ticket.toml)
   ACs 4, 5, and 6, linked from this ticket and from the epic.

## Non-Goals

- Changing the analyzer itself (`delegation_cost.rs`,
  `compute_delegation_cost_report_from_events`) — this ticket is a data
  capture and evidence-recording exercise against the existing harness.
- Re-litigating the 10d21210 replay-only baseline, which remains valid as a
  determinism check; it is simply insufficient alone for epic ACs 4, 5, 6.
- Fixing the `substitutable_shell_count` / `classify_shell_command`
  cd-chain-normalization divergence noted in the `77eb143b` re-review —
  tracked separately (see the follow-up ticket linked from `77eb143b` and
  `10d21210`); this ticket measures against the analyzer as it exists today.

## Traceability

- Epic: [79c4ac3e Sub-agent delegation is more expensive than the orchestration it replaces](.ticket/tickets/79c4ac3e-fd53-48bf-babb-43d27555c4bd/ticket.toml) — this ticket exists to satisfy that epic's ACs 4, 5, and 6, and is its sole remaining blocker.
- Depends on: [10d21210 Define a synthetic benchmark session with a checked-in baseline](.ticket/tickets/10d21210-7168-4ed4-8e99-f6fb0e6e08db/ticket.toml) (done) — supplies the harness this ticket replays through, and the baseline numbers (713 turns; 105/334 substitutable-shell; 10 redispatch) this ticket's replay is measured against.
- Depends on: [9d527ad1 Per-tool-call token-load telemetry via mcp-cost-gate](.ticket/tickets/9d527ad1-616b-45fb-b67c-64e0396841fe/ticket.toml) (done) — the capture must postdate this ticket's telemetry fix.
- Related: [b7c61f0e Promote the sub-agent cost analyzer into session-api with real token attribution](.ticket/tickets/b7c61f0e-ed42-4eef-8d3b-da934d7c0628/ticket.toml) (done) — owns the analyzer surfaces this ticket replays data through.
- Related spec: [7be68a48 Quality gates and session data collection for delegated sessions](.spec/specs/7be68a48-f4e5-49a2-b9a5-118f07b48b90/spec.toml) — records this gap in its "Known Gap — Epic AC6 Real-Telemetry Baseline" section.
