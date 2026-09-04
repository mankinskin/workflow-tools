---
description: "Start a surgical implementation slice from a ticket, failing behavior, file, or symbol. Anchors on one concrete target, validates immediately, and returns concise evidence."
name: "implement"
argument-hint: "Ticket id, failing behavior, file, symbol, or narrow implementation scope."
agent: "Implement Agent"
---

# Implement

Delegate to the Implement Agent for the full surgical workflow.

Use this prompt as a thin wrapper: provide the concrete target, rely on the agent contract for the detailed implementation loop, and return only the evidence-backed summary needed for the user.

Reference [AGENTS.md](./AGENTS.md) and [commit.instructions.md](../instructions/commit/) when the implementing agent needs repository-specific guardrails.
