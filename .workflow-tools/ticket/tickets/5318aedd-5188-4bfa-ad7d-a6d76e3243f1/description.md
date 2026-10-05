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