# Re-import current guidance files as canonical rule bodies

## Context

The rule stores still hold rule bodies for surfaces whose targets were deleted:

| `file_kind` | Count |
| --- | --- |
| `.agent` | 73 |
| `.prompt` | 17 |
| `copilot-instructions` | 6 |
| `.skill` | 1 |

No target consumes them, and the corresponding files have been hand-edited since. These 96 bodies are **drifted**, not merely orphaned. Reversing direction means reconciling content, not just restoring YAML.

## Decision

Current hand-owned files are the source of truth. Stale bodies are retired, not merged.

## Scope

- Use `rule import-file` (`import_markdown_blocks`, `MarkdownImportOptions`) to re-import from the live files: `.agents/instructions/**` (41), `.agents/agents/*.agent.md` (16), `.agents/prompts/*.prompt.md` (24), `.agents/skills/*/SKILL.md` (11 with frontmatter), root `AGENTS.md`, `.github/copilot-instructions.md`.
- Transition the 96 superseded rule entries to `deprecated` rather than deleting them, preserving `history.ndjson`.
- Preserve stable rule IDs where a body demonstrably corresponds to an existing entry, so feedback counts and provenance survive.
- Produce a reconciliation report: imported / re-linked / deprecated counts per surface.

## Blockers to handle

The worktree currently has uncommitted hand-edits in `.agents/agents/handoff.agent.md`, `.agents/agents/iteration.agent.md`, `.agents/instructions/orchestration/model-routing.instructions.md`, `.agents/instructions/orchestration/orchestrator-delegation.instructions.md`, `.agents/prompts/iteration.prompt.md`, plus untracked `model-prices.instructions.md` and `sync-model-prices.prompt.md`. Import must happen from a clean, committed tree or those edits are silently baked in.

## Acceptance criteria

1. Every in-scope file has a corresponding set of rule entries whose concatenated bodies reproduce the file content.
2. No `adopted` rule entry lacks a consuming target after Phase 2 lands.
3. The 96 drifted entries are `deprecated` with history intact.
4. The reconciliation report is attached to this ticket.
