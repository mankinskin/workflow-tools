---
description: "Use whenever a dispatched sub-agent completes a requested unit. Defines the single terminal deliverable, blocker handling, and evidence-backed success claims."
applyTo: "**"
---

## Terminal Delivery

A dispatched sub-agent returns exactly one terminal message. The terminal message
delivers every requested report section and does not ask questions, offer options,
or invite further work. The dispatched agent's output schema controls the headings
and details of the report.

When the caller supplies a character or line limit, that limit is binding. The
sub-agent MUST reserve at least 10 percent headroom and check the response length
before sending. A spilled response, a progress announcement without the terminal
deliverable, or an invitation to continue is a failed return-contract result.

## Blocked Items

For every requested item the dispatched sub-agent cannot complete, write a
top-level `BLOCKER: <specific reason>`. Missing tools, refused permissions, and
mode restrictions such as read-only access are blockers, not partial success.

A blocker for one item never omits the remaining requested sections. Complete each
unblocked section, then state the blocked item and its exact reason.

When every requested item is blocked, return only the required status, the
specific blocker, and the strongest evidence pointer allowed by the caller's
schema. Do not precede the terminal blocker with a narrative pre-flight report.

## Verified Claims

Never report a step as successful without command output confirming the result.
Unverified success is fabrication, not an estimate. When confirmation is missing,
report the affected item as a blocker and deliver every other requested section.

This contract supplies the terminal deliverable required by
[write-and-die.instructions.md](write-and-die.instructions.md).