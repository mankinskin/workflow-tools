# Author the Cline client profile

## Context

Cline is the existing working precedent. `rule-targets/25-cline.yaml` declares `defaults: { repo_scope: context-engine, file_kind: AGENTS }` and a `folders:` tree rooted at `.clinerules` with four `files:` entries. `10-core-rules.md` selects 8 child sections: `agent-rules/operating-principles`, `.../discovery-protocol-before-editing`, `.../task-routing`, `.../quality-gates`, `.../feedback-workflow`, `.../escalation-rules`, `.../fallback-mode-when-mcp-is-unavailable`, `.../canonical-sources`.

Cline uses **no frontmatter** — four flat numbered markdown files. Hooks live at `.clinerules/hooks/hooks.json` mirroring `.github/hooks/hooks.json`, both delegating to canonical scripts in `tools/agent-hooks/`.

Spec `7b0ad285` (`context-engine/repo-guidance/cline-agent-integration`, draft) is the existing anchor.

## Scope

- Port `25-cline.yaml` onto the client-profile model.
- Templates emit no frontmatter and the flat numbered layout.
- Generate `.clinerules/hooks/hooks.json` and its shell wrappers from the same canonical hook definitions as `.github/hooks/`.
- Update spec `7b0ad285` to reference the profile model.

## Acceptance criteria

1. The four `.clinerules/*.md` files render byte-identical to the current committed output.
2. Cline hook manifests are generated, not hand-copied.
3. `25-cline.yaml` no longer duplicates selection logic that the profile provides.
