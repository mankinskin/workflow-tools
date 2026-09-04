---
description: "Use whenever asking the user a question (interviews, reviews, escalations, clarifications). Defines what makes a question answerable: self-contained framing, explicit references, one decision per question, concrete options, and a verifiable answer."
applyTo: ".agents/prompts/**,**/*.md"
---

## Question Quality

A question is only useful if the reader can answer it **without reconstructing the context you already have**. The reader has not read the files you read, does not remember the ticket ids you loaded, and is not tracking the reasoning chain that produced the question. Every question must carry its own context; it must not require the reader to decode an internal term before they can answer.

The failure mode this prevents: questions that are technically precise but depend on unstated context ("Should the resolver normalize before matching?"), forcing the reader to guess what is being asked, or to answer a question they interpreted differently than intended.

## The Six Requirements

Every question asked of the user must satisfy all six:

1. **Self-contained.** The question plus its message body must be answerable on its own. Do not depend on the reader having read the surrounding transcript, a prior tool result, or a file. Restate the relevant fact inline, even if you stated it earlier in the session.
2. **Explicit references.** Follow the general entity disambiguation protocol in [entity-disambiguation.instructions.md](entity-disambiguation.instructions.md) — no bare pronouns, first-mention establishment, clickable references per the Clickable Reference Policy in `AGENTS.md`. Question-specific addition: when the question is about code behavior, quote or paraphrase the exact current behavior and link the line range.
3. **One decision per question.** Each question resolves exactly one choice. If answering requires two independent decisions, split it into two questions. Compound questions produce ambiguous answers.
4. **Concrete options with consequences.** Where the decision space is bounded, offer explicit options and state what each one causes. Prefer "A: reject the call (current behavior, safe but noisy) / B: normalize then match (accepts `Claude Sonnet 5 (copilot)`)" over "how should we handle model names?".
5. **Verifiable answer.** The answer must be checkable later — a behavior, a threshold, a state, a file, a yes/no. If any plausible answer would leave you unable to write an acceptance criterion, the question is not specific enough yet.
6. **Ground unfamiliar terms.** Before asking about a repository-specific or abstract term, define it in plain language from the current local behavior and show one short concrete example. State which persisted record, validation result, or user-visible behavior each option changes. Do not ask the user to choose a model label they have not been given enough context to evaluate.

## Before Asking: The Reader Test

Before sending a question, reread it as someone who has seen none of your tool output. Ask:

- Would they know **which** thing is being asked about, without inference?
- Would they know **what currently happens**, so they can judge the proposed change?
- Would they know **what changes** for each answer they could give?
- Could you turn each possible answer directly into a ticket, spec line, or acceptance criterion?
- If the question uses a term such as "gate", "evidence", "resolver", or "policy", did you define it and show one local example before asking the user to choose its behavior?

If any answer is no, rewrite the question before sending it.

## Structure

Use the message body to supply context; keep the question line short and single-purpose.

- **Question line:** one sentence, the decision only.
- **Message body:** current behavior (with reference), why it needs a decision, and what each option implies.
- **Options:** concrete and mutually exclusive; mark a recommended default when one exists, and say why it is recommended.

## Do the Reading First

Never ask the user for something the repository can answer. Resolve it yourself, then ask only about the part that requires human judgment — intent, tradeoffs, priorities, and acceptance thresholds. When you have already determined the current behavior, state it as fact in the question rather than asking the user to confirm what the code says.

## Ground Options in Findings

Offered options and example answers must be derived from what you actually found — the real current behavior, the real entities, the real constraints — and must serve the stated objective of the exchange. Generic options the reader cannot map onto something concrete in this repository are noise, and they push the reader into rewriting the question for you.

## Anti-Patterns

- Bare identifiers: "Should `<ticket-short-id>` block the release?" — name and link the ticket.
- Context-dependent deixis: "Should we do the same here?" / "Is that acceptable?" — restate the subject.
- Compound decisions: "Should we normalize model names and also relax the rejection policy?" — split.
- Unstated current behavior: asking for a change without saying what happens today.
- Open-ended dumps: "Any thoughts on the handoff format?" — propose options.
- Jargon without expansion: internal type or module names used as if the reader tracks them.
- Undefined abstractions: asking how a "gate" or "evidence" should behave before explaining what it checks or records in the current component.
- Unverifiable answers: "Should this be more robust?" — define the observable behavior instead.

## Rewrite Examples

Weak: "Should the cost gate be more lenient with model names?"

Strong: "When a caller sends `Claude Sonnet 5 (copilot)` but the price table id is `claude-sonnet-5`, should the gate reject the call or normalize the string and match?"
Body: today `resolve_output_mtok` does exact match then case-insensitive substring match, so the parenthetical suffix makes both directions fail and the call is rejected. Option A keeps rejection (unknown models never slip through, but harmless formatting differences break tool calls). Option B strips a trailing parenthetical qualifier and normalizes separators before matching (formatting differences pass; a genuinely unknown model is still rejected).

Weak: "Is the handoff good enough?"

Strong: "Must a handoff package contain an explicit ordered list of next steps before it can be accepted, or is prose describing remaining work sufficient?"

## Cross-References

- Escalation over inline clarification: [.agents/instructions/orchestration/escalation-gate.instructions.md](.agents/instructions/orchestration/escalation-gate.instructions.md)
- Phase separation: [.agents/instructions/orchestration/phase-separation.instructions.md](.agents/instructions/orchestration/phase-separation.instructions.md)
- Reference formatting: [AGENTS.md](../../../AGENTS.md)
