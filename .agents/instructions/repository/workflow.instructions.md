---
description: "Use when performing the repository commit workflow. Covers checking status, regenerating generated outputs, staging logical batches, committing, and verifying clean state."
---

**Branch precondition.** The main checkout is the default execution mode. Before committing there, check the board for overlapping ownership and stage only the task's explicit paths. For a worktree-backed task, confirm you are inside the chosen worktree on its feature branch: `git branch --show-current` must print `agent/<full-session-uuid>/<slug>`, not `main`. `git add -A` from an implementation session is forbidden in either mode. `./target/debug/worktree-ctl.exe list`, run from the main checkout, shows the registered worktrees and their branches. See [AGENTS.md](../../../AGENTS.md#universal-safeguards) for the selection rule and [worktree-workflow.instructions.md](../../../session/.agents/instructions/worktree/worktree-workflow.instructions.md) for the worktree protocol.

1. Check status before staging:

git status --short
git submodule foreach --recursive 'git status --short && echo "=== $name ==="'

Before staging, verify attached branches and branch/update tracking recursively
under the [submodule contract](submodule.instructions.md#branch-integration-and-gitlinks).
Every immediate parent on `main` requires child `main`; a feature parent
permits child `main` or a feature branch. Untouched children may stay on
`main`; worktree-owned edits require the matching session feature branch.
Detached checkouts block staging. Check `.gitmodules` and local overrides;
do not discard changes or move branches backward to repair a violation.

2. Regenerate generated outputs when applicable (see [generated-files.instructions.md](./generated-files.instructions.md)).

3. Stage in logical batches and commit each batch with an appropriate conventional commit message. Include task-owned tracking changes in the immediate parent. Before each commit, recheck indexed `.gitmodules` and gitlinks recursively: every gitlink must be contained in the selected child branch. An unstaged configuration repair does not validate the staged commit.

4. Update submodule pointers last (deepest-first cadence), then verify the gitlink invariant from [worktree-merge.instructions.md](../../../session/.agents/instructions/worktree/worktree-merge.instructions.md#bottom-up-integration-sequence-canonical).

5. Verify clean state:

git status --short
git submodule status

Also repeat the branch/tracking check: `git submodule status` alone cannot
detect detached HEADs or invalid parent/child branch combinations. Follow the
[submodule contract](submodule.instructions.md#branch-integration-and-gitlinks)
for the configured guard; use `--staged` for the pre-commit check.

6. Per AGENTS.md (`../../../AGENTS.md#quality-gates`)'s bottom-up rebase rule (submodules before the superproject), rebase the feature branch onto updated `main` in every affected repository. Resolve conflicts on feature branches, re-run validation, run the invariant check in [worktree-merge.instructions.md](../../../session/.agents/instructions/worktree/worktree-merge.instructions.md#bottom-up-integration-sequence-canonical), then mark the branch ready to merge with a `board check-out` whose reason starts `ready-to-merge:`.
