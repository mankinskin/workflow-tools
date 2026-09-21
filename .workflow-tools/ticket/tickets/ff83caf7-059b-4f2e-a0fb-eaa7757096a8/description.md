## Objective

Make the session worktree a MANAGED, long-lived resource with an explicit lifecycle, instead of an ad-hoc directory created per request and abandoned. A session should get one worktree, keep it as the topic evolves, and release it deliberately.

## Problem

Today the worktree protocol has a creation path (`tools/worktree/worktree.sh new`) and a teardown path (`remove`), but no lifecycle binding the two to a SESSION. The observed consequences:

1. **Uncommitted main-checkout work is at risk.** Creating a worktree mid-session, while the main checkout carries unstaged changes, leaves those changes stranded in a checkout the agent has just navigated away from. Nothing preserves or even warns about them.
2. **Worktree-per-request sprawl.** Nothing binds a worktree to a session, so a second request in the same session can create a second worktree for the same work, and neither is ever obviously the live one.
3. **A topic change orphans the worktree.** When the work in a session shifts subject, the natural operation is to RENAME the worktree and its branch. There is no rename path, so the practical outcome is a new worktree and an abandoned old one whose name no longer describes its contents.
4. **No defined finish.** There is no single operation that takes a session's worktree from "work done" to "merged and removed", so cleanup is manual and frequently skipped.

`e2189e9d` established session check-in and worktree assignment as a data model. `3d535b2c` adds a prompt-time bootstrap hook. This ticket owns the OPERATIONS that assignment implies.

## Constraint discovered in the field

`git worktree move` cannot be used in this repository:

```
fatal: working trees containing submodules cannot be moved or removed
```

There is no flag that relaxes this. Any rename or relocation MUST therefore be implemented as remove + recreate on the same branch, followed by a local submodule-object fetch from the main checkout — because a linked worktree receives a SEPARATE submodule clone at `.git/worktrees/<name>/modules/<submodule>` that lacks local-only commits. See `503b9711` for the full evidence.

## Acceptance Criteria

1. Creating a session worktree while the main checkout has uncommitted changes PRESERVES those changes. At minimum this is available as an explicit option (a `git stash push --keep-index`-style save, recorded so it can be restored), and the default behavior never silently strands them: if preservation is not requested, the operation reports what it found and requires an explicit acknowledgement.
2. A session reuses ONE worktree by default. A creation request for a session that already holds an `Active` worktree assignment returns the existing worktree rather than creating a second one, and creating an additional worktree for the same session requires an explicit override.
3. A topic change RENAMES the existing worktree and its branch rather than creating a new one. Because `git worktree move` refuses on submodule-containing worktrees, this is specified and implemented as: remove the old worktree, recreate it at the new path on the renamed branch, then fetch submodule objects from the main checkout's clones by local path so that local-only commits and the recorded gitlink both resolve.
4. After a rename, the worktree resolves the same submodule commits it resolved before the rename — including any commit ahead of the recorded gitlink — verified per submodule, with no network access.
5. The rename updates the session's `SessionWorktreeAssignment` (path and branch) in place, so the session anchor used by `fa2ba34b` keeps resolving across the rename rather than pointing at a removed directory.
6. A finish operation takes a session's worktree from "work complete" to released: rebase onto the updated `main`, mark the branch ready to merge, and remove the worktree. Consistent with the repository rule, the finish operation itself never merges into `main` and never commits to `main`.
7. Removing a worktree with uncommitted or unpushed work is refused unless explicitly forced, and the refusal names what would be lost.
8. Every lifecycle operation is available with `--dry-run`, consistent with the existing `worktree.sh` surface.
9. Tests cover: reuse of an existing assignment, the rename path including submodule-object resolution afterwards, refusal to remove dirty work, and preservation of main-checkout changes on creation.

## Out of scope

- The MCP-side session anchoring and workspace resolution (`fa2ba34b`).
- The prompt-time hook that decides WHEN to bootstrap (`3d535b2c`).
- Bootstrapping from local `main` instead of `origin` (`503b9711`).

## Design source

`transcripts/06-08-2026_worktree-session-proxy/merged.clean.md`
