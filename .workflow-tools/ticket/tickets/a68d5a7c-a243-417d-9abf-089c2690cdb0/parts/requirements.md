# W3 — Thin facade, CLI, and MCP adapters

## Objective
Expose W2 API authoring operations through thin shared-facade, CLI, and MCP transports with equivalent diagnostics and no client-owned identity, files, manifests, Markdown, or predecessor construction.

## Requirements and design
- Preserve legacy operation names and behavior for schema-v1 records. Add typed create/edit/save/publish/readback inputs as inline typed values plus expected revision; adapters return API IDs, revisions, source refs, state, and diagnostics.
- Facade/transports forward API requests; they never allocate UUIDs, resolve paths, write source/Part files, copy prompt into Part, construct manifests/Markdown, derive serial links, execute commands, or schedule work.
- Exercise real initialized MCP stdio (`initialize`, notification, `tools/list`, `tools/call`) and CLI calls for save/publish/readback. Compare equivalent malformed, stale, and publish-not-ready diagnostics and no-write outcomes.

## Examples
CLI saves an empty draft and MCP reads the returned ID/revision/source refs. Both reject publish for missing readiness fields and, after valid data, return the same API-derived predecessor and publication result.

## Validation
Add focused `direct_transports` CLI/MCP tests, enabled by the actual `cli,mcp` features. Run once: `cargo test -p mission --features cli,mcp --test direct_transports`. Full package suites are reserved for final W5.

## Non-goals and handoff
No typed-source semantics, migration, guidance, generated-target edit, Mission mutation, or commit during planning. W5 depends on this delivered user-facing contract. Execution checkpoint: `feat(mission): adapt transports to typed api`.
