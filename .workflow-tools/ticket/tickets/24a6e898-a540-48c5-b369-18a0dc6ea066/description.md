## Objective
Execute the approved specification migration through controlled cross-repository batches after the kernel, spec-domain, and test contracts pass.

## Requirements
- Rebuild the source-aware specification inventory and mark newly created safety specs as already at their correct owner stores.
- Use sequential destination-local batches only.
- Run preflight, apply, recovery, source/destination read-back, and reconciliation after every cross-repository batch.
- Commit in fixed order: source repository changes, destination repository changes, then parent/reference updates; record each commit before the next batch.
- Stop on any anomaly and exclude completed or misrouted records from regenerated plans.

## Acceptance Criteria
1. No batch starts without a blocker-free preflight and complete component closure.
2. Every applied batch has terminal journal state, source absence, destination presence, and reconciliation evidence.
3. The final inventory distinguishes moved, already-at-target, central-retain, source-missing, and execution-anomaly records.
4. W8 physical, aggregate, journal, and ledger counts agree.

## Validation
`workflow-tools/target/debug/spec.exe validate-links --workspace workflow-tools --json`
`workflow-tools/target/debug/spec.exe health --workspace workflow-tools --all --json`
