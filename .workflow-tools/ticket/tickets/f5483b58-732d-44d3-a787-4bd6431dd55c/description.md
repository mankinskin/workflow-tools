See transcripts/01-09-2026_move-performance-benchmarking/04-run-benchmarks-and-report.md for full detail.

## Outcome
With the ticket/spec/session/rule/audit move_health benches landed, run every domain's bench (cargo bench -p ticket-api|spec-api|session-api|rule-api|audit-api --bench move_health) and compile a single comparison report (benchmark-report.md in the dossier folder) with: a table per domain (entity count x link presence x link density x phase with real numbers), a cross-domain comparison of which domain is slowest per scenario and whether cost scales linearly or is dominated by fixed per-batch overhead, and an explicit call-out of which phase (preflight scan, snapshot+checksum, apply, verify, rollback) dominates cost in worst-case scenarios.

THIS IS A REQUIRED REVIEW CHECKPOINT: the kernel-redesign ticket must not start until this report has been reviewed and its bottleneck finding confirmed.

## Non-goals
Does not redesign anything -- report only.

## Acceptance Criteria
- benchmark-report.md exists, cites real Criterion output (not estimated numbers), and names the specific phase(s) that dominate cost across the five domains.
