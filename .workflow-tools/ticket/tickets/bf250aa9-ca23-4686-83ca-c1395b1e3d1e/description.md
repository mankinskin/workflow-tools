Add browser regression coverage for the new spec-viewer reachable page graph and `entry × view` navigation semantics.

Parent design ticket: `521e18b7-bed4-4588-886c-e25d6c8ddc8b`
Design note: `agents/designs/20260510_DESIGN_SPEC_VIEWER_ENTRY_VIEW_NAVIGATION.md`

## Scope

- Add or update browser tests that verify click-through reachability.
- Add or update browser tests that verify preserving the selected view while switching specs.
- Add or update browser tests that verify browser history restores both entry and view.
- Add or update browser tests that verify deep links still work.
- Add a regression assertion for legacy `/specs/tree` normalization.

## Acceptance

- Automated browser tests cover:
  - `/specs` → graph → detail click-through navigation
  - preserving a non-default tab while switching specs
  - browser back/forward restoring entry + view
  - deep links for specific spec/view states
  - legacy tree-route handling

## Notes

- This ticket should land after the navigation model and behavior tickets settle enough to produce stable assertions.