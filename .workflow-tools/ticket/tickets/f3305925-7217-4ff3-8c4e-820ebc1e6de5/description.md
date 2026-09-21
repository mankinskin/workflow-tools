Plan a reusable graph-rendering primitive for ticket and related CLI surfaces, including ASCII and Mermaid outputs and closure-aware expansion over dependency subgraphs.

Scope:
- generic DAG renderer that can draw arbitrary nodes and directed edges
- ticket graph command that expands the requested set to include all transitive bridge nodes
- human and machine outputs that can be embedded in tickets/specs or exported to docs

Acceptance criteria:
- spec defines closure semantics, output formats, and explainability requirements
- plan identifies reusable library ownership and transport responsibilities
- validation plan covers deterministic ordering and render stability
