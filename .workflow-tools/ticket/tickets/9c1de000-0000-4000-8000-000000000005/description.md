# Add structured guidance metadata to the rule entry schema

## Context

Client format differences are almost entirely *frontmatter* differences. Measured key sets:

| Surface | Files | Keys |
| --- | --- | --- |
| `.agents/instructions/**` | 39 with frontmatter | `description` ×39; `applyTo` ×1 |
| `.agents/agents/*.agent.md` | 16 | `name`, `description`, `tools`, `argument-hint` ×16; `user-invocable` ×15 |
| `.agents/prompts/*.prompt.md` | 24 | `description`, `agent` ×24; `name`, `argument-hint` ×20 |
| `.agents/skills/*/SKILL.md` | 11 | `name`, `description` ×11; `allowed-tools`, `license`, `metadata` ×2; `compatibility`, `applyTo` ×1 |
| Cline | 4 | none |
| OpenCode | — | single `instructions` pointer in `opencode.json` |

Today these live as raw YAML inside the *first matched rule's* `body.md` (27 of 120 root-store bodies begin with `---`), hoisted by `skip_provenance_for_yaml_frontmatter`. A shared fragment therefore carries one client's frontmatter, which blocks sharing entirely.

## Scope

- Extend `memory-api/crates/rule-api/schemas/rule-entry.toml` with optional structured metadata fields covering the union of the measured key sets: `description`, `apply_to`, `tools`, `argument_hint`, `user_invocable`, `agent`, `allowed_tools`, `license`, `compatibility`, `display_name`.
- Keep `file_kind` a string (it currently has 9 observed values across 4 stores and no enum).
- Model list-valued fields so a template can render them as a YAML sequence or a comma list per client.
- Extend `rule create` / `rule update` and the MCP equivalents to read and write the new fields.

## Acceptance criteria

1. Every measured frontmatter key across all four surfaces maps to a structured field.
2. Metadata is queryable via `rule list` / `rule search` filters.
3. Existing rules without metadata load unchanged.
