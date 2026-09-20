Preserve the active spec view while switching between specs, using deterministic per-spec fallback when needed.

Parent design ticket: `521e18b7-bed4-4588-886c-e25d6c8ddc8b`
Design note: `agents/designs/20260510_DESIGN_SPEC_VIEWER_ENTRY_VIEW_NAVIGATION.md`

## Scope

- Introduce the navigation behavior that treats spec selection as changing `entry` while preserving `view` when possible.
- Track a global active spec view.
- Track last active view per spec.
- Apply the fallback order from the design:
  1. preserve the current global view if valid for the destination
  2. otherwise use the destination spec's remembered last view
  3. otherwise fall back to `body`
- Ensure list, detail, and graph entry selection all use the same rule.

## Acceptance

- If the user is viewing `Sections` on spec `A` and selects spec `B`, spec `B` opens on `Sections` when valid.
- If the current global view is unavailable for the destination, the destination opens on its remembered last valid view.
- If neither applies, the destination opens on `Body`.
- The resulting URL reflects the resolved entry/view state.

## Notes

- This is the user-visible behavior change most directly modeled after the current log-viewer `entry + tab` pattern.