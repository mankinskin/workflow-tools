# Re-widen the pre-commit drift gate

## Context

`.githooks/pre-commit` was deliberately narrowed away from `.agents/**`. It carries the explicit comment: `.agents/** are hand-owned (not produced by rule-targets.yaml); do NOT trigger the sync-targets drift gate on them.` The trigger regex is `^(rule-targets\.yaml|\.rule/.*|\.clinerules/.*|\.github/README\.md)$`, plus three per-submodule triggers.

There are also no rule tasks in `Makefile.toml` — rule sync is pre-commit plus manual CLI only.

## Scope

- Re-widen the trigger to cover the rule stores and the rendered golden fixtures.
- Because generated outputs become gitignored, the gate must compare the **rule store against the fixtures**, not against the working-tree outputs.
- Remove the now-obsolete narrowing comment and replace it with the current contract.
- Add a `cargo-make` task so the same check runs outside the hook.
- Keep the gate fast enough for every-commit use; measure and record the added time.

## Acceptance criteria

1. Changing a rule body without regenerating fixtures fails the commit.
2. The gate does not reference gitignored outputs.
3. The same check is invocable via `cargo make`.
4. Added pre-commit time is recorded in the ticket.
