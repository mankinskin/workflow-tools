# Port log-viewer browser UI to Dioxus: file tree, entry list, search, stats

## Problem

After the scaffold ([LOG-5a]), the Dioxus log-viewer is a stub. This ticket ports the core browsing UI from the Preact frontend to Dioxus components with feature parity.

## Scope

Port these views from `tools/viewer/log-viewer/frontend/src/`:

| View | Dioxus Component | Notes |
|------|-----------------|-------|
| File tree (left pane) | `LogFileTree` | Uses `viewer-api-dioxus::FileTree`; shows file name + size + modified |
| Entry list (center pane) | `LogEntryList` | Virtualised scroll; each row shows timestamp, level badge, target, message |
| Search/filter bar | `SearchBar` | Connects to `[LOG-3a]` `search_fields` and `[LOG-3b]` `search_text` HTTP endpoints |
| Stats panel | `LogStats` | Reuses `analyze_log` endpoint; shows level histogram, top targets, span summary |
| Source viewer | `SourceViewer` | Clicking a log entry's `file:line` fetches `/api/source/*path` and renders a code snippet with the target line highlighted; reuse `viewer-api-dioxus::CodeViewer` |

### Layout

Three-pane layout identical to the Preact version (reuse `viewer-api-dioxus::Layout`, `Sidebar`, `ResizeHandle`):
```
[File Tree] | [Entry List + Search Bar] | [Source Viewer / Stats]
```

### Level Badge Colours

Match the Preact frontend's CSS `stats-table` colour scheme: ERROR=red, WARN=orange, INFO=blue, DEBUG=grey, TRACE=light-grey.

### Store

Dioxus signals store (follow `ticket-viewer` pattern):
- `selected_file: Signal<Option<String>>`
- `entries: Signal<Vec<LogEntry>>`
- `search_query: Signal<String>`
- `search_fields: Signal<HashMap<String, String>>`

## Acceptance Criteria

- File tree lists all log files in `target/logs/` (or configured log dir).
- Clicking a file loads its entries into the entry list.
- Search bar filters entries using the `search_fields` endpoint; results update on input (debounce 300 ms).
- Stats panel renders level histogram for the selected file.
- Source viewer shows a 5-line code snippet when clicking a `file:line` reference in an entry.
- All three panes are resizable.
- E2E test in `tools/viewer/e2e/tests/log-viewer/` covering file select + search.

## Files

- `tools/viewer/log-viewer/frontend/dioxus/src/app.rs` (full implementation)
- `tools/viewer/log-viewer/frontend/dioxus/src/components/log_file_tree.rs`
- `tools/viewer/log-viewer/frontend/dioxus/src/components/log_entry_list.rs`
- `tools/viewer/log-viewer/frontend/dioxus/src/components/search_bar.rs`
- `tools/viewer/log-viewer/frontend/dioxus/src/components/log_stats.rs`
- `tools/viewer/log-viewer/frontend/dioxus/src/components/source_viewer.rs`
- `tools/viewer/log-viewer/frontend/dioxus/src/store.rs`

## Depends on

- [LOG-5a] (scaffold)
- [LOG-3a] (search_fields endpoint)
- [LOG-3b] (search_text endpoint)
