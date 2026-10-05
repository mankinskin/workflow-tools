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