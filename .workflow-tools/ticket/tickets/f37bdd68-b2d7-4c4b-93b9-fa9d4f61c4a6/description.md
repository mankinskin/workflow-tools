# Log-to-table: render a filtered log view as ASCII/Markdown table

## Problem

There is no way to get a quick tabular summary of log entries for terminal/CLI use or for pasting into documentation. Engineers must write JQ expressions and post-process JSON manually.

## Scope

### MCP Tool: `log_to_table`

```json
{
  "filename": "ticket-viewer.log",
  "filter": "select(.level == \"ERROR\")",   // optional JQ pre-filter
  "columns": ["timestamp", "level", "target", "message"],  // optional column list
  "format": "markdown",       // "markdown" | "ascii"
  "limit": 50
}
```

Returns a formatted table string.

### HTTP Endpoint

```
GET /api/logs/:name/table?filter=select(...)&columns=timestamp,level,message&format=markdown&limit=50
```

Returns `Content-Type: text/plain`.

### Implementation

- Column values are extracted from `LogEntry` fields (top-level struct fields + `fields` map).
- Columns auto-sized to content (max width capped at 80 chars with `...` truncation).
- `markdown` format: GFM pipe table syntax.
- `ascii` format: box-drawing chars (`─`, `│`, `┼`) for terminal use.
- Default columns when not specified: `["timestamp", "level", "target", "message"]`.

### Output Example (markdown)

```
| timestamp              | level | target                        | message                                |
|------------------------|-------|-------------------------------|----------------------------------------|
| 2026-05-03T14:41:24Z   | INFO  | ticket_viewer                 | Ticket Viewer starting                 |
| 2026-05-03T14:41:28Z   | DEBUG | ticket_http::serve::handlers  | subgraph request received              |
```

## Acceptance Criteria

- `log_to_table` with default columns returns a valid GFM table for a non-empty log file.
- ASCII format renders correctly in a terminal (Unicode box chars).
- Long values are truncated with `…` and do not break column alignment.
- HTTP endpoint returns identical output.
- Columns from nested `fields.*` (e.g. `fields.workspace`) can be specified.

## Files

- `tools/viewer/log-viewer/src/mcp_server.rs` — add `log_to_table`
- `tools/viewer/log-viewer/src/handlers.rs` — add HTTP endpoint
- `tools/viewer/log-viewer/src/router.rs` — register route
- New: `tools/viewer/log-viewer/src/table.rs` — table rendering logic

## Depends on

- [LOG-1a/1b] (log files must exist)
- [LOG-3a] (field filtering reused)
