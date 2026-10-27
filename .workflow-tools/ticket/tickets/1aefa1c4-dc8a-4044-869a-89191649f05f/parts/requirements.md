# W1 — API ownership refactor

## Objective
Move the existing direct Mission lifecycle into `mission-api` without changing accepted-record behavior. The API owns UUID allocation, revision-1 bootstrap, create/edit/save/publish/order/get dispatch; the shared `mission` facade maps its stable calls to that API; CLI is parsing/JSON/exit mapping and MCP is JSON-RPC DTO/response mapping only.

## Requirements and design
- Preserve UUIDs, accepted revision/CAS behavior, history bytes, result append behavior, publication/recovery behavior, binary and direct-operation names, and Ticket/child-Mission ownership rejection.
- The API owns domain/file work. The facade and transports must not allocate IDs, resolve mission paths, mutate manifests/history, build Markdown, or derive predecessors. Retain a wire DTO only where it is a transport schema.
- Treat external Ticket and child-Mission ownership refs as opaque domain constraints; do not widen or rewrite them. Verify suspected duplication before deletion.

## Acceptance examples
- Creating through each existing entry point returns an API-allocated UUID and revision 1.
- A stale expected revision returns the existing diagnostic and leaves accepted data unchanged.
- Existing edit/read/result/publication-recovery calls retain their observed results; foreign ownership still rejects mutation; initialized MCP has CLI-equivalent status/diagnostics.

## Validation
Add focused `ownership_api` tests for create/bootstrap, every existing mutation/read operation, stale-no-write, history/result preservation, foreign ownership, and CLI/MCP diagnostic equivalence. Run once: `cargo test -p mission-api --test ownership_api`; then once: `cargo check -p mission --features cli,mcp`. These are planned targets; do not run a full suite here.

## Non-goals
No schema-v2 source, draft/publish policy, transport additions, migration, guidance, scheduler, installer, Python fixture, denial check, Mission mutation, or commit during planning.

## Handoff
Owns the behavior-preserving API boundary required by W2. Commit checkpoint after execution: `refactor(mission): move lifecycle ownership to api`.
