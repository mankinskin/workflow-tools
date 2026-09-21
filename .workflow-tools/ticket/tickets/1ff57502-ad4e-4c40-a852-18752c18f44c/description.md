## Objective

Track the Ticket-API-side inverse query ("which sessions worked on me", queried from the ticket rather than the session), explicitly deferred out of the initial context-enrichment workflow scope per the 04-08-2026 interview decision.

## Background

The session-side query (`sessions_for_ticket`) is covered by a separate ticket. This ticket exists only to record that the Ticket API inverse direction was considered and explicitly deferred, not dropped.

## Acceptance Criteria

1. This ticket remains in `backlog` until the session-side `sessions_for_ticket` capability has shipped and been dogfooded at least once.
2. When picked up, the implementation adds a Ticket API surface (ticket-api + ticket-cli + ticket-mcp) that, given a ticket id, returns the sessions related to it by delegating to (or mirroring) the session-api `sessions_for_ticket` relation-strength tiers (`strict`/`linked`/`mentioned`), rather than re-deriving its own relation logic.
3. No implementation work occurs against this ticket until it is explicitly re-prioritized out of backlog.