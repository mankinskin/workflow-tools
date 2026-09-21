## Objective

Author the hand-owned `.agents/skills/pdf/SKILL.md` skill doc and register it in the skills Master Index, instructing agents to prefer `pdf-mcp` named tools with `pdf-cli` documented as fallback.

## Target Files

- `.agents/skills/pdf/SKILL.md` (new)
- `.agents/skills/README.md` (add one row to the Master Index "Hand-owned skills" table)

## Design

`SKILL.md` frontmatter (required, per `.agents/skills/README.md`'s Skill Directory Contract):
```yaml
---
name: pdf
description: Extract text/images from PDFs, edit existing PDFs (merge, split, reorder/delete pages, metadata), create new PDFs (programmatic or via typst-cli), all through root-confined pdf-mcp named tools with pdf-cli as fallback. Trigger on any PDF read/write/create/merge/split task.
---
```
(Exact wording may be refined, but must satisfy the by-description loading contract: an agent can decide applicability from the description alone.)

Body content must:
- Document each of the six v1 capabilities and which `pdf-mcp` named tool (from T7) and which `pdf-cli` subcommand (from T6) implements it.
- State the sandboxing contract: every operation requires an explicit confinement root; paths outside the root are rejected.
- State the write-safety contract: outputs require an explicit path; in-place overwrite requires an explicit `overwrite: true`/`--overwrite` flag; nothing is silently clobbered.
- State that image extraction (T9) may be unavailable/limited depending on T0's crate findings, and that the typst-cli creation path requires `typst-cli` on `PATH` and degrades with a clear error otherwise.
- Explicitly instruct: prefer `pdf-mcp` named tools; use `pdf-cli` only as a fallback when MCP tools are unavailable — mirroring how `peek-mcp`/`peek-cli` fallback guidance is documented elsewhere in this repo (check `token-optimized-agentic-engineering` skill or `.agents/instructions/orchestration/file-inspection.instructions.md` for the peek fallback phrasing pattern to mirror in tone).
- Must NOT carry a `<!-- rule-api:file generated=true -->` header (this is hand-authored, not rule-mcp generated).

`.agents/skills/README.md` Master Index update: add a row to the "Hand-owned skills" table:
```
| [pdf](./pdf/SKILL.md) | Extract/edit/create/merge/split PDFs via root-confined pdf-mcp named tools (pdf-cli fallback): text/image extraction, page ops, metadata, PDF creation (programmatic or typst-cli). |
```

## Acceptance Criteria

- [ ] `.agents/skills/pdf/SKILL.md` exists with valid YAML frontmatter containing exactly `name: pdf` and a `description` field usable for by-description loading.
- [ ] The file contains no `rule-api:file generated=true` header.
- [ ] All six v1 capabilities are documented with their corresponding `pdf-mcp` tool name and `pdf-cli` subcommand.
- [ ] The sandboxing (confinement root) and write-safety (explicit output + overwrite flag) contracts are both stated explicitly.
- [ ] The doc explicitly instructs agents to prefer `pdf-mcp` and use `pdf-cli` only as fallback.
- [ ] `.agents/skills/README.md`'s Master Index "Hand-owned skills" table has exactly one new row linking to `./pdf/SKILL.md`.

## Validation Plan

Manual review: confirm frontmatter parses as valid YAML with required keys, confirm the README table row renders as valid markdown, and confirm no generated-file header is present. No automated test applies to hand-authored documentation; validation is manual content review against the acceptance criteria above.