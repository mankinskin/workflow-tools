## Objective
Test output defaults and explicit selectors across CLI, MCP, and HTTP.

## Context
Owner decisions: “TOON by default. Stable field presence, NO byte cap.” and “HTTP defaults to JSON; MCP defaults to TOON. Explicit selector on BOTH transports (Accept header / request field).”

## Acceptance criteria
- Cover CLI TOON, MCP TOON, HTTP JSON defaults.
- Cover both selectors and invalid-selector error.
- Validate semantics and valid TOON.

## Out of scope
- New output formats.