See transcripts/01-09-2026_move-performance-benchmarking/05-kernel-redesign.md for full detail.

## Outcome
GATED: do not start until the benchmark report ticket's report has been reviewed. Using the report's bottleneck finding, redesign workflow-tools/memory-kernel/src/storage/move_kernel.rs (and move_kernel_types.rs if the journal/plan data shape is the bottleneck) to reduce per-batch and per-record overhead. Candidate levers to evaluate against real numbers (do not commit to one without evidence): raising the Mandatory Batch Protocol's <=25-record batch cap if per-batch overhead dominates; reducing snapshot+checksum cost (hash only changed content, or stream instead of copy) given the ~212MB pre-apply snapshot bloat observed in the real migration; batch-level reference resolution instead of per-record if link resolution dominates in reference-heavy scenarios. Only touch a specific domain adapter if the report shows a bottleneck the kernel fix doesn't cover.

## Non-goals
Does not change the Mandatory Batch Protocol's safety guarantees (preflight -> snapshot+checksum -> apply -> verify -> accept-or-rollback stays intact). Does not re-run the real production migration as part of this ticket.

## Acceptance Criteria
- Re-run the same move_health benches after the kernel change and show a measured improvement in the scenario(s) identified as the bottleneck.
- All existing move-related tests (cargo test -p ticket-api, -p spec-api, -p session-api, -p rule-api, -p audit-api, and any memory-kernel tests) still pass.
