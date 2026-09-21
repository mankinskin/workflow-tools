# WP1: Reproduce and characterize session hook routing failure

## Objective

Reproduce and characterize the session capture, session lookup, and MCP routing
failure before redesign work begins. This ticket is WP1 of the approved plan in
`transcripts/17-08-2026-session-hook-routing/03-work-packages.md`. The
reproduction must be able to fail rather than assume the leading diagnosis.

## Evidence

1. The capture hook has produced no capture since **2026-08-13**
   (`.session/local/hook-captures/*.json` all stale) despite active sessions on
   2026-08-15, 16, and 17. Hook registration in
   `.github/hooks/hooks.json` and `chat.hookFilesLocations` in
   `.vscode/settings.json` is structurally valid, all referenced scripts exist,
   bash resolves, and the binary at `~/.cargo/bin/session-capture-hook.exe` has
   a valid interface. Leading hypothesis: VS Code has not reloaded hook
   registration since the 2026-08-12/2026-08-14 hook commits (`159c71ad`,
   `c972aed0`, `74057708`, `2f81d5ed`).
   Resolved paths: `.session/local/hook-captures/`, `.github/hooks/hooks.json`,
   `.vscode/settings.json`, `memory-api/crates/session-capture-hook/`.
2. `session.exe lookup` fails with `session data was not found at
   .session/sessions/<uuid>/transcript.json` — session resolution is
   hard-coupled to transcript capture, so no session can resolve while the hook
   is dead.
   Resolved path: `memory-api/crates/session-api/`.
3. `session.exe lookup` also rejects sessions with `does not have a persisted
   worktree assignment` **even when a matching `.worktrees/<uuid>/<slug>`
   exists** — positional discovery does not work as documented in
   `.agents/instructions/session/worktree-provisioning.instructions.md`.
   Resolved paths: `memory-api/crates/session-workspace-resolver/src/lib.rs`,
   `.agents/instructions/session/worktree-provisioning.instructions.md`.
   Direct evidence for adjacent ticket `200e9ecc-d61a-4b1a-a3a6-a9dd1e77d915`.
4. `.session/sessions/` is **git-tracked**; `.session/local/` is ignored.
   Creating a session record dirties the main checkout, which made
   `worktree-ctl new` refuse to provision until the record was committed.
   Resolved paths: `.session/sessions/`, `.session/local/`,
   `tools/worktree/worktree-ctl/`.
5. Every registered worktree carries a full replicated copy of
   `.session/sessions/` (~288 records). One worktree
   (`84d5bde2-.../session`) is missing three UUIDs present in main; another
   (`153deb7f-.../ticket-extraction-finish`) has one extra. Session state is
   fragmented across checkouts.
   Resolved paths: `.session/sessions/`, `.worktrees/84d5bde2-.../session/.session/sessions/`,
   `.worktrees/153deb7f-.../ticket-extraction-finish/.session/sessions/`.
6. `mcp_ticket_list_workspaces` — a READ operation — was rejected with `main
   checkout mutations are blocked; run session_check_in from an assigned
   worktree path...`. A read being classified as a mutation is itself a defect.
   Resolved paths: `memory-api/crates/ticket-api/`,
   `memory-api/crates/session-workspace-resolver/src/lib.rs`.
7. Suspected mechanism for the original misroute:
   `memory-api/crates/session-workspace-resolver/src/lib.rs` discovers
   worktrees positionally by session UUID and ignores caller cwd, while
   `memory-api/tools/mcp/mcp-toolmon/` anchors from its own cwd or
   `MCP_MAIN_CHECKOUT` (both unset here).
   Resolved paths: `memory-api/crates/session-workspace-resolver/src/lib.rs`,
   `memory-api/tools/mcp/mcp-toolmon/`.

## Decisions

- D1: The main checkout is the authoritative session-to-worktree registry. The
  registry is not checked into git; only the working branch is committed to the
  main-checkout session record.
- D2: Replace eager `UserPromptSubmit` provisioning with deferred provisioning:
  capture in main first, instantiate worktrees explicitly, then register.
- D4: If worktree creation fails after the session entry was committed, block
  the session; recover only with a new forward commit that unsets registration.
- D5: Validation is unit tests only; no E2E.

## Acceptance Criteria

1. Focused unit tests distinguish registered-worktree routing, main-only
   routing, and the observed main-store mutation for a deterministic fixture.
2. A fixture records hook input, resolver decision, proxy rewrite, and every
   mutated store, and fails when observed routing differs from the asserted
   outcome.
3. Tests demonstrate whether lookup can resolve independently of
   `transcript.json`, whether a matching positional worktree is accepted, and
   whether MCP read routing is classified as a mutation.
4. Tests cover unset `MCP_MAIN_CHECKOUT` and tool-local cwd behavior without
   relying on a running VS Code instance.
5. `cargo test` for each touched crate passes; no E2E or browser validation is
   required.

## Related Work

- `200e9ecc-d61a-4b1a-a3a6-a9dd1e77d915`: direct evidence for persisted
  assignment rejection.
- `0afe45b5-9ec8-4f4a-af74-f46f06cc7516`: completed prior art on mis-anchored
  store resolution.
- `2b657154-df78-4bb3-807a-66c9ff811ceb`: adjacent debris-removal work; do not
  duplicate its scope.

See `transcripts/17-08-2026-session-hook-routing/README.md` and the dossier
documents in the same directory for the approved operating model and work
package details.