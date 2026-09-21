# Multi-Client Guidance Rendering and Install-Time Materialization

Tracker for making the `.rule/` stores the single canonical source of agent guidance, rendering that guidance into client-specific output files via templates, and materializing those files at install time for the client the user selects.

Anchor spec: `rule-system/multi-client-guidance-rendering` (43b9a6ac).

## Why now

The `.clinerules/` target already proves the core mechanism: the same rule bodies that compose root `AGENTS.md` are re-projected into a different file layout for a different client. What blocks generalizing this is that per-client *format* differences (frontmatter key sets, file naming, discovery entry points) cannot be expressed — there is no templating engine and no front-matter field on `RenderTarget`.

## Phases

- **Phase 0 — Reconciliation.** Close the already-executed decommissioning work, purge machine-specific state, re-import current files as canonical bodies.
- **Phase 1 — Neutral model.** Promote frontmatter to structured rule-entry metadata so bodies become client-neutral prose.
- **Phase 2 — Templating.** Add minijinja, a client-profile model, and profiles for Copilot, Cline, OpenCode.
- **Phase 3 — Install.** Availability manifest, selection lockfile, `rule install --client`, `install-guidance.sh`, skill vendoring, client entry configs.
- **Phase 4 — Reverse sync.** Make the edit-generated-file loop a settled contract, plus overwrite protection.
- **Phase 5 — Validation.** Golden fixtures, round-trip idempotence, drift gate, live client smoke tests.
- **Phase 6 — Cutover.** Bootstrap layer, big-bang untracking, documentation.

## Done when

Every acceptance criterion in the anchor spec passes, and a fresh clone plus `install-guidance.sh --client <name>` yields a fully working guidance surface for each of the three v1 clients.
