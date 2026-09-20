## Summary

Replace the fragile file-based model-identity mechanism with a **Rust MCP middleware** that requires the active model to be supplied **with every `tools/call`**. This removes shared mutable state (the `.active-model` file can desync or be overwritten) and makes enforcement authoritative and per-request.

Supersedes the file-updater approach in [1157638e](../1157638e-edfe-4d29-a6ac-fc73010d5dd8/ticket.toml); refines BLOCK-3 for [a5ad2721](../a5ad2721-2d07-47dd-85f5-f180d4a030fa/ticket.toml). Part of [445a2d76](../445a2d76-5795-4d7a-aec8-d1536ec61416/ticket.toml).

## Mechanism

- A stdio JSON-RPC proxy binary (`mcp-cost-gate`) fronts each real MCP server: `mcp-cost-gate -- <real-server> [args]`.
- On `tools/list` responses, it injects a **required** `caller_model` string property into every tool's `inputSchema`, so the model is obligated by schema to supply its identity on each call.
- On `tools/call`, it reads `arguments.caller_model`:
  - missing/empty ⇒ return an error result telling the caller to include it (no forward);
  - resolve `output_mtok` from `model_prices.json` and apply the gate: `delegate` ⇒ refuse with delegation guidance; `allow` ⇒ strip `caller_model` and forward the cleaned call to the real server.
- All other traffic passes through unchanged.

## Scope / Deliverables

1. New Rust crate `mcp-cost-gate` (workspace member + `install-tools.sh` + `--mcp` set).
2. Rust port of the cost gate: price-table load, `output_mtok` resolution, threshold `X`, tool classification.
3. Pure, unit-tested interception functions (client message ⇒ forward/respond; server message ⇒ schema-injected).
4. Rewire [.vscode/mcp.json](../../../.vscode/mcp.json) and [.github/mcp.json](../../../.github/mcp.json) to launch every server through `mcp-cost-gate`.

## Acceptance Criteria

- Every advertised tool exposes a required `caller_model` argument.
- A `tools/call` without `caller_model` is rejected with guidance.
- An expensive model + token-heavy tool is refused with delegation guidance; a cheap model passes; `caller_model` is stripped before reaching the real server.
- Cost is resolved from the shared `model_prices.json`, not hardcoded.
- No shared mutable file is involved.