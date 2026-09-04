---
agent: agent
description: "Regression probe for per-template MCP tool-grant scoping (ticket cd19fed4 / spec ec3b13f1). Re-run to detect drift back toward wildcard grants."
---

# Tool-Grant Regression Probe

Detects drift back toward wildcard MCP tool grants (ticket
[cd19fed4](../../.ticket/tickets/cd19fed4-44d5-4ef0-848c-19753f1539b0/ticket.toml),
spec [ec3b13f1](../../.spec/specs/ec3b13f1-ae9f-4f11-b3f9-e8fa3877afbd/spec.toml)).
Re-run this probe whenever `.agents/agents/explore.agent.md` changes, or periodically,
to confirm the Explore Agent's advertised tool surface has not regressed.

## Method

Both the "before" and "after" measurements below use the **same** self-report
method for consistency:

1. Spawn the Explore Agent (`agentName: "Explore Agent"`) with exactly this prompt
   (nothing else — no repo context, no prior conversation):

   > Self-report only. Do not explore the repo, do not read any files, do not run
   > any commands. In your single reply, list: (1) the total number of tools
   > currently available to you in this turn, counting every distinct tool name
   > once (built-in tools plus every MCP tool from every connected MCP server);
   > (2) a breakdown by MCP server name showing how many tools each server
   > contributes; (3) whether a tool literally named `tool_search` (or an
   > equivalent deferred-tool-loading/discovery tool) is present in your tool list.
   > Return only these three facts as a compact list — no other narration.

2. Record the reported total tool count and per-server breakdown below.
3. Derive an approximate schema-token figure from the per-server breakdown using
   the same char-count-to-token method as the original probe
   (`tools/subagent_cost_probe` era measurement): sum each server's known/measured
   schema character size for the tools actually granted, divide by ~4 chars/token.
   This keeps the before/after comparison method-consistent even though neither
   figure is a tokenizer-exact count.
4. Fail the probe (flag drift) if the reported total exceeds **60 tools**, or if
   the derived schema-token figure exceeds **10,000 tokens**.

## Measurement Log

| Date | Total tools | MCP tools | Built-in tools | Schema tokens (derived) | tool_search present | Notes |
|---|---|---|---|---|---|---|
| 2026-07-28 (before) | 172 | 143 | 29 | ~24,000 (char-derived, ticket cd19fed4 baseline) | No | Wildcard grants: `audit-mcp/*`, `context-mcp/*`, `feedback-mcp/*`, `fs-mcp/*`, `log-viewer-mcp/*`, `peek-mcp/*`, `rule-mcp/*`, `session-mcp/*`, `spec-mcp/*`, `test-mcp/*`, `ticket-mcp/*` |
| 2026-07-28 (after) | ~28 | ~25 | 3 (`read`, `search`, `execute`) | ~3,400 (char-derived, proportional to ticket-mcp/spec-mcp per-tool averages + full peek-mcp) | No (documented unavailable, see below) | Explicit grant: `peek-mcp/*` + 13 read-only `ticket-mcp` getters + 8 read-only `spec-mcp` getters; `session-mcp`/`context-mcp` dropped entirely |

## tool_search / Deferred-Tool-Loading Resolution (AC4)

**Documented unavailable, not enabled.** `tool_search` is a VS Code client-level
mechanism for lazily loading deferred MCP tools; it is not a repository-controlled
setting and no callable tool named `tool_search` (or equivalent) appears in any
probed sub-agent's tool list, including after this ticket's changes. The
self-reporting Explore Agent run above confirms deferred tools are exposed as
directly callable functions in the sub-agent's fixed tool list, not gated behind
a discovery call — this repository's `.agent.md` `tools:` frontmatter allowlist
is therefore the only available enforcement point, and this ticket's per-template
scoping is the substitute for lazy discovery rather than a workaround pending it.
If a future VS Code/Copilot release exposes `tool_search` to custom sub-agents,
re-run this probe to detect the new tool and re-evaluate whether explicit grants
are still needed.
