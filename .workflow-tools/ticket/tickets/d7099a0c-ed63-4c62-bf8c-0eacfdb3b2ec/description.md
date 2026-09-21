See transcripts/01-09-2026_move-performance-benchmarking/01-ticket-benchmark-scenarios.md for full detail.

## Outcome
workflow-tools/ticket/crates/ticket-api/benches/move_health.rs already has bench_move_preflight_reference_heavy, bench_move_execute_reference_heavy, bench_move_rollback_reference_heavy but none vary entity count or link topology. Extend this file (do not replace it) with a parameterized scenario matrix: entity count (1, 25, 100, 500) x link presence (none, inside-batch, crossing-batch-boundary) x link density (0,1,5,20) x phase (preflight, apply, rollback). Use Criterion BenchmarkGroup with parameterized inputs. Fixture sizes should mirror the real corpus shape from the Sep 2026 migration (~1569 tickets) rather than arbitrary numbers.

## Non-goals
Does not change move_kernel.rs or any production move behavior. Does not benchmark other domains.

## Acceptance Criteria
- cargo bench -p ticket-api --bench move_health runs to completion and produces Criterion output for every new parameterized scenario.
- No existing bench function in the file is removed or broken.
