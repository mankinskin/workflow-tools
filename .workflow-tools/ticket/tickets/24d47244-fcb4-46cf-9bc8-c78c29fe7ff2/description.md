Phase D. Define and implement the scope/precedence model for the workflow-skill so it works both at the repository root and at deeply nested tool repos without conflicting or being read twice.

## Requirements
- The skill must be installable at the workflow-tools root and at each individual tool repo (optional agent entry points).
- A single active installation must: know all tools, use the artifacts, consciously ignore other nested skill installations, and be able to uninstall itself.
- Clear scope plan so guidance is not loaded twice across nested install sites.

## Acceptance criteria
- Documented scope/precedence rules for root vs nested installs.
- Nested-install de-duplication verified (guidance loaded once).
- Self-uninstall path implemented and verified.
- End state: one downloaded skill installs and correctly uses all repos/tools.

## Dependencies
- Blocked by workflow-skill authoring.