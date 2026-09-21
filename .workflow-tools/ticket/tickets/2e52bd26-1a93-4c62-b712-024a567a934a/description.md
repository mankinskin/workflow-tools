# Objective

Bundle the "best next ticket to implement" hardening work into a single handoff package that another engineer or agent can pick up without needing to reconstruct the backlog from chat history.

This package is the handover entry point. The implementation epic remains `51671748`, but this ticket is the top-level package that explains what is included, what order to tackle it in, and what validation routine must pass before the work is considered done.

# Package Scope

The package covers the full end-to-end next-workflow:

- canonical specification and operator docs
- CLI output and behavior
- MCP parity
- ticket-viewer and ticket-vscode user-facing surfaces
- automated regression coverage
- manual validation / handoff checklist

The target outcome is simple: a user asking for the best next tickets to implement should get a clear, correct, explainable answer across all supported surfaces.

# Included Tickets

- `51671748` — `[ticket-workflow] Harden best-next-ticket discovery across spec, CLI, MCP, and frontends`
- `68a08b34` — scope-aware board and next for multi-root workspaces
- `68e3c713` — fix `next --filter` matching for prefix and substring queries
- `61cbc31f` — explain why tickets are absent from `next`
- `07836f41` — make `get/search/list` workspace-aware across nested roots
- `86cde60c` — distinguish deferred and meta work from actionable tickets
- `14df656e` — surface the next-work workflow in ticket-viewer and ticket-vscode

# Suggested Execution Order

1. Lock down scope and cross-root metadata.
2. Fix filter semantics so targeted discovery is trustworthy.
3. Add omission explanations and actionable-vs-meta classification.
4. Expose the stabilized workflow in ticket-viewer and ticket-vscode.
5. Run the full regression matrix and manual handoff checklist.

# Required Deliverables

1. Canonical spec / docs that define the next-workflow contract.
2. CLI and MCP behavior that agree on scope, ranking, filters, omission reasons, and classification.
3. Frontend surfaces that display the same semantics instead of inventing client-side logic.
4. Automated regression coverage across:
   - CLI integration tests
   - MCP cross-interface tests
   - ticket-viewer release Playwright
   - ticket-vscode test coverage
5. A repeatable manual validation checklist for the original multi-root doc-viewer scenario.

# Validation Routine

The assignee should not close this package until all of the following are true:

- the canonical spec / docs are updated to describe the next-workflow contract
- CLI integration tests cover the documented scope, filter, why-not, and classification cases
- MCP tests confirm parity with the CLI contract
- ticket-viewer and ticket-vscode expose and test the user-facing next-workflow
- the manual validation checklist passes against the multi-root doc-viewer scenario that originally surfaced the failures

# Handoff Notes

- Treat `51671748` as the implementation epic.
- Treat this ticket as the work-package cover sheet for planning, assignment, and status handoff.
- Use the repo-root versus nested-root doc-viewer lookup as the canonical regression fixture.
- If additional child tickets are needed during implementation, link them under `51671748` and keep this package ticket focused on handoff-level scope and completion.

# Acceptance Criteria

- A new assignee can open this ticket and understand the scope, included tickets, execution order, and validation routine without reading prior chat context.
- `51671748` and its child tickets are the authoritative implementation graph for this package.
- The package only closes once the end-to-end next-workflow is clear and correct across spec/docs, CLI, MCP, and frontend surfaces.