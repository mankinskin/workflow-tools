# Spec API DomainStore Adapter and Workspace Parity

## Goal
Adapt `spec-api` and its supported transports to the kernel-owned `DomainStore` contract and preserve consistent nested-workspace behavior.

## Scope
- Make the Spec store implement only the typed operation capabilities its current API supports, preserving Spec entity types and references.
- Route supported CLI/MCP/HTTP selection and write behavior through the shared contract; retain pure transport boundaries and existing nested-store read behavior.
- Apply the workspace Spec semantics for explicit `.` normalization, canonical selection and `BothLayoutsPresent`, legacy read-only compatibility, owner-root reference resolution, and no auto-initialization for read-only access.

## Acceptance criteria
1. `spec-api` consumes the `memory-kernel` core trait and typed capabilities without a Spec-to-kernel dependency inversion.
2. Supported Spec operations preserve their existing models, validation behavior, and domain extensions.
3. The Spec CLI/MCP/HTTP surfaces resolve the same explicit workspace consistently; writes remain canonical-only and read behavior preserves documented legacy compatibility.
4. Focused API/transport tests verify selector, duplicate-layout diagnostic, owner-root resolution, and persisted canonical read-back.
5. No Rule surface or Rule-specific dependency is introduced.

## Validation
Run the Spec API command in G2: `cargo test --manifest-path workflow-tools/spec/Cargo.toml --features "cli mcp http"`. Record focused transport tests and read-back evidence in the implementation ticket.

## W4 completion evidence (2026-10-07)
- Implemented the Spec `DomainStore` adapter at `crates/spec-api/src/domain_store.rs`: canonical selected-workspace resolution, legacy read-only compatibility, and only Spec's create/read/update/delete typed capabilities. Spec manifests, fields, references, and schema are unchanged.
- Routed selected-workspace write selection through the shared core for CLI, MCP, and HTTP startup. Focused fixtures prove canonical persisted read-back for CLI (`create_in_selected_workspace_persists_to_its_canonical_store`), MCP (`explicit_dot_workspace_creates_and_reads_back_in_current_workspace`), and HTTP (`http_create_in_selected_workspace_persists_to_its_canonical_store`).
- Validation evidence: `cargo test --manifest-path workflow-tools/spec/Cargo.toml --features "cli mcp http"` passed (87 test groups) after the change; `git -C workflow-tools/spec diff --check` passed. Spec source/test checkpoint: `d4bdd15`.
- No separate bug fix; no schema, Rule, production Spec, or contract-owner change.

- Superseding source checkpoint: the finalized Spec source/test commit is `d51a6a8` (the prior abbreviated checkpoint was amended before final validation).