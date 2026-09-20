# Purpose

Gather all worktree provisioning, worktree tooling, and session-to-worktree assignment work under one tracker.

# Current state

Automatic provisioning works, but three distinct failure modes were observed on 2026-08-08: shared-config `core.worktree` hijack (`723c2bea`); prune destroying a live submodule registration (the linked critical tooling bug); and root-only operations run from inside a worktree (the linked high-priority tooling feature).

Renaming a worktree to a topic slug is documented but has no tooling (`72314c5e`) and no session-store support (`ff83caf7`).

# Themes

- Provisioning and the capture hook.
- Worktree tooling and the Rust rewrite.
- Session-to-worktree discovery and assignment.
- Board and MCP resolution against the active worktree.
- Agent guidance.

# Historical context

Tickets in `done` state are linked for historical context only and are not open work.