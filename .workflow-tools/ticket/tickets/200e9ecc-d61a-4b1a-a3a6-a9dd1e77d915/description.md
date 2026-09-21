## Symptom

`session_check_in` fails for session `b9020ba2-df5d-426a-b1b9-228ef159cad1` even though the session has exactly one nested worktree at `.worktrees/b9020ba2-df5d-426a-b1b9-228ef159cad1/guidance-learnings-impl` and the branch is `agent/b9020ba2-df5d-426a-b1b9-228ef159cad1/guidance-learnings-impl`.

Observed error from `session_check_in` with workspace selector set to the worktree path and ticket `07a3eb2d`:

```text
session error: session b9020ba2-df5d-426a-b1b9-228ef159cad1 does not have a persisted worktree assignment
```

`session.exe lookup --session-id b9020ba2-df5d-426a-b1b9-228ef159cad1 --workspace . --toon` returns the same failure.

## Reproduction evidence

`git worktree list`:

```text
C:/Users/linus/git/2/context-engine 8ac56325 [main]
C:/Users/linus/git/2/context-engine/.worktrees/84d5bde2-c860-4f24-8cb6-4f3a98c042b0/session 8c3a89ba [agent/84d5bde2-c860-4f24-8cb6-4f3a98c042b0/session]
C:/Users/linus/git/2/context-engine/.worktrees/b9020ba2-df5d-426a-b1b9-228ef159cad1/guidance-learnings-impl 30375247 [agent/b9020ba2-df5d-426a-b1b9-228ef159cad1/guidance-learnings-impl]
```

`ls -1 .worktrees/b9020ba2-df5d-426a-b1b9-228ef159cad1/` from the repository root:

```text
guidance-learnings-impl/

Summary: 0 files, 1 dirs
```

`cargo run --manifest-path memory-api/tools/cli/session-cli/Cargo.toml -- lookup --session-id b9020ba2-df5d-426a-b1b9-228ef159cad1 --workspace . --toon`:

```text
status: error
message: "session error: session b9020ba2-df5d-426a-b1b9-228ef159cad1 does not have a persisted worktree assignment"
error: process didn't exit successfully: `target\debug\session.exe lookup --session-id b9020ba2-df5d-426a-b1b9-228ef159cad1 --workspace . --toon` (exit code: 1)
```

Earlier in the same session, `lookup` also reported `session data was not found at .\.session\sessions\b9020ba2-df5d-426a-b1b9-228ef159cad1\transcript.json`, and `session_check_in` had previously failed with `session '<uuid>' matches 2 worktrees; refusing to choose: <two paths>` before manual cleanup.

`board_check_in` against the same worktree selector succeeds, so only session check-in is failing.

## Contradiction with the documented contract

The repository instructions explicitly describe positional discovery and explicitly say there is no persisted assignment index.

From `.agents/instructions/session/session-identity-and-handoff.instructions.md`:

> Lookup discovers the worktree positionally: exactly one nested `.worktrees/<session-uuid>/<slug>` directory wins over a valid legacy flat candidate. No candidate returns `MissingSessionWorktree`; multiple valid candidates return `AmbiguousSessionWorktree`. Lookup never silently resolves an unassigned session to the main checkout. On either error, inspect the layouts described in [worktree-provisioning.instructions.md](worktree-provisioning.instructions.md) and repair or create the worktree before running mutations; do not proceed in the main checkout. `git rev-parse --show-toplevel` is a hint, not an answer: a session commonly runs from the repository root while the provisioned worktree is elsewhere.

From `.agents/instructions/session/worktree-provisioning.instructions.md`:

> A successful provision creates the session worktree; worktree discovery is positional from the supported directory layouts, with no main-checkout session-to-worktree assignment index.

The emitted error `does not have a persisted worktree assignment` contradicts that contract.

## Contributing defect

`worktree-ctl rename <uuid>/session <uuid>/<slug>` created the new worktree but left the old `session` entry registered on disk and in git, which produced the transient `matches 2 worktrees` state. Cleanup then required manual removal plus `git worktree prune` because `worktree-ctl remove` refused over untracked `.session/**` files it was responsible for.

## Acceptance criteria

- A session with exactly one nested `.worktrees/<session-uuid>/<slug>` directory can complete `session_check_in` without any persisted assignment record.
- `lookup` resolves that session positionally and does not require a main-checkout session registry.
- The error message is updated so it matches the documented positional-discovery contract, or the code path is changed so it no longer depends on a persisted worktree assignment.
- The rename/remove flow does not leave behind a duplicate registered `session` worktree entry after retopicing.