## Overview
Implement durable, auditable budget-offset grants in session-api with create/list/revoke operations and grant_id-based resolution.

## Key Design Points
- **Storage**: durable grant record in session-api (memory-api/crates/session-api)
- **Fields**: grant_id, scope (session-wide | sub-agent-spawn), offset value, created_at, revoked_at, metadata
- **Operations**: create, list, revoke, get-by-id
- **Scopes** (v1): session-wide, sub-agent-spawn
- **grant_id reference**: gate (T3) resolves offset by reading a grant by id
- **Audit trail**: grants are durable + auditable; offset is never self-declared
- **Surfaces**: CLI (memory-api/tools/cli/session-cli) + MCP (memory-api/tools/mcp/session-mcp)

## Acceptance Criteria
- [ ] Grant record schema: grant_id, scope, offset, created_at, revoked_at, metadata
- [ ] Create/list/revoke operations in session-api
- [ ] get_grant(grant_id) → offset | None
- [ ] Session-wide and sub-agent-spawn scopes work
- [ ] Revoke sets revoked_at and excludes grant from active lookups
- [ ] CLI: session-cli grant create|list|revoke|get subcommands
- [ ] MCP: session-mcp grant tools (create_grant, list_grants, revoke_grant, get_grant)
- [ ] Tests: CRUD operations, scope filtering, revoke semantics, lookup correctness
- [ ] Gate (T3) integration: gate reads grant by id and applies offset

## References
- Depends on: T1 (tool_metrics core for offset context)
- Depended on by: T3 (graded cost model), T6 (escalation workflow)
- Target: memory-api/crates/session-api, memory-api/tools/cli/session-cli, memory-api/tools/mcp/session-mcp
