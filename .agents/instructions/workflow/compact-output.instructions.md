---
description: "Use when configuring CLI output formats, choosing between TOON and JSON, or applying the rtk proxy for token-efficient command output."
applyTo: "**/*.sh,**/*.ps1"
---

## Compact-by-Default Output

All CLI commands that support it should produce **compact output by default**. Verbose or full output is on-demand only.

| Situation | Preferred form | Verbose fallback |
|---|---|---|
| Machine-readable output | `--toon` | `--json` |
| Structured readable output | `--toon` or default | `--json --verbose` |
| Human scanning | default (no flag) | `--verbose` or `--json` |

Rules:
- Prefer `rtk <cmd>` over bare `<cmd>` — rtk filters/compresses output automatically.
- When a command emits a stream of file paths for a downstream command, run that
  producer bare so filtering cannot alter pipeline filenames; otherwise keep the
  normal `rtk` preference.
- `rtk` is an optional proxy. If `command -v rtk` fails, run the bare command and note the missing proxy in the status summary; never block work waiting on it.
- When a CLI supports `--toon`, prefer `--toon` over `--json` for compact machine-readable output.
- Never request `--json` output and then discard most of it; use a targeted filter (jq, toon-rust) instead.
- For ticket CLI: default text output is already compact; use `--json` only when you need to extract fields with jq or pipe to another tool.

## TOON vs JSON

- **TOON** (`--toon`): compact, binary-ish encoding. Prefer for data exchange between tools in the same pipeline.
  Use `toon-format` / `toon-rust` crates for encode/decode instead of hand-rolled text transforms.
- **JSON** (`--json`): verbose but universally parseable. Use when piping to external tools (jq, Python, etc.) or when debugging schema issues.
- **Do not** request JSON output when a plain-text or TOON representation would suffice — JSON adds 40–80% token overhead for the same data.
