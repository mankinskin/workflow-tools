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

## W8 completion evidence — 2026-10-07

- Audit source/tests: `workflow-tools/audit` commit `4c6da38` (`feat(audit): adopt shared DomainStore`).
- `RepositoryIndex` now implements the shared `DomainStore`, plus only Audit-supported `CreateEntity` and `ReadEntity` capabilities for persisted findings. Read-only resolution retains legacy lookup without initialization; create/open selection writes only to `.workflow-tools/audit`.
- API temporary-workspace fixture writes a persisted finding and reads it back from the selected canonical owning store, while proving parent and sibling stores remain untouched.
- CLI temporary-workspace fixture persists an audit run and reads its `audit_runs` row back directly from the selected canonical SQLite store, while proving parent and sibling stores remain untouched.
- MCP selected-workspace fixture reran and reads both audit and summary run rows back from the selected canonical SQLite store, while proving the ambient server workspace remains untouched.

Validation passed:
- `cargo test --manifest-path workflow-tools/audit/crates/audit-api/Cargo.toml` — 32 passed (3 suites).
- `cargo test --manifest-path workflow-tools/audit/crates/audit-cli/Cargo.toml` — 21 passed (4 suites).
- `cargo test --manifest-path workflow-tools/audit/crates/audit-mcp/Cargo.toml` — 6 passed (3 suites).
- `git -C workflow-tools/audit diff --check` — passed.

No separate bug fix was made; this was the bounded W8 adapter refactor.