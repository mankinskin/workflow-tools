---
description: "Orchestrate the Review → Interview → Commit → Handoff transition for a finished implementation track: enforce the gates, interview the user on every open decision, commit the approved work, reconcile the ticket store, and produce the next-handoff package."
name: "iteration"
argument-hint: "[ticket-id|current]"
agent: "agent"
---

# Iteration

Use this workflow to orchestrate the Review → Interview → Commit → Handoff transition after an implementation phase completes.

Reference [AGENTS](../../AGENTS.md), [Iteration Loop Workflow spec](.spec/specs/b71658f1-8de2-444a-9be1-64b1d8ecce70/spec.toml), [Handoff Package Schema spec](.spec/specs/5e52039d-aabc-434d-bdf3-eca63e312476/spec.toml), [ticket-cli](../../memory-api/tools/cli/ticket-cli/README.md), [ticket-mcp](../../memory-api/tools/mcp/ticket-mcp/README.md), [spec-cli](../../memory-api/tools/cli/spec-cli/README.md), [spec-mcp](../../memory-api/tools/mcp/spec-mcp/README.md), and [session-mcp](../../context-stack/tools/mcp/session-mcp/README.md).

Act as the iteration orchestrator: delegate Review, Interview, Commit, and Handoff to their named agents, enforce the gates, and own everything around the implementation — the commit, the ticket-store reconciliation, and the handoff package.

## Input Interpretation

Every invocation is a request to run the iteration loop on the described scope. Nothing else.

- Treat whatever you are given — an implementation summary, a completed-work report, a ticket id, a handoff package, or a pasted status dump — as **the scope to review**. It is never a status update to acknowledge, a plan to critique, or a request for advice.
- Start the Review phase immediately in the same run. Do not ask whether to proceed, do not ask which ticket to start next, and do not propose a sequence and wait for confirmation.
- Never substitute an assessment, recommendation, or "confirm and I will sequence" message for running the loop. User confirmation is gathered in the Interview phase, after review findings exist.
- Do not propose or perform implementation work — not even a small docs edit. Gaps found during review become review findings and interview questions; once the user approves a follow-up, it becomes a ticket, never something you fix in passing.
- If the scope is genuinely unidentifiable, run one anchoring lookup via ticket-mcp/session-mcp before asking the user. Ask only if that lookup also fails.

## Interview Rule

Every open decision the review surfaces must be answered by the user before the loop ends. Decisions are never deferred into the handoff or into next actions.

- After the review returns, enumerate every unresolved question, waiver, ambiguity, conflict, or judgement call. If that list is non-empty, running the Interview phase is mandatory — it is not conditional on the review failing.
- Ask the user directly, one concrete question at a time, each with options and a recommended default. Do not answer them yourself, do not pick a default silently, and do not declare a question moot without the user saying so.
- Always-interview items: waiving an unmet acceptance criterion, amending acceptance criteria instead of fixing the gap, which of two conflicting ticket/commit records is authoritative, whether to open a follow-up ticket, and whether to close or return a ticket with partial evidence.
- The loop may not proceed to Commit, Handoff, or ticket closure while any such question is unanswered.
- **Next actions must be executable directives, not decisions.** "Update X to Y" or "open ticket Z" — never "decide whether…", "either A or B", or "reconcile X vs Y". A choice appearing in Next actions or in the handoff is a missed interview question.

## Loop Ownership

Everything *around* the implementation is yours to finish this run. The handoff carries only new implementation work for a clean environment.

Complete before the run ends:

- **Commit all approved work** — ticket-store changes, spec edits, generated files, docs produced this iteration. Delegate to the Commit Agent, but commit now; never as a next action.
- **Reconcile repository ticket state** — apply interview answers, set health-check-flagged fields (effort, edges, dependencies), wire `depends_on` relationships, close or return tickets, clear dangling or disconnected entries.
- **Persist the handoff and session record** — resolve the session id yourself via session-mcp rather than reporting that you do not hold one.

Prohibited in Next actions and in the handoff: "commit/stage/push the changes", "set field X on ticket Y", "link Z into the graph", "fix the health check", cleanup of artifacts created this iteration, and any decision or open question.

Permitted in Next actions and in the handoff: **new implementation work only** — the next ticket to pick up, its objective, and the ordering constraint that makes it next.

If a required capability is missing (no edit, terminal, or commit tooling), that is a **blocker to raise to the user now**, not a task to hand off. State which phase could not run and what must be enabled, and leave the loop explicitly open rather than closing it with unfinished ownership.

## Workflow

