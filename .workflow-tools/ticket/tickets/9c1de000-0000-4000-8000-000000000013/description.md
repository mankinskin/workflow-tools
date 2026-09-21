# `rule install --client <name>` subcommand

## Context

The `rule` CLI (`memory-api/tools/cli/rule-cli`, binary `rule`) has `generate-file`, `generate-target`, `sync-targets`, `sync-rules`, `scan`, `import-file`, `benchmark-targets`, `missing-rule`, `store-index`, `add-root`, `move`, `init`.

Note: `sync-targets` is **CLI-only** — no MCP tool exists — so any install flow must shell out to the binary.

## Scope

- New `rule install` subcommand:
  - `--client <name>` (repeatable; `copilot`, `cline`, `opencode`)
  - `--surface <name>` (repeatable; defaults from the manifest) covering instructions, agents, prompts, skills, root guidance, client entry config, hooks
  - `--skills <name>` for vendored-skill selection
  - `--dry-run`, `--force`, `--check`
- Resolve profiles, render every selected surface, write outputs, and update the lockfile.
- Idempotent: a second run with identical input writes nothing and reports no changes.
- `--check` exits non-zero when installed output differs from what would be rendered.

## Blocker to handle first

The local `rule.exe` build is currently stale: `sync-targets --config rule-targets.yaml --check` fails with `workspace not initialized at rule-targets/../memory-api\.spec: run the 'init' command`. Rebuild and fix workspace resolution before wiring install on top.

## Acceptance criteria

1. `rule install --client copilot` materializes every default Copilot surface.
2. Repeated runs are no-ops.
3. `--dry-run` writes nothing and prints the full plan.
4. `--check` detects a single hand-edited byte in any installed file.
