# Author the GitHub Copilot / VS Code client profile

## Context

Copilot discovery is wired through `.vscode/settings.json`:

```jsonc
"chat.instructionsFilesLocations": { ".agents/instructions": true }
"chat.promptFilesLocations":       { ".agents/prompts": true }
                                   { ".agents/agents": true }
```

Global entry point is `.github/copilot-instructions.md`, which points at `AGENTS.md`.

## Scope

Templates for each surface, with the measured frontmatter contracts:

- **Instructions** — `description` required; `applyTo` optional. Nested folder layout under `.agents/instructions/<workflow>/`, plus `INDEX.md` and `README.md`.
- **Agents** — `name`, `description`, `tools` (list), `argument-hint`, `user-invocable`.
- **Prompts** — `description`, `agent`, `name`, `argument-hint`.
- **Skills** — `name`, `description`, plus optional `allowed-tools`, `license`, `metadata`, `compatibility`, `applyTo`.
- **Root** — `AGENTS.md` and `.github/copilot-instructions.md`.

## Acceptance criteria

1. Rendering reproduces every current file byte-identically against the golden fixtures.
2. Frontmatter key order and casing match the current files exactly.
3. List-valued `tools` renders in the current inline style.
