# Bootstrap layer: committed AGENTS.md pointing at install instructions

## Context

With all guidance outputs gitignored, a fresh clone would otherwise contain **zero** agent guidance — a bootstrap paradox, since an agent working in the fresh clone has no way to learn that an installer exists.

## Decision

`AGENTS.md` is the sole gitignore exemption. It stays committed and directs the reader to committed install instructions.

## Scope

- Keep root `AGENTS.md` committed. It remains generated, but it is also tracked.
- Add a committed install-instructions document describing client selection, `install-guidance.sh`, and available surfaces.
- Add a prominent section at the top of `AGENTS.md` stating that the remaining guidance surfaces are install-time artifacts and pointing at that document.
- Ensure the drift gate still validates `AGENTS.md` against the rule store even though it is tracked.
- Decide and document whether `.github/copilot-instructions.md` is gitignored (it is not exempt under the confirmed decision, so a fresh clone has no Copilot entry point until install runs — call this out explicitly in the install instructions).

## Acceptance criteria

1. A fresh clone contains `AGENTS.md` and the install-instructions document, and no other guidance surface.
2. `AGENTS.md` tells a cold-start agent exactly how to materialize the rest.
3. `AGENTS.md` is still covered by the drift gate.
