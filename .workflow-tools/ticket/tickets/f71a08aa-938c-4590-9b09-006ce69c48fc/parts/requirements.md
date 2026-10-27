# W9: Canonical Mission and Session operational guidance

- DOC-1: Update only canonical Mission/Session READMEs, superproject Mission tooling instructions and changed roadmap-authoring instructions; generated projections remain read-only.
- DOC-2: Document API-owned typed authoring/UUID/bootstrap, independent JSON Part sources and generated Markdown output, save versus publish, explicit/configured/cwd roots and canonical workspace.
- DOC-3: Document delivered repair/move journal IDs, full fingerprints/CAS, no-op/restart/compensation and approval boundaries, plus Session set-move and production-v1 schema contracts.
- DOC-4: Examples and final status correspond to actual W7 live results and W8 independent verification. Incomplete migration cannot be described as completed; canonical destination and current execution-Mission root are explicit.
- DOC-5: Validate changed links and actual CLI/MCP example fields against existing focused evidence, plus repository-specific whitespace checks. Do not repeat unchanged provider suites or use keyword grep as semantic proof.

## Failure and continuation
An example/field/owner mismatch is corrected in its canonical source. New unrelated generator configuration is not guessed, generated copies are not hand-edited, and failing live work is not papered over by successful docs.
Apply common and per-waypoint branches from [outcome contract](../../../../../../transcripts/24-10-2026_mission-typed-authoring/OUTCOMES.md); after native binding that content must be embedded in the accepted Mission, not require this dossier at runtime.

## Validation and checkpoint
- `rtk git diff --check -- .agents/instructions/workflow/mission-tooling.instructions.md`
- `rtk git -C workflow-tools diff --check -- mission/README.md session/README.md .agents/instructions/workflow/roadmap-authoring.instructions.md`
- `Semantic link and CLI/MCP example comparison against W1-W8 recorded results`
Focused selectors run once per owning changed revision, nonzero cases mandatory; no Python, active-denial machinery or unchanged broad final reruns. Planned guard `mission-recovery-guidance` is registered/read back as execution evidence, never claimed run in planning.

## Scope and authority
Product behavior changes, generated guidance edits, repeated broad suites or live operations disguised as docs.
Planning correction is not execution. Do not mutate live Mission/Session entities until the applicable execution and exact-set approval gates pass.
[Owning specification](../../../../spec/specs/a404c898-c859-485d-99c0-0bc539312477/body.md)
[Detailed execution package](../../../../../../transcripts/24-10-2026_mission-typed-authoring/09-guidance.md)
