## Objective
Test `subgraph` and `topgraph` behavior on CLI and MCP.

## Context
Owner decision: “declare exact flags plus BFS semantics — depth, cycle handling, empty-result behavior. Cycle handling MUST be pinned; the ticket graph can contain cycles.”

## Acceptance criteria
- Cover depth, cycles, and empty results on both transports.
- Assert pinned BFS semantics.

## Out of scope
- Graph algorithm redesign.