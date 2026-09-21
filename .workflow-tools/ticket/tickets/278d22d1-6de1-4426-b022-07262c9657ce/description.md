## Problem

`memory-api/crates/session-api/src/delegation_cost.rs` has two predicates
that both use the same `SUBSTITUTABLE_SHELL_HEADS` list but apply it to
**different inputs**:

- `substitutable_shell_count` (line ~563) classifies by the **raw**
  `command_head(raw_command)` — no cd-chain stripping.
- `classify_shell_command`'s `ReadLikeExploratory` branch (line ~232-235)
  classifies by `command_head(strip_leading_cd_chain(command))` — the
  cd-stripped command.

This is the same shape of bug that caused the original cd-chain
misclassification fixed in `77eb143b` round 2 — except here it was
identified and **deliberately left in place** rather than fixed, because 7
sibling-ticket threshold rows are frozen against the current
`substitutable_shell_count` numbers:

- `fb14754e` AC4, AC5
- `66acb737` AC5
- `46d8b25d` AC5
- `cc3324c9` AC5
- `77eb143b` AC4
- epic `79c4ac3e` AC4

Changing `substitutable_shell_count` to also strip cd-chains would move the
combined baseline off 105/334 and invalidate all of those frozen threshold
rows in `.benchmark/10d21210/README.md` without a coordinated rebase.

## Objective

Resolve the inconsistency by doing ONE of:

1. **Unify**: make `substitutable_shell_count` use the same
   `strip_leading_cd_chain` normalization as `classify_shell_command`'s
   `ReadLikeExploratory` branch, and rebase all 7 threshold rows listed
   above (recompute the baseline over the stripped rule, update
   `.benchmark/10d21210/README.md`'s baseline totals and thresholds table,
   and re-verify each sibling ticket's AC still holds or renegotiate the
   number).
2. **Document**: leave `substitutable_shell_count` on the raw-command rule,
   but add an explicit code comment on both predicates in
   `delegation_cost.rs` stating that the divergence is deliberate (frozen
   sibling thresholds), not an oversight, and cross-reference this ticket
   and the 7 frozen threshold rows so a future reader does not silently
   "fix" it and break the frozen numbers.

## Acceptance Criteria

1. A decision is made and recorded (in this ticket) between option 1
   (unify + rebase) and option 2 (document divergence).
2. If option 1: all 7 frozen threshold rows are rebased consistently and
   `.benchmark/10d21210/README.md` reflects the new baseline; if option 2:
   both predicates in `delegation_cost.rs` carry an explicit comment
   explaining the deliberate divergence and pointing at this ticket.
3. No sibling ticket's threshold is silently invalidated — either it is
   rebased (option 1) or explicitly confirmed unaffected (option 2).

## Traceability

- From: [77eb143b Enforce MCP-over-shell in agent templates](.ticket/tickets/77eb143b-0322-4c91-b3c4-deccc2b2927c/ticket.toml) re-review round 2 — identified the divergence.
- Depends on / references: [10d21210 Define a synthetic benchmark session with a checked-in baseline](.ticket/tickets/10d21210-7168-4ed4-8e99-f6fb0e6e08db/ticket.toml) — owns the frozen threshold table this ticket must not silently break.
