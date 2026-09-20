## Objective

Make the `install-ctl` interactive TUI selection flow (tools, agent client tools, instructions, hooks, installation home) the same flow `workflow-skill` documents and drives, so the skill's bootstrap instructions and the actual installer behavior never drift apart.

## Requirements

- `install-ctl`'s TUI exposes exactly the artifact categories `SKILL.md` describes (tools, instructions, hooks) sourced from `install/artifacts.toml`.
- The skill's documented bootstrap sequence invokes `install-ctl` non-interactively where needed for automated validation (e.g. via `ratatui-testlib` or a scriptable flag), without diverging from the interactive path a human would take.
- No behavior change to `bootstrap.sh`'s ticket/spec-only path; this covers the broader tool/instruction/hook selection surface only.

## Acceptance Criteria

- Running the documented skill bootstrap sequence end to end installs the same artifact set an interactive `install-ctl` session would produce for the same selections.
- A scripted (`ratatui-testlib`-driven) run of `install-ctl` matching the skill's documented selections completes without manual intervention.

## Validation

Drive `install-ctl` with `ratatui-testlib` using the skill-documented selections and diff the resulting installed artifact set against the interactive TUI's output for the same choices.