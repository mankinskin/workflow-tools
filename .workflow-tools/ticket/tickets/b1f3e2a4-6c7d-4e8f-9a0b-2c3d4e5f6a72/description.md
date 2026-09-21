# [bootstrap][T2] Enforce Assignment Start Context — Branch and CWD Checks

## Context

When a worker is dispatched to implement a ticket, the assignment packet includes an explicit branch name and working directory. Before the worker can claim the ticket, the executor must verify the worker is operating in the correct context. Mismatches are recoverable errors — the worker must correct the context before proceeding, not silently operate on the wrong branch.

This ticket covers the branch/cwd enforcement contract and the integration test that verifies it.

## Branch and CWD Policy

Per the Phase 1.5 design:

- **Working directory**: repository root of the current workspace checkout
- **Branch source**: `execution.branch.feature` from the assignment packet
- **Merge target**: `execution.branch.merge_target` from the assignment packet

Rules enforced by the executor:
1. Worker must report current branch + cwd at session start (before claim)
2. If the feature branch does not exist, worker creates it from merge target HEAD
3. Worker must not switch to unrelated branches during the assignment
4. Any mismatch is a recoverable `execution.branch_mismatch` or `execution.cwd_mismatch` error

## Scope

### What is being built / tested
- Assignment packet parsing: branch + cwd fields read and validated
- Context report: worker session start protocol includes branch + cwd
- Mismatch detection and structured error emission
- Recovery path: worker corrects context and retries — same assignment_id

### Not in scope
- Git worktree provisioning (owned by `51471c3e` sandbox manager)
- Auth token mechanics (covered by T1)
- Ticket state transitions (covered by T3)

## Error Classes

```
execution.branch_mismatch  # reported branch ≠ assignment branch
execution.cwd_mismatch     # reported cwd ≠ expected repo root
```

Both are **recoverable**: executor returns the error with structured metadata; coordinator can instruct the worker to correct and retry without creating a new assignment_id.

## Acceptance Criteria

- [ ] Assignment packet includes `execution.branch.feature` and `execution.branch.merge_target` fields
- [ ] Worker session start sends current branch + cwd as part of the handshake
- [ ] Executor accepts the handshake when branch and cwd match the assignment packet
- [ ] Executor returns `execution.branch_mismatch` (structured) when branch does not match
- [ ] Executor returns `execution.cwd_mismatch` (structured) when cwd does not match
- [ ] Worker can correct context and retry with the same assignment_id — no new assignment required
- [ ] Mismatch event includes: assignment_id, expected values, and reported values

## Dependencies

- Depends on: T1 (`a8d6c1d2`) — executor must be running and auth must work
- Blocks: T3 (claim is gated behind correct context)