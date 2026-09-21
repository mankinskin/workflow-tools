# Objective

Group the unresolved operator-facing workflow and discoverability gaps surfaced while reviewing ticket, spec, and rule tool usage during the May 20-21 session.

# Session Evidence

- The session recorded 14 MCP ticket executions, 145 ticket CLI executions, 30 spec CLI executions, 17 tool_search calls, and 14 tool execution failures.
- Existing backlog already covers part of the finding set:
  - `51671748` - best-next workflow contract
  - `07836f41` - workspace-aware `get/search/list` across nested roots
  - `68a08b34` - scope-aware `board show` and `next` for multi-root workspaces
  - `61cbc31f` - explain why a ticket is absent from `next`
- The remaining uncovered findings cluster around detail-read fragmentation, stale board checkout, transition guidance, spec-store targeting, cross-CLI contract drift, and discoverability/help.

# Scope

Coordinate the missing tickets needed to make ticket/spec/rule workflows understandable without CLI spelunking.

# Child Tickets

- Consolidated ticket detail/context read surface
- Stale board checkout by entry id or inferred owner
- Invalid transition guidance with allowed next states
- Root-aware spec CLI/MCP flows for nested `.spec` stores
- Normalized ticket/spec CLI grammar and JSON envelopes
- Self-describing help and capability catalog surfaces for ticket/spec/rule workflows

# Acceptance Criteria

- The existing multi-root next-workflow tickets and the new child tickets together cover the full session finding set.
- Each child ticket has concrete validation and contract requirements.
- This tracker does not close until the operator-facing gaps in the session findings are either implemented or explicitly folded into another surviving parent ticket.
