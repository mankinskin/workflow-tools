## Objective
Adapt Ticket API store operations to the kernel-owned `DomainStore` core and only the typed capabilities TicketStore supports.

## Scope
Preserve Ticket entity types and current workflows while routing workspace/store selection through `memory-kernel`. Cover supported CLI/MCP/HTTP consumers without duplicating resolution logic in transports. Preserve explicit-dot normalization, canonical writes, legacy read-only behavior, duplicate-layout diagnostics, owner-root references, and read-only no-auto-init.

## Acceptance criteria
1. TicketStore implements the core contract and supported typed capabilities without moving Ticket semantics into the kernel.
2. Ticket APIs preserve existing identifiers, results, errors, and domain extensions.
3. Supported transports resolve the same local workspace and write only to the canonical Ticket store.
4. Focused tests read back created/updated entities from the selected canonical store and verify no parent, sibling, or ambient store was modified.

## Validation
Run the Ticket API command in G2: `cargo test --manifest-path workflow-tools/ticket/Cargo.toml --features "cli mcp http"`; run affected transport tests in G3.