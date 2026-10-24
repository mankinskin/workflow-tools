# Workflow-tools instruction routing

This is a compact, description-gated index for the canonical workflow-tools
corpus. Start with [AGENTS.md](../../AGENTS.md), then read the named
instruction only for the matching task; this table does not auto-attach bodies.

| Task category | Canonical area |
| --- | --- |
| Repository, dependency, generated-file, commit, or submodule work | [repository/](repository/) |
| Planning, implementation, delegation, dossier, prototype, audit, graph, or workflow operations | [workflow/](workflow/) |
| Audit CLI and findings | [audit](../../audit/.agents/instructions/audit/audit.instructions.md) |
| Memory-kernel and context-engine crates | [memory-kernel](../../memory-kernel/.agents/instructions/engine/) |
| Session identity, handoff, and worktree operations | [session](../../session/.agents/instructions/) |
| Specification lifecycle | [spec](../../spec/.agents/instructions/spec/spec-system.instructions.md) |
| Test design, execution, and validation evidence | [test](../../test/.agents/instructions/testing/) |
| Ticket lifecycle, board, and engine work | [ticket](../../ticket/.agents/instructions/ticket/) |
| Viewer API and frontend work | [viewer-api](../../viewer-api/.agents/instructions/frontend/) |

Use the precise file description within an area to choose the owner; do not
infer relevance from a generic extension or Markdown path.
