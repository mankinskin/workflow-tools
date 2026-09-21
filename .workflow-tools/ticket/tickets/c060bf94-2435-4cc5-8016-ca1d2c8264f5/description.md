Problem: the branch-and-worktree isolation protocol is guidance-only. The draftboard cannot answer "which worktree is this ticket being worked in", "which session owns that worktree", or "which worktrees are active right now". Branch and worktree currently survive only as an unvalidated text convention inside the board `intent` field, and the authoritative session-side assignment in `SessionWorktreeAssignment` is never joined to the board.

Goal: make the worktree binding a first-class, queryable property of a board entry, and add a read surface that lists active worktrees with their owning sessions.

Acceptance criteria:
1. `BoardEntry` gains three optional fields — `session_id: Option<String>`, `worktree_path: Option<String>`, `branch: Option<String>` — without breaking deserialization of board rows written before the change.
2. `board_check_in` accepts the three new values on both the MCP tool schema and the `ticket board check-in` CLI, as optional arguments, and persists them on the entry.
3. `board_show` returns the three new fields on every entry it reports, in both human and machine-readable output.
4. A new read surface lists active worktrees: for each distinct `worktree_path` among active entries, it reports the worktree path, its branch, the owning session id, the owning agent id, and the ticket ids being worked in it. It is exposed on both the CLI and the MCP surface.
5. An entry checked in with a `worktree_path` that is already held by a different active entry with a different `session_id` is reported as a conflict rather than silently accepted.
6. Existing board rows lacking the new fields continue to load, and report the new fields as absent rather than failing or defaulting to a misleading value.
7. Tests cover: round-trip persistence of the three fields, backward-compatible load of a pre-change row, the active-worktree listing grouping, and the duplicate-worktree conflict case.

Out of scope: automatically creating or removing git worktrees; verifying that a recorded `worktree_path` exists on disk; reconciling the board against the session store as a background job; CI enforcement.