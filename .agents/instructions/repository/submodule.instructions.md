---
description: "Use when committing changes that involve Git submodules or nested repositories. Covers commit order, detecting dirty submodules, and updating submodule pointers."
---

## Commit order (deepest-first)

1. Commit inside nested submodules (e.g. `memory-api`) first.
2. Commit in their parent (e.g. `viewer-api` / `memory-viewers`) after staging pointer updates.
3. Update and commit the pointer in the root repo last.

## Detecting dirty submodules

```bash
git status --short
git submodule status
```

Lowercase `m` indicates local changes inside a submodule; uppercase `M` indicates the parent records a different SHA.

## Updating submodule pointers

After committing inside a submodule, stage the submodule directory in the parent and commit the pointer update:

```bash
cd memory-viewers && git add memory-api viewer-api && git commit -m "chore: update submodule pointers"
cd .. && git add memory-viewers && git commit -m "chore: update memory-viewers submodule pointer"
```

## Branch integration and gitlinks

Keep all submodule checkouts attached to `main` or feature branches recursively.
An immediate superproject on `main` requires each child on `main`; a feature
superproject permits `main` or feature children. Never leave detached HEADs
as the working state. Configure each `.gitmodules` entry's `branch` to match
the selected child (`.` only for an inherited identical branch name) and
`update = merge` or `rebase`; check local overrides too. Initial population
may detach HEAD: attach the intended branches before edits, builds or commits.
Do not force-switch, discard changes or move backward to satisfy the rule.
See the Meta-workspace's
[branch guard and hook setup](https://github.com/mankinskin/meta-workspace/blob/main/CONTRIBUTING.md#submodule-branches)
when operating inside that checkout.

Commit order is not integration order. Integrate affected submodule branches deepest first, fast-forward each submodule's `main`, then bump and commit its gitlink on the superproject feature branch before fast-forwarding the superproject. The full sequence is [worktree-merge.instructions.md](../../../session/.agents/instructions/worktree/worktree-merge.instructions.md#bottom-up-integration-sequence-canonical).

**Invariant:** every recorded gitlink must be contained in the child's selected
branch; publication on a `main` superproject requires containment in child
`main`. Feature-parent drafts may reference feature-child commits before
bottom-up integration. `git submodule status` shows `+` when a checked-out
HEAD differs from the recorded gitlink and `-` when uninitialized; a clean
integration has neither marker for any submodule. It does not prove branch
attachment: also inspect `git symbolic-ref --short HEAD` recursively.

| Failure signature | Diagnostic | Repair |
|---|---|---|
| Orphan gitlink | `git -C <sm> branch --contains <gitlink-sha>` prints nothing; `git -C <sm> merge-base --is-ancestor <gitlink-sha> main` fails | Immediately pin `rescue/<name>-<short-sha>`, then merge the pinned line into that submodule's `main`. |
| Backward gitlink | `git -C <sm> merge-base --is-ancestor <gitlink-sha> main` succeeds and `git -C <sm> merge-base --is-ancestor main <gitlink-sha>` fails | Bump the gitlink forward; never move the submodule checkout backward. |

The two `merge-base --is-ancestor` directions identify which side is ahead.
