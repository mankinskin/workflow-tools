## Objective
Replace hard-coded legacy ticket-type choices with catalog-driven client behavior.

## Requirements
- Update ticket CLI creation defaults and type selection using the resolved registry catalog.
- Update the VS Code ticket extension picker, commands, and validation to consume catalog-provided types.
- Surface lifecycle transition guidance from the resolved lifecycle graph.
- Reload cache versions coherently after atomic registry generation changes.

## Acceptance Criteria
CLI, MCP, and HTTP behavior agree on types and transitions. VS Code browser/client coverage proves the updated type picker and transition workflow. No client may silently select an unavailable schema type.


## Legacy-List Removal Contract
Inventory every hard-coded legacy type-list call site. Delete each list or replace each list with a named catalog compatibility adapter. Regression tests must prove that a catalog change alters CLI and VS Code selection without source-list drift.