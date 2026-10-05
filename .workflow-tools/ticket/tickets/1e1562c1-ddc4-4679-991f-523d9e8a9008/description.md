## Objective
Prove the shared workspace and `DomainStore` contract across Ticket, Spec, Test, Session, Feedback, Audit, and Log after adapter implementation.

## Scope
Extend the existing `val-memory-kernel-workspace-resolution` guard and producer-shaped fixtures. Cover explicit and omitted/blank/default/dot/traversal selectors; canonical/legacy-only/both-layout behavior; read-only no-auto-init; direct canonical and rejected legacy overrides; owner-root references; parent/global discovery without write-base changes; supported typed capability behavior; transport parity; and selected-store read-back with no sibling/parent/ambient mutation.

## Acceptance criteria
1. Every G5 matrix case has a repeatable focused test or documented deterministic check for all seven active domains.
2. G6 producer-shaped CLI/MCP/HTTP/capture fixtures create records in the selected workspace, read back the same ids/fields from canonical stores, and prove unrelated stores are unchanged.
3. Test CLI Log capture writes to `.workflow-tools/log`, not `.workflow-tools/.log`.
4. Existing `val-memory-kernel-workspace-resolution` is extended; no duplicate TestSpec is created.
5. No Rule API/store/transport or entity distribution operation is included.

## Validation
Run G5 and G6 from the workspace Spec validation matrix. Record per-domain command, transport, entity id, canonical read-back path, and observed no-cross-write result. Run the focused G2/G3 commands for each touched slice; do not run workspace-wide Cargo tests.