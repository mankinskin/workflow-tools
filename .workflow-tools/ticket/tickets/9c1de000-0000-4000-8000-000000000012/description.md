# Guidance availability manifest and per-machine selection lockfile

## Context

There is currently **no** notion of installing guidance files. `install-tools.sh` (368 lines, 25 Rust binaries), `install-extensions.sh` (350 lines, one VS Code extension), and `install-deps.sh` (241 lines, `ripgrep`/`rtk`/`trunk`/`cargo-llvm-cov`) all source `tools/install/common.sh` and handle binaries and dependencies only. Grepping all three for `.agents|clinerules|copilot|AGENTS.md|opencode|skills` yields 3 hits, all for a binary named `copilot-capture-hook`.

`skills-lock.json` (`{ version, skills: { <name>: { source, sourceType: "github", skillPath, computedHash } } }`) is the only existing fetch-at-install-time precedent.

## Decision

A committed availability manifest plus a gitignored per-machine selection lockfile.

## Scope

- **Committed `guidance-install.toml`** — declares available clients, the surfaces each supports, the default surface selection, and available vendored skills.
- **Gitignored `.guidance-install.lock`** — records the selected client(s), selected surfaces, resolved output paths, and a content hash per generated file, so re-runs and drift detection are reproducible.
- Follow the `skills-lock.json` shape for the vendored-skill portion.
- CLI flags override the manifest per run and are written back to the lockfile.

## Acceptance criteria

1. The manifest validates against a schema; unknown clients or surfaces are rejected.
2. The lockfile is gitignored and machine-portable in structure but machine-specific in content.
3. Re-running install with no flags reproduces the previous selection exactly.
4. No absolute path appears in the committed manifest.
