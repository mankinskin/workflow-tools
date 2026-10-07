## Objective
Adapt Test API store operations and supported Test CLI/MCP consumers to the kernel-owned `DomainStore` contract.

## Scope
Preserve TestSpec and execution models, existing explicit-workspace writes, and `test-mcp` optional-workspace aggregation on reads. Implement only Test operations supported today; route selection through `memory-kernel` without duplicating resolver logic in transport code.

## Acceptance criteria
1. TestStoreConfig consumes the core contract and typed capabilities while preserving existing record/list semantics.
2. Explicit `workspace="."` normalizes to the concrete local workspace; invalid creation aliases fail before initialization.
3. Writes use canonical `.workflow-tools/test`; legacy reads remain read-only and duplicate layouts return the specified diagnostic.
4. Producer-shaped CLI/MCP tests read back each created validation spec/execution from the selected store without changing parent, sibling, or ambient stores.
5. The existing `val-memory-kernel-workspace-resolution` guard is reused; no duplicate TestSpec is created.

## Validation
Run G2 `cargo test --manifest-path workflow-tools/test/crates/test-api/Cargo.toml` and affected `test-cli`/`test-mcp` tests in G3. Preserve the existing nested-store aggregation regression.

## W5 completion evidence (2026-10-07)

- Implemented `TestStoreConfig`'s shared `DomainStore` resolution and the Test-only typed create/read/list adapter for validation specs and executions. Existing TestSpec, execution, benchmark, and store-index behavior remains domain-owned.
- Routed explicit workspace selection in Test CLI and MCP through the Test API adapter. Canonical writes, legacy read-only compatibility, duplicate-layout diagnostics, invalid creation aliases, and legacy write-path rejection remain enforced by the shared workspace contract.
- Added isolated producer-shaped CLI and MCP fixtures that create a validation spec and execution, read both back through the same consumer, and prove parent, sibling, and ambient stores were unchanged. The canonical Test CLI Log-capture path was not edited.
- Validation passed: `cargo test --manifest-path workflow-tools/test/crates/test-api/Cargo.toml` (46 tests), `cargo test --manifest-path workflow-tools/test/crates/test-cli/Cargo.toml` (14 tests), `cargo test --manifest-path workflow-tools/test/crates/test-mcp/Cargo.toml` (16 tests), and `git -C workflow-tools/test diff --check`.
- Test source/test checkpoint: `f1060c1`. No separate bug fix, production test-record mutation, new TestSpec, or Log adapter work.