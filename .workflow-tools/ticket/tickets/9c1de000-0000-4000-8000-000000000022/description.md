# Live client smoke test

## Context

Golden fixtures prove rendering is stable; they do not prove a client actually *loads* the result. Each client has a different discovery mechanism, so each needs a real load check.

## Scope

For each of Copilot, Cline, and OpenCode:

1. Install into a clean temporary checkout.
2. Launch or configure the client against that checkout.
3. Confirm the guidance is discovered: instructions listed, agents selectable, prompts invocable, skills visible.
4. Record the evidence in the test store via `test record-execution`, linked to this ticket.

For the Copilot surface specifically, verify discovery through the `.vscode/settings.json` keys rather than assuming path conventions.

## Acceptance criteria

1. All three clients load installed guidance from a clean checkout.
2. Evidence is recorded in the test store and linked to this ticket.
3. The procedure is documented well enough to repeat by hand.
