## Objective
Adapt Feedback API store operations and supported CLI/MCP/HTTP consumers to the kernel-owned `DomainStore` contract.

## Scope
Preserve Feedback's canonical per-entry store, append/ingest semantics, legacy migration behavior, and domain-specific query types. Implement only supported typed capabilities and keep transport layers free of independent workspace/store resolution.

## Acceptance criteria
1. Feedback API consumes the core trait and typed capabilities without changing feedback record schemas or ratings.
2. Workspace-based writes target canonical `.workflow-tools/feedback`; legacy reads and migration behavior remain compatible with the namespace contract.
3. Explicit-dot normalization, duplicate-layout diagnostics, owner-root selection, and read-only no-auto-init match the workspace Spec.
4. CLI/MCP/HTTP tests read back written entries from the selected canonical store and verify no ambient store was modified.

## Validation
Run G2 `cargo test --manifest-path workflow-tools/feedback/crates/feedback-api/Cargo.toml` and affected Feedback CLI/MCP/HTTP tests in G3.