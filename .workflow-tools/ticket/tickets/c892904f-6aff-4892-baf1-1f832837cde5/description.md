High-priority roadmap to improve observability of the internal stores through the ticket-viewer and shared viewer-api. Consolidates and refines fragmented existing planning into one tracked program.

## Scope (workstreams)
1. W1 — Consolidate the right details panel into the floating content panel (compact header, inline edit/action buttons, metadata footer). Remove the duplicate details panel.
2. W2 — Render only a bounded neighbourhood subgraph around the selection; cache the full workspace graph client-side and re-derive the focused view from cache (no per-selection refetch).
3. W3 — Fix FileTree left-panel formatting and resize/render performance (resize lag, per-frame relayout thrash).
4. W4 — List immediate parents/children in the content panel as clickable neighbours that retarget selection; graph updates from cache.
5. W5a — Minimal viewer-template bootstrap (entity tree explorer, tabbed center main view, right + bottom panels, floating panels) for new viewers (rule-viewer, audit-viewer).
6. W5b — Complete demo-viewer exercising all viewer components with arbitrary data and a root-level domain selector.
7. W6 — Optimize the viewer-api Graph3D renderer for performance and future extensibility across contexts.
8. W7 — Enumerate discovered descendant stores in list_workspaces so viewers can offer a domain/workspace selector (discovery-vs-enumeration gap).

## Priority ordering
P0: W1, W2, W3 (direct daily-use observability pain).
P1: W4 (depends on W1+W2), W6 (enables W2 at scale).
P2: W5a → W5b (platform generalization), W7 (enables demo-viewer domain selection).

## Execution hygiene gates (mandatory for every workstream)
- Ownership parity gate: board ownership must include every modified file path before implementation continues, including submodule paths such as viewer-api.
- Validation evidence gate: each W-ticket transition to in-review must reference at least one recorded `.test/default/executions/*.json` artifact linked to the corresponding validation spec id.
- Command discipline gate: one bounded status snapshot at phase start and one final confirmation snapshot; avoid repeated no-op status polling.
- Deterministic terminal gate: run sync commands with explicit completion outcomes and avoid ambiguous background/sync interpretation in session logs.
- Commit scoping gate: stage only roadmap-owned files plus required submodule pointer updates.

## Existing planning reconciled
Specs: ticket-viewer/detail-document-and-focused-graph (8c4d51ef), ticket-viewer/graph-focus-property-rendering-and-2d-presentation (98b4f75d), viewer-api/components/graph3d (4f14356f), viewer-api/components/tree-view (a20a0395), viewer-api/demo-viewer (4c3b62b4), viewer-api/components/layout (b3362691). New spec needed: viewer-api/viewer-template.
In-flight tickets to fold in / re-scope: 6e7a15c9 (keep full graph visible — superseded by W2's bounded-neighbourhood direction), 10c94251 (graph focus + 2D follow-up), 322ba030 (multi-level node detail), f9e9aaae (property-based node tiers), 2b3a6e2e (TicketDetail theme colors — obsoleted by W1 removal), b779c650 (demo-viewer scaffold), 92964ada (extract viewer-theme/widgets), d1e4ab96 (converge shared shells).

Each workstream ticket carries acceptance criteria; this tracker depends_on all workstreams.