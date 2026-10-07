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

## Completion evidence
- `SessionStoreConfig` now implements the kernel `DomainStore` contract and resolves selected workspaces with the established canonical-write, read-only legacy-read, and layout-diagnostic policy.
- `SessionDomainStore` implements only Session's supported create, read, and list capabilities. Capture inputs preserve the existing Session record and hook-event lifecycle representations.
- The canonical Session capture path and hook-only event path use the typed create capability; lifecycle, relation, and worktree behavior remain in `session-api`.
- Focused isolated-fixture test `domain_store::tests::selected_workspace_capture_and_hook_event_read_back_from_its_canonical_store` passed. It writes a captured record and hook event beneath the selected workspace's `.workflow-tools/session`, reads the record back through `ReadEntity`, and proves parent and sibling canonical stores were untouched.
- Baseline passed: `cargo test --manifest-path workflow-tools/session/crates/session-api/Cargo.toml`; `cargo test --manifest-path workflow-tools/session/Cargo.toml`.
- Final validation passed: `cargo test --manifest-path workflow-tools/session/crates/session-api/Cargo.toml` (285 passed, 1 ignored); `cargo test --manifest-path workflow-tools/session/Cargo.toml`; `git -C workflow-tools/session diff --check`.
- No broad Session failures or separate bug fixes were encountered in this waypoint.