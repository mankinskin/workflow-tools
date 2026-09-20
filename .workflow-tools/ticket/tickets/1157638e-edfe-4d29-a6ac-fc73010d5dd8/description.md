## Summary

Automatically populate the active-model marker that drives price-awareness enforcement, so no manual entry or prompt is ever needed.

The MCP cost-gate proxy ([tools/model-prices/mcp_cost_gate_proxy.py](../../tools/model-prices/mcp_cost_gate_proxy.py)) already resolves the caller model *fresh per `tools/call`* from `CALLER_MODEL_FILE` (default `.vscode/.active-model`, git-ignored). What is missing is an updater that writes the currently-active chat model to that file whenever it changes.

Part of: [445a2d76 Model price awareness](../445a2d76-5795-4d7a-aec8-d1536ec61416/ticket.toml); unblocks fully-automatic enforcement wired in [a5ad2721 MCP tool wrapper](../a5ad2721-2d07-47dd-85f5-f180d4a030fa/ticket.toml).

## Why this is needed

VS Code does not expose the active chat model to MCP servers, and MCP tool schemas are fixed so the model cannot inject its own identity into a `tools/call`. The only way to get automatic, dynamic model identity is for a component that *does* know the active model (a VS Code extension / hook) to write it to the marker file the proxy reads.

## Approach (candidate)

- Extend the existing `ticket-vscode` extension (or add a small dedicated one) to observe the active Copilot chat model.
- On model selection/change, write the model id to `${workspaceFolder}/.vscode/.active-model` (atomic write).
- Clear or leave stale-safe on shutdown (empty file = passthrough).
- Investigate whether the VS Code Language Model API surfaces the *user-selected chat model* to a passive extension; if not, document the closest available signal.

## Acceptance Criteria

- Switching the active model in chat updates `.vscode/.active-model` without user action.
- With an expensive model active, a token-heavy MCP tool call is refused with delegation guidance; with a cheap model active, it passes — no prompt, no manual env.
- If the model cannot be observed, the file is left empty (fail-open passthrough), never a wrong value.

## Notes

- Fallback remains: `CALLER_MODEL` env var and manual `echo <model> > .vscode/.active-model`.
- Proxy behavior is already validated (9 e2e tests, evidence `exec-vt-cost-gate-proxy-20260725`); this ticket only adds the writer.