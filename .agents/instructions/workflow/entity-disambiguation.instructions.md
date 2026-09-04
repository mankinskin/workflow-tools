---
description: "Use in every agent response, not only interview questions: the general entity disambiguation protocol. Covers first-mention establishment, proximity shorthand, context-reset re-establishment, and the ban on bare pronoun references to entities."
applyTo: "**"
---

## Entity Disambiguation Protocol

Ambiguous references ("this", "that", "the engine", "it") force the reader to reconstruct which entity is meant from surrounding context. This protocol applies to all agent responses — chat narration, status summaries, handoffs, and reviews — not only interview questions. Interview-specific reference formatting still lives in [question-quality.instructions.md](question-quality.instructions.md); link format itself is owned by the Clickable Reference Policy in [AGENTS.md](../../../AGENTS.md).

### The Three Tiers

1. **First mention — full establishment.** The first time an entity appears in a response (or after a context reset, see below), state its type, its fully-qualified name or id, and a short summary or tag. Use a clickable reference per the Clickable Reference Policy where the entity has one (ticket, spec, doc, log).
   - Template: "Ticket [<ticket-short-id> <ticket-title>](.ticket/tickets/<ticket-uuid>/ticket.toml) (component <component>, priority <priority>) is open with no dependencies."
2. **Proximity — short id or name only.** Within the same turn or the immediately following turns, while still discussing the same entity, a short id or name is sufficient. Do not re-establish type and summary every sentence.
   - Template, continuing the same turn: "<ticket-short-id> has no linked spec yet."
3. **Context reset — re-establish.** When the conversation switches file, step, task, or subject, treat the next reference to a previously-mentioned entity as a first mention again: restate type, id, and summary before using shorthand.
   - Template: after a detour into a different ticket or file, "Back to ticket <ticket-short-id> (<ticket-title>, <component>): the updated instructions file is in place."

### The Rule

No bare pronoun ever refers to an entity (ticket, spec, instruction file, doc, log, ref, agent, tool). If a sentence would read "it", "this", or "that" where an entity is meant, replace the pronoun with the short id or name established under tier 1 or 2.

- Bad: "This needs a spec before it can move to in-review."
- Good: "Ticket <ticket-short-id> needs a spec before it can move to in-review." (or, in proximity, "<ticket-short-id> needs a spec...")

- Bad: "That file already bans ambiguous pronouns, but only for questions."
- Good: "[question-quality.instructions.md](question-quality.instructions.md) already bans ambiguous pronouns, but only for interview questions."

- Bad: "The engine handles that during insert."
- Good: "context-engine's insert crate handles atom deduplication during insert." (or, in proximity after first mention, "insert handles it" is still a pronoun — use "insert handles the dedup" instead)

Pronouns are fine for the reader ("you"), the agent itself ("I"), or grammatical subjects that are not entities (generic "it" in "it is important that..."). The ban is scoped to references that stand in for a specific ticket, spec, file, tool, or other named entity.
