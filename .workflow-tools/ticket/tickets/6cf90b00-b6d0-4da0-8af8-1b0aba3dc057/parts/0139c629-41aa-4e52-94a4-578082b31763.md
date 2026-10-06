# Reject Explicit Legacy-Shaped Write Paths

## Problem
A caller can explicitly name a legacy-shaped store path such as `.test` or `.ticket`, and the resolver may redirect a nonexistent path to canonical storage. The caller receives neither the requested legacy path nor a clear rejection.

## Selected contract
Reject explicit legacy-shaped write overrides whether or not the legacy directory exists. Do not create a legacy directory or silently redirect the write. Workspace-based writes target `<local_workspace>/.workflow-tools/<domain>`; supported direct concrete canonical-store overrides remain available. For reads, legacy compatibility is allowed only when canonical storage is absent. If both layouts exist, select canonical and return `BothLayoutsPresent`.

## Scope
Update `resolve_store_root_at_workspace` and focused tests in `workflow-tools/memory-kernel`, including the existing Test-MCP regression that currently expects an explicit nonexistent `.test` path to be created. The behavior follows the workspace Spec attached in `spec_refs`; selector normalization for `.` is tracked separately by the linked selector ticket. Rule is excluded.

## Acceptance criteria
1. Explicit legacy-shaped write paths fail with an actionable error whether or not the path exists.
2. No legacy directory or redirected canonical write is created by the rejected request.
3. Legacy-only reads and canonical-plus-legacy selection continue to follow the Spec's compatibility and diagnostic contract.
4. Regression tests cover representative Ticket/Test paths and preserve canonical direct overrides.