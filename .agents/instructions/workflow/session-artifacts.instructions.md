---
description: "Use when reading prior session transcripts, handoff documents, or Copilot chat artifacts. Covers bounded session artifact inspection and durable-first reading."
applyTo: "**/*.md"
---

## Session Artifact Reading

When the task involves prior sessions, transcript inspection, handoff recovery, or Copilot chat artifacts, follow [session-optimization instructions](../session/session-optimization.instructions.md).

Rules:
- Do not read raw session transcript files into the model prompt by default.
- Prefer the smallest durable artifact first: ticket, spec, handoff, validation note, or compact session summary.
- Treat raw transcript JSON, event streams, and chat-session resource payloads as `reference-only` unless a bounded slice is required to answer one specific question.
- Do not replay raw `toolRequests`, empty `reasoningText`, duplicated tool lifecycle events, or full spill-file bodies when a one-line summary or targeted extraction is sufficient.
- When you need evidence from a prior session, normalize it to: scope, finding, outcome, blocker, and pointer.
