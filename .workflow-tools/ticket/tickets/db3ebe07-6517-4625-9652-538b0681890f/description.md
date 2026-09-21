## Problem

"default" is used as an ambient/fallback workspace value in two distinct, currently-unrelated ways across the workflow-tools crates, and both are sources of real bugs.

### A. `workspace_slug` defaults to the literal string "default"

`workspace_slug` is a scoping/namespace field used by several stores to lay out `<root>/<workspace_slug>/...` (feedback, session, spec's EntityUrn, test-api's `<root>/<workspace_slug>/{specs,executions}/`). Several crates fall back to the literal string "default" when no explicit value is supplied, which becomes a real directory name on disk when combined with an incorrectly-resolved store root (see ticket 31e70dab, which reproduced/filed the meta-workspace/default/{specs,executions} incident this caused). Confirmed runtime (non-test) defaults:

- feedback-http/src/main.rs:53 - .unwrap_or_else(|_| "default".to_string())
- feedback-mcp/src/main.rs:49 - same pattern
- test-mcp/src/main.rs:46 - same pattern
- test-cli/src/args.rs:46 - #[arg(long, global = true, default_value = "default")]
- session/crates/mcp-toolmon/src/proxy/workspace_resolution.rs:48 - anchored_resolver()'s ResolverConfig::from_working_dir("default") fallback

Test-only occurrences of the literal string (session-api unit tests, spec-api manifest tests) are lower priority - audit each during implementation.

### B. The `workspace` path selector already has a shared validator

`memory_kernel::workspace::validate_explicit_workspace_selector` is already used across all six domains (rule, session, spec, ticket, feedback, test - 13 call sites) to reject None | "" | "default" | "." | ".." for entity-creation workspace parameters. "default" as a path value is already rejected everywhere.

Open design question (NOT resolved by this ticket): should "." become an accepted explicit "current directory" selector, replacing "default" as the original request suggested? "." is banned today specifically because "current directory" is ambiguous between the MCP server process's cwd and the caller's intended repo root. Flipping this without resolving that ambiguity would reintroduce the exact bug class this ticket exists to close. Needs explicit sign-off before implementation.

## Live incident hit while filing this ticket (2026-09-01)

While drafting this exact ticket via ticket-mcp, EVERY MCP tool call in the session (ticket_create_ticket, ticket_list_tickets, session_lookup) failed with:

    workspace selector 'default' for session '<session-uuid>' is unanchored; refused to select a store from candidates: C:/Users/linus/git/3/meta-workspace/.session

This is `UnanchoredDefault` from `session-workspace-resolver` (workflow-tools/session/crates/session-workspace-resolver/src/lib.rs:394), a deliberate safety rejection when a session has no worktree check-in to disambiguate "default" against. This appears to be part of the already-tracked epic `db6980d1` ("Worktree provisioning and session-worktree lifecycle", 24 linked sub-tickets) rather than a new defect - but it directly and concretely demonstrates why "default" as an ambient value is dangerous, and it fully blocked MCP tool usage for this session (worked around here by falling back to the `ticket` CLI binary directly, which bypasses the mcp-toolmon proxy and resolves the store correctly with no ambiguity).

Per session-identity-and-handoff/worktree-provisioning instructions, a main-checkout session (no dedicated worktree) should be able to proceed without a session-to-worktree assignment after checking board ownership - but the current mcp-toolmon gate refuses ALL calls unconditionally when unanchored, with no main-checkout exemption observed. Whether this is expected epic-db6980d1 in-progress behavior or a regression needs triage as part of that epic, not this ticket.

## Scope (plan)

1. Foundational (memory-kernel): once part B's design question is answered, update validate_explicit_workspace_selector (and its two test suites: workflow-tools/memory-kernel/src/workspace_tests.rs, workflow-tools/session/src/mcp/server.rs::workspace_validation_rejects_ambient_aliases) accordingly. Do NOT change this before the design question is answered.
2. workspace_slug default elimination - replace every runtime "default" fallback in part A with either a required explicit value or a derived, non-ambiguous default. Candidate crates: feedback-http, feedback-mcp, test-cli, test-mcp, mcp-toolmon.
3. Regression test: fail if any entity artifact is ever created under a path with a literal "default" path component.

## Relationship to other tickets

- 31e70dab (Fix resolve_store_root_from_with_diagnostics no-marker fallback): the path-resolution bug that let a bad root land at the workspace root. Complementary to this ticket.
- db6980d1 (epic, Worktree provisioning and session-worktree lifecycle): owns the mcp-toolmon session-anchoring/UnanchoredDefault behavior surfaced by the live incident above. Triage whether that epic's scope already covers part A's mcp-toolmon bullet before duplicating work.

## Escalation

Part B (whether to allow "." and what it resolves to) is a design decision with correctness implications across every MCP domain and needs explicit user sign-off before implementation.
