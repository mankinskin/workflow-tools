## Objective

Eliminate startup-time filesystem artifact pollution across MCP servers and viewers: no tool may create a `.ticket` store, `test-logs/`/`target/test-logs/` directories, or any other persistent artifact merely by starting up in a directory/workspace with no configured or initialized store/log sink. Creation of such artifacts must be deferred to the first actual configured write/init operation, never to process startup or read-only queries.

## Authoritative Behavior (user-approved)

1. ticket-mcp startup in a directory/workspace with no configured or initialized `.ticket` store must not create a store; persistent store creation requires explicit initialization/configuration.
2. Generated output directories such as `test-logs/` and `target/test-logs/` are created lazily only on the first actual configured write, never during mere startup.
3. When no filesystem log destination is configured, filesystem logging remains disabled (no log file/dir created as a side effect of enabling tracing/logging infrastructure).
4. Validation is designed first, using fresh temporary storeless workspaces:
   - Snapshot the filesystem before startup.
   - Spawn every relevant MCP server/viewer startup with exit code, stdout, and stderr captured.
   - Snapshot the filesystem after startup.
   - Assert the filesystem delta is exactly empty (no created files/dirs) for the read-only/no-config case.
   - Then run a positive case with explicit init/configured write and prove the intended artifact is created as expected.

## Scope

Do not assume `ticket-mcp` and `log-viewer` are the only creators. Investigate startup behavior across all installed MCP servers and viewers, including (subject to confirmation during implementation):

- `memory-api/tools/mcp/ticket-mcp`
- `memory-api/tools/mcp/session-mcp` (and `session-cli`)
- `context-stack/tools/mcp/context-mcp`
- `memory-viewers/ticket-viewer`
- `memory-viewers/log-viewer`
- `memory-viewers/doc-viewer`
- `memory-viewers/spec-viewer`
- any `spec-mcp`, `test-mcp`, `audit-mcp`, `feedback-mcp`, `fs-mcp` binaries present in `memory-api/tools/mcp/` or equivalent locations

The investigation phase must enumerate the actual installed set (do not hardcode this list into the implementation without verifying against the repo) and identify, per tool, whether startup alone creates any of:
- a `.ticket`/`.spec`/`.session`/`.feedback`/`.test` store directory
- `test-logs/` or `target/test-logs/` (or any other logging output directory)
- any other file/directory not explicitly requested by the caller

## Root-Cause Investigation Notes (starting points, not exhaustive)

- `memory-api/crates/memory-api/src/workspace.rs` workspace/store resolution helpers (referenced by ticket fa2ba34b-59ec-4321-a4ce-a3c3a9295ea3 for a related but distinct workspace-resolution defect) are a plausible shared resolution path worth checking for eager directory creation, but this must be confirmed by direct repository evidence rather than assumed.
- Tracing/logging initialization code paths (wherever `config/tracing.toml` or equivalent is read) must be checked for eager `test-logs/`/`target/test-logs/` directory creation independent of an actual write.
- Each MCP server's `main.rs`/server bootstrap should be checked for calls that create a store index root or log directory before any tool call is dispatched.

## Acceptance Criteria

1. A reusable empty-workspace fixture exists (e.g., a temp-dir helper) that produces a fresh, storeless workspace directory with no `.ticket`/`.spec`/`.session`/`.feedback`/`.test`/log-output artifacts, usable by both focused package tests and a repository-level fixture command.
2. A parameterized startup test matrix exists covering every relevant installed MCP server/viewer entry point identified during investigation (not just ticket-mcp and log-viewer).
3. For each tool in the matrix, starting the tool (or running its read-only/no-op startup path) against the empty-workspace fixture with no configured store/log sink produces a zero filesystem delta: before/after snapshots of the fixture directory are identical, verified by direct comparison, not by absence-of-error alone.
4. For any tool whose normal operation requires a store to exist, invoking an operation against the empty-workspace fixture (no store configured/initialized) produces a clear, non-mutating error — the tool must not silently create the missing store as a side effect of the failed operation.
5. A positive-path test exists proving that explicit initialization or an explicit configured write still creates the intended artifact (store directory, log file/dir) when the caller actually requests it — confirming the fix does not break legitimate lazy-creation behavior.
6. Regression coverage exists that pinpoints the exact code path/creator responsible for the previously observed `test-logs/`/`target/test-logs/` creation-on-startup behavior (not just a passing/failing assertion, but a test that would fail again at the specific creator if reintroduced).
7. Focused package-level tests pass for each affected crate/tool.
8. A repository-level fixture/validation command exists (e.g., a script or test target) that can run the full startup matrix and report the delta-per-tool result in one invocation.

## Non-Goals

- Does not resolve the session-anchored workspace-resolution/store-divergence defect tracked in ticket fa2ba34b-59ec-4321-a4ce-a3c3a9295ea3; that ticket addresses cwd-vs-server-cwd store resolution, a related but distinct concern from startup-time artifact creation.
- Does not import or restate unrelated pre-dispatch-gate requirements from orchestration instructions.

## Related

- Linked (not depends_on, pending direct evidence of a hard implementation dependency): ticket fa2ba34b-59ec-4321-a4ce-a3c3a9295ea3, "Session-anchored MCP workspace resolution: require session_id and resolve every proxied call to the session's active worktree" (state: open). Both concern MCP/tool startup and workspace/store resolution correctness, but fa2ba34b is scoped to session-to-worktree store resolution while this ticket is scoped to artifact creation timing (eager vs lazy). No repository evidence currently shows one is a hard prerequisite for the other.
