---
description: "Commit staged or unstaged changes across the repo and all submodules, handling pre-commit hooks and generated files correctly."
name: "commit"
argument-hint: "[message]"
agent: "agent"
---

# Commit Changes

Commit all pending changes across the root repo and submodules following the repository's commit conventions.

Reference [commit.instructions.md](../instructions/commit/) and [AGENTS.md](./AGENTS.md).

## Workflow

1. Run `git status --short` and `git submodule foreach --recursive 'git status --short'` to survey all changes.
2. Identify dirty submodules (lowercase `m` in status output) and plan commit order: deepest-first, then parent pointer updates.
3. Check whether any generated files have drifted and regenerate them before staging (see [generated-files.instructions.md](../instructions/commit/generated-files.instructions.md)).
4. Stage and commit in logical batches (see [commit.instructions.md](../instructions/commit/) for batch order).
5. For each batch, write a conventional-commit message: `<type>(<scope>): <imperative summary>`.
6. After all root-repo commits are done, update submodule pointers deepest-first.
7. Verify clean state with `git status --short`.

## Key rules

- Never edit generated files directly. Always regenerate via the owning generator. Root `AGENTS.md`, `.github/copilot-instructions.md`, and everything under `.agents/**` are hand-owned — edit them directly.
- Commit submodules in deepest-first order before updating parent pointers.
- The pre-commit hook blocks commits that stage generated outputs that have drifted from disk. Fix by regenerating and re-staging.
- Use `git commit --no-verify` only for confirmed false-positive hook failures; document why in the commit message.

## Response

Return:
- files committed per batch, with commit message
- any pre-commit hook failures and how they were resolved
- submodule pointer updates made
- final `git status --short` output confirming clean state
