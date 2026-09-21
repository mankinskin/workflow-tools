# Add live-tail view to log-viewer-dioxus using SSE

## Problem

Engineers launching a server with `viewer-ctl start --fg` need to see live log output in the browser rather than switching between the terminal and the viewer. There is no streaming/tail view in either the Preact or the Dioxus log-viewer.

## Scope

### Server side (`tools/viewer/log-viewer/src/`)

Add an SSE endpoint:

```
GET /api/logs/:name/tail
```

Behaviour:
- Resolves the log file path (same logic as `/api/logs/:name`).
- Opens the file and seeks to EOF.
- Polls for new lines at 100 ms intervals (using `tokio::fs` + inotify/`notify` crate on Unix or polling on Windows).
- Each new line is parsed by `log_parser::parse_line()` and sent as `data: <JSON LogEntry>\n\n`.
- Sends `event: heartbeat\n\ndata: {}\n\n` every 5 s to keep connections alive.
- Respects Axum's `axum::response::sse::Event` / `Sse<>` response type (already used in ticket-viewer).

### Client side (`tools/viewer/log-viewer/frontend/dioxus/src/`)

New route `/tail/:name` with component `LiveTailView`:

- Opens `EventSource` to `/api/logs/:name/tail`.
- Maintains a ring buffer of last 1000 entries (signal `tail_entries: Signal<VecDeque<LogEntry>>`).
- Auto-scrolls to bottom on new entries unless the user has scrolled up (pause-on-scroll).
- Level badge colours same as `LogEntryList`.
- "Pause / Resume" button toggling auto-scroll.
- "Clear" button empties the ring buffer.
- Status indicator: "Live" (green dot) / "Reconnecting…" / "Disconnected" (red dot).

### Router Integration

- Add "Live Tail" link to the left sidebar nav.
- Router: `route("/tail/:name", LiveTailView)`.

## Acceptance Criteria

- Starting a server and opening `/tail/ticket-viewer.log` shows new log entries as they are written.
- Auto-scroll pauses when user scrolls up; resumes when scrolled back to bottom or "Resume" clicked.
- Connection loss shows "Reconnecting…" status and reconnects automatically.
- Heartbeat keeps the connection alive for at least 30 s with no log output.
- E2E test: SSE test verifying at least one event received within 2 s.

## Files

- `tools/viewer/log-viewer/src/sse.rs` (new) — SSE handler and file tail logic
- `tools/viewer/log-viewer/src/handlers.rs` — expose SSE route
- `tools/viewer/log-viewer/src/router.rs` — register route
- `tools/viewer/log-viewer/frontend/dioxus/src/components/live_tail.rs` (new)
- `tools/viewer/log-viewer/frontend/dioxus/src/routes.rs` — add `/tail/:name` route
- `tools/viewer/log-viewer/Cargo.toml` — add `notify` dep (polling on Windows)

## Depends on

- [LOG-5b] (Dioxus UI with layout and shared components)
- [LOG-1a] (log files must be written to disk for tail to work)
