Implement the canonical `entry × view` navigation contract for `spec-viewer`.

Parent design ticket: `521e18b7-bed4-4588-886c-e25d6c8ddc8b`
Design note: `agents/designs/20260510_DESIGN_SPEC_VIEWER_ENTRY_VIEW_NAVIGATION.md`

## Scope

- Define the concrete in-app representation for:
  - collection-level states (`browse`, `graph`)
  - spec-level states (`body`, `sections`, `coderefs`, `health`)
- Implement URL parsing and serialization for the canonical model.
- Normalize legacy or invalid URLs to the nearest valid state.
- Preserve browser history semantics so back/forward restores both `entry` and `view`.
- Keep deep links working for specific spec/view combinations.

## Acceptance

- `/specs` resolves to the canonical browse state.
- `/specs/graph` resolves to the canonical graph state.
- `/specs/:id?view=<tab>` resolves to the matching spec/view state.
- Invalid or legacy URLs normalize deterministically.
- Browser back/forward restores both selected spec and selected view.
- Deep links to a specific spec/view continue to work.

## Notes

- This ticket should establish the routing/state foundation consumed by the browse-page and tab-preservation tickets.