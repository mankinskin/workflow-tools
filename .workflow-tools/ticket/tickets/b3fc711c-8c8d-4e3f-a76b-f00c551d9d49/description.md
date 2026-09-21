# Log full-text search: add search_text MCP tool and HTTP endpoint

## Problem

`mcp_log-viewer-mc_search_all_logs` accepts a JQ expression, not a plain text or regex query. There is no simple "grep for a string across log files" interface for users who need a quick `contains("panic")` or `/re_pattern/` search without writing JQ.

## Scope

### MCP Tool: `search_text`

```json
{
  "pattern": "panic",        // plain text or /regex/ syntax
  "filename": null,          // null = all log files
  "level": null,             // optional pre-filter by level
  "limit": 50,
  "context_lines": 0         // lines before/after each match to include
}
```

Returns: `[{ "file": "...", "line": 42, "entry": LogEntry, "context": [...] }]`

### HTTP Endpoint

```
GET /api/search?pattern=panic&level=ERROR&limit=50&context=2
GET /api/logs/:name/search?pattern=panic
```

### Implementation

- Plain-text pattern: case-insensitive substring match on `entry.message` + all `fields.*` values serialised to string.
- `/regex/` prefix: compile as Rust regex (use the `regex` crate already in the workspace).
- Context lines: return up to N `LogEntry` items before and after each match.
- Apply to a single file or scan all files in the log dir.
- Reuse `log_parser::parse_log_file()` for parsing.

## Acceptance Criteria

- `search_text { "pattern": "Listening on" }` returns the server startup entry from `ticket-viewer.log`.
- `search_text { "pattern": "/5[0-9]{2}/" }` finds all 5xx HTTP error log lines.
- `context_lines: 2` returns 2 entries before and after each match.
- `filename: null` searches all files and groups results by file.
- HTTP endpoint returns identical results.

## Files

- `tools/viewer/log-viewer/src/mcp_server.rs` — add `search_text`
- `tools/viewer/log-viewer/src/handlers.rs` — add HTTP endpoint
- `tools/viewer/log-viewer/src/router.rs` — register route
- `tools/viewer/log-viewer/src/query.rs` or new `search.rs` — text/regex search logic
- `tools/viewer/log-viewer/Cargo.toml` — add `regex` dep if not present

## Depends on

- [LOG-1a/1b] (log files must exist to search)
