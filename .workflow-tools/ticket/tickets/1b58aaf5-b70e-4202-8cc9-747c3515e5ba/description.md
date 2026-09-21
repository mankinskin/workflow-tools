## Objective
Coordinate the coverage and contract-hardening program across transport, fixture, replay, reliability, and process work.

## Context
Audit reports: `tmp/test-coverage-audit/01-existing-tests.md`, `01b-coverage-verification.md`, `02-code-surface.md`, `02b-code-surface-addendum.md`, `03-requirements.md`, `04-capture-hook.md`, and `05-gap-analysis.md`.
The audit found ~352,000 Rust LOC, ~39 tool surfaces, 380+ operations, uneven test density, 104 of 122 specs still draft, synthetic fixtures, and no generative testing.

## Decisions log
1. TOON CLI default; stable fields; no byte cap; semantics plus valid TOON tests.
2. HTTP defaults JSON; MCP defaults TOON; both expose explicit selectors.
3. Allow symlinks resolving inside root; reject escapes.
4. One shared feedback API spec; CLI and MCP are transports.
5. Versioned graded-budget cost-policy table, not outright refusal.
6. N=5 guidance; edit-distance not-found; highest-tier-first too-expensive suggestions.
7. mcp-toolmon pre-tool session guard; built-ins exempt initially.
8. Orchestrator leases inherit to sub-sessions; reject unrelated sessions loudly; auto-release on finish.
9. Publish strict generated CLI/MCP/HTTP field matrix.
10. Hardware-classified adaptive latency baseline.
11. Browser preference is contributor policy; move it to instructions and retire the spec.
12. Pin graph flags and BFS depth/cycle/empty-result semantics.
13. Generate and validate TOON examples from command schemas.

## Acceptance criteria
- All listed phase tickets and dependency edges exist and pass ticket-store health validation.

## Out of scope
- Implementing contracts, tests, fixtures, or documentation changes.