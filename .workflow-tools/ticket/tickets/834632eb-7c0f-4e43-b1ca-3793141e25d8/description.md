Adopt extended error envelope in CLI and MCP surfaces.

Scope:
- apply schema to rule/spec/ticket CLI outputs
- apply schema to rule/spec/ticket MCP error mappings
- preserve backwards compatibility where required

Acceptance criteria:
- machine outputs include required fields for CLI and MCP
- focused tests cover corruption and scan/index failures
