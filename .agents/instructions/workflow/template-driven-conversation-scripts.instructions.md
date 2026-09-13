---
description: "Use when turning a document template plus situational context into a natural conversation script or plan for complex conversations. Covers the analyze → structure → contextually-adapt → isolated-humanize → validate flow, tool/core-agent boundaries, persona layering, and content-preservation checks."
applyTo: "**"
---

# Template-Driven Conversation Scripts

Use this workflow when a template or playbook must be made into a natural,
context-aware conversation script or execution guide for a sensitive or complex
conversation: de-escalation, negotiation, sales, onboarding, HR-sensitive
situations, or any high-stakes discussion where the conversation must still sound
human and tailored without inventing facts.

This guidance complements the structural pipeline already implemented in
[`skriptformator/backend/src/combinator_agent/latent.rs`](../../../../skriptformator/backend/src/combinator_agent/latent.rs),
[`skriptformator/backend/src/combinator_agent/generation.rs`](../../../../skriptformator/backend/src/combinator_agent/generation.rs),
and [`skriptformator/backend/src/combinator_agent/execution/prompts.rs`](../../../../skriptformator/backend/src/combinator_agent/execution/prompts.rs).
The code proves the repository already has a strong structural pass: analyze the
source, carry source facts, derive stages, and generate artifact sections in
order. This workflow adds the missing human-facing layer: a bounded
humanization pass that preserves content and structure while adapting tone to
context.

## The canonical pattern

The required sequence is:

1. Analyze
2. Structure
3. Contextually adapt
4. Isolated humanize
5. Validate

Do not skip the stages or merge the humanization pass into the analysis or
structural generation stage. The structure is intentionally split so the model
can do factual preparation once, then apply human wording separately without
mixing content decisions with style decisions.

## Stage 1: Analyze

Read the template, the context, the customer or participant profile, and the
conversation goal as separate evidence sources. Extract the facts that are
explicitly present and record which items are still missing or inferred.

Required behavior:

- Recognize the conversation template's stage order and its required sections.
- Detect output language and domain without translating content.
- Carry source evidence separately from adapted wording.
- Preserve an explicit record of which statements came from the template and
  which came from the situational context.

This is the stage the latent representation in
[`latent.rs`](../../../../skriptformator/backend/src/combinator_agent/latent.rs)
formalizes: title, domain, language, stages, customer context, motivations,
questions, objections, tone, source spans, and artifact declarations.

## Stage 2: Structure

Convert the raw material into a staged, ordered, checkable plan.

Required behavior:

- Keep the template's ordering intact unless an explicit rule requires a
  different sequence.
- Split the content into sections or stages that map to the actual call flow.
- Preserve the exact set of required headings or template sections and never
  invent additional sections.
- Keep a strict artifact contract: the generation layer validates that the
  produced section list matches the ordered template stages.

The repository's generation validation logic in
[`generation.rs`](../../../../skriptformator/backend/src/combinator_agent/generation.rs)
shows the expected rule: a generated artifact must match the declared section
order exactly and reject invented or reordered headings.

## Stage 3: Contextually adapt

Apply participant context to the structure without changing the facts.

Required behavior:

- Map customer-specific goals, constraints, motivations, objections, and known
  information onto the correct stage.
- Adapt the wording of questions and prompts to the customer and situation.
- Maintain the original intent, decision path, and supportable claims.
- Distinguish `source` facts from recommendations or framing choices.

A change is allowed here only when it is directly grounded in the source
material or the provided context. The structural model may produce a tailored
plan, but it must not invent missing facts or replace the template's core
claims with new, unsupported ones.

## Stage 4: Isolated humanize

This is the separate human-language pass. It must run after structural planning
and adaptation, never before. The humanizer should be constrained to style,
flow, and naturalness, not to deciding what the conversation says.

Required behavior:

- Rewrite for tone, natural speech, pacing, empathy, and clarity.
- Use optional variation in phrasing and rhythm without changing the meaning.
- Prefer a conversational cadence appropriate to the scenario: respectful,
  calm, direct, or assertive based on the context.
- Keep the main content, decision structure, and factual commitments unchanged.
- If a humanized version changes meaning, it is invalid and must be rejected.

