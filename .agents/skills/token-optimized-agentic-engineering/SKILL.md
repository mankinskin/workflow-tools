---
name: token-optimized-agentic-engineering
description: Use when an autonomous coding agent needs to maximize efficiency, minimize token cost, and preserve context-window capacity — covers token discipline, prefix-cache preservation, state externalization, forced compaction, surgical tool I/O, and multi-tier model routing.
---

# Token-Optimized Agentic Engineering

Integrate this skill when you want autonomous coding agents to maximize efficiency, minimize token cost, and preserve context window capacity.

## 1. Core Principles

- **Token discipline**: Balance logical precision with minimal token usage in every interaction. Uncontrolled context growth drives input cost toward $O(N^2)$.
- **Prefix cache preservation**: Keep static system components fixed at the start of the payload. Any mutation in early context invalidates cache reuse.
- **State externalization**: Chat history is ephemeral. The real project state and active plans must live in persistent workspace files.

## 2. Operational Rules

### Communication and Syntax

- **No prose padding**: Remove pleasantries, introductions, transitions, and apologetic explanations.
- **Minimal syntax**: Use ultra-compact imperative status lines and tool messages. Prefer compact formats such as TOON or other dense encodings over verbose JSON/XML when available.
- **Example**:
  - Wrong: `I analyzed the problem. The auth controller has a duplicate import on line 14. I will remove it now.`
  - Right: `AUTH_CTRL: Dup import line 14. Action: Rm line 14.`

### Context and File Management

- **Forced compaction**: Trigger a compression pass once active context exceeds 30% of the model limit.
- **History distillation**: Replace earlier multi-turn dialogue with a single consolidated status block:

```text
[CONSOLIDATED HISTORICAL SUMMARY]
- TASK: Refactor database connection pool.
- STATE: Finished pg_pool init. Completed migrations 01 & 02.
- RESOLVED: Fixed timeout memory leak.
- NEXT: Optimize idle connection pruning.
```

- **Selective compression**:
  - **Leave intact or externalize**: code definitions and system logs with high specificity.
  - **Compress aggressively**: stack traces and narrative dialogue with high redundancy.

### Tool I/O and Workspace Architecture

- **Externalize output**: Never dump large tool output such as test logs or file trees into chat. Redirect to transient files like `.agent/scratchpad/tool_out.log`.
- **Query surgically**: Read those logs with focused filters such as `grep` or `sed`, not by loading the full file.
- **Use pointer references**: Pass structural anchors or file IDs instead of reinjecting full code blocks.
- **Apply localized edits**: Prefer differential edit tools such as patch-based updates over full-file rewrites.
- **Cap inspection**: File-read tools should default to a hard limit such as 150 lines. Larger reads require explicit ranges.
- **Skeletonize structure**: For dependency analysis, use views that strip function bodies and return only signatures, classes, and interfaces.

### Workspace Infrastructure and State Persistence

- **Static repo maps**: Keep a compressed root map such as `repo_map.toon`, refreshed by `peek --repo-map`, to avoid expensive tree walks.
- **Externalized instructions**: Move coding standards and business logic into `.agent/instructions.md`, then access them via line ranges or targeted search.
- **Error boundary at write time**: Integrate syntax checking into file-write boundaries. Reject invalid code before it enters an expensive debug loop.
- **Externalized progress**: Store plans and to-dos in `.agent/plan.md` so chat can be reset without losing working state.

### Multi-Tier Model Routing

- **Utility tier (low cost)**: Route simple diagnostics, formatting, and small code additions to cheaper fast models.
- **Premium tier (high cost)**: Reserve top-tier models for architecture decisions, cross-module refactors, and algorithmic regressions.
