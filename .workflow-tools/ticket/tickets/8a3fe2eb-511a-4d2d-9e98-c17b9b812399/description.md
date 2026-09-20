Fold the old tree route into the root browse page and make the primary page graph reachable by clicks only.

Parent design ticket: `521e18b7-bed4-4588-886c-e25d6c8ddc8b`
Design note: `agents/designs/20260510_DESIGN_SPEC_VIEWER_ENTRY_VIEW_NAVIGATION.md`

## Scope

- Make `/specs` the single root browse surface for the app.
- Absorb the current tree-browsing role into `/specs`.
- Remove `/specs/tree` as a primary UX destination.
- Add or refine click paths so users can reach:
  - `/specs`
  - `/specs/graph`
  - `/specs/:id?...`
  without manual URL editing.
- Define and implement legacy `/specs/tree` handling.

## Acceptance

- Starting at `/specs`, users can reach the graph view and a specific spec detail state entirely by clicking through the UI.
- The root browse page exposes the former tree-browsing functionality.
- `/specs/tree` is redirected or normalized according to the design.
- No primary spec-viewer page is URL-only.

## Notes

- This ticket is about page-graph reachability and browse-surface consolidation, not tab-preservation logic.