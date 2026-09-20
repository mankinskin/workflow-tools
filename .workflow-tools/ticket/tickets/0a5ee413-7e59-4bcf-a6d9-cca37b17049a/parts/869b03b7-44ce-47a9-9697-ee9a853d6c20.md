## Objective

Publish `workflow-skill` to skills.sh so a user or agent host can install the entire workflow-tools bootstrap and guidance loop by downloading one skill.

## Requirements

- Package metadata satisfies skills.sh publication requirements (name, version, description, license).
- Publication is scripted/documented (not a manual one-off), pinned to a specific `workflow-tools` commit consistent with `install.sh`'s pin.
- Publishing does not require any local Cargo patch or vendored workflow-tools checkout.

## Acceptance Criteria

- `workflow-skill` is installable from skills.sh in a clean environment.
- Installing it and following its instructions bootstraps workflow-tools + tools + artifacts without additional undocumented steps.
- Compatible with VS Code Copilot by-description loading (per Agent Skill Foundation contract `b13c5d89`).

## Validation

Install the published skill from skills.sh in a fresh environment and complete one ticket/spec operation using only the skill's own instructions.