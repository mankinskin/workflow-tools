The `audit-mcp` server's `ticket_graph` check does not respect the `repo_root` parameter's scoping.

Reproduction: run `mcp_audit-mcp_audit` (or `audit_summary`) with `repo_root` pinned to a narrow subdirectory, e.g. `workflow-tools/session/crates/mcp-toolmon` (a crate with no `.ticket` store of its own). Expected: `ticket_graph` check is skipped or reports nothing (no ticket store in scope). Actual: it discovered and reported 253+ findings against tickets living under `context-engine/memory-api/.ticket/tickets/*` — an entirely unrelated repository/directory outside `repo_root`, drowning out the ~10 real code-quality findings (file_length, static_complexity, compiler_warning, coverage, test_execution) in noise (261 of 272 total findings were this scope leak).

Root cause is presumably in the audit server's ticket-graph discovery: it appears to walk up and/or sideways from `repo_root` to find *some* ticket store on disk rather than confining itself to stores actually inside `repo_root`, and/or it falls back to a globally-discovered default store when none exists locally instead of skipping the check.

Fix: `ticket_graph` (and any other store-backed check: `session_workflow_graph`, `rule_overlap`, `spec_fulfillment`) should only report findings for ticket/rule/spec stores that are descendants of the resolved `repo_root`, and should report `status: "unavailable"` (like `spec_fulfillment` already correctly does when no `.spec` store exists) rather than silently substituting an unrelated store found elsewhere.

Acceptance criteria:
- Auditing a subdirectory with no local `.ticket`/`.rule`/`.spec` store produces `status: "unavailable"` for those checks, not findings against a store outside `repo_root`.
- Auditing the actual owning repo root still reports the same findings as before (no regression in the normal case).
- Add a focused test covering: audit invoked with `repo_root` = a subdirectory that has no local ticket store but where an unrelated ticket store exists elsewhere on disk reachable via upward/sideways discovery — asserts zero ticket_graph findings and `status: "unavailable"`.