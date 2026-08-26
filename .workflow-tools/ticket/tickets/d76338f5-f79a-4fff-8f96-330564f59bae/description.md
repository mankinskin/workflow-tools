Audit of the mcp-toolmon crate ([workflow-tools/session/crates/mcp-toolmon](workflow-tools/session/crates/mcp-toolmon)) found `src/proxy.rs` at 2500 lines mixing five concerns: cost-gate policy enforcement, workspace/session resolution, path-argument rewriting, schema injection, and ~50 unit tests. `handle_client_message` alone is 283 lines at cyclomatic complexity 24 (threshold 12).

This complexity directly contributed to a real bug: a workspace-resolution gating decision was keyed off the literal `workspace` argument string instead of the resolved checkout scope, buried inside a large match arm in this file (fixed separately in commit b263104).

Split `proxy.rs` into focused modules, keeping the public API (`handle_client_message`, `handle_server_message`, `ClientAction`, `PendingList`, `PendingCalls`) re-exported from the crate root unchanged:
- `workspace_resolution.rs` — `resolve_workspace`, `resolve_workspace_for_tool`, `resolve_unassigned_session_target`, `repository_root_target`, `session_is_unassigned`, `try_resolve_session_check_in_bootstrap_workspace`, `anchored_resolver`.
- `path_rewriting.rs` — `PATH_ARGUMENT_REGISTRY`, `PathArgument`, `PathArgumentKind`, and the argument-rewriting logic that uses them.
- `schema_injection.rs` — tools/list schema-injection logic (`caller_model` argument injection).
- `gating.rs` — `ToolAccess`, `tool_access`, `KNOWN_READ_TOOLS`, and the cost-gate policy decision plumbing in `handle_client_message`/`handle_server_message`.
- `proxy.rs` stays as thin orchestration wiring the above together.

Split the ~50 unit tests alongside the code they exercise (e.g. workspace-resolution tests move to `workspace_resolution.rs`'s own `#[cfg(test)] mod tests`), not into one giant leftover test file.