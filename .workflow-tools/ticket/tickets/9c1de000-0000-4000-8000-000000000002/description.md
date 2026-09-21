# Re-scope 23e81ad8 into the multi-client track

## Context

Ticket `23e81ad8` ("[rule+skill] Rule-store sources for domain-store scaffolding instructions", `priority: high`, `effort: 700`) mandates creating canonical rule entries **and generation targets** for instruction files and slash-command prompt assets.

This contradicts `f43cb5cb`/`76d0ace3`/`16cfd19f` head-on, and it predates the multi-client decision. It is the only existing ticket already pointing in the new direction, so it must be absorbed rather than left dangling as a contradiction.

## Scope

- Rewrite `23e81ad8` so its scope is the domain-store scaffolding *fragments*, not a parallel generation mechanism.
- Make it depend on the client-profile work so it consumes the new templating path rather than re-adding raw targets.
- Link it to the anchor spec and to this epic.
- Audit for any other ticket that assumes the old single-client generation model and reconcile the same way.

## Acceptance criteria

1. `23e81ad8` no longer describes a mechanism that conflicts with the multi-client design.
2. It has an explicit dependency on the client-profile work.
3. A store-wide search for tickets mandating direct `.agents/**` rule targets returns only tickets in this epic.
