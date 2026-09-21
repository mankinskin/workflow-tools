# doc-viewer + spec-viewer: wire `init_tracing_full` with file logging

## Problem

`doc-viewer` and `spec-viewer` (if they have `main.rs` entry points) currently use `init_tracing()` (console-only) or have no explicit tracing setup. Logs are lost in detached mode.

## Scope

For each of `tools/viewer/doc-viewer/src/main.rs` and `tools/viewer/spec-viewer/src/main.rs`:

1. Replace `init_tracing(...)` call with `init_tracing_full(&TracingConfig {...})`.
2. Default log dir: `{workspace_root}/target/logs/`.
3. Default file prefixes: `doc-viewer`, `spec-viewer`.
4. Honour `LOG_LEVEL` / `LOG_DIR` env vars.
5. Log dir created at startup if absent.

Also verify the `viewer-api` crate is already a dependency in both `Cargo.toml` files (it should be).

## Acceptance Criteria

- `viewer-ctl start doc-viewer` produces `target/logs/doc-viewer.log` with JSONL entries.
- `viewer-ctl start spec-viewer` produces `target/logs/spec-viewer.log` with JSONL entries.
- Both are discoverable by `mcp_log-viewer-mc_list_logs`.
- Console output unaffected.

## Files

- `tools/viewer/doc-viewer/src/main.rs`
- `tools/viewer/spec-viewer/src/main.rs`
