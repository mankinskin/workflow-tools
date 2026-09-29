---
description: "Use when creating, testing, rolling back, or promoting an isolated prototype version of a component using a parallel git worktree while an existing implementation remains the default."
applyTo: "**"
---

# Rapid Prototyping And Version Transitions

Use this workflow when a new implementation must be developed separately while
an existing version remains the default for tools and users. The mechanism is
a **prototype branch checked out into its own git worktree**, alongside the
default branch's own checkout, so both are simultaneously readable and
editable without stashing or switching. This generalizes the repository's
existing session-isolation convention (`.worktrees/<session-uuid>/<slug>`, see
[worktree-workflow.instructions.md](../../../session/.agents/instructions/worktree/worktree-workflow.instructions.md))
to component/version prototyping instead of session isolation.

The workflow was rehearsed twice in the `template/` repository: an earlier
folder + environment-variable-selector pilot (commit `b3b6c9f`, branch
`prototype/session-api-v2-pilot-clean`, preserved as historical evidence), and
the current worktree-based pilot (commit `b86f380`, branch `proto/v2-pilot`,
see [07-template-worktree-pilot.md](https://github.com/mankinskin/meta-workspace/blob/main/transcripts/21-09-2026_session-api-v2-prototyping/07-template-worktree-pilot.md)).
Prefer the worktree mechanism below for new prototyping work; the folder +
selector pattern is superseded because it required stashing/switching to
compare the default and prototype states, and did not generalize past a
single repository.

## Fixed Boundaries

- Keep the stable implementation as the default branch/target until explicit promotion approval.
- Put the experimental implementation on its own branch, checked out into its own worktree; never edit the prototype directly inside the default checkout.
- Name the prototype branch `proto/<slug>` and its worktree `.worktrees/proto/<slug>` (or a component-scoped equivalent), so the convention reads consistently with the existing session-worktree naming.
- Keep development data separate from stable data by relying on the worktree boundary itself: each worktree has its own working directory, its own Cargo `target/` build output, and can default any runtime store root to a worktree-relative path. Do not introduce a separate environment-variable selector (like the superseded `TEMPLATE_TARGET`/`SESSION_API_TARGET`/`SESSION_API_STORE_ROOT` pattern) unless the worktree boundary alone is demonstrably insufficient for a specific component.
- Do not change stable configuration, APIs, runtime records, or launch behavior merely to make the prototype easier to test.

## Prototype Setup

1. Record the stable baseline commit and working-tree status before creating a prototype branch.
2. Create the prototype branch: `git branch proto/<slug>`.
3. Add the prototype worktree: `git worktree add .worktrees/proto/<slug> proto/<slug>`.
4. Make every prototype change inside the prototype worktree only, leaving the default checkout's working directory untouched and simultaneously readable.
5. Test the experimental version by running its build/binary from inside the prototype worktree, without changing the default checkout.

The template rehearsal uses:

```bash
git branch proto/v2-pilot
git worktree add .worktrees/proto/v2-pilot proto/v2-pilot
# edit only inside .worktrees/proto/v2-pilot/, commit there
git merge proto/v2-pilot --no-edit   # promotion
git worktree remove .worktrees/proto/v2-pilot   # rollback
```

## Validation And Rollback

Record all four observations:

- the default checkout's files remain readable and unchanged while the prototype worktree is edited;
- the prototype worktree's build/test runs independently of the default checkout (separate `target/` output, separate store root if applicable);
- promotion (`git merge`/fast-forward of the prototype branch into the default branch) completes and the change lands on the default branch;
- rollback (`git worktree remove`, optionally `git branch -D` the prototype branch) removes the prototype worktree cleanly, leaving the default branch/worktree unaffected.

Rollback is a worktree operation: stop using the prototype worktree, remove it
with `git worktree remove <path>`, optionally delete its branch if the
prototype is discarded rather than promoted, and confirm the default
checkout's files and history are unchanged. Do not merge an unfinished or
unvalidated prototype branch into the default branch during rollback.

## Promotion Gate

Promotion is a separate approved operation, not an automatic consequence of a
passing prototype test. Before promotion, collect reproducible evidence for the
library/package contract, CLI, MCP server, capture hook, handoffs, runtime
store, lifecycle behavior, rollback, and every launch configuration. Require a
human approval checkpoint and preserve the default branch's own worktree as the
rollback reference throughout — never delete or repurpose it while a prototype
is under evaluation.

For the Session API rewrite specifically, do not create a Session API v2
worktree/branch, change any default target, write v1 runtime records, or begin
the rewrite until the approved roadmap's research and pilot waypoints are
complete and its final handoff is compiled.

## Required Evidence

Each transition record must name the baseline commit, prototype branch name,
worktree path, commands run, outputs, changed files, default-checkout
no-write check, promotion or rollback result, commit id, and approval status.
An unrehearsed worktree lifecycle, an ambiguous branch/worktree naming, a dirty
unexplained baseline, or a missing rollback result blocks promotion.

