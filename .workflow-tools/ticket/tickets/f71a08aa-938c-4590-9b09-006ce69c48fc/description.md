# Canonical Mission and Session operational guidance

## Goal and scope
DOC-1: Update only canonical Mission/Session READMEs, superproject Mission tooling instructions and changed roadmap-authoring instructions; generated projections remain read-only.

## Acceptance and validation
The explicit objective and requirements Parts are the complete current plan; do not infer completion from a preview or old note.
- `rtk git diff --check -- .agents/instructions/workflow/mission-tooling.instructions.md`
- `rtk git -C workflow-tools diff --check -- mission/README.md session/README.md .agents/instructions/workflow/roadmap-authoring.instructions.md`
- `Semantic link and CLI/MCP example comparison against W1-W8 recorded results`

## Non-goals
Product behavior changes, generated guidance edits, repeated broad suites or live operations disguised as docs.

## Status
Planning preparation only; implementation and live operations have not started. Use the linked owning specification and requirements through the plan view.
