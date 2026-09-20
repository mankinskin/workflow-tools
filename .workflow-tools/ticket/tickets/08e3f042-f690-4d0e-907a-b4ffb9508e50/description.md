# Design: ticket-viewer UX wireframes + interaction spec

## Objective
Define the baseline UX for tree/file/graph views with workspace switching and state styling.

## Layout proposal
- Left pane: workspace switcher + ticket file tree
- Center pane: selected file view (`description.md` default, tab to `ticket.toml`)
- Right pane: dependency hypergraph + filters

## Interaction model
- Selecting a node in graph focuses matching ticket in tree.
- Selecting a ticket in tree highlights graph node and neighbors.
- Filters: state chips, text search, edge kind toggles.
- Workspace switch preserves per-workspace filter/layout state.

## State styling baseline
- `open`: neutral
- `in-progress`: active highlight
- `review`: review accent
- `validating`: validation accent
- `blocked`: warning/error accent
- `done`: subdued/completed

## Scale guardrails
- Virtualized lists and graph viewport culling.
- Cluster collapse controls for dense regions.
- Debounced search/filter updates.

## Wireframe deliverables
- Overview layout
- Ticket detail/file panel flow
- Graph interaction flow
- Workspace switching flow
- Empty/error/loading states

## Draft artifacts
- Wireframes and interaction notes: `assets/design/wireframes-v0.1.md`

## Checklist
- [x] Wireframe set complete
- [x] Interaction spec complete
- [x] State styling mapping approved
- [x] Accessibility baseline notes added
- [x] Handoff note posted to shell/graph/style impl tickets (ref: 02dea1fa, 2772fe5d, b594864a — Wave 2 kickoff pending)
