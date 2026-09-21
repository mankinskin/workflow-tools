# Problem

`ticket board show` exposed a stale entry with an `entry_id`, `ticket_id`, `agent_id`, and owned files, but `ticket board check-out <ticket-id>` still failed with `no active board entry found` until the operator manually repeated the agent identity.

That is too strict for stale-entry cleanup. The stale entry was real; the checkout path was just missing a direct way to act on the thing that `board show` had already identified.

# Session Evidence

- The session observed a stale board entry for ticket `deeeb26d` after the ticket had already moved to `in-review`.
- `board show` returned the stale `entry_id` and `agent_id`.
- `board check-out deeeb26d ...` failed until `--agent "GitHub Copilot"` was added explicitly.
- The failure message did not suggest `entry_id` checkout, stale-entry inference, or the exact retry shape.

# Scope

1. Add a direct checkout path by `entry_id`.
2. Allow `board check-out <ticket-id>` to infer the target entry when exactly one active or stale entry matches the ticket.
3. Improve error messages for stale-entry cleanup so they explain whether the command needs `--agent`, `--entry-id`, or a narrower selector.
4. Return the same stale-aware guidance in JSON output for machine clients.
5. Add MCP parity so board cleanup workflows do not require a CLI-only escape hatch.

# Regression Validation Requirements

- **Specification / docs:** define checkout semantics for active vs stale entries and the precedence rules for `ticket-id`, `agent`, and `entry-id` selectors.
- **CLI:** add integration coverage for active-entry checkout, stale-entry checkout by `entry_id`, and stale-entry checkout by unique ticket inference.
- **MCP:** add parity coverage for stale-entry release and selector conflict handling.
- **Board consistency:** include a regression where `board show` exposes a stale entry and the next suggested cleanup command succeeds without manual trial and error.
- **Manual validation:** reproduce the `deeeb26d` stale-entry scenario and confirm one retry-free checkout path works.

# Acceptance Criteria

- A stale board entry can be checked out directly by `entry_id`.
- A unique stale entry for a ticket can be checked out without manually restating the agent id.
- Error output tells the user exactly how to recover when the selector is ambiguous.
- CLI and MCP surfaces expose the same stale-entry cleanup semantics.
- The documented cleanup flow matches the behavior of `board show` and `board check-out`.

# Likely Surfaces

- `tools/ticket-cli/`
- `tools/ticket-mcp/`
- `crates/ticket-api/`
- `.agents/instructions/ticket-system.instructions.md`
