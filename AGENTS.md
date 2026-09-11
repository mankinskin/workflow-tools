# workflow-tools Agent Guide

This repository owns the installable ticket, spec, session, rule, audit, and test workflow tooling used by consumer workspaces. Agents working here should treat `workflow-tools` as the canonical guidance root for tool implementation, validation, release, and repository-guidance lifecycle work.

## Guidance Roots

- `.agents/agents/` contains custom agent templates. Use these templates for context-isolated delegation when a task is bulky, risky, or benefits from a dedicated role.
- `.agents/instructions/` contains operational rules. Load only the instruction files that apply to the task or edited paths.
- `.agents/prompts/` contains reusable prompt workflows for implementation, research, handoff, iteration, commits, model-price sync, and tool-grant regression probes.
- `.agents/skills/` contains on-demand skills. Load a skill only when the task falls inside the skill description.

Nested `AGENTS.md` files may refine behavior for their own subtree. When a nested guide applies, follow the nested guide for that subtree and keep this file as the repository-level default.

## Before Editing

Start from the concrete anchor named by the request: a failing command, file, symbol, ticket, spec, or nearby implementation surface. Gather only enough evidence to name the controlling path, one falsifiable local hypothesis, and the cheapest check that could disprove that hypothesis. Once that evidence exists, make the smallest grounded edit and validate immediately.

Use bounded inspection. Prefer `repo_map.toon`, skeletons, targeted search, and small file windows over broad file reads. Use the peek tools for file inspection and compact terminal tooling for command output when those tools are available.

## Task Routing

- Use research or exploration before implementation when ownership, behavior, validation, or requirements are unclear.
- Use implementation only after a complete handoff exists: objective, target paths, relevant ticket or spec, constraints, and validation commands.
- Use review for bugs, regressions, missing tests, acceptance-criteria checks, and code-quality findings.
- Use commit workflow only after validation and stage only the explicit paths for the current task.

The model ladder and delegation rules live in `.agents/instructions/workflow/model-routing.instructions.md` and `.agents/instructions/workflow/orchestrator-delegation.instructions.md`. Do not guess model tiers from vendor names or dispatch a sub-agent without a self-contained prompt.

## Session and Worktree Discipline

Resolve the target repository from the active workspace before running session, Git, or worktree commands. Main-checkout work is the default for small, self-contained changes. Create or use a session worktree only when isolation is required by active ownership, explicit branch requirements, or planned Git operations that need an independent branch.

When a task touches submodules, follow the deepest-first rule: commit inside the nested repository first, then update parent gitlinks. Never stage unrelated dirty files, and never use broad `git add -A` from an implementation session.

Relevant rules:

- `.agents/instructions/repository/workflow.instructions.md`
- `.agents/instructions/repository/submodule.instructions.md`
- `session/.agents/instructions/session/session-identity-and-handoff.instructions.md`
- `session/.agents/instructions/worktree/worktree-workflow.instructions.md`

## Validation

Every repository-changing task needs executable evidence unless the task is pure prose with an explicitly documented review method. Run the narrowest check that can falsify the change first, then broaden only when the touched surface requires it.

Common workflow-tools validation anchors:

- `cargo test -p <crate>` for Rust crate changes.
- `cargo test --workspace --no-run` when compile coverage matters more than a single crate test.
- `install/install-ctl` catalog checks for install registry or guidance installer work.
- `audit.exe links .` for repository-link and markdown-reference changes.
- Focused `rg` checks for guidance moves, generated references, and old paths.

Record validation results in the final response. If a validation command cannot run, report the exact blocker and the strongest evidence gathered instead.

## Guidance Authoring

Guidance files should be small, owned, and easy to route:

- Put always-on repository rules in this file or a subtree `AGENTS.md`.
- Put path-scoped operational rules in `.agents/instructions/**/*.instructions.md`.
- Put repeatable workflows in `.agents/prompts/**/*.prompt.md`.
- Put context-isolated roles in `.agents/agents/**/*.agent.md`.
- Put domain-specific, opt-in knowledge in `.agents/skills/**/SKILL.md`.

Do not duplicate a detailed rule across several files. Keep one authoritative location and replace secondary mentions with references. When moving guidance, update links and validation paths in the same logical change.

## Code Quality

Fix code-quality findings that affect the touched unit or its direct dependencies before completing the task. For unrelated findings, create or cite a follow-up instead of expanding the current change. Keep edits scoped to the owning module, crate, prompt, instruction, or skill.

## Response Contract

Final responses should name what changed, where validation ran, and what risk or blocker remains. For reviews, lead with findings ordered by severity. For implemented work, keep the summary compact and include the validation evidence that supports completion.
