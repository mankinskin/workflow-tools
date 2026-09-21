# Origin

Review finding on fdf53556 (FUP-ERR). The reviewer rejected the strict single-hop default: FUP-ERR made `update --to-state` block skipped-waypoint transitions (e.g. `new -> in-implementation`) with a recovery message instead of auto-walking.

**Desired behavior (reviewer verdict):** auto-hops ARE desired. When a target state is reachable via a legal path in the transition graph, `update --to-state` should traverse that path automatically (the pre-existing auto-walk). Strict block+explain should be **opt-out via a command option** (e.g. `--single-hop`/`--strict`), not the default.

# Scope

1. Restore auto-walk as the default for `update --to-state`: when `find_path(from, to)` returns a reachable multi-hop path, traverse it (visiting required intermediate states) instead of rejecting.
2. Add an opt-out flag (CLI + ticket-MCP + HTTP parity) that re-enables the strict single-hop block+explain behavior on demand.
3. Keep the rich `InvalidTransition` recovery error for genuinely unreachable targets and for the strict/opt-out path.
4. Restore/adjust the two regression tests that were rewritten for the block+explain default so they assert auto-walk-by-default + strict-on-flag.
5. Keep `close` fast-forward, the `ticket transitions` inspection command, and the recovery-error shape (those were accepted).
6. **Coupled doc fix (folded from FUP-CAT review):** update the capability catalog `mutate` workflow note in crates/ticket-api/src/contracts/capability_catalog.rs — it currently states `update --to-state` "performs one legal hop and rejects skipped-waypoint transitions". Reword to describe auto-walk-by-default with the strict opt-out flag once this change lands.

# Acceptance Criteria

- `update --to-state` auto-walks a reachable multi-hop path by default, visiting required intermediate states.
- A documented opt-out flag makes the transition strict (single hop) and emits the recovery error on a skipped waypoint.
- The rich recovery error still fires for unreachable targets and under the strict flag.
- Tests assert both default auto-walk and strict-on-flag; docs updated.
- The capability catalog `mutate` note reflects the auto-walk-by-default + opt-out behavior.

# Kept from FUP-ERR (do not regress)

- `SchemaValidationError::InvalidTransition` shape + Display, `allowed_next_states`/`invalid_transition_error`, `ticket transitions` command, docs in ticket-system.instructions.md.