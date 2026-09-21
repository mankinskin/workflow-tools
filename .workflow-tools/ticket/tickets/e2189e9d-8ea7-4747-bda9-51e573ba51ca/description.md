Implement the first executable slice of the default worktree-backed session workflow after `68a49ca7` locks the contract.

# Scope
- add the `session-api` check-in and lookup surfaces that create or resume a session and return the authoritative working directory for that session
- persist the worktree assignment metadata that guidance, hooks, and future resume flows must consume
- enforce the planning contract for deterministic reuse versus rotation without allowing multiple active sessions to silently share one worktree
- expose enough lifecycle detail for startup helpers or MCP callers to distinguish `new`, `reused`, and `rotated` assignments

# Out of Scope
- changing the ticket draftboard ownership model
- rewriting repository instructions or hooks beyond the minimal surface needed to exercise the new API

# Acceptance Criteria
1. A focused `session-api` surface creates or resumes a session and returns at least the session id, authoritative worktree path, branch, allocation mode, and relevant ticket/owner context.
2. A focused lookup or resume surface lets startup helpers and hooks retrieve the authoritative worktree path for an existing session without reconstructing it from ticket or branch naming.
3. The implementation enforces the planning contract for reuse versus rotation: same-session revival can reuse a healthy assignment, while fresh sessions after handoff or invariant failure rotate to a new worktree and record predecessor lineage.
4. The implementation never silently assigns one active worktree to multiple active sessions; tests cover new assignment, same-session reuse, handoff-driven rotation, and invalid-worktree recovery.
5. The implementation keeps board ownership separate: `session-api` may carry ticket context, but it does not create or mutate draftboard ownership as part of worktree assignment.

# Traceability
- Governing spec: [default worktree-backed session workflow](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.spec/specs/context-engine/session-worktree-default-workflow/spec.toml)
- Planning prerequisite: [68a49ca7 Plan default worktree-backed session workflow](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/.workflow-tools/ticket/tickets/68a49ca7-a6f6-42a8-b820-0a86e6a4de2e/ticket.toml)
- Follow-on rollout: [326bfe38 Add worktree-first session guidance and hooks](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/.workflow-tools/ticket/tickets/326bfe38-6f5e-4000-9ffc-e5be0839194f/ticket.toml)