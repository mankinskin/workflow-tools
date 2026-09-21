# Objective

Overhaul the repository guidance for describing target behavior so specs become concise behavior contracts anchored by the spec store, entity references, validation triangulation, and related implementation tickets.

# Why

The current spec entry shape and the surrounding agent guidance still mix behavior contracts with workflow narration, rollout prose, and housekeeping instructions. That makes specs noisy, inconsistent across prompts, and weak as the canonical description of what a component promises.

# Scope

- rewrite the canonical spec contract for component behavior around a short behavior story, provided surface contracts, executable validation, and related tickets
- remove unnecessary workflow prose from the spec contract and move ticket/process guidance to the right workflow surfaces
- align AGENTS, spec instructions, prompts, and the Spec Agent so they all describe the same behavior-first contract
- make validation triangulation explicit: executable tests, natural-language contract text, and code/schema references where available
- preserve entity-link based background knowledge instead of duplicating expanded entity contents inside specs

# Acceptance Criteria

- the owning spec entry defines specs as behavior contracts anchored in the spec database rather than workflow-heavy documents
- AGENTS plus the relevant spec/ticket prompts and agent guidance all converge on the same spec shape and responsibilities
- the guidance tells agents to describe new features as required validation steps across executable, natural-language, and code/schema forms when available
- the guidance explicitly treats full entity expansion as optional and prefers references plus context rendering
- focused validation demonstrates the updated guidance is structurally sound

# Validation

- focused reads/diff review of the rewritten guidance surfaces
- spec health for the updated canonical spec
- targeted file diagnostics for touched markdown/guidance files

# Restart Proof (2026-07-08)

- Guidance slice remains isolated to `AGENTS.md`, `.agents/instructions/spec-system.instructions.md`, and the touched prompt set under `.agents/prompts/`.
- Session-capture artifacts under `.session/` were explicitly excluded from the commit scope during restart review.
- Workspace spec health re-check passed cleanly: 190 specs checked, 0 issues.
- Root board ownership was re-established before packaging so the guidance and migration slices can be reviewed without unrelated file claims.

# Commit Scope

- `AGENTS.md`
- `.agents/instructions/spec-system.instructions.md`
- `.agents/prompts/handoff.prompt.md`
- `.agents/prompts/memory-setup.prompt.md`
- `.agents/prompts/spec.prompt.md`
- `.agents/prompts/tdd.prompt.md`
- `.agents/prompts/ticket-next.prompt.md`
- `.agents/prompts/ticket.prompt.md`
- `.agents/prompts/tickets.prompt.md`
- `.agents/prompts/user-training.prompt.md`
- exclude all `.session/` capture directories from review and commit prep