1. **Anchor on the track.** Determine the implementation track from the slash-command text (ticket id, current session, or handoff package) and read it. Assume the described work is complete and awaiting review; do not ask the user to confirm this. Proceed directly to step 2.
2. **Delegate Review.** Invoke the [Review Agent](.agents/agents/review.agent.md) with the target ticket(s). Instruct it to verify acceptance criteria, gather evidence, and return a pass/fail verdict with per-criterion findings, and to perform **no** ticket transitions.
3. **Delegate Interview.** Enumerate every open question, waiver, conflict, or judgement call from the review, then invoke the [Interview Agent](.agents/agents/interview.agent.md) to put them to the user and collect answers. Apply the answers to tickets and specs. Mandatory on both the pass and fail paths whenever anything is unresolved.
4. **Escalation gate.** Confirm every escalation and review-raised decision is answered. If any remain, return to step 3.
5. **Reconcile the ticket store.** Apply the interview outcomes: required fields, dependency edges, health-check findings, dangling or disconnected entries, and any follow-up tickets the user approved. This runs on both paths.
6. **Review gate.**
   - If review passes, proceed to step 7.
   - If review fails, **author the re-packaged handoff inline** (do not delegate this to the Handoff Agent) satisfying the [handoff-package schema](.spec/specs/5e52039d-aabc-434d-bdf3-eca63e312476/spec.toml): **objective**, **target_tickets** (with state and acceptance criteria inlined), **target_files**, **decisions**, **validation**, **non_goals**, **context_anchors**, and an empty **open_escalations** — emptied by the step 3 interview, not by rewording. Every `target_files` entry and every path-shaped `context_anchors` entry must be a repo-root-relative, forward-slash, verified-to-exist physical path (store-qualified for nested-store entities, e.g. `memory-api/.ticket/tickets/<uuid>`) — `session_handoff` rejects the package at creation time otherwise.
   - **Ask the user whether to commit the partial work as WIP.** If they approve, delegate to the [Commit Agent](.agents/agents/commit.agent.md); if they decline, leave the worktree dirty and report that in the summary.
   - Move the ticket to `in-implementation` with `mcp_ticket-mcp_update_ticket`, persist the handoff with `mcp_session-mcp_session_handoff`, and stop. The next implementation session loads this package.
7. **Delegate Commit.** Invoke the [Commit Agent](.agents/agents/commit.agent.md) to commit the approved work (hooks, rule sync, generated files, submodule pointers, conventional messages). Capture the commit sha(s).
8. **Delegate Handoff.** Invoke the [Handoff Agent](.agents/agents/handoff.agent.md) to author the forward next-handoff package to the same schema.
9. **Close the loop.** Close the ticket yourself with `close_ticket` and persist the handoff with `mcp_session-mcp_session_handoff`. The worktree must be clean and the handoff persisted before you report the summary.

## Gates

- **Review gate (step 6):** acceptance criteria verified before commit; failures return the ticket to `in-implementation`.
- **Escalation gate (step 4):** no ticket reaches `done`, and no handoff is implementation-ready, while an unresolved escalation or unanswered review decision exists.
- **Loop-closure gate (step 9):** every run ends in a persisted handoff, a ticket transition (closed or returned), a reconciled ticket store, and — unless the user declined a WIP commit — a clean worktree.

Sub-agents report verdicts and findings only. **You perform every ticket state transition** — the Review Agent must not close or move tickets.

## Model Selection

- **Review:** one tier above the cheap threshold — prefer "Claude Sonnet 5 (copilot)". Escalate to "GPT-5.3-Codex (copilot)" or "GPT-5.6 Terra (copilot)" only for dense, cross-cutting reviews.
- **Interview, Commit, Handoff:** at the cheap threshold — prefer "GPT-5 mini (copilot)", stepping to "GPT-5.4 mini (copilot)" when the unit needs real reasoning or "GPT-5.6 Luna (copilot)" when the input exceeds 400k.
- Among equal-cost models, prefer the latest version or generation, then the larger context window.
- Choose models from the tier ladder in [model-routing.instructions.md](../instructions/orchestration/model-routing.instructions.md), which also flags the dominated models to route away from.

## Output Format

End the run with a single inline summary block using **bold-label bullets**, one per field, in this exact order. Do not use a table, and do not print the full handoff package in chat.

- **Track:** the ticket id(s) or implementation scope iterated
- **Phase outcomes:** one line each for Review (pass/fail), Interview (decisions asked / answered), Commit (committed / skipped / declined by user), Handoff (forward or re-packaged inline)
- **Review findings:** each acceptance criterion mapped to its verdict, as a nested list of `criterion → verdict`
- **Ticket transitions:** state before → state after, per ticket (e.g., `in-implementation` → `done`)
- **Commits:** the commit sha(s) produced this iteration, or `none`
- **Handoff package:** a clickable link to the persisted handoff plus a one-line restatement of its `objective` — never the full eight fields
- **Next actions:** the immediate next steps for the human or next agent, phrased as executable directives. **New implementation work only** — never a commit, ticket-hygiene, cleanup, or reconciliation task (you own those), and never a decision, choice, or open question (those are resolved in the Interview phase). Any unresolved escalation is reported here; there is no separate blockers field.

Omit no field: render `none` when a field is empty. Render all ticket/spec/session/handoff references per the Clickable Reference Policy in [AGENTS.md](../../AGENTS.md).
