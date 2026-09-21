# Log schema-field search: extend query_logs with structured field filtering

## Problem

`mcp_log-viewer-mc_query_logs` accepts a raw JQ expression, which is powerful but requires users to know the exact field path. There is no dedicated "find all log entries where field X = Y" API that works without JQ knowledge. The HTTP endpoint `GET /api/query/:name?jq=...` has the same limitation.

## Scope

Add a `search_fields` MCP tool and a corresponding HTTP endpoint that accepts structured key/value filters and translates them to JQ internally:

### MCP Tool: `search_fields`

Input:
```json
{
  "filename": "ticket-viewer.log",
  "fields": { "level": "ERROR", "target": "ticket_http::serve" },
  "limit": 100
}
```

Returns matching `LogEntry` objects, same shape as `query_logs`.

### HTTP Endpoint

```
GET /api/logs/:name/fields?level=ERROR&target=ticket_http::serve&limit=100
```

### Implementation

- Translate `fields` map to a compound JQ expression: `select(.level == "ERROR" and (.target // "") | startswith("ticket_http::serve"))`.
- Reuse existing `query::run_jq_query()` internally.
- Expose field name suggestions via a new `GET /api/logs/:name/schema` endpoint that returns the distinct field keys seen in a log file (scan up to first 500 entries).

### MCP Tool: `list_log_fields`

```json
{ "filename": "ticket-viewer.log" }
```
Returns `{ "fields": ["level", "target", "message", "span", ...] }` — the distinct top-level field keys present in that log file.

## Acceptance Criteria

- `search_fields` with `{ "level": "ERROR" }` returns all ERROR entries.
- `list_log_fields` lists all keys seen in the first 500 entries.
- HTTP endpoint returns the same results as the MCP tool.
- Existing `query_logs` tool is unchanged.

## Files

- `tools/viewer/log-viewer/src/mcp_server.rs` — add `search_fields`, `list_log_fields`
- `tools/viewer/log-viewer/src/handlers.rs` — add HTTP endpoints
- `tools/viewer/log-viewer/src/router.rs` — register new routes
- `tools/viewer/log-viewer/src/query.rs` — add field-filter-to-JQ translation helper

## Depends on

- [LOG-1a/1b] (log files must exist to search)
