# Audit API DomainStore Adapter and Workspace Parity

## Goal
Adapt `audit-api` and its supported transport consumers to the kernel-owned `DomainStore` contract while retaining Audit-specific index and query behavior.

## Scope
- Make the Audit store implement only the typed operation capabilities supported by its current API; preserve Audit models and repository-index behavior.
- Route supported CLI/MCP/HTTP store selection through the shared contract and keep transport layers free of duplicate path-resolution logic.
- Apply the workspace Spec semantics for explicit `.` normalization, canonical selection and `BothLayoutsPresent`, legacy read-only compatibility, canonical-only writes, and read-only no-auto-init.

## Acceptance criteria
1. `audit-api` consumes the `memory-kernel` core trait and typed capabilities without moving Audit business semantics into the kernel.
2. Supported Audit operations retain existing results and domain-specific extensions.
3. Supported Audit transports resolve the same explicit local workspace; discovered ancestor/global stores do not replace the write base.
4. Focused API/transport tests verify canonical write/read-back, duplicate-layout diagnostics, and selector behavior.
5. No Rule surface or Rule-specific dependency is introduced.

## Validation
Run the Audit API command in G2: `cargo test --manifest-path workflow-tools/audit/crates/audit-api/Cargo.toml`. Run affected Audit transport tests from G3 and record persisted canonical read-back evidence.