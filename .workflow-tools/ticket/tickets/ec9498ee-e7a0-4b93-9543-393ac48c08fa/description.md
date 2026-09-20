# Objective

Harden workspace linking and search beyond the immediate bug fixes by closing the remaining design holes identified in review.

# Why this exists

The current fixes repaired two concrete failures, but review still found structural gaps:

- search and index reconciliation still have asymmetric freshness rules
- HTTP search can still surface unresolved hits as fabricated fallback rows
- workspace identity is only folder-name-deep and therefore not collision-safe
- filtered explorer state is still vulnerable to live-update drift outside the request-sequencing fix
- the current tests do not lock the duplicate-name, deletion-drift, unresolved-hit, and filtered-live-update cases

# Workstreams in this tracker

## Existing related work

- immediate nested-workspace search and stale-response fix
- query-plus-state filter correctness work
- authoritative ticket-folder-path output work

## New redesign work

- canonical workspace identity and search consistency design
- aggregate scan, prune, and search reconciliation hardening
- authoritative resolved-hit policy for HTTP and query flows
- collision-safe public workspace identity and ticket refs
- filtered explorer synchronization under live refresh
- a cross-layer regression matrix with focused unit, integration, and browser coverage

# Done means

- search, list, get, detail, history, files, and asset flows agree on visibility and ownership
- duplicate workspace names no longer make ticket refs ambiguous
- filtered explorer state remains authoritative under overlapping requests, SSE updates, and workspace switches
- the regression matrix covers deletion drift, duplicate names, unresolved search hits, and filtered live updates

# Validation gate

- No child ticket moves to in-review without adding or expanding tests in its owning layer; wording alone does not satisfy this tracker.
- Each child ticket must record exact focused commands and expected passing evidence in its body and acceptance criteria.
- The tracker does not close until storage, HTTP, and browser-facing work each have passing validations or a documented blocker with the strongest available substitute check.
- If a child intentionally omits one test layer, it must say why that layer is not applicable and what compensating validation replaces it.
