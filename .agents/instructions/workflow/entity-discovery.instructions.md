---
description: "Use during prompt-ingestion Stage 2 research, and whenever a roadmap waypoint proposes creating a ticket, spec, validation spec, session record, feedback entry, or dossier. Covers the six-store discovery sweep, recording per-store coverage, and the three permitted reuse forms when a relevant entity already exists."
applyTo: "**/*.md"
---

## Purpose

An agent that decides how to implement a request without first looking at what
the stores already hold proposes creating entities that already exist. This
file is the single authoritative rule for that check. The `ticket` and `spec`
stores each used to carry their own copy; those sections now point here.

## When This Rule Binds

This rule binds the following planning and execution boundaries. It is
deliberately not a per-turn tax.

| Phase | Owner | Obligation |
| --- | --- | --- |
| Prompt-ingestion Stage 2 (research and artifact inventory) | [prompt-ingestion.instructions.md](prompt-ingestion.instructions.md) | Sweep all six stores and record the coverage table in `ARTIFACTS.md`. |
| Planning ticket/spec preparation after settled scope | [Planning Entities During Refinement](prompt-ingestion.instructions.md#planning-entities-during-refinement) | Cite Stage 2 evidence or refresh the relevant bounded search, record creation/reuse and planning/draft state, then read every mutated entity back by its specific id. |
| Any roadmap waypoint proposing entity creation | [roadmap-authoring.instructions.md](roadmap-authoring.instructions.md) | Cite the search that proved non-existence, or record a reuse form instead of creating. |

Outside these boundaries the rule is advisory: prefer a bounded search over an
assumption, but do not run a six-store sweep before every edit.

## The Six Stores

A discovery sweep covers all six. A store is never skipped silently — an
unreachable store is recorded as `degraded` with the error, not as "no results".

| Store | Primary surface | Fallback when the primary surface fails or does not exist |
| --- | --- | --- |
| ticket | `ticket search <terms>`, `mcp_ticket_list_tickets` | `ticket get <id>`; `rg` over `.workflow-tools/ticket/tickets/*/ticket.toml` |
| spec | `mcp_spec_spec_search`, `mcp_spec_spec_list` | `rg` over `.workflow-tools/spec/specs/*/{spec.toml,body.md}` |
| test | `mcp_test-mcp_test_list_specs`, `mcp_test-mcp_test_list_executions` | **list-only store — no full-text search exists.** List, then `rg` the listed records. |
| session | `mcp_session_session_sessions_for_ticket` | **`session query` has a known defect** that aborts the whole listing on one unreadable record (ticket `7be23bd8`). Prefer `sessions-for-ticket`; otherwise list `.workflow-tools/session/sessions/` directly. |
| feedback | `mcp_feedback-mcp_feedback_query`, `feedback_inbox`, `feedback_summary` | `rg` over the `.feedback/` store directory. |
| transcripts/ dossiers | **no search surface exists** | `find transcripts -mindepth 1 -maxdepth 1 -type d`, then `rg` and bounded heading reads. |

Do not assume a store offers full-text search. `test`, `feedback`, and the
dossier folder do not; a rule-following sweep uses list-plus-grep there.

## Recording Coverage

A sweep that is not recorded did not happen. Write one row per store:

```text
| store | searched? | surface used | result |
```

`result` states the concrete finding and the decision it drives — for example
"4 related tickets, none owning this work → create new", or "spec `af9ebba9`
is the extend candidate → link now". `searched?` is `yes`, `no`, or
`partial — degraded`; a `degraded` row names the failure.

`transcripts/29-09-2026_entity-store-research-discovery/ARTIFACTS.md` §D0 is
the reference example, including an honestly-degraded `session` row.

## When A Relevant Entity Already Exists

Reuse it. Pick exactly one of these three forms and record which one was
chosen and why — an unrecorded choice is not compliance:

1. update the existing entity in place (edit its body or fields);
2. add a child/sub-entity under the existing one (a sub-ticket, a new spec
   section);
3. link the new work to the existing entity through a dependency or `linked`
   edge, leaving the existing entity unchanged.

Creating a near-duplicate is not a fourth option. Duplicate tickets degrade
store quality and duplicate specs weaken the repository contract.

## When Creation Is Correct

Creation is correct when the sweep found no entity that owns the work. In that
case the creating waypoint or agent cites the sweep that proved it — the
`ARTIFACTS.md` coverage table, or the exact search command and its result.
"I did not find one" without a cited search is not evidence.

A related-but-not-owning entity is linked, not annexed: linking a new ticket to
an adjacent one is reuse form 3, and does not transfer ownership of that
entity's scope.

Planning preparation is restricted to necessary planning/draft tickets/specs.
Reject unrelated or active-entity mutations, missing discovery/reuse evidence,
and a bounded list offered as id-specific read-back. Record the verified
canonical id/link and state in the dossier before the final dry run/review.
This does not authorize activation, execution dispatch or implementation.

## Related Guidance

- [prompt-ingestion.instructions.md](prompt-ingestion.instructions.md) — Stage 2 applies this rule and owns the `ARTIFACTS.md` contract.
- [roadmap-authoring.instructions.md](roadmap-authoring.instructions.md) — creation waypoints apply this rule.
- [evidence-grounded-refinement.instructions.md](evidence-grounded-refinement.instructions.md) — the general "gather evidence before critiquing or asking" loop this rule specializes.
- [file-inspection.instructions.md](file-inspection.instructions.md) — bounded reads for the grep-based fallbacks above.
