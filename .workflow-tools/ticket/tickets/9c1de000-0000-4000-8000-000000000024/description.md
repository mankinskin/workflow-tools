# Big-bang untrack of generated guidance surfaces

## Context

Roughly 197 tracked markdown files (41 instructions, 16 agents, 24 prompts, 116 skill files) plus `.clinerules/` and `.github/copilot-instructions.md` become generated artifacts. The confirmed strategy is a single cutover commit rather than a phased or shadow-path migration.

This is the point of no return, so it is gated behind the entire validation phase.

## Scope

- Add gitignore entries for `.agents/**`, `.clinerules/**`, `.github/copilot-instructions.md`, and the selection lockfile, with an explicit negative rule preserving `AGENTS.md`.
- Single `git rm --cached` commit untracking all generated surfaces.
- Verify a fresh clone plus install reproduces the pre-cutover tree byte-identically.
- Record the pre-cutover tree hash in the ticket so the reproduction is checkable later.
- **Requires explicit user confirmation before execution** — this permanently ends per-file git history for the untracked guidance.

## Hard prerequisites

Golden fixtures, round-trip idempotence, drift gate, live client smoke tests, and overwrite protection must all be green first.

## Acceptance criteria

1. A fresh clone plus `./install-guidance.sh --client copilot` reproduces the pre-cutover `.agents/**` tree byte-identically.
2. The same holds for Cline and OpenCode against their surfaces.
3. `AGENTS.md` remains tracked.
4. No generated file is tracked after the cutover.
