---
description: "Use in every agent response, not only when something breaks: the end-of-turn feedback visibility requirement. Covers when to log a deferred feedback entry versus fixing immediately, and the mandatory feedback_session_summary check before closing a response."
applyTo: "**"
---

## Purpose

The feedback store (`feedback-mcp`) exists to capture unexpected tool or tooling behavior for later resolution, but it is rarely used in practice because nothing prompts an agent to call it. This instruction closes that gap with two rules: when to log a feedback entry, and a mandatory visibility check before closing a response so logged entries are never silently forgotten.

## When To Log A Feedback Entry (the defer rule)

When something unexpected happens during a turn — a tool behaves differently than assumed, a command produces a surprising result, an instruction turns out to be wrong or incomplete for the situation — and no immediate fix is being attempted, call `feedback_ingest` (source `agent`, an appropriate `rating`, and a `note_text` describing the finding) instead of silently continuing or only narrating the surprise in prose.

This mirrors the existing in-scope/out-of-scope split in [code-quality.instructions.md](code-quality.instructions.md): the same two-branch shape applies here, generalized from code-quality findings to any unexpected tool/tooling behavior.

| Situation | Required action |
| --- | --- |
| A fix is explicitly requested by the user or is otherwise required to complete the current task | Fix it now. Do not only log a feedback entry in place of the requested fix. |
| Something unexpected happens and no fix is being attempted right now | Call `feedback_ingest` to record the finding (target: the relevant ticket/spec/rule/doc entity URN where one exists, otherwise the tool or file most responsible), then continue the turn. This defers resolution — either improving the tooling code or extending its instructions — to a later, separate pass. |

Do not use a feedback entry as a substitute for a fix that was explicitly requested; do not skip logging a genuine surprise just because it did not block the current turn's outcome.

## Mandatory End-Of-Turn Visibility Check

Near the end of every substantive response, derive your current turn number, then call the `feedback_session_summary` MCP tool (see [workflow-tools/feedback/crates/feedback-mcp/src/server.rs](../../../feedback/crates/feedback-mcp/src/server.rs)) with the current `session_id` and that turn number, and surface the result so new entries are visible rather than buried in the transcript.

- **Deriving the turn number.** There is no live, authoritative "what turn am I on" signal. Derive a best-effort number from the currently running session — call `session_peek_skeleton` for the current `session_id` and use its returned `total_turns` as your current turn. This is a **best-effort** number, not an authoritative capture-time stamp: state it as such if you ever report it to the user (e.g. "turn ~4", not "turn 4 exactly").
- **Stamping new entries.** When calling `feedback_ingest` to log a finding during the turn, pass this same derived number as `turn_sequence` so the entry is attributable to this turn specifically.
- **Querying.** Call `feedback_session_summary` with `session_id` and the derived `turn_sequence` to see only this turn's entries. Omit `turn_sequence` (or call it once without, once with) when you also want the whole-session count for context — both are valid; the turn-scoped call is the one this instruction mandates.
- Render the result via the feedback-summary segment of the existing Closing Traceability Footer (see [session-identity-and-handoff.instructions.md](../../../session/.agents/instructions/session/session-identity-and-handoff.instructions.md)) rather than inventing a second, separate report block.
- When the summary has zero entries, still include the footer segment showing zero — omitting it silently is indistinguishable from forgetting the check.

## Cost Discipline

This is two bounded tool calls per response (derive the turn, then query/ingest with it), not a second interview or research pass. Per [routine-actions.instructions.md](routine-actions.instructions.md), do not spend reasoning budget narrating why the check is being made — call the tools and report the result.

## Non-Goals

- This instruction does not require hook-side automatic backfill of `turn_sequence` for every entry — that remains the separate, retroactive recovery mechanism in `session-api/src/transcript_feedback/*`, unrelated to this agent-side self-reported number.
- This instruction does not change any individual `.agents/agents/*.agent.md` template — the `applyTo: "**"` scope here is the single enforcement point.
</content>
