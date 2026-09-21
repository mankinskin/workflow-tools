# Goal

Add a pre-session bootstrap hook that establishes the session and its authoritative
worktree from the submitted prompt, then injects the resolved working directory into
the agent context so all subsequent implementation, validation, and board check-in
operations run from the assigned worktree.

This is the fourth rollout slice of the default worktree-backed session workflow
(spec `2860a8db-0c4e-4e94-984a-c10a72a67ffc`). It builds on the completed
infrastructure (`e2189e9d`) and guidance (`326bfe38`) slices and depends on the
two newly opened prerequisite tickets that make `session-api` callable from a hook.

## Problem / Current State

- The completed slices made `session-api` authoritative for worktree assignment and
  documented the worktree-first startup order, but nothing automatically performs the
  session check-in. An agent must manually invoke the (library-only) check-in flow.
- Hooks today only fire on `PreToolUse`, `PostToolUse`, and `Stop`
  (`.github/hooks/hooks.json`). There is no startup/prompt-time hook that can read the
  prompt intent and bootstrap a session before reasoning begins.
- A hook cannot relocate the already-running agent process; it can only allocate the
  session/worktree and surface the path via `hookSpecificOutput.additionalContext`
  (the same mechanism `tools/agent-hooks/terminal-pwd.sh` already uses).

## Scope

- Add a `UserPromptSubmit` (prompt-time) bootstrap hook script under
  `tools/agent-hooks/` that:
  - parses the ticket id / intent from the submitted prompt,
  - calls the `session-api` check-in surface (via `session-cli`/`session-mcp` from
    `f76b0fa9`) to obtain the authoritative worktree path, branch, and allocation mode,
  - respects the spec reuse-vs-rotation contract (rotation is the default on handoff),
  - emits `hookSpecificOutput.additionalContext` carrying the resolved worktree path
    plus an instruction to operate from that directory.
- Wire the new hook into the root hook configuration (`.github/hooks/hooks.json` and
  the synced `.clinerules/hooks/hooks.json`) alongside the existing reminders, without
  replacing or removing them.
- Document the launcher boundary: the hook performs allocation + context injection;
  a separate launcher (e.g. opening the worktree as a VS Code window or spawning a CLI
  agent with `cwd = worktree`) is required for true process-level placement and is
  out of scope here.

## Non-goals

- Building the `session-cli` / `session-mcp` surfaces themselves (tracked by `f76b0fa9`).
- Fixing session store-root resolution relative to tool execution (tracked by `cf4d1e1a`).
- Implementing a window/process launcher that re-roots the agent into the worktree.
- Allocating worktrees or transferring board ownership inside the hook independently of
  `session-api` (the spec forbids silent allocation / ownership transfer in hooks).

## Acceptance Criteria

1. A prompt-time (`UserPromptSubmit`) bootstrap hook script exists under
   `tools/agent-hooks/` and is referenced from the root hook configuration alongside the
   existing `PreToolUse`/`PostToolUse`/`Stop` hooks.
2. The hook reads the prompt, resolves ticket/owner context, and calls the `session-api`
   check-in surface to obtain the authoritative worktree path, branch, and allocation
   mode — it does not synthesize or allocate the worktree path itself.
3. The hook injects the resolved worktree path through
   `hookSpecificOutput.additionalContext` so the agent is instructed to operate from
   that directory.
4. Reuse vs rotation follows the spec contract, with rotation as the default handoff path;
   the hook never silently transfers board ownership.
5. The existing hook reminders remain configured and functional; a focused simulated hook
   invocation proves the bootstrap path resolves and injects a worktree path for a
   representative prompt.

## Dependencies

- `f76b0fa9` — session-cli / session-mcp subcommands (the callable surface the hook shells out to).
- `cf4d1e1a` — workspace-relative session store-root resolution (so the hook, run from repo root, writes/reads the correct `.memory-api`).

## Traceability

- Spec: `context-engine/session-worktree-default-workflow` (`2860a8db-0c4e-4e94-984a-c10a72a67ffc`)
- Tracker: `b6af9f40-e1f7-4f68-92e7-0a063a4ac020`
- Builds on completed slices: `e2189e9d` (infrastructure), `326bfe38` (guidance/hooks)