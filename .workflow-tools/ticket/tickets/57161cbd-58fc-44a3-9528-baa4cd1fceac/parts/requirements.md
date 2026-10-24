1. Add shared CLI and stdio-MCP operations that atomically create a Mission from nonempty title/objective and the complete ordered nonempty waypoint set; preserve stable Mission/waypoint IDs.
2. Require `expected_current_revision` for every existing-record write. Unknown IDs, stale revisions, invalid content/order/dependencies, or invalid evidence fail without any accepted-data mutation.
3. Add direct stored-ID `get`, validation, and publication operations; they take no external dossier/manifest input and return actual accepted ID, revision, content, validation/publication state, and evidence.
4. Record only local progress and typed operator results. Each result contains actual outcome, operator command, and evidence pointer; no Mission operation launches installation/tests/commands or an agent. Preserve derived Ticket/child-Mission authority and native bundle compatibility.
5. MCP validation is stdio JSON-RPC in isolated workspace cwd `/home/linus/.copilot/session-state/ae41c0ca-75fa-497e-b4ac-90e3fa04f6ec/files/mission-install-pilot-workspace`: `initialize` → `notifications/initialized` → `tools/list` → `tools/call`, with process cleanup. The MCP executable has no help-based validation surface.

## Acceptance criteria

- CLI and MCP parity tests demonstrate matching success/error shapes for direct operations and the MCP protocol/cleanup sequence.
- A fixture proves atomic create → objective/waypoint edit/order → stale rejection → typed result → validate/publish → get, using the real returned ID/revision and read-back of persisted content/evidence.
- Invalid or stale writes prove no accepted revision/content mutation; waypoint IDs stay stable through order changes.
- W1 evidence runs from `workflow-tools`: `cargo test -p mission --features cli,mcp`, `cargo test -p mission-api`, and `cargo check -p mission --features cli,mcp`.
- No Mission operation spawns installation/tests, a command, or an agent.
