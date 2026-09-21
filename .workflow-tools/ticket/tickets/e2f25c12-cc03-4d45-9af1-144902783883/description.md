Approved scope for retiring the rule system:
- Rule content stores (.rule/** at repo root and in submodules) and all rule-targets.yaml + rule-targets/** configs are deleted.
- The rule-api, rule-cli, and rule-mcp crates are kept in the workspace, but rule-mcp is unwired from MCP config.
- 27 rendered documentation files (submodule README.md/AGENTS.md, CLI/MCP/HTTP READMEs, spec bodies and sections in 4 spec stores) are frozen: generation markers stripped, content kept.
- .clinerules/** (4 files) is deleted, along with the AGENTS.md reference to it as the Cline adapter surface.
- session-api is changed so a missing rule store degrades gracefully instead of failing session instruction rendering.
- Orphaned rule content (rule bc3758b9) is preserved into a hand-maintained instruction file before deletion.
Rationale: the rule system is not in use; hand-maintained files are the source of truth.
