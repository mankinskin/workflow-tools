## Objective
Test session CLI/MCP workflows, including access-lease contention.

## Context
Owner decision: “The orchestrator session claims leases; sub-sessions INHERIT the orchestrator's leases. Unrelated host sessions ... are rejected LOUDLY.”
Audit: `tmp/test-coverage-audit/02-code-surface.md`.

## Acceptance criteria
- Cover normal workflow and inherited lease paths.
- Cover unrelated-session conflict message.

## Out of scope
- Lease contract refinement.