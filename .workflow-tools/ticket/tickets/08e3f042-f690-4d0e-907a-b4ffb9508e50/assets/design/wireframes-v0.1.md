# Ticket Viewer Wireframes v0.1

Status: draft for review

## Frame 1: Main tri-pane layout
- Left: workspace picker + tree
- Center: file tabs (`description.md`, `ticket.toml`)
- Right: graph + filters

```text
+------------------+------------------------------+--------------------------+
| Workspace: [v]   | File Tabs: desc | ticket.toml | Filters: state, search  |
| Search tree...   | ---------------------------- | ------------------------ |
| - Ticket A       | Markdown/File view           | Hypergraph canvas        |
| - Ticket B       |                              | (depends_on / blocks)    |
| - Ticket C       |                              |                          |
+------------------+------------------------------+--------------------------+
```

## Frame 2: Node selection flow
- Click graph node -> select ticket in tree and open `description.md`.
- Secondary tab retains `ticket.toml` for details.

## Frame 3: Workspace switch persistence
- Switching workspace restores prior filter/search/layout state.
- Graph viewport and selected node are workspace-scoped state.

## Frame 4: Dense graph controls
- Cluster toggle
- Depth slider
- Edge kind toggle
- Subgraph query controls

## Frame 5: Empty/loading/error states
- Empty workspace: informative placeholder with guidance.
- Stream disconnected: non-blocking banner + reconnect state.
- Auth failure: actionable message with token guidance.

## Accessibility baseline notes
- Keyboard navigation between tree, file tabs, and graph focus targets.
- Non-color-only state encoding (icons/labels in legend).
- Minimum contrast target for all state styles.
