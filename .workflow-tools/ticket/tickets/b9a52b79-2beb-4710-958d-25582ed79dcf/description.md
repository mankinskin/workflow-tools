Phase D. Author the `workflow-skill` — a skills.sh-native SKILL.md package that is the installable entry point for the whole workflow-tooling system. A single install should be able to discover, retrieve, and install all tool repos, use their artifacts, and drive the agent workflow.

## Scope
- Author SKILL.md (skills.sh-native, compatible with VS Code Copilot by-description loading), building on the Agent Skill Foundation contract (`b13c5d89`).
- Skill knows all tools and can retrieve/install them (pulls workflow-tools and its aggregated repos).
- Skill exposes the guidance/workflows that use the tools (discover tools -> find next tasks -> build workflow -> gather feedback -> self-improve).
- Publish to skills.sh so a user only downloads this one skill to bootstrap everything.

## Acceptance criteria
- `workflow-skill` installs from skills.sh and bootstraps workflow-tools + tools + artifacts.
- Skill drives the discover/plan/execute/feedback loop against the installed tools.
- Compatible with by-description loading in VS Code Copilot.

## Dependencies
- Blocked by umbrella creation.
- Builds on Agent Skill Foundation epic `b13c5d89` (linked).