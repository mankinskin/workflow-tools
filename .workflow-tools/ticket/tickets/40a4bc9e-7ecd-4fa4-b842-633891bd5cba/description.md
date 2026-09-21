# Log-to-Mermaid: convert a filtered log session to a sequence diagram

## Problem

When debugging multi-component interactions (e.g. `ticket-http` handler → `TicketStore` → `tracing` spans), engineers have no automatic way to visualise the call flow as a Mermaid `sequenceDiagram`. Today this requires manually reading span enter/exit entries and building the diagram by hand.

## Scope

### MCP Tool: `log_to_mermaid`

```json
{
  "filename": "ticket-viewer.log",
  "filter": "select(.target | startswith(\"ticket_http\"))",  // optional JQ pre-filter
  "participants": ["ticket_http::serve", "ticket_api::storage"],  // optional explicit list
  "limit": 200
}
```

Returns a string containing a valid Mermaid `sequenceDiagram` block.

### HTTP Endpoint

```
GET /api/logs/:name/mermaid?filter=select(...)&limit=200
```

Returns `Content-Type: text/plain` with the diagram text.

### Conversion Algorithm

1. Parse the log file, optionally apply JQ pre-filter.
2. Extract span_enter / span_exit events; correlate by span name + thread_id.
3. Map `target` (e.g. `ticket_http::serve::handlers::graph`) to a participant label (last two path segments or explicit override).
4. For each span_enter emit `A->>B: span_name(fields)` and for span_exit emit `B-->>A: [elapsed Xms]`.
5. For bare events (not span enter/exit) emit `Note over A: message`.
6. Truncate the diagram at `limit` events with a trailing `Note: ... N more events ...`.

### Format Example

```mermaid
sequenceDiagram
  participant browser
  participant ticket_http::handlers
  participant ticket_api::storage
  browser->>ticket_http::handlers: subgraph(root=4a228c24, depth=4)
  ticket_http::handlers->>ticket_api::storage: bfs_graph
  ticket_api::storage-->>ticket_http::handlers: nodes=90 edges=240 [1ms]
  ticket_http::handlers-->>browser: 200 OK
```

## Acceptance Criteria

- `log_to_mermaid` for a `ticket-viewer.log` session produces a valid Mermaid diagram (passes `mermaid --check` or equivalent).
- Participants are auto-derived from `target` fields when not specified.
- Diagram is truncated (not erroring) when `limit` is exceeded.
- HTTP endpoint returns the same text.

## Files

- `tools/viewer/log-viewer/src/mcp_server.rs` — add `log_to_mermaid`
- `tools/viewer/log-viewer/src/handlers.rs` — add HTTP endpoint
- `tools/viewer/log-viewer/src/router.rs` — register route
- New: `tools/viewer/log-viewer/src/mermaid.rs` — conversion logic

## Depends on

- [LOG-2a] (normalised field names required for reliable participant extraction)
- [LOG-3a] (field search reused for pre-filtering)
