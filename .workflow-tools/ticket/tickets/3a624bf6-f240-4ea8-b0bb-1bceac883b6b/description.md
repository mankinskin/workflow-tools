## Requirement

Each git worktree and every folder within a worktree MUST be completely isolated and independent from the content of other worktrees. No component may persist a dependency on another worktree's path or content. Recording bare worktree existence is permitted but not currently required, because a stored existence dependency can break when a worktree is deleted.

## Verified findings

1. **Store-resolution leak:** `resolve_requested_store_root_from` in `memory-api/crates/memory-api/src/workspace.rs` (around line 374) and `resolve_index_root_from` in `memory-api/tools/cli/ticket-cli/src/cli/dispatch.rs` (around line 187) can override an explicit `--index-root` and resolve a root invocation to a sibling worktree store. A deleted sibling path previously blocked every repository ticket operation until another worktree appeared. Severity: BREAKS-ON-DELETE.
2. **Persisted descendant scan roots:** `register_descendant_scan_roots` in `memory-api/tools/cli/ticket-cli/src/cli/dispatch.rs` (around lines 246 and 338) recursively registers descendant `.ticket` paths through discovery in `memory-api/crates/memory-api/src/workspace.rs` (around lines 454 and 543). `TicketStore::add_scan_root` in `memory-api/crates/ticket-api/src/storage/store/scan.rs` (around line 54) writes sibling absolute paths to root `tickets.db`; `ticket-http` repeats the behavior at startup in `memory-api/tools/http/ticket-http/src/main.rs` (around line 77).
3. **No scan-root pruning:** `memory-api/crates/ticket-api/src/storage/store/scan.rs` has no `DELETE FROM scan_roots` or removal API. `reapply_workspace_policy` (around line 129) only marks rows ignored and retains them.
4. **Persisted session assignments:** `check_in_worktree` in `memory-api/crates/session-api/src/store/config/worktree_runtime.rs` and `worktree_capture_inference.rs` (around line 30) write `metadata.worktree.path` into the root `.session` store. `SessionWorkspaceResolver::resolve` in `memory-api/crates/session-workspace-resolver/src/lib.rs` (around lines 201 and 230) canonicalizes a deleted assignment and returns `SessionWorktreeMissing`; `memory-api/tools/mcp/mcp-toolmon/src/proxy.rs` (around lines 343 and 358) then rejects calls. Severity: BREAKS-ON-DELETE.
5. **Phantom board claims:** `BoardEntry` in `memory-api/crates/memory-api/src/storage/board.rs` (around line 20) stores absolute worktree paths through `memory-api/crates/memory-api/src/storage/board/ops.rs` (around line 176). `memory-api/crates/memory-api/src/storage/board/ops/snapshot.rs` (around line 20) does not check existence, so deleted worktrees remain active claims. Severity: BREAKS-ON-DELETE.
6. **Cross-worktree reclaim policy:** `memory-api/crates/session-worktree-provision/src/policy.rs` (around lines 61, 201, 212, and 345) reads every root `.session/sessions/*/session.json`, candidate dirty state, dirty submodules, mtimes, and branch ahead/behind state. Severity: violates state isolation.
7. **Unkeyed global frontend cache:** `install-ctl prepare <viewer>` writes to `~/.context-engine/static/<frontend-name>` (`viewer-ctl.toml` around line 21; `tools/install/install-ctl/src/config.rs` around line 220; `tools/install/install-ctl/src/commands/frontend.rs` around line 72). The name-only slot is deleted and recopied, allowing different worktrees to silently overwrite one another. Severity: SILENT-CORRUPTION.
8. **Ancestor-store leakage:** ancestor discovery for `.feedback`, `.spec`, and `.test` can resolve a worktree to the main checkout's store in `memory-api/crates/memory-api/src/workspace.rs` (around line 344), `memory-api/tools/mcp/feedback-mcp/src/server.rs` (around line 78), `memory-api/crates/spec-api/src/store.rs` (around line 125), and `memory-api/tools/mcp/test-mcp/src/server.rs` (around line 235). Severity: SILENT-CORRUPTION.

