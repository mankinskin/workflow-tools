# Generate client entry configs and hook manifests

## Context

Each client has a discovery entry point that is currently hand-maintained:

- **Copilot** — `.vscode/settings.json` keys `chat.instructionsFilesLocations`, `chat.promptFilesLocations` (and the `.agents/agents` entry), plus `.github/copilot-instructions.md`.
- **OpenCode** — `opencode.json` `instructions` array.
- **Cline** — `.clinerules/` layout plus `.clinerules/hooks/hooks.json`.

Hook manifests are duplicated: `.github/hooks/hooks.json` and `.clinerules/hooks/hooks.json` plus four `.sh` copies, all delegating to canonical scripts in `tools/agent-hooks/`.

## Scope

- Generate each client's discovery config from the profile, as a **merge** into existing JSON so unrelated user settings and the `mcp` block survive.
- Generate hook manifests for every client from one canonical hook definition set.
- Ensure generated JSON is stable-ordered so re-running produces no diff.

## Acceptance criteria

1. Installing a client wires up discovery with no manual editing.
2. Unrelated keys in `.vscode/settings.json` and `opencode.json` are preserved across regeneration.
3. Hook manifests for both clients derive from one source; adding a hook updates both.
4. Regeneration is byte-stable.