### Tool vs core-agent responsibility boundary

| Responsibility | Allowed to do | Must not do |
|---|---|---|
| Tool / prompt orchestration | parse the template, detect language, carry source facts, assemble ordered stages, emit structured output, check schema contracts | invent new facts, reorder key stages, rewrite strategic meaning |
| Core agent / model pass | interpret context, adapt the plan to the customer, draft natural language, vary phrasing, add empathy and flow | change facts, invent missing claims, override constraints, add unsupported recommendations |

The humanization pass is a core-agent concern. The tool layer arranges the
schema, ordering, and validation; the model layer gives it natural speech and
contextual tailoring. Do not give the humanization pass authority to change the
core facts or to widen the scope of the conversation.

### Persona layering

When a prototype or script needs a persona, layer it as a style overlay, not as a
content source.

Allowed:

- calm and empathetic tone
- direct but respectful phrasing
- concise or detailed pacing by stage
- natural speech variation in openers and transitions

Not allowed:

- adding new motivations, objections, or constraints that are not in the input
- giving the participant a different problem or decision than the actual source
- turning a factual statement into a fabricated promise or commitment

### Few-shot before/after pattern

Use a minimal before/after example to teach the boundary clearly.

Example:

Before (structural, unhumanized):

> Opening: confirm the customer's goals. Ask whether they want a lower-risk
> setup and explain the decision timeline.

After (contextually adapted and humanized):

> "I want to make sure I understand what matters most to you here. If we move
> forward, are you looking for the lower-risk option, or would you rather keep
> the timeline flexible while we work through the trade-offs?"

The before/after pair shows the same content preserved while the delivery shifts
from a schema to natural speech. The humanization pass may change rhythm and
wording, but it must not change the decision or objective.

### Optional natural variation

When the workflow supports optional variation, allow the humanizer to choose a
reasonable speech cadence or tone variation, but keep it scoped to the same
structure and content. For example:

- gentle and collaborative
- direct and time-aware
- calm and reassuring
- confident and concise

This is a style variant, not a content redefinition.

## Stage 5: Validate

Validation is mandatory. The generated output must be checked against the source
material and structural contract, not only against a subjective impression of
"naturalness."

Required checks:

- confirm requested sections exist in the correct order
- confirm no invented headings, facts, or claims were introduced
- confirm the humanized language preserves the source intent and decision path
- confirm the output stays within the declared artifact format and schema
- confirm any optional spoken variation remains bound to the same content
- confirm source facts remain traceable to the template or context

The repository's validation behavior is intentionally structural and evidence-
anchored: a generation stage rejects invalid JSON and invalid ordering, and the
final artifact keeps source facts attached so the output can be audited.

### Content-preservation rule

The humanization pass must preserve meaning and factual content before any
judgment about "naturalness." Treat this as a structural/content-diff check:

- same stage order
- same decision objective
- same commitments and constraints
- same facts represented, with wording adjusted only for flow

If the humanized output changes the commitment, adds unsupported context, or
reorders the decision path, it fails validation. Do not treat subjective
quality as a replacement for this evidence-based test.

## Integration with repository guidance

This pattern is aligned with the wider guidance system:

- [`trace-artifact-analysis.instructions.md`](../workflow/trace-artifact-analysis.instructions.md)
  treats the trace and generated artifact as separate evidence surfaces that
  must be read back together.
- [`shared-context-bundle.instructions.md`](../workflow/shared-context-bundle.instructions.md)
  requires self-contained, compact compiled prompts rather than raw dumps.
- [`error-boundary-handling.instructions.md`](../workflow/error-boundary-handling.instructions.md)
  keeps content stage boundaries and user-visible failures clear.
- [`evidence-grounded-refinement.instructions.md`](../workflow/evidence-grounded-refinement.instructions.md)
  requires critique and revision to be grounded in the actual artifacts and
  source material.

## Implementation note

This document is intentionally guidance-only. It does not modify the Rust
implementation or create a new ticket/spec; it clarifies the correct workflow for
future template-to-script generation and mirrors the already-working
structural pattern in the repository.
