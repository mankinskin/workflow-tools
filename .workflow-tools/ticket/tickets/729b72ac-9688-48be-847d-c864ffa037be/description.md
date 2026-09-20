## Problem

No guidance file documents how to author session workflows, so agents rediscover the node-kind model and URN rules by trial and error (session `aedf210d`, turns 60→63). The Review Agent mode even instructs per-criterion `session_workflow_add_node` calls while the schema fights that usage.

## Requirement

Add a path-scoped instruction file that teaches correct, flexible session-workflow authoring and documents the known rejections + their fixes.

## Acceptance criteria

1. New file `.agents/instructions/session-workflow.instructions.md` with frontmatter `applyTo` covering session workflow tool usage (e.g. `.agents/prompts/*.prompt.md`, session-api/session-mcp paths, and agent modes that drive workflows).
2. Documents the closed `kind` enum (ticket/validation/spec/task) AND the `category` free-text escape hatch for custom labels.
3. Documents URN rules: gating URNs (`ticket_urn`/`spec_urn` on their kinds) vs. the non-gating `anchor_urn` for referencing entities from any node.
4. Documents batch `add_nodes`/`add_edges` usage and encourages many-link graphs.
5. Includes a "known rejections → fix" table mirroring the self-correcting error messages, and a worked canonical example: persisting review criteria as `task` nodes with `category="review-criterion"` anchored to the ticket, plus edges.
6. Cross-linked from AGENTS.md canonical-sources or session-bootstrap so it is discoverable.

## Dependencies

Content must match the final field names/messages from the errors, batch, and schema tickets; sequence this after those land (or keep it in lockstep).