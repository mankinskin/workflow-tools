# ticket-viewer: wire `init_tracing_full` with file logging

## Problem

`tools/viewer/ticket-viewer/src/main.rs` calls `init_tracing("info,ticket_http::serve::handlers=debug")`. All structured log output goes to stderr only. When started in detached mode (default `viewer-ctl start`), logs are silently discarded (`Stdio::null()`). No server logs survive to disk.

## Scope

- Replace `init_tracing(...)` with `init_tracing_full(&TracingConfig {...})` in `tools/viewer/ticket-viewer/src/main.rs`.
- Default log dir: `{cargo_workspace_root}/target/logs/` (use `CARGO_MANIFEST_DIR`/`..`/`..` or an env var `LOG_DIR`).
- Default file prefix: `ticket-viewer`.
- Default level: `info,ticket_http::serve::handlers=debug` (preserve existing level).
- Honour `LOG_LEVEL` and `LOG_DIR` env vars via `TracingConfig::from_env("ticket-viewer", default_log_dir)` if that helper exists, or manual env-var reads.
- Rolling daily appender (already implemented by `viewer-api::init_tracing_full`).
- Log output directory is created at startup if absent.

## Acceptance Criteria

- Starting `viewer-ctl start ticket-viewer --fg` produces a `target/logs/ticket-viewer.log` (or dated variant) containing JSONL-formatted log entries.
- `viewer-ctl start ticket-viewer` (detached) also produces the log file (file sink is independent of stdout/stderr).
- `mcp_log-viewer-mc_list_logs` discovers the file.
- `mcp_log-viewer-mc_query_logs` with `select(.level == "INFO")` returns server startup events.
- Console output is unaffected.

## Files

- `tools/viewer/ticket-viewer/src/main.rs` — change `init_tracing` call
- Possibly `tools/viewer/ticket-viewer/Cargo.toml` — verify `viewer-api` dep already present (it is)
