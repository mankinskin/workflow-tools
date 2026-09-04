---
description: "Create a short, reference-centric session handoff prompt and create or match the ticket or tracker follow-up items needed for the handoff track."
name: "handoff-tickets"
argument-hint: "[ticket-id|track|current]"
agent: "agent"
---

# Handoff Tickets

Create a compact handoff prompt for a new session and formalize the handoff track through the ticket workflow when needed.

Reference [AGENTS](../../AGENTS.md), [session-optimization instructions](../instructions/session/session-optimization.instructions.md), [ticket](./ticket.prompt.md), [tickets](./tickets.prompt.md), [ticket-next](./ticket-next.prompt.md), [ticket-system instructions](../instructions/ticket/), [rule-target](./rule-target.prompt.md), [ticket-cli](../../memory-api/tools/cli/ticket-cli/README.md), [ticket-mcp](../../memory-api/tools/mcp/ticket-mcp/README.md), and [audit-cli](../../workflow-tools/audit/crates/audit-cli/README.md).

## Workflow

1. Read the slash-command text and determine the handoff track, current implementation slice, and whether new ticketing is needed.
2. Search existing tickets first per [workflow.instructions.md#discovery-before-creating](../instructions/ticket/workflow.instructions.md#discovery-before-creating) so the handoff flow reuses or updates the authoritative ticket set.
3. Inspect the current board, spec, validation, implementation references, session-audit evidence, and any persisted cross-session history needed to describe the track accurately.
4. Produce a short, paragraph-style, reference-centric handoff prompt for a new session.
5. If the current track is not already represented well enough in the ticket graph, create or refine the needed ticket or tracker ticket items:
- create one actionable ticket when the handoff points to one concrete next slice
- create a tracker ticket when the handoff needs a parent work package spanning multiple follow-up slices
- link or reference existing tickets when they already cover the work
6. Keep the ticketing workflow minimal and explicit:
- avoid duplicates
- keep titles scoped to the handoff track
- preserve canonical ticket paths and references
7. In the handoff paragraph, mention the ticket or tracker references that the next session should open first, plus any board check-in, check-out, or stale-entry issue that must be resolved before implementation resumes, and call out session-audit selector or schema-version guidance when it affects the restart path.
8. When persisted `session-api` history captured by the Stop hook materially improves the restart path, reference that history alongside the ticket and board pointers rather than restating the whole prior conversation.
9. Treat transcript history as diagnostic evidence for future prompt quality rather than as a prompt payload to replay:
- prefer concise findings about repeated tool chatter, oversized outputs, and routine-action reasoning
- keep the next-session handoff focused on the durable work state and the next concrete action
10. Ticket references are strict:
- use exact full ticket UUIDs for all ticket mentions in the handoff output
- do not use shorthand-only ticket references unless they are resolved in a legend
- if shorthand labels are used for readability, include an explicit ticket legend mapping shorthand label -> full UUID + canonical ticket title
11. Shorthand and placeholder declarations are mandatory and must appear at the top of the handoff output before the overview:
- include a `Shorthand And Placeholder Legend` section near the top
- define every shorthand token and placeholder used later in the handoff (for example `T1`, `EPIC`, `SPEC-A`, `SESSION-X`, `<workspace>`)
- if none are used, explicitly say `None used.`
12. Do not implement the work in this prompt; stop after producing the handoff and the ticketing setup.

## Response

Return:
- the full handoff response inside a fenced plain-text block using `~~~text` and closing `~~~` so copied output preserves literal markdown links
- render all entity references per the Clickable Reference Policy in `AGENTS.md`
- the short handoff prompt in one paragraph
- created or matched tickets, rendered as canonical markdown links when available
- all file references must use markdown links with forward slashes only
- for files in the current directory, use `./`-prefixed links (for example `[./AGENTS.md](./AGENTS.md)`)
- do not emit bare file paths or Windows-style backslashes
- strict ticket references using full UUIDs
- a `Shorthand And Placeholder Legend` section near the top that defines all shorthand/placeholders used later in the handoff, or `None used.` when none are introduced
- a ticket legend section mapping any shorthand labels used in the handoff to exact full UUID + canonical ticket title
- whether a tracker ticket was needed and why
- any board or persisted-session note that affects how the next session should resume
- the ordered next-action list for the new session with the instruction to continue until all actions are completed or explicitly blocked
- a follow-up completion status section covering each action (`not-started`, `in-progress`, `completed`, or `blocked`) with a short blocker or outcome note
- no unresolved references: every shorthand, ticket id, spec id, session id, rule id, or file mentioned in the handoff body is defined in the legend sections or linked in key entities
