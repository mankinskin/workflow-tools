# Implement full sync-rules round-trip over structured metadata

## Context

Because all client outputs become gitignored and generated, editing a generated file is the *only* ergonomic authoring path — and the confirmed decision is that editing the generated file, then reverse-syncing, is the authoring loop.

Spec `a969562b` (`rule-system/single-target-reverse-sync`, component `rule-cli`) covers `rule sync-rules` reverse-sync including frontmatter round-trip (AC7), but it is state **draft**. That makes it a hard blocker for the cutover rather than a nice-to-have.

Once frontmatter moves to structured metadata, reverse-sync must reconstruct *metadata*, not re-attach a YAML block to the first body.

## Scope

- Promote spec `a969562b` from draft and align it with the structured-metadata model.
- Implement reverse-sync across every surface and every client profile: parse provenance markers (`<!-- rule-api:entry id=<uuid> slug=<slug> -->`), attribute each block to its owning rule, and write body plus metadata back.
- Handle the case where a client profile is lossy (e.g. Cline emits no frontmatter): reverse-sync from a lossy profile must not erase metadata it cannot observe.
- Reject reverse-sync from a file whose provenance markers were removed or reordered, with actionable recovery guidance.

## Acceptance criteria

1. Editing prose in any generated file and running `sync-rules` updates the correct rule body.
2. Editing frontmatter in a Copilot output updates structured metadata, not a body.
3. Reverse-sync from Cline output never clears metadata absent from Cline's format.
4. A file with tampered markers is rejected rather than partially applied.
