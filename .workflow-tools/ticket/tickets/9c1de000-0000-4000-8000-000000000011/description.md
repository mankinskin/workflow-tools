# Author the OpenCode client profile

## Context

OpenCode currently consumes `.agents/` through a single pointer. `opencode.json` at repo root is **hand-maintained** and declares:

- `"instructions": [".agents/instructions/INDEX.md"]` — one guidance entry point
- an `mcp` map wrapping every MCP server in `mcp-cost-gate` with `COST_GATE_*` env vars

`gen_opencode_config.py` is **unrelated** to guidance: it fetches the Eden AI model catalog, filters to function-calling models, converts pricing to per-million, and writes an `edenai` provider block to `~/.config/opencode/opencode.json` in the user's home directory. It must not be conflated with this work.

## Scope

- Determine whether OpenCode's discovery is best served by the current single-INDEX pointer or by a fuller per-file surface; encode that choice in the profile.
- Generate the guidance portion of `opencode.json` while preserving the hand-maintained `mcp` block. Generation must be a merge, not an overwrite.
- Render `INDEX.md` from the rule store rather than maintaining it by hand.
- Document the frontmatter contract OpenCode honours, or the absence of one.

## Open items

The exact OpenCode agent/prompt/skill frontmatter contract is not evidenced in this repo — only the `instructions` pointer is. Confirm against OpenCode documentation before finalizing the templates.

## Acceptance criteria

1. `opencode.json` guidance keys are generated; the `mcp` block survives regeneration untouched.
2. `INDEX.md` is generated and lists every installed instruction file.
3. Regeneration is idempotent.
