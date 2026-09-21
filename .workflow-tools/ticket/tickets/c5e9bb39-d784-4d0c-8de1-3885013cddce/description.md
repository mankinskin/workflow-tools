Build a generator that reads the ticket store (ticket-api) and emits a committed markdown index co-located in `.ticket/README.md` along with its TOON sidecar at `.ticket/index.toon`. The purpose is to give agents and humans a compact, stable view of the ticket store without scanning raw TOML files.

## Scope
- Implement a `store-index` subcommand (or extend `ticket-cli`) that reads all tickets from the active workspace, groups them by state and component, and emits `.ticket/README.md`.
- Each entry includes: ticket id (short), title, state, priority, summary extracted from the ticket description, and an `IndexRef` pointing to the canonical ticket.toml path.
- The output must conform to the `IndexEntry` schema (`0dba399a`).
- Emit an agent-consumable instruction hook under `.agents/` (`ContentKind::agent_hook`) pointing agents at `.ticket/README.md` / `.ticket/index.toon` (D1 third surface).
- Wire as a git pre-commit hook (D2): regenerates `.ticket/README.md`, `.ticket/index.toon`, and the `.agents/` hook when ticket store files under `.ticket/tickets/` are staged. Profile the command; if the pre-commit latency budget is exceeded, fall back to post-commit.
- All generated files committed to git (D5).

## Acceptance criteria
- Running the generator produces `.ticket/README.md`, `.ticket/index.toon`, and the `.agents/` hook.
- Each entry is an `IndexRef` with stable id, source path, and semantic summary.
- Index is grouped by state and by component.
- Re-running with unchanged input produces files with the same digest.
- Pre-commit hook completes within the profiled latency threshold; the measurement is recorded.

## Non-goals
- No central store folder outside `.ticket/`.
- Does not emit full ticket bodies, only summaries and references.

## Resolved design decisions
- D1: workspace-folder index in `.ticket/`, plus an `.agents/` agent-hook entry.
- D2: git pre-commit hook, profiled (post-commit fallback if budget exceeded).
- D5: outputs committed to git.