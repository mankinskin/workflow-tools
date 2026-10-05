## Objective
Adapt Session API store operations and capture consumers to the kernel-owned `DomainStore` contract.

## Scope
Preserve Session record models, canonical hook capture, and existing session-id/worktree ownership semantics. Route SessionStoreConfig and supported CLI/MCP/capture paths through the shared store contract; implement only capabilities Session API supports. Do not duplicate the separate session-anchored MCP workspace-resolution work.

## Acceptance criteria
1. Session API uses the core trait and typed capabilities without moving session lifecycle semantics into `memory-kernel`.
2. Session writes resolve against the explicitly selected local workspace and persist its concrete normalized identity.
3. Canonical writes, read-only no-auto-init, legacy compatibility, and duplicate-layout diagnostics match the workspace Spec.
4. Hook/CLI/MCP tests read back captured records from the owning canonical Session store and confirm no ambient store was modified.

## Validation
Run G2 `cargo test --manifest-path workflow-tools/session/crates/session-api/Cargo.toml` and affected Session transport/capture tests in G3.