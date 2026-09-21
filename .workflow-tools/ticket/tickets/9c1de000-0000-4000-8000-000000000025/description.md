# Documentation cutover

## Scope

- Update root `README.md` with a setup stage covering client selection and `install-guidance.sh`.
- Update `AGENTS.md` sections that describe guidance files as hand-owned.
- Update `.agents/instructions/**` fragments referencing the old ownership model — specifically the commit-workflow and generated-files instructions, which currently document which files are rule-generated.
- Update `.rule/README.md` and the store catalog to describe the client-profile model.
- Update spec `7b0ad285` (Cline integration) and spec `a9b7ef39` (skill infrastructure) so they no longer assert the decommissioned direction.
- Add a short "how to add a new client" guide covering profile authoring, template conventions, and fixture requirements.

## Acceptance criteria

1. No committed document describes `.agents/**` as hand-owned.
2. The setup path is discoverable from `README.md` and `AGENTS.md`.
3. Adding a fourth client is documented end to end.
4. Affected specs are updated and cross-linked to the anchor spec.
