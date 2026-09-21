## Objective
Generate ticket CLI/MCP/HTTP parity tests from the P1.5 matrix artifact.

## Context
Audit `tmp/test-coverage-audit/02-code-surface.md` ranks ticket-cli (54 operations), ticket-mcp (40+), and ticket-http as high risk.

## Acceptance criteria
- Derive tests from the machine-readable parity matrix.
- Assert declared fields and transport differences.

## Out of scope
- Changing API contracts.