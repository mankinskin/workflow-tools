---
description: "Use when planning, approving, executing, recovering, or reviewing journaled ticket, specification, session, or feedback entity moves."
applyTo: "**"
---

# Entity Move Execution

Use the owning domain's journal lifecycle for every production move:
preflight, apply, source/target read-back, then matching resume or rollback for
any non-terminal journal. Filesystem copy, rename, or deletion is never a
recovery mechanism.

## Plan Before Approval

Generate batches from destination-local connected-component partitions. Keep a
partition intact; pack compatible partitions only when they share one domain
and one resolved destination store. A batch command must name every selected
UUID explicitly and use the domain's set-move surface.

Before approval, run one representative dry-run command for each domain and
batch shape the generated plan emits. Confirm that the domain CLI or MCP
surface accepts a multi-ID selection, returns a set plan, and exposes matching
set recovery operations. A generated command that fails argument parsing is a
planning defect, not a production preflight failure.

For every generated destination, inspect preflight output before apply:

- `target_workspace_root` must equal the requested destination workspace.
- `target_store_root` and every destination entity path must remain under the
  requested destination workspace.
- The target store must be the intended domain store, not an ancestor or a
  sibling store selected through compatibility discovery.

A target-store mismatch blocks the batch. Roll back any already-applied journal
through the owning domain operation, repair destination resolution, regenerate
the plan, and obtain approval for the changed batch list.

## Execute And Observe

Run one batch at a time. Record the batch ID, full command, entity count,
preflight verdict, set-journal ID, terminal phase, and per-entity journal IDs.

Estimate the expected duration from a representative move-health result and
batch entity count before starting a set operation. Treat a quiet command as
`stop-and-observe`, not as a failed move. Inspect the durable set journal and
the active process before cancellation. The durable journal's
`completed_entity_ids`, `phase`, `failure`, and `next_recovery_step` determine
whether to wait, resume, roll back, or escalate.

After a validated apply, read back every selected ID: the source entity must be
absent and the resolved destination entity must be present. The batch is not
complete from a green command or a set-journal phase alone.

## Recovery And Replanning

On the first unexpected blocker, stop the sequence. Record the exact batch,
journal ID, entity IDs, planned target store, observed target store, terminal
phase, and read-back result. Resolve any non-terminal journal through that
journal's `resume` or `rollback` operation before replanning or continuing.

When a plan is regenerated after partial execution, classify records as:

- `candidate`: still in the declared source store and eligible for a new batch;
- `completed`: present at the declared destination with a validated journal;
- `execution-anomaly`: present at a different destination, or otherwise
  inconsistent with the declared destination.

Exclude `completed` and `execution-anomaly` records from new execute commands.
An anomaly requires explicit reconciliation and a fresh approval; never allow a
planner's source-store scan to silently reclassify a completed or misrouted
record.

## Retrospective Evidence

For an entity-move learning pass, read move journals before raw agent
transcripts. Preserve compact findings as:

```text
scope | finding | outcome | blocker | pointer
```

Feedback provenance carries the session ID, while `feedback_ingest.source` is
the source class such as `agent`, `user`, or `system`; a session URN is not a
valid feedback source value.
