# Canonical no-marker workspace fallback

## Problem
`resolve_store_root_from_with_diagnostics` returns the bare workspace directory when no local or ancestor store marker exists. Fresh domain stores can therefore be initialized outside their canonical namespace.

## Expected behavior
When no existing store marker is found, derive `<local_workspace>/.workflow-tools/<domain>` with the shared `canonical_store_root` contract. Do not return the bare workspace or create a legacy hidden-store path. Preserve read-only behavior and diagnostics.

## Scope
Fix the no-marker fallback in `workflow-tools/memory-kernel/src/workspace.rs` and add focused resolver regression coverage. The active domain set is Ticket, Spec, Test, Session, Feedback, Audit, and Log. Rule is excluded from this contract and this ticket's acceptance scope.

## Acceptance criteria
1. A fresh workspace with no ancestor store marker resolves to the canonical `.workflow-tools/<domain>` path.
2. Resolution never returns the workspace directory itself as a store root.
3. Regression tests cover representative domains and confirm existing explicit/local selection behavior is unchanged.
4. The regression is recorded against the workspace Spec referenced in `spec_refs`.