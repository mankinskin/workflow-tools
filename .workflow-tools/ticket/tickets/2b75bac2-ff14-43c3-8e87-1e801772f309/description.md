## Problem

`sessions_for_ticket` (added in ticket bba9b313-ff13-4fd1-91d4-6485a6c2f4de) is functionally inert against the real `.session` store:

- Dogfooding the query against ticket `06cfe998` returned **0 matches at all three relation-strength tiers**.
- Store-wide, only **1 of 227** sessions has `metadata.ticket_id` populated (~0.4%); 0 of a sampled 10.
- The tier logic itself has no known defect — the query is correct. The underlying linkage data was simply never captured by historical sessions.

Consequence: the entire context-enrichment workflow built on top of `sessions_for_ticket` is currently inert in practice, because there is almost nothing for it to find.

## Scope — both halves required

This ticket must cover BOTH of the following, and acceptance criteria must verify both:

1. **Forward fix**: ensure future sessions reliably populate `SessionMetadata.ticket_id` and/or `SessionLinks.ticket_ids` at check-in time (`session_check_in` / board check-in flow), so the linkage exists going forward.
2. **Backfill decision**: decide and specify whether historical sessions (the ~226 currently unlinked) get a backfill, and if so, from what signal. Transcript-text scanning is **forbidden** by spec `e5f8a2c1` (no scanning session transcript text to infer ticket linkage). A backfill, if pursued, must derive linkage only from structured data such as:
   - branch names of the form `agent/<short-id>-<slug>` (parse `<short-id>` as a ticket-id prefix)
   - worktree paths (same `<short-id>-<slug>` pattern)
   - handoff packages (`target_tickets` field)

If backfill is decided against, the ticket must record that decision explicitly rather than silently omitting it.

## Acceptance Criteria

- AC1: `session_check_in` (and/or the board check-in path that sets session metadata) reliably writes `SessionMetadata.ticket_id` and/or `SessionLinks.ticket_ids` whenever a ticket id is available at check-in time, verified by a regression test that checks in a session with a ticket id and asserts the field is persisted and readable back via `sessions_for_ticket`.
- AC2: A written decision (in this ticket's description or a linked spec) states whether historical backfill is pursued; if yes, the exact structured-data signal(s) used (branch name pattern / worktree path pattern / handoff `target_tickets`) and the derivation logic; if no, the rationale.
- AC3: If backfill is pursued, an implementation exists that populates linkage from the chosen structured signal(s) only — no transcript-text scanning — and a test asserts that a sample of previously-unlinked sessions with a parseable branch name or handoff package become linked and discoverable via `sessions_for_ticket` after the backfill runs.
- AC4: Re-running the `sessions_for_ticket` dogfood query against ticket `06cfe998` (or an equivalent verifiable ticket) after this ticket's changes returns at least one match, or the ticket explicitly documents why `06cfe998` specifically has no linkable session.

## Evidence

- Dogfood run: 0/3 tiers matched for ticket 06cfe998 (see ticket 33463861-ffba-4ead-905e-5d867b707936).
- Store scan: 1/227 sessions has `metadata.ticket_id` set; 0/10 in a random sample.
- Spec e5f8a2c1 forbids transcript-text scanning for linkage inference.

## Priority

HIGH — the context-enrichment workflow depends entirely on this linkage existing.
