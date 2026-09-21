# Golden-file fixtures for every client profile

## Scope

- Snapshot the current committed state of every in-scope surface **before** any generation is re-enabled; these snapshots are the correctness baseline.
- Add a fixture test that renders each `(profile, surface)` pair and asserts byte-identical output.
- Cover: Copilot instructions / agents / prompts / skills / root guidance; Cline's four files plus hooks; OpenCode `INDEX.md` plus the `opencode.json` guidance keys.
- Include an intentionally awkward case per surface: an instruction file carrying `applyTo`, an agent with a long `tools` list, a prompt with no `name`, a skill with `metadata` and `compatibility`.
- Make fixture regeneration an explicit, reviewable command rather than an automatic side effect.

## Acceptance criteria

1. Every profile/surface pair has a fixture.
2. The suite fails on any single-byte rendering change.
3. Fixture updates require an explicit command and show a readable diff.
