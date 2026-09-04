---
description: "Research a topic, interview the user, and turn the result into specification-book updates."
name: "interview"
argument-hint: "<topic>"
agent: "agent"
---

# Specification Interview

Create an interview with the user about a specific topic so the specification book can be updated with clearer requirements, goals, and acceptance criteria.

Reference [spec-cli](../../memory-api/tools/cli/spec-cli/README.md), [spec-mcp](../../memory-api/tools/mcp/spec-mcp/README.md), [ticket-cli](../../memory-api/tools/cli/ticket-cli/README.md), and [ticket-mcp](../../memory-api/tools/mcp/ticket-mcp/README.md).

## Workflow

1. Treat the slash-command text as the interview topic.
2. Search existing specs and related tickets before asking questions so the interview starts from current repository knowledge.
3. Summarize the current known state briefly:
- the closest matching spec or gap in the spec book
- related tickets or implementation surfaces
- unresolved requirements that matter for the next update
4. Ask concise, decision-driving interview questions. Each question must be self-contained (answerable without reading the transcript or any file), name and link every entity it refers to instead of using pronouns or bare ids, resolve exactly one decision, offer concrete options with their consequences, and have an answer you can turn directly into an acceptance criterion. See [question-quality.instructions.md](../instructions/orchestration/question-quality.instructions.md).
5. Prefer questions that refine:
- goals and non-goals
- acceptance criteria
- edge cases and operator expectations
- evidence or validation requirements
6. After the interview, propose the exact spec changes or sections that should be updated.
7. Create or update tickets only when the user asks or when a missing implementation/planning ticket is clearly required by the agreed scope.

## Response

Return:
- topic and current spec anchor
- questions asked or still needed
- confirmed answers from the user
- proposed spec updates and any required ticket follow-up
