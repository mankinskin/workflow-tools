## Objective

Author the `workflow-skill` `SKILL.md` package content: a skills.sh-native skill (compatible with VS Code Copilot by-description loading) that teaches an agent to discover, install, and use the workflow-tools bundle via `install.sh`/`install-ctl`/`bootstrap.sh`.

## Requirements

- `SKILL.md` frontmatter follows the Agent Skill Foundation contract (`b13c5d89`).
- Content documents: what workflow-tools is, how to bootstrap it (`install.sh` → `install-ctl` TUI → `bootstrap.sh`), and the discover/plan/execute/feedback loop against the installed tools (ticket, spec, session, test, feedback, etc. MCP/CLI surfaces).
- Description is written for by-description auto-loading: a short, specific trigger description an agent host can match against user intent without loading the full body.
- No skills.sh publication or install-ctl TUI changes in this ticket — authoring the package content only.

## Acceptance Criteria

- `SKILL.md` exists with valid frontmatter and passes any repo-level skill lint/validation used for other skills in this workspace.
- A reviewer can follow the skill's instructions from a clean checkout through a working ticket/spec operation without consulting other docs.

## Validation

Manually walk the skill's documented steps in a scratch consumer directory and confirm each command executes as written.