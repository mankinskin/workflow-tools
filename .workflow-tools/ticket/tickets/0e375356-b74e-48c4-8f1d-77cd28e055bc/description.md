# Problem

The workflow discovery surfaces currently build scope locally and inconsistently.

- CLI `ticket next` narrows candidates with a title-prefix filter and optional reverse-dependent root.
- CLI `ticket board show` computes recommendations from the global board snapshot with no reusable scope selector.
- MCP `next_tickets` duplicates its own prefix-filter logic and does not expose root scoping.
- MCP `board_show` only filters by agent.
- HTTP `/api/workflow/next` has a separate query struct that partly overlaps the CLI surface.

Without a shared implementation, any new selector feature will drift again across CLI, MCP, and HTTP and users will keep seeing different answers for the same intended scope.

# Scope

1. Introduce a shared selector model or normalizer in the ticket workflow layer so board / next discovery can compose workspace or root selection, field predicates, text query, and graph reachability constraints without each adapter reimplementing them.
2. Extend CLI `ticket next` and CLI `ticket board show` to accept the approved selector inputs and surface the selected scope in both human and JSON output.
3. Extend MCP `next_tickets` and `board_show` to accept the same selector fields, keeping compatibility aliases only where the spec requires them.
4. Extend HTTP `/api/workflow/next` with the same selector contract and response scope metadata.
5. Reuse the existing ranking and board-awareness logic after selector narrowing instead of introducing a second ranking path.
6. Keep viewer-facing follow-up work out of scope here unless required only for transport parity; this ticket is about the workflow API surface, not frontend UX.

# Acceptance Criteria

- One shared selector implementation narrows candidate tickets for CLI, MCP, and HTTP workflow-next flows.
- CLI `ticket board show` and `ticket next` can scope recommendations to a related ticket set by workspace or root, field filters such as `component`, text query, and graph reachability as defined by the selector spec.
- MCP `board_show` and `next_tickets` accept matching selector fields.
- HTTP `/api/workflow/next` exposes matching selector parameters and selected-scope metadata.
- Compatibility coverage preserves current `--filter` and `root` behavior where the spec says they remain supported.
- Focused CLI, MCP, and HTTP tests prove that scoped queries ignore unrelated tickets outside the selected module, component, workspace, or graph slice.

# Dependencies and Coordination

This implementation should build on the selector specification ticket and align with existing scope and workflow transport work instead of inventing parallel query semantics.

# Likely Surfaces

- `memory-api/crates/ticket-api/`
- `memory-api/tools/cli/ticket-cli/`
- `memory-api/tools/mcp/ticket-mcp/`
- `memory-api/tools/http/ticket-http/`
- `memory-api/tools/cli/ticket-cli/tests/`
- `memory-api/tools/mcp/ticket-mcp/tests/`
- `memory-api/tools/http/ticket-http/src/serve/handlers/workflow.rs`
