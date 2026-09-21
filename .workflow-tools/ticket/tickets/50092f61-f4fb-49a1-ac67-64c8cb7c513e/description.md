## Problem

This session originally assumed a read-index staleness defect in ticket-api as a premise for spec ticket 54114fc9 (deterministic Rust execution controller). Research could not reproduce a staleness defect in the read-index itself: `memory-api/crates/ticket-api/src/storage/store/scan_helpers.rs::entry_is_current` (L176-214) correctly detects on-disk/index `type` mismatches, and `integrate_entry` overwrites stale entries correctly, with a passing regression suite.

The more likely root cause is workspace/root RESOLUTION picking a different `.ticket` store than the one actually reconciled — not index staleness per se. This matches the symptom already reported in ticket 1d6a033e: `add_edge` fails with entity-not-found on entities that `get_ticket`/`update_ticket` resolve successfully moments earlier against the same explicit workspace path, suggesting `add_edge` resolves entities through a different (stale or narrower-scoped) index/lookup path than other ticket-mcp handlers.

## Goal

Investigate and confirm (or rule out) the actual root cause: workspace/root resolution divergence between ticket-mcp handlers (e.g. `add_edge` vs. `get_ticket`/`update_ticket`), rather than assuming an unproven read-index staleness defect. Reconcile findings with 1d6a033e, which may be the same underlying bug or a duplicate.

## Acceptance criteria
- Root cause is confirmed with reproduction steps (not assumed), covering the divergence between `add_edge` and other ticket-mcp handlers on workspace/root resolution.
- Relationship to 1d6a033e is stated explicitly: same bug, duplicate, or genuinely distinct.
- No downstream spec (including 54114fc9) is left assuming an unconfirmed root cause for this class of failure.

## Source
Split out of ticket 54114fc9 per interview decision (2026-07-31, Q6): "Split a separate investigation ticket for the unreproduced staleness bug (suspected workspace/root resolution, cf. 1d6a033e) rather than letting the spec assume a root cause" — answered yes.