## Acceptance criteria

1. An explicit `--index-root` or `--workspace` override is always honored and is never overridden by discovery.
2. Store resolution never resolves a worktree's store to a sibling worktree's store, and descendant discovery excludes `.worktrees/**`.
3. No absolute path belonging to another worktree is persisted in root `tickets.db` `scan_roots` or `board_entries`, or in root `.session` records.
4. Deleting any worktree cannot break store operations in the main checkout or any other worktree. A regression test deletes a worktree and then exercises ticket read/write, board show, and session resolution.
5. Existing stale rows and records are either pruned or tolerated on open, according to the decision recorded for this work; the selected behavior has regression coverage.
6. The frontend asset cache is checkout-local or keyed so two worktrees cannot silently overwrite one another.
7. `.feedback`, `.spec`, and `.test` resolution has equivalent worktree-isolation coverage.

## Related work

Related tickets: `5e6cf4f8`, `3d535b2c`, `fa2ba34b`, `ff83caf7`, `614fae19`, `c060bf94`, and `723c2bea`.

Related specifications: `2860a8db` (default worktree-backed session workflow), `10dee1dc` (board-to-session worktree binding), `0f5acbfe` (session-id worktree routing), `aff42efb` (session-anchored MCP workspace resolution), and `5d9e5a99` (workspace store resolution anchoring).

`5d9e5a99` claims sibling `.worktrees/*` directories are never resolution candidates. Finding 1 demonstrates that the current implementation violates that claim; implementation must reconcile the behavior with the existing specification.

## Open design questions

1. Should stale scan-root, board, and session-assignment rows be pruned during open/reconciliation or tolerated as inert records?
2. Should descendant discovery be removed entirely, or retained with `.worktrees/**` excluded?
3. Are nested per-worktree ticket stores an intended feature, and would excluding `.worktrees/**` break a supported workflow?
4. Should frontend assets become checkout-local or use a content-hash/worktree-keyed global cache?
5. What migration safely cleans existing polluted databases and session records without removing still-valid records?


## Binding architecture decisions (2026-08-11)

The user resolved the implementation choices from the isolation principle:

1. **Direct CLI is local.** The CLI runs in a caller-controlled working directory and never uses the MCP/session proxy. A CLI store operation resolves only from its current checkout or an explicit selector. `--index-root` and `--workspace` are absolute instructions, never hints that discovery may override.
2. **MCP is session-routed.** MCP servers run behind the proxy because an agent-controlled CWD is not reliable. The proxy resolves each call only to the calling session's assigned worktree. No MCP route may discover, read, or persist a sibling worktree path or store.
3. **Discovery remains local.** Nested stores within the caller's checkout, including submodules, remain supported. `.worktrees/**` is always excluded from discovery and aggregation.
4. **Artifacts and caches are local.** Build outputs, viewer static assets, and every store are checkout-local. Rebuild cost is acceptable; no global or shared cache is permitted as a fallback.
5. **Legacy references are pruned.** A persisted path outside the owning checkout is invalid state, not an inert dependency to retain. Open/reconciliation migrates or deletes cross-worktree scan-root, board, and session-assignment records without consulting the deleted worktree.

## Child implementation tickets

The epic depends on the following tickets:

- `8130027d` contain CLI ticket-store resolution and purge cross-worktree scan roots
- `968e863b` make session assignments and MCP routing independent of deleted worktrees
- `f7a0f5b5` remove cross-worktree board claims and reconcile deleted-worktree entries
- `fde76de2` make viewer frontend assets checkout-local
- `461ddbb1` contain feedback, spec, and test stores within the caller checkout
- `55403b85` add the final two-worktree deletion isolation regression harness

Ticket `55403b85` depends on the first five tickets and is the end-to-end acceptance gate for this epic. These decisions supersede the open design questions above.